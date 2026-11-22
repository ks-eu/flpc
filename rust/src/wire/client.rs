//! Wire implementations for client units.

use super::{
    Encode, EncodeError,
    fields::{encode_frame, short_strings_len, write_short_string},
};
use crate::unit::{Login, Register};

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
