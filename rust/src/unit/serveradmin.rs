//! Replies to administrator requests.

use super::FlpcUnit;

/// Reports the outcome of an add-user request (opcode `0xE0`).
///
/// The outcome is preserved as a raw `u8`. Peers agree on its meaning.
///
/// # Examples
///
/// ```
/// use flpc::{unit::{FlpcUnit, ServerAdd}, wire::{decode, Encode}};
///
/// let unit = ServerAdd::new(0x2a);
/// let bytes: &[u8] = &[0xe0, 0x2a];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerAdd {
    outcome: u8,
}

impl ServerAdd {
    pub(crate) const OPCODE: u8 = 0xE0;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerAdd> for FlpcUnit {
    fn from(unit: ServerAdd) -> Self {
        Self::ServerAdd(unit)
    }
}

/// Reports the outcome of a remove-user request (opcode `0xE1`).
///
/// The outcome is preserved as a raw `u8`. Peers agree on its meaning.
///
/// # Examples
///
/// ```
/// use flpc::{unit::{FlpcUnit, ServerRemove}, wire::{decode, Encode}};
///
/// let unit = ServerRemove::new(0x2a);
/// let bytes: &[u8] = &[0xe1, 0x2a];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerRemove {
    outcome: u8,
}

impl ServerRemove {
    pub(crate) const OPCODE: u8 = 0xE1;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerRemove> for FlpcUnit {
    fn from(unit: ServerRemove) -> Self {
        Self::ServerRemove(unit)
    }
}

/// Reports the outcome of a kick request (opcode `0xE2`).
///
/// The outcome is preserved as a raw `u8`. Peers agree on its meaning.
///
/// # Examples
///
/// ```
/// use flpc::{unit::{FlpcUnit, ServerKick}, wire::{decode, Encode}};
///
/// let unit = ServerKick::new(0x2a);
/// let bytes: &[u8] = &[0xe2, 0x2a];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerKick {
    outcome: u8,
}

impl ServerKick {
    pub(crate) const OPCODE: u8 = 0xE2;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerKick> for FlpcUnit {
    fn from(unit: ServerKick) -> Self {
        Self::ServerKick(unit)
    }
}

/// Reports the outcome of a grant-administrator request (opcode `0xE3`).
///
/// The outcome is preserved as a raw `u8`. Peers agree on its meaning.
///
/// # Examples
///
/// ```
/// use flpc::{unit::{FlpcUnit, ServerMakeAdmin}, wire::{decode, Encode}};
///
/// let unit = ServerMakeAdmin::new(0x2a);
/// let bytes: &[u8] = &[0xe3, 0x2a];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(decode(bytes).unwrap(), Some((wrapped, bytes.len())));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerMakeAdmin {
    outcome: u8,
}

impl ServerMakeAdmin {
    pub(crate) const OPCODE: u8 = 0xE3;

    pub fn new(outcome: u8) -> Self {
        Self { outcome }
    }

    pub fn outcome(&self) -> &u8 {
        &self.outcome
    }
}

impl From<ServerMakeAdmin> for FlpcUnit {
    fn from(unit: ServerMakeAdmin) -> Self {
        Self::ServerMakeAdmin(unit)
    }
}
