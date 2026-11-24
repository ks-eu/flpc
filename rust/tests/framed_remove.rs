#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{AdminUnit, FlpcUnit, Ping, PingUnit, Remove},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 7E bytes for big-endian ID 0x89AB.
const FRAME: &[u8] = b"\x7e\x89\xab";

fn expected() -> FlpcUnit {
    FlpcUnit::Admin(AdminUnit::Remove(Remove::new(0x89ab)))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(8);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(Remove::new(0x89ab).into()).await.unwrap();
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
    peer.write_all(&[&FRAME[2..], &[0x00]].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(PingUnit::Ping(Ping::new()))
    );
    assert!(framed.next().await.is_none());
}
