#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{ClientUnit, FlpcUnit, Ping, PingUnit, Register},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const PING: &[u8] = &[0x00];
const REGISTER: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

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
    assert_eq!(bytes, [PING, PING].concat());
}

#[tokio::test]
async fn receives_adjacent_pings_then_a_differently_sized_frame() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&[PING, PING, REGISTER].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(framed.next().await.unwrap().unwrap(), unit());
    assert_eq!(framed.next().await.unwrap().unwrap(), unit());
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::Register(Register::new(
            "t".into(),
            "é".into(),
            "pw".into(),
        )))
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn split_sink_sends_converted_unit() {
    let (stream, mut peer) = duplex(PING.len() + 1);
    let (mut sink, stream_half) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(Ping::new().into()).await.unwrap();
    drop(sink);
    drop(stream_half);
    let mut actual = Vec::new();
    peer.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, PING);
}
