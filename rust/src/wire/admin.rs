//! Wire implementations for admin units.

use super::{
    Encode, EncodeError,
    fields::{encode_frame, short_strings_len, write_short_string},
};
use crate::unit::{Add, MakeAdmin, Remove, RemoveAdmin};

impl Encode for Add {
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

impl Encode for MakeAdmin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for RemoveAdmin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for Remove {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}
