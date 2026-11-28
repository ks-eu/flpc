#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{FlpcUnit, RemoveAdmin},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 64 bytes for big-endian ID 0x1234.
const FRAME: &[u8] = b"\x64\x12\x34";

fn expected() -> FlpcUnit {
    FlpcUnit::RemoveAdmin(RemoveAdmin::new(0x1234))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(8);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(RemoveAdmin::new(0x1234).into()).await.unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_frame() {
    let (stream, mut peer) = duplex(8);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&FRAME[..2]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[2..]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_id_at_eof() {
    let (stream, mut peer) = duplex(8);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&[0x64, 0x12]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
