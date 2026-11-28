#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, Ping, WrongUnit},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x94];

#[tokio::test]
async fn sends_opcode_to_a_raw_peer() {
    let (stream, mut peer) = duplex(16);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();

    sink.send(WrongUnit::new().into()).await.unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_wrong_unit_then_ping_and_clean_eof() {
    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);

    peer.write_all(&[0x94]).await.unwrap();

    assert_eq!(
        framed.next().now_or_never().unwrap().unwrap().unwrap(),
        FlpcUnit::WrongUnit(WrongUnit::new())
    );

    peer.write_all(&[0x00]).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(Ping::new())
    );
    assert!(framed.next().await.is_none());
}
