//! Unit for Pings, used by clients and servers alike.

use super::FlpcUnit;

/// A ping encoded as the single byte `0x00`.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, Ping},
///     wire::{decode, Encode},
/// };
///
/// let ping = Ping::new();
/// let bytes = [0x00];
/// assert_eq!(ping.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = ping.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
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

impl From<Ping> for FlpcUnit {
    fn from(unit: Ping) -> Self {
        Self::Ping(unit)
    }
}
