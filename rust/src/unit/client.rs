//! Units for Clients

use super::FlpcUnit;

/// Client requests and messages sent to the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientUnit {
    Register(Register),
    Login(Login),
    Message(Message),
    PrivateMessage(PrivateMessage),
    ChangeUsername(ChangeUsername),
    RequestSync(RequestSync),
}

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
        Self::Client(ClientUnit::Register(unit))
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
        Self::Client(ClientUnit::Login(unit))
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
        Self::Client(ClientUnit::Message(unit))
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
///     unit::{ClientUnit, FlpcUnit, PrivateMessage},
///     wire::{decode, Encode},
/// };
///
/// let message = PrivateMessage::new(0x1234, "é".into());
/// let bytes = [0x04, 0x12, 0x34, 0x00, 0x02, 0xc3, 0xa9];
/// assert_eq!(message.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((
///         FlpcUnit::Client(ClientUnit::PrivateMessage(message)),
///         bytes.len(),
///     )),
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
///     unit::{ChangeUsername, ClientUnit, FlpcUnit},
///     wire::{decode, Encode},
/// };
///
/// let unit = ChangeUsername::new("Zoë".into());
/// let bytes = [0x05, 0x04, 0x5a, 0x6f, 0xc3, 0xab];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((
///         FlpcUnit::Client(ClientUnit::ChangeUsername(unit)),
///         bytes.len(),
///     )),
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

/// A request for a range of synchronized messages (opcode `0x06`).
///
/// The `first` and `last` bounds are opaque unsigned 64-bit values encoded in
/// that order as big-endian integers. Peers agree on what the bounds
/// represent and whether they are inclusive.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{ClientUnit, FlpcUnit, RequestSync},
///     wire::{decode, Encode},
/// };
///
/// let unit = RequestSync::new(1, 2);
/// let bytes = [
///     0x06, 0, 0, 0, 0, 0, 0, 0, 1,
///     0, 0, 0, 0, 0, 0, 0, 2,
/// ];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((FlpcUnit::Client(ClientUnit::RequestSync(unit)), bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestSync {
    first: u64,
    last: u64,
}

impl RequestSync {
    pub(crate) const OPCODE: u8 = 0x06;

    pub fn new(first: u64, last: u64) -> Self {
        Self { first, last }
    }

    pub fn first(&self) -> &u64 {
        &self.first
    }

    pub fn last(&self) -> &u64 {
        &self.last
    }
}
