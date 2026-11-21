//! Each individual message in the flpc communication protocol is called a unit. This protocol
//! works by explcitly specifying the length of characters within the payload of each unit. For
//! example, the unit used by a client sending a message to a server has the form:
//! [1 byte code - client message] [2 byte code - user_id] [2 byte - length of message] [message
//! encoded with UTF8]
//!

mod admin;
mod client;
mod ping;
mod server;

/// Unit
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    Client,
    Server,
    Admin,
    Ping,
}


