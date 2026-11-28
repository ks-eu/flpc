#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{FlpcUnit, Ping, RequestSyncPrivateMessage},
    wire::DecodeError,
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x09, 0x12, 0x34, 1, 2, 3, 4, 5, 6, 7, 8, 1];
const NEXT: &[u8] = &[0x00];

fn unit() -> RequestSyncPrivateMessage {
    RequestSyncPrivateMessage::new(0x1234, 0x0102_0304_0506_0708, 1)
}

#[tokio::test]
async fn sends_literal_frame_to_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(unit().into()).await.unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_adjacent_frames_then_clean_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    let split = FRAME.len() - 1;
    peer.write_all(&FRAME[..split]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&[&FRAME[split..], NEXT].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::RequestSyncPrivateMessage(unit())
    );
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(Ping::new())
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_frame_at_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}

#[tokio::test]
async fn rejects_zero_request_count() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&[0x09, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0])
        .await
        .unwrap();
    peer.shutdown().await.unwrap();
    assert!(matches!(
        framed.next().await,
        Some(Err(CodecError::Decode(DecodeError::InvalidCount)))
    ));
}
