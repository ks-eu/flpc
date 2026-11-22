//! Units for Clients

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
///     unit::{ClientUnit, FlpcUnit, Register},
///     wire::{decode, Encode},
/// };
///
/// let register = Register::new("t".into(), "é".into(), "pw".into());
/// let bytes = [0x01, 0x01, b't', 0x02, 0xc3, 0xa9, 0x02, b'p', b'w'];
/// assert_eq!(register.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((
///         FlpcUnit::Client(ClientUnit::Register(register)),
///         bytes.len(),
///     )),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    token: String,
    username: String,
    password: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    username: String,
    password: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    message: String,
}

impl Message {
    pub(crate) const OPCODE: u8 = 0x03;

    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

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
