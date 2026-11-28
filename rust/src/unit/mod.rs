//! Units exchanged by FLPC clients and servers.
//!
//! The [wire codec](crate::wire) reads and writes one unit at a time.
//! Constructors store the supplied values. Encoding checks that token,
//! username, and password fields fit in 255 UTF-8 bytes and message fields
//! fit in 65,535 UTF-8 bytes. Synchronization request counts must be nonzero.
//! Longer strings return
//! [`crate::wire::EncodeError::StringTooLong`].

mod admin;
mod client;
mod ping;
mod requestsync;
mod server;
mod serveradmin;
mod serversync;

pub use admin::{Add, Kick, MakeAdmin, Remove, RemoveAdmin};
pub use client::{
    ChangePassword, ChangeUsername, Login, Message, PrivateMessage, Register, RemoveSelf,
};
pub use ping::Ping;
pub use requestsync::{RequestSyncMessage, RequestSyncPrivateMessage, RequestSyncUsername};
pub use server::{
    ChangedUsername, OnboardingEnd, ServerChangedPassword, ServerChangedUsername, ServerLogin,
    ServerMessage, ServerMessageFail, ServerPrivateMessage, ServerRegister, ServerRemoveSelf,
    ServerResponse, UserAdmin, UserAdminRemoved, UserJoined, UserLeft, UserNotOnline,
    UserRegistered, UserRemoved, WrongUnit,
};
pub use serversync::{SyncEnd, SyncMessage, SyncPrivateMessage, SyncUsername};

/// A unit that can be encoded to or decoded from an FLPC byte stream.
///
/// Each variant contains the concrete unit for one opcode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlpcUnit {
    Kick(Kick),
    Remove(Remove),
    Add(Add),
    MakeAdmin(MakeAdmin),
    RemoveAdmin(RemoveAdmin),
    Register(Register),
    Login(Login),
    Message(Message),
    PrivateMessage(PrivateMessage),
    ChangeUsername(ChangeUsername),
    RequestSyncMessage(RequestSyncMessage),
    RemoveSelf(RemoveSelf),
    RequestSyncUsername(RequestSyncUsername),
    RequestSyncPrivateMessage(RequestSyncPrivateMessage),
    Ping(Ping),
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
    WrongUnit(WrongUnit),
    ServerRemoveSelf(ServerRemoveSelf),
    SyncEnd(SyncEnd),
    UserNotOnline(UserNotOnline),
    SyncUsername(SyncUsername),
    ChangePassword(ChangePassword),
    ServerResponse(ServerResponse),
    UserAdmin(UserAdmin),
    UserAdminRemoved(UserAdminRemoved),
    ServerChangedPassword(ServerChangedPassword),
    OnboardingEnd(OnboardingEnd),
    ServerMessageFail(ServerMessageFail),
    ServerChangedUsername(ServerChangedUsername),
}
