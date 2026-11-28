#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, Ping, UserNotOnline},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0xb2];
const NEXT: &[u8] = &[0x00];

fn unit() -> UserNotOnline {
    UserNotOnline::new()
}

#[tokio::test]
async fn sends_literal_frame_to_raw_peer() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(unit().into()).await.unwrap();
    drop(sink);

    let mut bytes = Vec::new();
    peer.read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, FRAME);
}

#[tokio::test]
async fn receives_adjacent_frames_then_clean_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&[FRAME, NEXT].concat()).await.unwrap();
    peer.shutdown().await.unwrap();

    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::UserNotOnline(unit())
    );
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(Ping::new())
    );
    assert!(framed.next().await.is_none());
}
