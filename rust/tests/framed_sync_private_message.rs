#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, ServerUnit, SyncPrivateMessage},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent bytes from README opcode 0x8A field layout.
const FRAME: &[u8] = &[
    0x8A, 0x12, 0x34, 0x56, 0x78, 1, 2, 3, 4, 5, 6, 7, 8, 0, 2, 0xC3, 0xA9,
];

#[tokio::test]
async fn framed_sends_and_receives_independent_wire_bytes() {
    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed
        .send(SyncPrivateMessage::new(
            0x1234,
            0x5678,
            0x0102030405060708,
            "é".into(),
        ))
        .await
        .unwrap();
    drop(framed);
    let mut actual = Vec::new();
    peer.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, FRAME);

    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(&FRAME[..FRAME.len() - 1]).await.unwrap();
    assert!(framed.next().now_or_never().is_none());
    peer.write_all(&FRAME[FRAME.len() - 1..]).await.unwrap();
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Server(ServerUnit::SyncPrivateMessage(SyncPrivateMessage::new(
            0x1234,
            0x5678,
            0x0102030405060708,
            "é".into()
        )))
    );
}
