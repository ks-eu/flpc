//! Units for Clients

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientUnit {
    Register,
    Login,
    Message,
    PrivateMessage,
    ChangeUsername,
    RequestSync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register {
    token: String,
    username: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    message: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateMesage {
    target: u16,
    message: String,
}

pub struct ChangeUsername {
    username: String,
}

pub struct RequestSync {
    first: u64,
    last: u64,
}
