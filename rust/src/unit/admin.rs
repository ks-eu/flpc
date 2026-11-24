use super::FlpcUnit;

/// Administrative requests sent by a client to the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminUnit {
    Kick(Kick),
    Remove(Remove),
    Add(Add),
    MakeAdmin(MakeAdmin),
    RemoveAdmin(RemoveAdmin),
}

/// A request to kick a user (opcode `0x7F`).
///
/// The user ID is an unsigned 16-bit integer encoded in big-endian byte order.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{AdminUnit, FlpcUnit, Kick},
///     wire::{decode, Encode},
/// };
///
/// let unit = Kick::new(0x1234);
/// let bytes = [0x7f, 0x12, 0x34];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((FlpcUnit::Admin(AdminUnit::Kick(unit)), bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kick {
    id: u16,
}

impl Kick {
    pub(crate) const OPCODE: u8 = 0x7F;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

/// A request to remove a user (opcode `0x7E`).
///
/// The user ID is an unsigned 16-bit integer encoded in big-endian byte order.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{AdminUnit, FlpcUnit, Remove},
///     wire::{decode, Encode},
/// };
///
/// let unit = Remove::new(0x1234);
/// let bytes = [0x7e, 0x12, 0x34];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((FlpcUnit::Admin(AdminUnit::Remove(unit)), bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove {
    id: u16,
}

impl Remove {
    pub(crate) const OPCODE: u8 = 0x7E;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<Remove> for FlpcUnit {
    fn from(unit: Remove) -> Self {
        Self::Admin(AdminUnit::Remove(unit))
    }
}

/// A request to add a user (opcode `0x7D`).
///
/// The username and password are UTF-8 strings, each preceded by a one-byte
/// length counting UTF-8 bytes. Each field may contain at most 255 bytes.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{Add, FlpcUnit},
///     wire::{decode, Encode},
/// };
///
/// let unit = Add::new("Zoë".into(), "p".into());
/// let bytes = [0x7d, 0x04, 0x5a, 0x6f, 0xc3, 0xab, 0x01, b'p'];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Add {
    username: String,
    password: String,
}

impl std::fmt::Debug for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Add")
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl Add {
    pub(crate) const OPCODE: u8 = 0x7D;

    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

impl From<Add> for FlpcUnit {
    fn from(unit: Add) -> Self {
        Self::Admin(AdminUnit::Add(unit))
    }
}

/// A request to grant a user's administrator privileges (opcode `0x7C`).
///
/// The user ID is an unsigned 16-bit integer encoded in big-endian byte order.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, MakeAdmin},
///     wire::{decode, Encode},
/// };
///
/// let unit = MakeAdmin::new(0x1234);
/// let bytes = [0x7c, 0x12, 0x34];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeAdmin {
    id: u16,
}

impl MakeAdmin {
    pub(crate) const OPCODE: u8 = 0x7C;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<MakeAdmin> for FlpcUnit {
    fn from(unit: MakeAdmin) -> Self {
        Self::Admin(AdminUnit::MakeAdmin(unit))
    }
}

/// A request to remove a user's administrator privileges (opcode `0x7B`).
///
/// The user ID is an unsigned 16-bit integer encoded in big-endian byte order.
///
/// # Examples
///
/// ```
/// use flpc::{
///     unit::{FlpcUnit, RemoveAdmin},
///     wire::{decode, Encode},
/// };
///
/// let unit = RemoveAdmin::new(0x1234);
/// let bytes = [0x7b, 0x12, 0x34];
/// assert_eq!(unit.to_bytes().unwrap(), bytes);
/// let wrapped: FlpcUnit = unit.into();
/// assert_eq!(
///     decode(&bytes).unwrap(),
///     Some((wrapped, bytes.len())),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveAdmin {
    id: u16,
}

impl RemoveAdmin {
    pub(crate) const OPCODE: u8 = 0x7B;

    pub fn new(id: u16) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
}

impl From<RemoveAdmin> for FlpcUnit {
    fn from(unit: RemoveAdmin) -> Self {
        Self::Admin(AdminUnit::RemoveAdmin(unit))
    }
}
