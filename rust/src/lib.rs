//! FLPC is a binary protocol for chat applications over TCP.
//!
//! [`unit`](mod@unit) defines the protocol units. [`wire`] provides the byte codec API.
//! Enable the `tokio` feature to use the codec with asynchronous framed I/O.

#[cfg(feature = "tokio")]
pub mod tokio;
pub mod unit;
pub mod wire;
