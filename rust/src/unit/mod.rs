//! Units exchanged by FLPC clients and servers.
//!
//! The [wire codec](crate::wire) reads and writes one unit at a time.
//! Constructors store the supplied values. Encoding checks that token,
//! username, and password fields fit in 255 UTF-8 bytes and message fields
//! fit in 65,535 UTF-8 bytes. Longer fields return
//! [`crate::wire::EncodeError::StringTooLong`].

mod admin;
mod client;
mod ping;
mod server;

pub use admin::{Add, AdminUnit, Kick, MakeAdmin, Remove, RemoveAdmin};
pub use client::{
    ChangeUsername, ClientUnit, Login, Message, PrivateMessage, Register, RemoveSelf, RequestSync,
    RequestSyncUsername,
};
pub use ping::{Ping, PingUnit};
pub use server::{
    ChangedUsername, ServerLogin, ServerMessage, ServerPrivateMessage, ServerRegister, ServerUnit,
    SyncMessage, SyncPrivateMessage, UserJoined, UserLeft, UserRegistered, UserRemoved, WrongUnit,
};

/// A unit that can be encoded to or decoded from an FLPC byte stream.
///
/// The outer variant identifies the unit family. The inner variant contains
/// the concrete unit for the opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlpcUnit {
    /// An administrative request.
    Admin(AdminUnit),
    /// A regular client request or message.
    Client(ClientUnit),
    /// A ping from either peer.
    Ping(PingUnit),
    /// A server response, notification, or message.
    Server(ServerUnit),
}
