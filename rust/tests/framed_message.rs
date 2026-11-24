#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{ClientUnit, FlpcUnit, Message, Ping, PingUnit},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x03\x00\x05hello";

fn message(value: &str) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Message(Message::new(value.into())))
}

#[tokio::test]
async fn sends_independent_bytes_to_a_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(Message::new("hello".into()).into())
        .await
        .unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_prefix_and_utf8_before_ping_and_clean_eof() {
    const UTF8_FRAME: &[u8] = b"\x03\x00\x04Zo\xc3\xab";
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&UTF8_FRAME[..2]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&UTF8_FRAME[2..6]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&UTF8_FRAME[6..]).await.unwrap();
    peer.write_all(&[0x00]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), message("Zoë"));
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(PingUnit::Ping(Ping::new()))
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_message_at_eof_after_complete_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.write_all(&[0x03, 0x00, 0x02, b'x']).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), message("hello"));
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
