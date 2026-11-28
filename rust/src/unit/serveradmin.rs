//! Replies to administrator requests.

use super::FlpcUnit;

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
