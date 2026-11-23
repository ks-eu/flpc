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
///
/// # Example
///
/// This example uses the following dependencies:
///
/// ```toml
/// [dependencies]
/// flpc = { version = "0.1", features = ["tokio"] }
/// futures-util = { version = "0.3", features = ["sink"] }
/// tokio = { version = "1", features = ["io-util", "macros", "rt"] }
/// tokio-util = { version = "0.7", features = ["codec"] }
/// ```
///
/// ```
/// use flpc::{
///     tokio::FlpcCodec,
///     unit::{ClientUnit, FlpcUnit, Message},
/// };
/// use futures_util::{SinkExt, StreamExt};
/// use tokio::io::duplex;
/// use tokio_util::codec::Framed;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (sender, receiver) = duplex(32);
///     let mut sender = Framed::new(sender, FlpcCodec);
///     let mut receiver = Framed::new(receiver, FlpcCodec);
///
///     sender.send(Message::new("hello".into())).await?;
///     let unit = receiver.next().await.expect("expected a message")?;
///     assert_eq!(
///         unit,
///         FlpcUnit::Client(ClientUnit::Message(Message::new("hello".into())))
///     );
///     Ok(())
/// }
/// ```
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
