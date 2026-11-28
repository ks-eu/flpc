//! Wire implementations for synchronization requests.

use super::{Encode, EncodeError, fields::encode_frame};
use crate::unit::{RequestSyncMessage, RequestSyncPrivateMessage, RequestSyncUsername};

impl Encode for RequestSyncPrivateMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        if *self.count() == 0 {
            return Err(EncodeError::InvalidCount);
        }
        Ok(12)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (target, dst) = dst.split_at_mut(2);
            target.copy_from_slice(&self.target().to_be_bytes());
            let (timestamp, dst) = dst.split_at_mut(8);
            timestamp.copy_from_slice(&self.timestamp().to_be_bytes());
            dst[0] = *self.count();
        })
    }
}

impl Encode for RequestSyncUsername {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 3, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for RequestSyncMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        if *self.count() == 0 {
            return Err(EncodeError::InvalidCount);
        }
        Ok(10)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (timestamp, count) = dst.split_at_mut(8);
            timestamp.copy_from_slice(&self.timestamp().to_be_bytes());
            count[0] = *self.count();
        })
    }
}
