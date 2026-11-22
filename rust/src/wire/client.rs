//! Wire implementations for client units.

use super::{
    Encode, EncodeError,
    fields::{encode_frame, short_strings_len, write_short_string},
};
use crate::unit::{Login, Message, Register};

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
        let len = self.message().len();
        u16::try_from(len).map_err(|_| EncodeError::StringTooLong)?;
        Ok(3 + len)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let payload = self.message().as_bytes();
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (prefix, body) = dst.split_at_mut(2);
            prefix.copy_from_slice(&(payload.len() as u16).to_be_bytes());
            body.copy_from_slice(payload);
        })
    }
}
