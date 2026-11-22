#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{ClientUnit, FlpcUnit, Message},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x03\x00\x05hello";

fn expected() -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Message(Message::new("hello".into())))
}

#[tokio::test]
async fn sends_independent_bytes_to_a_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed.send(Message::new("hello".into())).await.unwrap();
    drop(framed);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_message_and_following_unit() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..4]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[4..]).await.unwrap();
    peer.write_all(&[0x00]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert!(matches!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(_)
    ));
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_payload_at_eof_after_complete_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.write_all(&[0x03, 0x00, 0x02, b'x']).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
