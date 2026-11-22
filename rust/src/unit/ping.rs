//! Unit for Pings, used by clients and servers alike.

#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum PingUnit {
    Ping(Ping),
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct Ping {}

impl Ping {
    pub(crate) const OPCODE: u8 = 0x00;

    pub fn new() -> Self {
        Self {}
    }
}
