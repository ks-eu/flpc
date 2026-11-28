//! Wire implementations for replies to administrator requests.

use super::{Encode, EncodeError, fields::encode_frame};
use crate::unit::{ServerAdd, ServerKick, ServerMakeAdmin, ServerRemove, ServerRemoveAdmin};

impl Encode for ServerAdd {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}

impl Encode for ServerRemove {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}

impl Encode for ServerKick {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}

impl Encode for ServerMakeAdmin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}

impl Encode for ServerRemoveAdmin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}
