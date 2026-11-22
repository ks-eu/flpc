//! Wire implementations for server units.
use super::{
    Encode, EncodeError,
    fields::{encode_frame, long_string_len, write_long_string},
};
use crate::unit::{ServerLogin, ServerMessage, ServerRegister};

impl Encode for ServerRegister {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst[0] = *self.outcome();
        })
    }
}

impl Encode for ServerLogin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst[0] = *self.outcome();
        })
    }
}

impl Encode for ServerMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(11 + long_string_len(self.message())?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (id, dst) = dst.split_at_mut(2);
            id.copy_from_slice(&self.id().to_be_bytes());
            let (timestamp, dst) = dst.split_at_mut(8);
            timestamp.copy_from_slice(&self.timestamp().to_be_bytes());
            write_long_string(dst, self.message());
        })
    }
}
