//! Wire implementations for server units.
use super::{Encode, EncodeError, fields::encode_frame};
use crate::unit::{ServerLogin, ServerRegister};

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
