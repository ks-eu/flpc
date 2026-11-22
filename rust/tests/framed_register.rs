#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{ClientUnit, FlpcUnit, Register},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Keep the peer's expected wire bytes independent of the codec under test.
const FRAME: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

#[tokio::test]
async fn sends_readme_vector_to_a_raw_peer() {
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
async fn reports_clean_eof_after_the_last_complete_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::Register(Register::new(
            "t".into(),
            "é".into(),
            "pw".into(),
        )))
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn receives_fragmented_frame_then_truncated_frame_at_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);

    // Split the two-byte UTF-8 username between reads.
    peer.write_all(&FRAME[..5]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[5..]).await.unwrap();
    peer.write_all(&[0x01, 0, 0, 0]).await.unwrap();
    peer.write_all(&[0x01, 1, b'x']).await.unwrap();
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
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
