#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, MakeAdmin, RemoveAdmin},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 7C bytes for big-endian ID 0x4567.
const FRAME: &[u8] = b"\x7c\x45\x67";

fn expected() -> FlpcUnit {
    FlpcUnit::MakeAdmin(MakeAdmin::new(0x4567))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(8);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(MakeAdmin::new(0x4567).into()).await.unwrap();
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
    peer.write_all(&[&FRAME[2..], &[0x7b, 0x89, 0xab]].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::RemoveAdmin(RemoveAdmin::new(0x89ab))
    );
    assert!(framed.next().await.is_none());
}
