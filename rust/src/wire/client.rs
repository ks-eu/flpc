//! Wire implementations for client units.

use super::{
    Encode, EncodeError,
    fields::{encode_frame, short_strings_len, write_short_string},
};
use crate::unit::{Login, Message, PrivateMessage, Register};

impl Encode for Register {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + short_strings_len(&[self.token(), self.username(), self.password()])?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |mut dst| {
            write_short_string(&mut dst, self.token());
            write_short_string(&mut dst, self.username());
            write_short_string(&mut dst, self.password());
        })
    }
}

impl Encode for Login {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + short_strings_len(&[self.username(), self.password()])?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |mut dst| {
            write_short_string(&mut dst, self.username());
            write_short_string(&mut dst, self.password());
        })
    }
}

impl Encode for Message {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + long_string_len(self.message())?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            write_long_string(dst, self.message());
        })
    }
}

impl Encode for PrivateMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3 + long_string_len(self.message())?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (target, dst) = dst.split_at_mut(2);
            target.copy_from_slice(&self.target().to_be_bytes());
            write_long_string(dst, self.message());
        })
    }
}

fn long_string_len(value: &str) -> Result<usize, EncodeError> {
    u16::try_from(value.len()).map_err(|_| EncodeError::StringTooLong)?;
    Ok(2 + value.len())
}

fn write_long_string(dst: &mut [u8], value: &str) {
    let (prefix, body) = dst.split_at_mut(2);
    prefix.copy_from_slice(&(value.len() as u16).to_be_bytes());
    body.copy_from_slice(value.as_bytes());
}
