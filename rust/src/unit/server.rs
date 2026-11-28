//! Units for servers.

use super::FlpcUnit;

/// Registration outcome from the server (opcode `0x90`).
///
/// The codec preserves the outcome byte. Peers agree on what each value means.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerRegister},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerRegister::new(1);
/// let bytes: &[u8] = &[0x90, 0x01];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((
///         FlpcUnit::ServerRegister(unit),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRegister {
    outcome: u8,
}

impl ServerRegister {
    pub(crate) const OPCODE: u8 = 0x90;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerRegister> for FlpcUnit {
    fn from(unit: ServerRegister) -> Self {
        Self::ServerRegister(unit)
    }
}

/// Login outcome from the server (opcode `0x92`).
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
/// let bytes: &[u8] = &[0x92, 0x01];
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
    pub(crate) const OPCODE: u8 = 0x92;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerLogin> for FlpcUnit {
    fn from(unit: ServerLogin) -> Self {
        Self::ServerLogin(unit)
    }
}

/// A chat message from a user (opcode `0xA0`).
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
/// let bytes: &[u8] = &[0xA0, 0x00, 0x07, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
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
    pub(crate) const OPCODE: u8 = 0xA0;

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
        Self::ServerMessage(unit)
    }
}

/// A private chat message from a user (opcode `0xA1`).
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
/// let bytes: &[u8] = &[0xA1, 0, 7, 0, 8, 0, 0, 0, 0, 0, 0, 0, 42, 0, 2, b'h', b'i'];
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
    pub(crate) const OPCODE: u8 = 0xA1;

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
        Self::ServerPrivateMessage(unit)
    }
}

/// A notification that a user registered (opcode `0x81`).
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
/// let bytes: &[u8] = &[0x81, 0, 7, 2, 0xc3, 0xa9];
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
    pub(crate) const OPCODE: u8 = 0x81;

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
        Self::UserRegistered(unit)
    }
}

/// A notification that a user was removed (opcode `0x82`).
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
/// let bytes: &[u8] = &[0x82, 0, 7];
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
    pub(crate) const OPCODE: u8 = 0x82;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<UserRemoved> for FlpcUnit {
    fn from(unit: UserRemoved) -> Self {
        Self::UserRemoved(unit)
    }
}

/// A notification that a user joined (opcode `0x83`).
///
/// The user ID is big-endian.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, UserJoined},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserJoined::new(7);
/// let bytes: &[u8] = &[0x83, 0, 7];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserJoined {
    id: u16,
}

impl UserJoined {
    pub(crate) const OPCODE: u8 = 0x83;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<UserJoined> for FlpcUnit {
    fn from(unit: UserJoined) -> Self {
        Self::UserJoined(unit)
    }
}

/// A notification that a user left (opcode `0x84`).
///
/// The user ID is big-endian.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, UserLeft},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserLeft::new(7);
/// let bytes: &[u8] = &[0x84, 0, 7];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserLeft {
    id: u16,
}

impl UserLeft {
    pub(crate) const OPCODE: u8 = 0x84;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<UserLeft> for FlpcUnit {
    fn from(unit: UserLeft) -> Self {
        Self::UserLeft(unit)
    }
}

/// A notification that a username changed (opcode `0x85`).
///
/// The user ID is big-endian. The UTF-8 username has a one-byte length prefix.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ChangedUsername},
///     wire::{decode, Encode},
/// };
///
/// let unit = ChangedUsername::new(7, "é".into());
/// let bytes: &[u8] = &[0x85, 0, 7, 2, 0xc3, 0xa9];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedUsername {
    id: u16,
    username: String,
}

impl ChangedUsername {
    pub(crate) const OPCODE: u8 = 0x85;

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

impl From<ChangedUsername> for FlpcUnit {
    fn from(unit: ChangedUsername) -> Self {
        Self::ChangedUsername(unit)
    }
}

/// A server response indicating that a client's first unit was unexpected (opcode `0x94`).
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, WrongUnit},
///     wire::{decode, Encode},
/// };
///
/// let unit = WrongUnit::new();
/// let bytes: &[u8] = &[0x94];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrongUnit {}

impl WrongUnit {
    pub(crate) const OPCODE: u8 = 0x94;

    pub fn new() -> Self {
        Self {}
    }
}

impl From<WrongUnit> for FlpcUnit {
    fn from(unit: WrongUnit) -> Self {
        Self::WrongUnit(unit)
    }
}

/// A raw result byte returned for a remove-self request (opcode `0x91`). The
/// codec preserves the byte without assigning meaning to its values.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, ServerRemoveSelf},
///     wire::{decode, Encode},
/// };
///
/// let unit = ServerRemoveSelf::new(0xa5);
/// let bytes = [0x91, 0xa5];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRemoveSelf {
    result: u8,
}

impl ServerRemoveSelf {
    pub(crate) const OPCODE: u8 = 0x91;

    pub fn new(result: u8) -> Self {
        Self { result }
    }
    pub fn result(&self) -> &u8 {
        &self.result
    }
}

impl From<ServerRemoveSelf> for FlpcUnit {
    fn from(unit: ServerRemoveSelf) -> Self {
        Self::ServerRemoveSelf(unit)
    }
}

/// A server response indicating that a requested user is not online
/// (opcode `0xB2`). The unit contains only its opcode.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, UserNotOnline},
///     wire::{decode, Encode},
/// };
///
/// let unit = UserNotOnline::new();
/// let bytes = [0xb2];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserNotOnline;

impl UserNotOnline {
    pub(crate) const OPCODE: u8 = 0xB2;

    pub fn new() -> Self {
        Self
    }
}

impl From<UserNotOnline> for FlpcUnit {
    fn from(unit: UserNotOnline) -> Self {
        Self::UserNotOnline(unit)
    }
}

/// A response from the server (opcode `0x80`).
///
/// The UTF-8 response uses a two-byte big-endian byte-length prefix and may contain at most 65,535 bytes.
///
/// # Examples
///
/// ```
/// use flpc::{unit::{FlpcUnit, ServerResponse}, wire::{decode, Encode}};
///
/// let unit = ServerResponse::new("é".into());
/// let bytes: &[u8] = &[0x80, 0x00, 0x02, 0xc3, 0xa9];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerResponse {
    response: String,
}

impl ServerResponse {
    pub(crate) const OPCODE: u8 = 0x80;

    pub fn new(response: String) -> Self {
        Self { response }
    }

    pub fn response(&self) -> &String {
        &self.response
    }
}

impl From<ServerResponse> for FlpcUnit {
    fn from(unit: ServerResponse) -> Self {
        Self::ServerResponse(unit)
    }
}
