#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, OnboardingEnd},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x95];

#[tokio::test]
async fn exchanges_literal_bytes_with_raw_peer() {
    let unit = OnboardingEnd::new();
    let expected = FlpcUnit::OnboardingEnd(unit.clone());
    let (stream, mut peer) = duplex(64);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(unit.into()).await.unwrap();
    drop(sink);
    let mut sent = Vec::new();
    peer.read_to_end(&mut sent).await.unwrap();
    assert_eq!(sent, FRAME);

    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&[FRAME, &[0x00]].concat()).await.unwrap();
    peer.shutdown().await.unwrap();
    assert_eq!(framed.next().await.unwrap().unwrap(), expected);
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(flpc::unit::Ping::new())
    );
    assert!(framed.next().await.is_none());
}
