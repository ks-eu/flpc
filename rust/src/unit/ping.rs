//! Unit for Pings, used by clients and servers alike.

#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum PingUnit {
    Ping,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct Ping {}
