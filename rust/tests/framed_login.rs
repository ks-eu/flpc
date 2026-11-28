#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{FlpcUnit, Login, Ping},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[
    0x02, 0x04, 0x5a, 0x6f, 0xc3, 0xab, 0x05, 0x70, 0xc4, 0x81, 0x73, 0x73,
];

fn unit() -> Login {
    Login::new("Zoë".into(), "pāss".into())
}

fn expected() -> FlpcUnit {
    let unit = unit();
    FlpcUnit::Login(unit)
}

#[tokio::test]
async fn sends_the_literal_vector_to_a_raw_peer() {
    let (stream, mut peer) = duplex(128);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(unit().into()).await.unwrap();
    drop(sink);
    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_fragmented_unit_before_ping_and_clean_eof() {
    let (stream, mut peer) = duplex(128);
    let mut framed = Framed::new(stream, FlpcCodec);
    // Split the two-byte UTF-8 encoding of `ë` across reads.
    let split = 5;
    peer.write_all(&FRAME[..split]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[split..]).await.unwrap();
    peer.write_all(&[0x00]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(Ping::new())
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn delivers_complete_unit_before_reporting_partial_eof() {
    let (stream, mut peer) = duplex(128);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert_eq!(framed.next().await.unwrap().unwrap(), expected());
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
