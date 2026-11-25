#![cfg(feature = "tokio")]
use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, ServerRemoveSelf, ServerUnit},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;
const FRAME: &[u8] = &[0x8c, 0xa5];
#[tokio::test]
async fn raw_peer_roundtrip() {
    let (s, mut p) = duplex(32);
    let (mut sink, _) = Framed::new(s, FlpcCodec).split::<FlpcUnit>();
    sink.send(ServerRemoveSelf::new(0xa5).into()).await.unwrap();
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
        FlpcUnit::Server(ServerUnit::ServerRemoveSelf(ServerRemoveSelf::new(0xa5)))
    );
    assert!(f.next().await.is_none());
}
