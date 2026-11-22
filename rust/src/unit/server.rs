//! Units for servers.

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRegister {
    outcome: u8
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerLogin {
    outcome: u8
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

impl ServerMessage {
    pub(crate) const OPCODE: u8 = 0x82;

    pub fn new(id: u16, timestamp: u64, message: String) -> Self {
        Self { id, timestamp, message }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

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

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct SyncMessage {
    id: u16,
    timestamp: u64,
    message: String,
}

impl SyncMessage {
    pub(crate) const OPCODE: u8 = 0x89;

    pub fn new(id: u16, timestamp: u64, message: String) -> Self {
        Self { id, timestamp, message }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

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

    pub fn timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}
