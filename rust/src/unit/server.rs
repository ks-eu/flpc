//! Units for servers.

#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum ServerUnit {
    ServerRegister,
    ServerLogin,
    ServerMessage,
    ServerPrivateMessage,
    UserRegistered,
    UserRemoved,
    UserJoined,
    UserLeft,
    ChangedUsername,
    SyncMessage,
    SyncPrivateMessage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRegister {
    outcome: u8
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerLogin {
    outcome: u8
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct ServerPrivateMessage {
    id: u16,
    target: u16,
    timestamp: u64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRegistered {
    id: u16,
    username: String,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct UserRemoved {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct UserJoined {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct UserLeft {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedUsername {
    id: u16,
    username: String,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct SyncMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct SyncPrivateMessage {
    id: u16,
    target: u16,
    timestamp: u64,
    message: String,
}
