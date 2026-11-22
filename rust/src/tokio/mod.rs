//! Tokio framing for FLPC streams. Enable the `tokio` Cargo feature to use it.

use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    unit::FlpcUnit,
    wire::{self, Encode},
};

mod error;

pub use error::CodecError;

/// Stateless FLPC codec for [`tokio_util::codec::Framed`].
///
/// Send any unit that implements [`Encode`]. Incoming units are returned as [`FlpcUnit`].
#[derive(Debug, Default, Clone, Copy)]
pub struct FlpcCodec;

impl<T: Encode> Encoder<T> for FlpcCodec {
    type Error = CodecError;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.encoded_len().map_err(CodecError::Encode)?;
        let start = dst.len();
        dst.resize(start + len, 0);

        if let Err(error) = item.encode_into(&mut dst[start..]) {
            dst.truncate(start);
            return Err(CodecError::Encode(error));
        }
        Ok(())
    }
}

impl Decoder for FlpcCodec {
    type Item = FlpcUnit;
    type Error = CodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let Some((unit, consumed)) = wire::decode(src).map_err(CodecError::Decode)? else {
            return Ok(None);
        };

        src.advance(consumed);
        Ok(Some(unit))
    }
}
