//! Units for Clients

use super::FlpcUnit;

/// A client registration request (opcode `0x01`).
///
/// The wire format contains the token, username, and password in that order.
/// Each field is UTF-8 preceded by a one-byte length, so each may contain at
/// most 255 UTF-8 bytes. Encoding a longer field returns
/// [`crate::wire::EncodeError::StringTooLong`].
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, Register},
///     wire::{decode, Encode},
/// };
///
/// let register = Register::new("t".into(), "é".into(), "pw".into());
/// let bytes = [0x01, 0x01, b't', 0x02, 0xc3, 0xa9, 0x02, b'p', b'w'];
/// assert_eq!(register.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = register.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Register {
    token: String,
    username: String,
    password: String,
}

impl std::fmt::Debug for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Register")
            .field("token", &"[redacted]")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl Register {
    pub(crate) const OPCODE: u8 = 0x01;

    pub fn new(token: String, username: String, password: String) -> Self {
        Self {
            token,
            username,
            password,
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

impl From<Register> for FlpcUnit {
    fn from(unit: Register) -> Self {
        Self::Register(unit)
    }
}

/// The `Login` wire unit (opcode `0x02`).
///
/// After the opcode, the username comes first, followed by the password. Each
/// field has a one-byte length prefix that counts UTF-8 bytes and allows up to
/// 255 bytes.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, Login},
///     wire::{Encode, decode},
/// };
///
/// let unit = Login::new("Zoë".into(), "pāss".into());
/// let bytes = [
///     0x02, 0x04, 0x5a, 0x6f, 0xc3, 0xab, 0x05, 0x70, 0xc4, 0x81, 0x73, 0x73,
/// ];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Login {
    username: String,
    password: String,
}

impl std::fmt::Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Login")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl Login {
    pub(crate) const OPCODE: u8 = 0x02;

    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

impl From<Login> for FlpcUnit {
    fn from(unit: Login) -> Self {
        Self::Login(unit)
    }
}

/// A message sent to the server (opcode `0x03`).
///
/// The message is encoded as UTF-8 after a two-byte big-endian length prefix.
/// The prefix counts UTF-8 bytes, so a message may contain at most 65,535
/// bytes. Encoding a longer message returns
/// [`crate::wire::EncodeError::StringTooLong`].
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, Message},
///     wire::{decode, Encode},
/// };
///
/// let message = Message::new("é".into());
/// let bytes = [0x03, 0x00, 0x02, 0xc3, 0xa9];
/// assert_eq!(message.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = message.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    message: String,
}

impl Message {
    pub(crate) const OPCODE: u8 = 0x03;

    /// Creates a message. Encoding checks the 65,535 UTF-8 byte limit.
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<Message> for FlpcUnit {
    fn from(unit: Message) -> Self {
        Self::Message(unit)
    }
}

/// A private message sent to a target (opcode `0x04`).
///
/// The target is an unsigned 16-bit big-endian integer. The message follows
/// as UTF-8 preceded by a two-byte big-endian length counting UTF-8 bytes, so
/// it may contain at most 65,535 bytes. Encoding a longer message returns
/// [`crate::wire::EncodeError::StringTooLong`].
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, PrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let message = PrivateMessage::new(0x1234, "é".into());
/// let bytes = [0x04, 0x12, 0x34, 0x00, 0x02, 0xc3, 0xa9];
/// assert_eq!(message.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = message.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateMessage {
    target: u16,
    message: String,
}

impl PrivateMessage {
    pub(crate) const OPCODE: u8 = 0x04;

    pub fn new(target: u16, message: String) -> Self {
        Self { target, message }
    }

    pub fn target(&self) -> &u16 {
        &self.target
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl From<PrivateMessage> for FlpcUnit {
    fn from(unit: PrivateMessage) -> Self {
        Self::PrivateMessage(unit)
    }
}

/// A request to change the client's username (opcode `0x05`).
///
/// The username is UTF-8 preceded by a one-byte length counting UTF-8 bytes,
/// so it may contain at most 255 bytes. Encoding a longer username returns
/// [`crate::wire::EncodeError::StringTooLong`].
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{ChangeUsername, FlpcUnit},
///     wire::{decode, Encode},
/// };
///
/// let unit = ChangeUsername::new("Zoë".into());
/// let bytes = [0x05, 0x04, 0x5a, 0x6f, 0xc3, 0xab];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeUsername {
    username: String,
}

impl ChangeUsername {
    pub(crate) const OPCODE: u8 = 0x05;

    pub fn new(username: String) -> Self {
        Self { username }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

impl From<ChangeUsername> for FlpcUnit {
    fn from(unit: ChangeUsername) -> Self {
        Self::ChangeUsername(unit)
    }
}

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

/// A client request to remove an account authenticated by the supplied
/// username and password (opcode `0x07`). Both fields are UTF-8 strings with
/// one-byte lengths that count UTF-8 bytes, so each can contain at most 255
/// bytes.
/// Debug output redacts the password. The credentials are sent as plaintext.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, RemoveSelf},
///     wire::{decode, Encode},
/// };
///
/// let unit = RemoveSelf::new("Zoë".into(), "pä".into());
/// let bytes = [0x07, 0x04, b'Z', b'o', 0xc3, 0xab, 0x03, b'p', 0xc3, 0xa4];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(&bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct RemoveSelf {
    username: String,
    password: String,
}

impl std::fmt::Debug for RemoveSelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoveSelf")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl RemoveSelf {
    pub(crate) const OPCODE: u8 = 0x07;

    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

impl From<RemoveSelf> for FlpcUnit {
    fn from(unit: RemoveSelf) -> Self {
        Self::RemoveSelf(unit)
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
