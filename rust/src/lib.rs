//! FLPC is a binary protocol for chat applications over TCP.
//!
//! [`unit`](mod@unit) defines the protocol units. [`wire`] provides the byte codec API.
//! Enable the `tokio` feature to use the codec with asynchronous framed I/O.
//!
//! # Quickstart
//!
//! Encode a concrete unit with [`wire::Encode`]. [`wire::decode()`] returns
//! the first decoded unit as a [`unit::FlpcUnit`] and the number of
//! bytes it consumed. Keep the full input when it returns `Ok(None)` so you
//! can retry with more bytes.
//!
//! ```
//! use flpc::{
//!     unit::{FlpcUnit, Message, Ping},
//!     wire::{Encode, decode},
//! };
//!
//! let mut input = Message::new("é".into()).to_bytes()?;
//! input.extend(Ping::new().to_bytes()?);
//!
//! let (unit, consumed) = decode(&input)?.expect("expected a complete message");
//! match unit {
//!     FlpcUnit::Message(message) => assert_eq!(message.message(), "é"),
//!     _ => panic!("expected a client message"),
//! }
//! assert!(matches!(
//!     decode(&input[consumed..])?,
//!     Some((FlpcUnit::Ping(_), 1))
//! ));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Credentials
//!
//! [`unit::Register`], [`unit::Login`], [`unit::RemoveSelf`],
//! [`unit::ChangePassword`], and [`unit::Add`] carry passwords as plaintext
//! UTF-8. The registration token in [`unit::Register`] is also
//! plaintext. The unit types display passwords and the registration token
//! as `[redacted]` in `Debug` output.

#[cfg(feature = "tokio")]
pub mod tokio;
pub mod unit;
pub mod wire;
