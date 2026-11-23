//! Unit for Pings, used by clients and servers alike.

/// Ping units sent by either peer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PingUnit {
    Ping(Ping),
}

/// A ping encoded as the single byte `0x00`.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, Ping, PingUnit},
///     wire::{decode, Encode},
/// };
///
/// let ping = Ping::new();
/// let bytes = [0x00];
/// assert_eq!(ping.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((FlpcUnit::Ping(PingUnit::Ping(ping)), bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ping {}

impl Ping {
    pub(crate) const OPCODE: u8 = 0x00;

    pub fn new() -> Self {
        Self {}
    }
}
