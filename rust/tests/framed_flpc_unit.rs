#![cfg(feature = "tokio")]

use flpc::{
    tokio::FlpcCodec,
    unit::{FlpcUnit, ServerRegister, ServerUnit},
};
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream, duplex};
use tokio_util::codec::Framed;

// Independent bytes from README opcode 0x80 field layout.
const FRAME: &[u8] = &[0x80, 0x00];

async fn send_registration(mut sink: SplitSink<Framed<DuplexStream, FlpcCodec>, FlpcUnit>) {
    let unit: FlpcUnit = ServerRegister::new(0).into();
    sink.send(unit).await.unwrap();
}

#[tokio::test]
async fn split_flpc_sink_sends_and_receives_server_register() {
    let (stream, mut peer) = duplex(16);
    let (sink, stream_half) = Framed::new(stream, FlpcCodec).split();
    send_registration(sink).await;
    drop(stream_half);
    let mut actual = Vec::new();
    peer.read_to_end(&mut actual).await.unwrap();
    assert_eq!(actual, FRAME);

    let (stream, mut peer) = duplex(16);
    let mut framed = Framed::new(stream, FlpcCodec);
    peer.write_all(FRAME).await.unwrap();
    assert_eq!(
        framed.next().await.unwrap().unwrap(),
        FlpcUnit::Server(ServerUnit::ServerRegister(ServerRegister::new(0)))
    );
}
