//! Wire implementations for client units.

use super::{Encode, EncodeError};
use crate::unit::Register;

impl Encode for Register {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        let mut len = 1;
        for field in [self.token(), self.username(), self.password()] {
            if field.len() > u8::MAX as usize {
                return Err(EncodeError::StringTooLong);
            }
            len += 1 + field.len();
        }
        Ok(len)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let len = self.encoded_len()?;
        let dst = buf.get_mut(..len).ok_or(EncodeError::BufferTooSmall)?;
        dst[0] = Self::OPCODE;
        let mut offset = 1;
        for field in [self.token(), self.username(), self.password()] {
            dst[offset] = field.len() as u8;
            offset += 1;
            dst[offset..offset + field.len()].copy_from_slice(field.as_bytes());
            offset += field.len();
        }
        Ok(len)
    }
}
