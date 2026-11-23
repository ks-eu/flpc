//! FLPC is a binary protocol for chat applications over TCP.
//!
//! [`unit`](mod@unit) defines the protocol units. [`wire`] provides the byte codec API.
//! Enable the `tokio` feature to use the codec with asynchronous framed I/O.
//!
//! # Credentials
//!
//! [`unit::Register`], [`unit::Login`], and [`unit::Add`] carry passwords
//! as plaintext UTF-8. The registration token in [`unit::Register`] is also
//! plaintext. The unit types display passwords and the registration token
//! as `[redacted]` in `Debug` output.

#[cfg(feature = "tokio")]
pub mod tokio;
pub mod unit;
pub mod wire;
