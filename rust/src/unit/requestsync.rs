//! Synchronization requests sent to the server.

use super::FlpcUnit;

/// Requests a batch of messages whose timestamps are strictly before the
/// supplied timestamp (opcode `0x06`). The timestamp is an unsigned 64-bit
/// big-endian value and the count must be nonzero. A zero count returns
/// [`crate::wire::EncodeError::InvalidCount`] when encoding and
/// [`crate::wire::DecodeError::InvalidCount`] when decoding.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, RequestSyncMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = RequestSyncMessage::new(0x0102_0304_0506_0708, 255);
/// let bytes = [0x06, 1, 2, 3, 4, 5, 6, 7, 8, 0xff];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(wrapped, FlpcUnit::RequestSyncMessage(
///     RequestSyncMessage::new(0x0102_0304_0506_0708, 255),
/// ));
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestSyncMessage {
    timestamp: u64,
    count: u8,
}

impl RequestSyncMessage {
    pub(crate) const OPCODE: u8 = 0x06;

    pub fn new(timestamp: u64, count: u8) -> Self {
        Self { timestamp, count }
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn count(&self) -> &u8 {
        &self.count
    }
}

impl From<RequestSyncMessage> for FlpcUnit {
    fn from(unit: RequestSyncMessage) -> Self {
        Self::RequestSyncMessage(unit)
    }
}

/// A client request for a user's current username during synchronization
/// (opcode `0x08`). The user ID is an unsigned 16-bit big-endian integer.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, RequestSyncUsername},
///     wire::{decode, Encode},
/// };
///
/// let unit = RequestSyncUsername::new(0x1234);
/// let bytes = [0x08, 0x12, 0x34];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(wrapped, FlpcUnit::RequestSyncUsername(
///     RequestSyncUsername::new(0x1234),
/// ));
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestSyncUsername {
    id: u16,
}

impl RequestSyncUsername {
    pub(crate) const OPCODE: u8 = 0x08;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<RequestSyncUsername> for FlpcUnit {
    fn from(unit: RequestSyncUsername) -> Self {
        Self::RequestSyncUsername(unit)
    }
}

/// A client request for private messages addressed to a user (opcode `0x09`).
/// The target ID is an unsigned 16-bit big-endian integer, followed by the
/// timestamp of the newest message to include as an unsigned 64-bit big-endian
/// integer and a nonzero message count. Zero counts return
/// [`crate::wire::EncodeError::InvalidCount`] when encoding and
/// [`crate::wire::DecodeError::InvalidCount`] when decoding.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, RequestSyncPrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = RequestSyncPrivateMessage::new(0x1234, 0x0102_0304_0506_0708, 1);
/// let bytes = [0x09, 0x12, 0x34, 1, 2, 3, 4, 5, 6, 7, 8, 1];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(wrapped, FlpcUnit::RequestSyncPrivateMessage(
///     RequestSyncPrivateMessage::new(0x1234, 0x0102_0304_0506_0708, 1),
/// ));
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestSyncPrivateMessage {
    target: u16,
    timestamp: u64,
    count: u8,
}

impl RequestSyncPrivateMessage {
    pub(crate) const OPCODE: u8 = 0x09;

    pub fn new(target: u16, timestamp: u64, count: u8) -> Self {
        Self {
            target,
            timestamp,
            count,
        }
    }

    pub fn target(&self) -> &u16 {
        &self.target
    }
    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }
    pub fn count(&self) -> &u8 {
        &self.count
    }
}

impl From<RequestSyncPrivateMessage> for FlpcUnit {
    fn from(unit: RequestSyncPrivateMessage) -> Self {
        Self::RequestSyncPrivateMessage(unit)
    }
}
