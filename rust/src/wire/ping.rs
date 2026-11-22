//! Wire implementations for ping units.

use super::{Encode, EncodeError};
use crate::unit::Ping;

impl Encode for Ping {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        *buf.first_mut().ok_or(EncodeError::BufferTooSmall)? = Self::OPCODE;
        Ok(1)
    }
}
