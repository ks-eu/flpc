//! Wire implementations for client units.

use super::{
    Encode, EncodeError,
    fields::{
        encode_frame, long_string_len, short_strings_len, write_long_string, write_short_string,
    },
};
use crate::unit::{
    ChangePassword, ChangeUsername, Login, Message, PrivateMessage, Register, RemoveSelf,
};

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

impl Encode for ChangePassword {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(1 + short_strings_len(&[self.username(), self.password(), self.new_password()])?)
    }
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        encode_frame(buf, self.encoded_len()?, Self::OPCODE, |mut dst| {
            write_short_string(&mut dst, self.username());
            write_short_string(&mut dst, self.password());
            write_short_string(&mut dst, self.new_password());
        })
    }
}
