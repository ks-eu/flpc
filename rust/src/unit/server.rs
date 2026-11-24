//! Units for servers.

use super::FlpcUnit;

/// Responses, notifications, and messages sent by the server.
///
/// The codec preserves timestamps as unsigned 64-bit values and outcomes as
/// single bytes. Peers agree on the timestamp epoch and precision and on the
/// meaning of outcome codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerUnit {
    ServerRegister(ServerRegister),
    ServerLogin(ServerLogin),
    ServerMessage(ServerMessage),
    ServerPrivateMessage(ServerPrivateMessage),
    UserRegistered(UserRegistered),
    UserRemoved(UserRemoved),
    UserJoined(UserJoined),
    UserLeft(UserLeft),
    ChangedUsername(ChangedUsername),
    SyncMessage(SyncMessage),
    SyncPrivateMessage(SyncPrivateMessage),
    WrongUnit(WrongUnit),
}

/// Registration outcome from the server (opcode `0x80`).
///
/// The codec preserves the outcome byte. Peers agree on what each value means.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, ServerRegister},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerRegister::new(1);
/// let bytes: &[u8] = &[0x80, 0x01];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::ServerRegister(unit)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRegister {
    outcome: u8,
}

impl ServerRegister {
    pub(crate) const OPCODE: u8 = 0x80;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerRegister> for FlpcUnit {
    fn from(unit: ServerRegister) -> Self {
        Self::Server(ServerUnit::ServerRegister(unit))
    }
}

/// Login outcome from the server (opcode `0x81`).
///
/// The codec preserves the outcome byte. Peers agree on what each value means.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerLogin},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerLogin::new(1);
/// let bytes: &[u8] = &[0x81, 0x01];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerLogin {
    outcome: u8,
}

impl ServerLogin {
    pub(crate) const OPCODE: u8 = 0x81;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerLogin> for FlpcUnit {
    fn from(unit: ServerLogin) -> Self {
        Self::Server(ServerUnit::ServerLogin(unit))
    }
}

/// A chat message from a user (opcode `0x82`).
///
/// The sender ID and timestamp are big-endian. The UTF-8 message has a
/// two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerMessage::new(7, 42, "hi".into());
/// let bytes: &[u8] = &[0x82, 0x00, 0x07, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

impl ServerMessage {
    pub(crate) const OPCODE: u8 = 0x82;

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

impl From<ServerMessage> for FlpcUnit {
    fn from(unit: ServerMessage) -> Self {
        Self::Server(ServerUnit::ServerMessage(unit))
    }
}

/// A private chat message from a user (opcode `0x83`).
///
/// The sender ID, target ID, and timestamp are big-endian. The UTF-8 message
/// has a two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerPrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerPrivateMessage::new(7, 8, 42, "hi".into());
/// let bytes: &[u8] = &[0x83, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerPrivateMessage {
    id: u16,
    target: u16,
    timestamp: u64,
    message: String,
}

impl ServerPrivateMessage {
    pub(crate) const OPCODE: u8 = 0x83;

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

impl From<ServerPrivateMessage> for FlpcUnit {
    fn from(unit: ServerPrivateMessage) -> Self {
        Self::Server(ServerUnit::ServerPrivateMessage(unit))
    }
}

/// A notification that a user registered (opcode `0x84`).
///
/// The user ID is big-endian. The UTF-8 username has a one-byte length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, UserRegistered},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserRegistered::new(7, "é".into());
/// let bytes: &[u8] = &[0x84, 0, 7, 2, 0xc3, 0xa9];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRegistered {
    id: u16,
    username: String,
}

impl UserRegistered {
    pub(crate) const OPCODE: u8 = 0x84;

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

impl From<UserRegistered> for FlpcUnit {
    fn from(unit: UserRegistered) -> Self {
        Self::Server(ServerUnit::UserRegistered(unit))
    }
}

/// A notification that a user was removed (opcode `0x85`).
///
/// The user ID is big-endian.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, UserRemoved},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserRemoved::new(7);
/// let bytes: &[u8] = &[0x85, 0, 7];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRemoved {
    id: u16,
}

impl UserRemoved {
    pub(crate) const OPCODE: u8 = 0x85;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<UserRemoved> for FlpcUnit {
    fn from(unit: UserRemoved) -> Self {
        Self::Server(ServerUnit::UserRemoved(unit))
    }
}

/// A notification that a user joined (opcode `0x86`).
///
/// The user ID is big-endian.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, UserJoined},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserJoined::new(7);
/// let bytes: &[u8] = &[0x86, 0, 7];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::UserJoined(unit)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserJoined {
    id: u16,
}

impl UserJoined {
    pub(crate) const OPCODE: u8 = 0x86;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<UserJoined> for FlpcUnit {
    fn from(unit: UserJoined) -> Self {
        Self::Server(ServerUnit::UserJoined(unit))
    }
}

/// A notification that a user left (opcode `0x87`).
///
/// The user ID is big-endian.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, UserLeft},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserLeft::new(7);
/// let bytes: &[u8] = &[0x87, 0, 7];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::UserLeft(unit)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserLeft {
    id: u16,
}

impl UserLeft {
    pub(crate) const OPCODE: u8 = 0x87;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

/// A notification that a username changed (opcode `0x88`).
///
/// The user ID is big-endian. The UTF-8 username has a one-byte length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, ChangedUsername},
///     wire::{decode, Encode},
/// };
///
/// let unit = ChangedUsername::new(7, "é".into());
/// let bytes: &[u8] = &[0x88, 0, 7, 2, 0xc3, 0xa9];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::ChangedUsername(unit)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedUsername {
    id: u16,
    username: String,
}

impl ChangedUsername {
    pub(crate) const OPCODE: u8 = 0x88;

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

/// A chat message returned during synchronization (opcode `0x89`).
///
/// The sender ID and timestamp are big-endian. The UTF-8 message has a
/// two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, SyncMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncMessage::new(7, 42, "hi".into());
/// let bytes: &[u8] = &[0x89, 0, 7, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::SyncMessage(unit)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

impl SyncMessage {
    pub(crate) const OPCODE: u8 = 0x89;

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

/// A private chat message returned during synchronization (opcode `0x8A`).
///
/// The sender ID, target ID, and timestamp are big-endian. The UTF-8 message
/// has a two-byte big-endian length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, SyncPrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let unit = SyncPrivateMessage::new(7, 8, 42, "hi".into());
/// let bytes: &[u8] = &[0x8A, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::Server(ServerUnit::SyncPrivateMessage(unit)),
///         bytes.len(),
///     )),
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
    pub(crate) const OPCODE: u8 = 0x8A;

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

/// A server response indicating that a client's first unit was unexpected (opcode `0x8B`).
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerUnit, WrongUnit},
///     wire::{decode, Encode},
/// };
///
/// let unit = WrongUnit::new();
/// let bytes: &[u8] = &[0x8B];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((FlpcUnit::Server(ServerUnit::WrongUnit(unit)), bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrongUnit {}

impl WrongUnit {
    pub(crate) const OPCODE: u8 = 0x8B;

    pub fn new() -> Self {
        Self {}
    }
}
