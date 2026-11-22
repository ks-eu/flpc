//! Units for Clients

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientUnit {
    Register(Register),
    Login(Login),
    Message(Message),
    PrivateMessage(PrivateMesage),
    ChangeUsername(ChangeUsername),
    RequestSync(RequestSync),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    token: String,
    username: String,
    password: String,
}

impl Register {
    pub(crate) const OPCODE: u8 = 0x01;

    pub fn new(
        token: String,
        username: String,
        password: String,
    ) -> Self {
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
    message: String
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
pub struct PrivateMesage {
    target: u16,
    message: String,
}

impl PrivateMesage {
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
