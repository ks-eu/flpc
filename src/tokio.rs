use bytes::Buf;
use bytes::BytesMut;
use tokio_util::codec::{Encoder, Decoder};

use crate::header::{
    Encode,
    Decode,
    Join,
    JoinSuccess,
    JoinFail,
    Message,
    ClientMessage,
    UserJoin,
};

use crate::error::ProtocolError;

pub enum FLPCMsg {
    Join(Join),
    JoinSuccess(JoinSuccess),
    JoinFail(JoinFail),
    Message(Message),
    ClientMessage(ClientMessage),
    UserJoin(UserJoin),
}

pub struct FLPCodec;

impl FLPCodec {
    pub fn new() -> Self {
        Self
    }
}

impl Encoder<FLPCMsg> for FLPCodec {
    type Error = ProtocolError;

    fn encode(
        &mut self,
        item: FLPCMsg,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let bytes = match item {
            FLPCMsg::Join(message) => message.encode()?,
            FLPCMsg::JoinSuccess(message) => message.encode()?,
            FLPCMsg::JoinFail(message) => message.encode()?,
            FLPCMsg::Message(message) => message.encode()?,
            FLPCMsg::ClientMessage(message) => message.encode()?,
            FLPCMsg::UserJoin(message) => message.encode()?,
        };

        dst.extend_from_slice(&bytes);

        Ok(())
    }
}

impl Decoder for FLPCodec {
    type Item = FLPCMsg;
    type Error = ProtocolError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        // Determine the frame length from the message type.
        let frame_length = match src[0] {
            // Join
            0x01 => {
                if src.len() < 2 {
                    return Ok(None);
                }
                2 + src[1] as usize
            }

            // JoinSuccess
            0x02 => 1,

            // JoinFail
            0x03 => 2,

            // Message
            0x04 => {
                if src.len() < 3 {
                    return Ok(None);
                }
                let length = u16::from_be_bytes([src[1], src[2]]);
                3 + length as usize
            }

            // ClientMessage
            0x05 => {
                if src.len() < 5 {
                    return Ok(None);
                }
                let length = u16::from_be_bytes([src[3], src[4]]);
                5 + length as usize
            }

            // UserJoin
            0x06 => {
                if src.len() < 4 {
                    return Ok(None);
                }
                4 + src[3] as usize
            }

            _ => return Err(ProtocolError::UnknownHeader),
        };

        if src.len() < frame_length {
            return Ok(None);
        }

        let frame = &src[..frame_length];

        let message = match frame[0] {
            0x01 => FLPCMsg::Join(Join::decode(Some(frame))?),
            0x02 => FLPCMsg::JoinSuccess(JoinSuccess::decode(Some(frame))?),
            0x03 => FLPCMsg::JoinFail(JoinFail::decode(Some(frame))?),
            0x04 => FLPCMsg::Message(Message::decode(Some(frame))?),
            0x05 => FLPCMsg::ClientMessage(ClientMessage::decode(Some(frame))?),
            0x06 => FLPCMsg::UserJoin(UserJoin::decode(Some(frame))?),
            _ => unreachable!(),
        };

        src.advance(frame_length);

        Ok(Some(message))
    }
}

#[cfg(test)]
#[path = "tests/tokio.rs"]
mod tests;
