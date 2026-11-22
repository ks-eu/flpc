#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{ClientUnit, FlpcUnit, Register},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

#[tokio::test]
async fn sends_reference_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed
        .send(Register::new("t".into(), "é".into(), "pw".into()))
        .await
        .unwrap();
    drop(framed);
    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_and_adjacent_frames() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);

    // Split the two-byte username between reads.
    peer.write_all(&FRAME[..5]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[5..]).await.unwrap();
    peer.write_all(&[1, 0, 0, 0]).await.unwrap();
    peer.shutdown().await.unwrap();

    for expected in [
        Register::new("t".into(), "é".into(), "pw".into()),
        Register::new(String::new(), String::new(), String::new()),
    ] {
        assert_eq!(
            framed.next().await.unwrap().unwrap(),
            FlpcUnit::Client(ClientUnit::Register(expected))
        );
    }
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_partial_frame_at_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
