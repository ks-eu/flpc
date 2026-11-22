#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{ClientUnit, FlpcUnit, RequestSync},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent opcode 06 bytes with distinct bounds and byte patterns.
const FRAME: &[u8] = b"\x06\x01\x02\x03\x04\x05\x06\x07\x08\x11\x22\x33\x44\x55\x66\x77\x88";
const REVERSED_FRAME: &[u8] = b"\x06\xff\xff\xff\xff\xff\xff\xff\xff\0\0\0\0\0\0\0\0";

fn request_sync() -> RequestSync {
    RequestSync::new(0x0102_0304_0506_0708, 0x1122_3344_5566_7788)
}

fn expected() -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::RequestSync(request_sync()))
}

#[tokio::test]
async fn sends_wire_bytes_to_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed.send(request_sync()).await.unwrap();
    drop(framed);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_frame() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&FRAME[..9]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&[&FRAME[9..], REVERSED_FRAME].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::RequestSync(RequestSync::new(u64::MAX, 0)))
    );
    assert!(framed.next().await.is_none());
}
