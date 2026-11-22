#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{ChangedUsername, FlpcUnit, ServerUnit},
};
use futures_util::{FutureExt, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent bytes from README opcode 0x88 field layout.
const FRAME: &[u8] = &[0x88, 0x12, 0x34, 2, 0xC3, 0xA9];

#[tokio::test]
async fn framed_sends_and_receives_independent_wire_bytes() {
    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed
        .send(ChangedUsername::new(0x1234, "é".into()))
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
        FlpcUnit::Server(ServerUnit::ChangedUsername(ChangedUsername::new(
            0x1234,
            "é".into()
        )))
    );
}
