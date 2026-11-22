#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, Ping, PingUnit},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x00];

fn unit() -> FlpcUnit {
    FlpcUnit::Ping(PingUnit::Ping(Ping::new()))
}

#[tokio::test]
async fn sends_adjacent_ping_frames_to_a_raw_peer() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    framed.send(Ping::new()).await.unwrap();
    framed.send(Ping::new()).await.unwrap();
    drop(framed);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, [FRAME, FRAME].concat());
}

#[tokio::test]
async fn receives_adjacent_ping_frames_from_a_raw_peer() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&[0x00, 0x00]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), unit());
    assert_eq!(framed.next().await.unwrap().unwrap(), unit());
    assert!(framed.next().await.is_none());
}
