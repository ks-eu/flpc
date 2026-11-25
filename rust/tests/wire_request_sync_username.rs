use flpc::unit::{ClientUnit, FlpcUnit, RequestSyncUsername};
use flpc::wire::{Encode, decode};
const FRAME: &[u8] = &[0x08, 0x12, 0x34];
fn expected() -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::RequestSyncUsername(RequestSyncUsername::new(
        0x1234,
    )))
}
#[test]
fn literal_wire_decode_encode_and_conversion() {
    let unit = RequestSyncUsername::new(0x1234);
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
