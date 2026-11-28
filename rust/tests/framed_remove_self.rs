#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, RemoveSelf},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x02\x04Zo\xc3\xab\x03p\xc3\xa4";

#[tokio::test]
async fn sends_literal_frame_and_receives_fragmented_adjacent_frames() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(RemoveSelf::new("Zoë".into(), "pä".into()).into())
        .await
        .unwrap();
    drop(sink);
    let mut sent = Vec::new();
    peer.read_to_end(&mut sent).await.unwrap();
    assert_eq!(sent, FRAME);

    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..3]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&[FRAME[3..].to_vec(), FRAME.to_vec()].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();
    for _ in 0..2 {
        assert_eq!(
            framed.next().await.unwrap().unwrap(),
            FlpcUnit::RemoveSelf(RemoveSelf::new("Zoë".into(), "pä".into()))
        );
    }
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_partial_frame_at_eof() {
    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert!(framed.next().await.unwrap().is_err());
}
