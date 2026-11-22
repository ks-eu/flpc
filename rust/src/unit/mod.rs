//! Units exchanged by FLPC clients and servers.
//!
//! The [wire codec](crate::wire) reads and writes one unit at a time.

mod admin;
mod client;
mod ping;
mod server;

pub use admin::{Add, AdminUnit, Kick, MakeAdmin, Remove, RemoveAdmin};
pub use client::{
    ChangeUsername, ClientUnit, Login, Message, PrivateMessage, Register, RequestSync,
};
pub use ping::{Ping, PingUnit};
pub use server::{
    ChangedUsername, ServerLogin, ServerMessage, ServerPrivateMessage, ServerRegister, ServerUnit,
    SyncMessage, SyncPrivateMessage, UserJoined, UserLeft, UserRegistered, UserRemoved,
};

/// A unit decoded from an FLPC byte stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlpcUnit {
    Admin(AdminUnit),
    Client(ClientUnit),
    Ping(PingUnit),
    Server(ServerUnit),
}
