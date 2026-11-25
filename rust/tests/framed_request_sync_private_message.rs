#![cfg(feature = "tokio")]
use flpc::{
    tokio::FlpcCodec,
    unit::{ClientUnit, FlpcUnit, RequestSyncPrivateMessage},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;
const FRAME: &[u8] = &[0x09, 0x12, 0x34, 1, 2, 3, 4, 5, 6, 7, 8, 1];
#[tokio::test]
async fn raw_peer_roundtrip() {
    let (s, mut p) = duplex(32);
    let (mut sink, _) = Framed::new(s, FlpcCodec).split::<FlpcUnit>();
    sink.send(RequestSyncPrivateMessage::new(0x1234, 0x0102030405060708, 1).into())
        .await
        .unwrap();
    drop(sink);
    let mut got = Vec::new();
    p.read_to_end(&mut got).await.unwrap();
    assert_eq!(got, FRAME);
    let (s, mut p) = duplex(32);
    let mut f = Framed::new(s, FlpcCodec);
    p.write_all(FRAME).await.unwrap();
    p.shutdown().await.unwrap();
    assert_eq!(
        f.next().await.unwrap().unwrap(),
        FlpcUnit::Client(ClientUnit::RequestSyncPrivateMessage(
            RequestSyncPrivateMessage::new(0x1234, 0x0102030405060708, 1)
        ))
    );
    assert!(f.next().await.is_none());
}
