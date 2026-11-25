use flpc::unit::{FlpcUnit, ServerSyncUsername, ServerUnit};
use flpc::wire::{Encode, decode};
const FRAME: &[u8] = &[0x8f, 0x12, 0x34, 2, 0xc3, 0xa9];
fn expected() -> FlpcUnit {
    FlpcUnit::Server(ServerUnit::ServerSyncUsername(ServerSyncUsername::new(
        0x1234,
        "é".into(),
    )))
}
#[test]
fn literal_wire_decode_encode_and_conversion() {
    let unit = ServerSyncUsername::new(0x1234, "é".into());
    let wrapped: FlpcUnit = unit.clone().into();
    assert_eq!(wrapped, expected());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(decode(FRAME).unwrap(), Some((expected(), FRAME.len())));
    for n in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..n]).unwrap(), None);
    }
    assert_eq!(
        decode(&[FRAME, FRAME].concat()).unwrap(),
        Some((expected(), FRAME.len()))
    );
}
#[test]
fn username_lengths_and_utf8() {
    let u = ServerSyncUsername::new(1, "x".repeat(255));
    assert_eq!(u.to_bytes().unwrap()[3], 255);
}
