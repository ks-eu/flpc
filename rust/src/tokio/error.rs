use std::{error::Error, fmt, io};

use crate::wire::{DecodeError, EncodeError};

/// A wire or I/O failure reported by the Tokio codec.
#[derive(Debug)]
pub enum CodecError {
    /// The underlying stream failed or ended with a partial unit.
    Io(io::Error),
    /// An outgoing unit could not be encoded.
    Encode(EncodeError),
    /// The incoming bytes could not be decoded.
    Decode(DecodeError),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "FLPC I/O failed: {error}"),
            Self::Encode(error) => write!(f, "FLPC encoding failed: {error}"),
            Self::Decode(error) => write!(f, "FLPC decoding failed: {error}"),
        }
    }
}

impl Error for CodecError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Io(error) => error,
            Self::Encode(error) => error,
            Self::Decode(error) => error,
        })
    }
}

impl From<io::Error> for CodecError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
