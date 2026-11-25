#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{ClientUnit, FlpcUnit, RequestSyncMessage},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = b"\x06\x01\x02\x03\x04\x05\x06\x07\x08\xff";
const SECOND: &[u8] = b"\x06\xff\xff\xff\xff\xff\xff\xff\xff\x01";

#[tokio::test]
async fn raw_peer_send_and_fragmented_adjacent_receive() {
    let (stream, mut peer) = duplex(32);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(RequestSyncMessage::new(0x0102_0304_0506_0708, 255).into())
        .await
        .unwrap();
    drop(sink);
    let mut sent = Vec::new();
    peer.read_to_end(&mut sent).await.unwrap();
    assert_eq!(sent, FRAME);

    let (stream, mut peer) = duplex(32);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..5]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&[&FRAME[5..], SECOND].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::RequestSyncMessage(RequestSyncMessage::new(
            0x0102_0304_0506_0708,
            255
        )))
    );
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::RequestSyncMessage(RequestSyncMessage::new(
            u64::MAX,
            1
        )))
    );
    assert!(framed.next().await.is_none());
}
