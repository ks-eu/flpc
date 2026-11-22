#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{AdminUnit, FlpcUnit, Kick},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 7F bytes for big-endian ID 0xCDEF.
const FRAME: &[u8] = b"\x7f\xcd\xef";

fn expected() -> FlpcUnit {
    FlpcUnit::Admin(AdminUnit::Kick(Kick::new(0xcdef)))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(8);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed.send(Kick::new(0xcdef)).await.unwrap();
    drop(framed);

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
