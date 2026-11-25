//! Wire implementations for client units.

use super::{
    Encode, EncodeError,
    fields::{
        encode_frame, long_string_len, short_strings_len, write_long_string, write_short_string,
    },
};
use crate::unit::{
    ChangeUsername, Login, Message, PrivateMessage, Register, RemoveSelf, RequestSyncMessage,
    RequestSyncPrivateMessage, RequestSyncUsername,
};

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

impl Encode for RemoveSelf {
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

impl Encode for ChangeUsername {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + short_strings_len(&[self.username()])?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |mut dst| {
            write_short_string(&mut dst, self.username());
        })
    }
}

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
