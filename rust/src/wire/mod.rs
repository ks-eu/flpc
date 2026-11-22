//! Byte encoding and decoding for FLPC units.
//!
//! Units begin with a one-byte opcode. [`decode()`] reads at most one unit
//! from the start of a byte slice and reports how many bytes it used.

mod admin;
mod client;
mod decode;
mod error;
mod fields;
mod ping;
mod server;

pub use decode::decode;
pub use error::{DecodeError, EncodeError};

/// Encodes one FLPC unit, including its opcode.
pub trait Encode {
    /// Returns the exact encoded length in bytes.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError::StringTooLong`] if a field exceeds its wire limit.
    fn encoded_len(&self) -> Result<usize, EncodeError>;

    /// Writes the unit at the start of `buf` and returns the number of bytes written.
    ///
    /// Bytes after the encoded unit are preserved. The destination is preserved on error.
    ///
    /// # Errors
    ///
    /// Returns an error if a field exceeds its wire limit or `buf` is too small.
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError>;

    /// Allocates a byte vector containing one complete unit.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError::StringTooLong`] if a field exceeds its wire limit.
    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = vec![0; self.encoded_len()?];
        self.encode_into(&mut bytes)?;
        Ok(bytes)
    }
}
