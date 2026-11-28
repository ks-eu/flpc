#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{FlpcUnit, PrivateMessage},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x11\x12\x34\x00\x02\xc3\xa9";

fn expected(target: u16, message: &str) -> FlpcUnit {
    FlpcUnit::PrivateMessage(PrivateMessage::new(target, message.into()))
}

#[tokio::test]
async fn sends_independent_bytes_to_a_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(PrivateMessage::new(0x1234, "é".into()).into())
        .await
        .unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_target_and_utf8_then_adjacent_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&FRAME[..2]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[2..6]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[6..]).await.unwrap();
    peer.write_all(&[0x11, 0x00, 0x01, 0x00, 0x00])
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected(0x1234, "é"));
    assert_eq!(framed.next().await.unwrap().unwrap(), expected(1, ""));
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_private_message_at_eof_after_complete_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.write_all(&[0x11, 0x12, 0x34, 0x00, 0x02, b'x'])
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected(0x1234, "é"));
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
