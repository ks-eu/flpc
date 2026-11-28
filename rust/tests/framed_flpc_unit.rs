#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, ServerRegister},
};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, duplex};
use tokio_util::codec::Framed;

// Independent bytes from README opcode 0x90 field layout.
const FRAME: &[u8] = &[0x90, 0x00];

#[tokio::test]
async fn split_flpc_sink_sends_and_receives_server_register() {
    let (stream, mut peer) = duplex(16);
    let (mut sink, stream_half) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    sink.send(ServerRegister::new(0).into()).await.unwrap();
    drop(sink);
    drop(stream_half);
    let mut actual = Vec::new();
    peer.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, FRAME);

    let (stream, mut peer) = duplex(16);
    let (_sink, mut stream_half) = Framed::new(stream, FlpcCodec).split::<FlpcUnit>();
    peer.write_all(FRAME).await.unwrap();
    assert_eq!(
        stream_half.next().await.unwrap().unwrap(),
        FlpcUnit::ServerRegister(ServerRegister::new(0))
    );
}
