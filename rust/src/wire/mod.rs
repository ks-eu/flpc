//! Byte encoding for protocol units

use crate::unit::FlpcUnit;
pub use self::error::{
    EncodeError,
    DecodeError,
};

mod admin;
mod client;
mod ping;
mod server;

mod error;

pub trait Encode {
    fn encoded_len(&self) -> Result<usize, EncodeError>;

    fn encode_into(
        &self,
        buf: &mut [u8]
    ) -> Result<usize, EncodeError>;

    fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        let mut bytes = vec![0; self.encoded_len()?];
        self.encode_into(&mut bytes)?;
        Ok(bytes)
    }
}

pub fn decode(buf: &[u8]) -> Result<Option<(FlpcUnit, usize)>, DecodeError> {
    todo!()
}
