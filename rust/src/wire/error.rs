//! Errors for encoding and decoding

use std::{error::Error, fmt};

/// A failure to encode one FLPC unit.
#[derive(Debug)]
pub enum EncodeError {
    /// A UTF-8 string exceeds the maximum size of its length prefix.
    StringTooLong,
    /// The destination cannot hold the complete unit.
    BufferTooSmall,
    /// A synchronization request count must be nonzero.
    InvalidCount,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::StringTooLong => "string exceeds the protocol byte limit",
            Self::BufferTooSmall => "destination buffer is too small",
            Self::InvalidCount => "synchronization request count must be nonzero",
        })
    }
}

impl Error for EncodeError {}

/// A failure to decode one FLPC unit.
#[derive(Debug)]
pub enum DecodeError {
    /// The protocol has no unit assigned to the opcode.
    UnknownOpcode,
    /// The opcode is assigned, but this crate does not yet decode the unit.
    UnsupportedOpcode(u8),
    /// A complete string field contains invalid UTF-8.
    InvalidUTF8,
    /// A synchronization request contains an invalid zero count.
    InvalidCount,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOpcode => f.write_str("unknown FLPC opcode"),
            Self::UnsupportedOpcode(opcode) => {
                write!(f, "FLPC opcode 0x{opcode:02X} is not implemented")
            }
            Self::InvalidUTF8 => f.write_str("string field contains invalid UTF-8"),
            Self::InvalidCount => f.write_str("synchronization request count must be nonzero"),
        }
    }
}

impl Error for DecodeError {}
