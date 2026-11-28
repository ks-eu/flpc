//! Wire implementations for server units.
use super::{
    Encode, EncodeError,
    fields::{
        encode_frame, long_string_len, short_strings_len, write_long_string, write_short_string,
    },
};
use crate::unit::{
    ChangedUsername, ServerChangedPassword, ServerLogin, ServerMessage, ServerPrivateMessage,
    ServerRegister, ServerRemoveSelf, ServerResponse, UserAdmin, UserAdminRemoved, UserJoined,
    UserLeft, UserNotOnline, UserRegistered, UserRemoved, WrongUnit,
};

impl Encode for ServerRemoveSelf {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.result())
    }
}

impl Encode for UserNotOnline {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 1, Self::OPCODE, |_| {})
    }
}

impl Encode for WrongUnit {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |_| {})
    }
}

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

impl Encode for ServerMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(11 + long_string_len(self.message())?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (id, dst) = dst.split_at_mut(2);
            id.copy_from_slice(&self.id().to_be_bytes());
            let (timestamp, dst) = dst.split_at_mut(8);
            timestamp.copy_from_slice(&self.timestamp().to_be_bytes());
            write_long_string(dst, self.message());
        })
    }
}

impl Encode for ServerPrivateMessage {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(13 + long_string_len(self.message())?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (id, dst) = dst.split_at_mut(2);
            id.copy_from_slice(&self.id().to_be_bytes());
            let (target, dst) = dst.split_at_mut(2);
            target.copy_from_slice(&self.target().to_be_bytes());
            let (timestamp, dst) = dst.split_at_mut(8);
            timestamp.copy_from_slice(&self.timestamp().to_be_bytes());
            write_long_string(dst, self.message());
        })
    }
}

impl Encode for UserRegistered {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3 + short_strings_len(&[self.username()])?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (id, mut dst) = dst.split_at_mut(2);
            id.copy_from_slice(&self.id().to_be_bytes());
            write_short_string(&mut dst, self.username());
        })
    }
}

impl Encode for UserRemoved {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for UserJoined {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for UserLeft {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes());
        })
    }
}

impl Encode for ChangedUsername {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3 + short_strings_len(&[self.username()])?)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            let (id, mut dst) = dst.split_at_mut(2);
            id.copy_from_slice(&self.id().to_be_bytes());
            write_short_string(&mut dst, self.username());
        })
    }
}

impl Encode for ServerResponse {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + long_string_len(self.response())?)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |dst| {
            write_long_string(dst, self.response())
        })
    }
}

impl Encode for UserAdmin {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 3, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes())
        })
    }
}

impl Encode for UserAdminRemoved {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 3, Self::OPCODE, |dst| {
            dst.copy_from_slice(&self.id().to_be_bytes())
        })
    }
}

impl Encode for ServerChangedPassword {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(2)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, 2, Self::OPCODE, |dst| dst[0] = *self.outcome())
    }
}
