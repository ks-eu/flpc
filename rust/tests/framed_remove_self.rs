#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{ClientUnit, FlpcUnit, RemoveSelf},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x07\x02Zo\x02pw";

#[tokio::test]
async fn sends_literal_frame_and_receives_fragmented_adjacent_frames() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(RemoveSelf::new("Zo".into(), "pw".into()).into())
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
            FlpcUnit::Client(ClientUnit::RemoveSelf(RemoveSelf::new(
                "Zo".into(),
                "pw".into()
            )))
        );
    }
    assert!(framed.next().await.is_none());
}
