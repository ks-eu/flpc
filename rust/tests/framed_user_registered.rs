#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, ServerUnit, UserRegistered},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

const FRAME: &[u8] = &[0x84, 0x12, 0x34, 2, 0xC3, 0xA9];

#[tokio::test]
async fn framed_send_and_receive_use_protocol_bytes() {
    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    framed
        .send(UserRegistered::new(0x1234, "é".into()))
        .await
        .unwrap();
    drop(framed);
    let mut actual = Vec::new();
    peer.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, FRAME);

    let (stream, mut peer) = duplex(64);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Server(ServerUnit::UserRegistered(UserRegistered::new(
            0x1234,
            "é".into()
        )))
    );
}
