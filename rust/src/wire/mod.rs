//! Byte encoding and decoding for FLPC units.
//!
//! Units begin with a one-byte opcode. [`decode()`] reads at most one unit
//! from the start of a byte slice and reports how many bytes it used.

mod admin;
mod client;
mod decode;
mod error;
mod fields;
mod ping;
mod server;

pub use decode::decode;
pub use error::{DecodeError, EncodeError};

use crate::unit::{AdminUnit, ClientUnit, FlpcUnit, PingUnit, ServerUnit};

/// Encodes one FLPC unit, including its opcode.
pub trait Encode {
    /// Returns the exact encoded length in bytes.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError::StringTooLong`] if a field exceeds its wire limit.
    fn encoded_len(&self) -> Result<usize, EncodeError>;

    /// Writes the unit at the start of `buf` and returns the number of bytes written.
    ///
    /// Bytes after the encoded unit are preserved. The destination is preserved on error.
    ///
    /// # Errors
    ///
    /// Returns an error if a field exceeds its wire limit or `buf` is too small.
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

    /// Allocates a byte vector containing one complete unit.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError::StringTooLong`] if a field exceeds its wire limit.
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = vec![0; self.encoded_len()?];
        self.encode_into(&mut bytes)?;
        Ok(bytes)
    }
}

impl Encode for FlpcUnit {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        concrete_unit(self).encoded_len()
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        concrete_unit(self).encode_into(buf)
    }
}

fn concrete_unit(unit: &FlpcUnit) -> &dyn Encode {
    match unit {
        FlpcUnit::Admin(unit) => match unit {
            AdminUnit::Kick(unit) => unit,
            AdminUnit::Remove(unit) => unit,
            AdminUnit::Add(unit) => unit,
            AdminUnit::MakeAdmin(unit) => unit,
            AdminUnit::RemoveAdmin(unit) => unit,
        },
        FlpcUnit::Client(unit) => match unit {
            ClientUnit::Register(unit) => unit,
            ClientUnit::Login(unit) => unit,
            ClientUnit::Message(unit) => unit,
            ClientUnit::PrivateMessage(unit) => unit,
            ClientUnit::ChangeUsername(unit) => unit,
            ClientUnit::RequestSync(unit) => unit,
        },
        FlpcUnit::Ping(unit) => match unit {
            PingUnit::Ping(unit) => unit,
        },
        FlpcUnit::Server(unit) => match unit {
            ServerUnit::ServerRegister(unit) => unit,
            ServerUnit::ServerLogin(unit) => unit,
            ServerUnit::ServerMessage(unit) => unit,
            ServerUnit::ServerPrivateMessage(unit) => unit,
            ServerUnit::UserRegistered(unit) => unit,
            ServerUnit::UserRemoved(unit) => unit,
            ServerUnit::UserJoined(unit) => unit,
            ServerUnit::UserLeft(unit) => unit,
            ServerUnit::ChangedUsername(unit) => unit,
            ServerUnit::SyncMessage(unit) => unit,
            ServerUnit::SyncPrivateMessage(unit) => unit,
            ServerUnit::WrongUnit(unit) => unit,
        },
    }
}
