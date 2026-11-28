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
mod requestsync;
mod server;
mod serveradmin;
mod serversync;

pub use decode::decode;
pub use error::{DecodeError, EncodeError};

use crate::unit::FlpcUnit;

/// Encodes one FLPC unit, including its opcode.
pub trait Encode {
    /// Returns the exact encoded length in bytes.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError::StringTooLong`] if a field exceeds its wire limit,
    /// or [`EncodeError::InvalidCount`] for a zero synchronization request count.
    fn encoded_len(&self) -> Result<usize, EncodeError>;

    /// Writes the unit at the start of `buf` and returns the number of bytes written.
    ///
    /// Bytes after the encoded unit are preserved. The destination is preserved on error.
    ///
    /// # Errors
    ///
    /// Returns an error if a field exceeds its wire limit, `buf` is too small,
    /// or a synchronization request count is zero.
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

    /// Allocates a byte vector containing one complete unit.
    ///
    /// # Errors
    ///
    /// Returns an error if a field exceeds its wire limit or a synchronization
    /// request count is zero.
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
        FlpcUnit::Kick(unit) => unit,
        FlpcUnit::Remove(unit) => unit,
        FlpcUnit::Add(unit) => unit,
        FlpcUnit::MakeAdmin(unit) => unit,
        FlpcUnit::RemoveAdmin(unit) => unit,
        FlpcUnit::Register(unit) => unit,
        FlpcUnit::Login(unit) => unit,
        FlpcUnit::Message(unit) => unit,
        FlpcUnit::PrivateMessage(unit) => unit,
        FlpcUnit::ChangeUsername(unit) => unit,
        FlpcUnit::RequestSyncMessage(unit) => unit,
        FlpcUnit::RemoveSelf(unit) => unit,
        FlpcUnit::RequestSyncUsername(unit) => unit,
        FlpcUnit::RequestSyncPrivateMessage(unit) => unit,
        FlpcUnit::Ping(unit) => unit,
        FlpcUnit::ServerRegister(unit) => unit,
        FlpcUnit::ServerLogin(unit) => unit,
        FlpcUnit::ServerMessage(unit) => unit,
        FlpcUnit::ServerPrivateMessage(unit) => unit,
        FlpcUnit::UserRegistered(unit) => unit,
        FlpcUnit::UserRemoved(unit) => unit,
        FlpcUnit::UserJoined(unit) => unit,
        FlpcUnit::UserLeft(unit) => unit,
        FlpcUnit::ChangedUsername(unit) => unit,
        FlpcUnit::SyncMessage(unit) => unit,
        FlpcUnit::SyncPrivateMessage(unit) => unit,
        FlpcUnit::WrongUnit(unit) => unit,
        FlpcUnit::ServerRemoveSelf(unit) => unit,
        FlpcUnit::SyncEnd(unit) => unit,
        FlpcUnit::UserNotOnline(unit) => unit,
        FlpcUnit::SyncUsername(unit) => unit,
        FlpcUnit::ChangePassword(unit) => unit,
        FlpcUnit::ServerResponse(unit) => unit,
        FlpcUnit::UserAdmin(unit) => unit,
        FlpcUnit::UserAdminRemoved(unit) => unit,
        FlpcUnit::ServerChangedPassword(unit) => unit,
        FlpcUnit::OnboardingEnd(unit) => unit,
        FlpcUnit::ServerMessageFail(unit) => unit,
        FlpcUnit::ServerChangedUsername(unit) => unit,
        FlpcUnit::SyncMessageFail(unit) => unit,
        FlpcUnit::SyncUsernameFail(unit) => unit,
        FlpcUnit::ServerAdd(unit) => unit,
        FlpcUnit::ServerRemove(unit) => unit,
        FlpcUnit::ServerKick(unit) => unit,
        FlpcUnit::ServerMakeAdmin(unit) => unit,
    }
}
