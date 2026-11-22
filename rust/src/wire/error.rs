//! Errors for encoding and decoding

use std::{error::Error, fmt};

#[derive(Debug)]
pub enum EncodeError {
    StringTooLong,
    BufferTooSmall,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::StringTooLong => "string exceeds the protocol byte limit",
            Self::BufferTooSmall => "destination buffer is too small",
        })
    }
}

impl Error for EncodeError {}

#[derive(Debug)]
pub enum DecodeError {
    UnknownOpcode,
    InvalidUTF8,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnknownOpcode => "unknown FLPC opcode",
            Self::InvalidUTF8 => "string field contains invalid UTF-8",
        })
    }
}

impl Error for DecodeError {}

