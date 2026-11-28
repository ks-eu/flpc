//! Synchronization replies sent by the server.

use super::FlpcUnit;

/// A chat message returned during synchronization (opcode `0xC0`).
///
/// The sender ID and timestamp are big-endian. The UTF-8 message has a
/// two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, SyncMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncMessage::new(7, 42, "hi".into());
/// let bytes: &[u8] = &[0xC0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

impl SyncMessage {
    pub(crate) const OPCODE: u8 = 0xC0;

    pub fn new(id: u16, timestamp: u64, message: String) -> Self {
        Self {
            id,
            timestamp,
            message,
        }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }

    /// Returns the raw timestamp. Peers agree on its epoch and precision.
    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<SyncMessage> for FlpcUnit {
    fn from(unit: SyncMessage) -> Self {
        Self::SyncMessage(unit)
    }
}

/// A private chat message returned during synchronization (opcode `0xC1`).
///
/// The sender ID, target ID, and timestamp are big-endian. The UTF-8 message
/// has a two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, SyncPrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncPrivateMessage::new(7, 8, 42, "hi".into());
/// let bytes: &[u8] = &[0xC1, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPrivateMessage {
    id: u16,
    target: u16,
    timestamp: u64,
    message: String,
}

impl SyncPrivateMessage {
    pub(crate) const OPCODE: u8 = 0xC1;

    pub fn new(id: u16, target: u16, timestamp: u64, message: String) -> Self {
        Self {
            id,
            target,
            timestamp,
            message,
        }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }

    pub fn target(&self) -> &u16 {
        &self.target
    }

    /// Returns the raw timestamp. Peers agree on its epoch and precision.
    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<SyncPrivateMessage> for FlpcUnit {
    fn from(unit: SyncPrivateMessage) -> Self {
        Self::SyncPrivateMessage(unit)
    }
}

/// Marks the end of a synchronization response and reports its item count
/// (opcode `0xC3`). A zero count is valid.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, SyncEnd},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncEnd::new(1);
/// let bytes = [0xc3, 1];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncEnd {
    count: u8,
}

impl SyncEnd {
    pub(crate) const OPCODE: u8 = 0xC3;

    pub fn new(count: u8) -> Self {
        Self { count }
    }
    pub fn count(&self) -> &u8 {
        &self.count
    }
}

impl From<SyncEnd> for FlpcUnit {
    fn from(unit: SyncEnd) -> Self {
        Self::SyncEnd(unit)
    }
}

/// Sends a user's ID and username during synchronization (opcode `0xC2`). The
/// ID is an unsigned 16-bit big-endian integer. The username is UTF-8 with a
/// one-byte length prefix counting UTF-8 bytes, up to 255 bytes.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, SyncUsername},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncUsername::new(0x1234, "é".into());
/// let bytes = [0xc2, 0x12, 0x34, 2, 0xc3, 0xa9];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncUsername {
    id: u16,
    username: String,
}

impl SyncUsername {
    pub(crate) const OPCODE: u8 = 0xC2;

    pub fn new(id: u16, username: String) -> Self {
        Self { id, username }
    }
    pub fn id(&self) -> &u16 {
        &self.id
    }
    pub fn username(&self) -> &String {
        &self.username
    }
}

impl From<SyncUsername> for FlpcUnit {
    fn from(unit: SyncUsername) -> Self {
        Self::SyncUsername(unit)
    }
}
