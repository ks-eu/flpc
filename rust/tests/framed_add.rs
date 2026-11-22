#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{Add, AdminUnit, FlpcUnit},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 7D bytes for username "Zoë" and password "p".
const FRAME: &[u8] = b"\x7d\x04Zo\xc3\xab\x01p";

fn expected() -> FlpcUnit {
    FlpcUnit::Admin(AdminUnit::Add(Add::new("Zoë".into(), "p".into())))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed
        .send(Add::new("Zoë".into(), "p".into()))
        .await
        .unwrap();
    drop(framed);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_frame() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&FRAME[..5]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[5..]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert!(framed.next().await.is_none());
}
