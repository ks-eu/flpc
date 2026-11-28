#![cfg(feature = "tokio")]

use flpc::{
    tokio::{CodecError, FlpcCodec},
    unit::{FlpcUnit, ServerResponse},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x80, 0x00, 0x06, 0x68, 0xc3, 0xa9, 0x6c, 0x6c, 0x6f];

#[tokio::test]
async fn exchanges_literal_bytes_with_raw_peer() {
    let unit = ServerResponse::new("héllo".into());
    let expected = FlpcUnit::ServerResponse(unit.clone());
    let (stream, mut peer) = duplex(64);
    let (mut sink, _) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(unit.into()).await.unwrap();
    drop(sink);
    let mut sent = Vec::new();
    peer.read_to_end(&mut sent).await.unwrap();
    assert_eq!(sent, FRAME);

    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    let split = FRAME.len() - 1;
    peer.write_all(&FRAME[..split]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&[&FRAME[split..], &[0x00]].concat())
        .await
        .unwrap();
    peer.shutdown().await.unwrap();
    assert_eq!(framed.next().await.unwrap().unwrap(), expected);
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Ping(flpc::unit::Ping::new())
    );
    assert!(framed.next().await.is_none());
}

#[tokio::test]
async fn reports_truncated_frame_at_eof() {
    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    peer.shutdown().await.unwrap();
    assert!(matches!(framed.next().await, Some(Err(CodecError::Io(_)))));
}
