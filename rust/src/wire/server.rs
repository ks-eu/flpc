//! Wire implementations for server units.
use super::{Encode, EncodeError, fields::encode_frame};
use crate::unit::ServerRegister;

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
