#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{ChangeUsername, ClientUnit, FlpcUnit},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independently specified opcode 05 frame for username "é".
const FRAME: &[u8] = b"\x05\x02\xc3\xa9";

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(16);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(ChangeUsername::new("é".into()).into())
        .await
        .unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_complete_frame_then_clean_eof() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::ChangeUsername(ChangeUsername::new("é".into())))
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn receives_fragmented_frame_and_truncated_frame_at_eof() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&FRAME[..3]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[3..]).await.unwrap();
    peer.write_all(&[0x05, 2, b'x']).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::ChangeUsername(ChangeUsername::new("é".into())))
    );
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
