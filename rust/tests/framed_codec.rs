#![cfg(feature = "tokio")]

use bytes::BytesMut;
use flpc::{
    tokio::{CodecError, FlpcCodec},
    wire::{DecodeError, Encode, EncodeError},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::{Encoder, Framed};

// A test encoder exercises the adapter before a concrete unit is implemented.
struct TestFrame;

impl Encode for TestFrame {
    fn encoded_len(&self) -> Result<usize, EncodeError> {
        Ok(3)
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let dst = buf.get_mut(..3).ok_or(EncodeError::BufferTooSmall)?;
        dst.copy_from_slice(&[0x10, 0x00, 0x00]);
        Ok(dst.len())
    }
}

#[tokio::test]
async fn framed_sends_the_unit_bytes() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    framed.send(TestFrame).await.unwrap();
    drop(framed);
    let mut actual = [0; 3];
    peer.read_exact(&mut actual).await.unwrap();

    assert_eq!(actual, [0x10, 0x00, 0x00]);
}

#[tokio::test]
async fn framed_reports_wire_errors() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&[0xff]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert!(matches!(
        framed.next().await,
        Some(Err(CodecError::Decode(DecodeError::UnknownOpcode)))
    ));
}

#[test]
fn encode_appends_without_overwriting_buffered_frames() {
    let mut dst = BytesMut::from(&[0xaa][..]);

    FlpcCodec.encode(TestFrame, &mut dst).unwrap();

    assert_eq!(&dst[..], &[0xaa, 0x10, 0x00, 0x00]);
}
