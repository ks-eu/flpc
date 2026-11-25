use flpc::unit::{FlpcUnit, ServerSyncEnd, ServerUnit};
use flpc::wire::{Encode, decode};
const FRAME: &[u8] = &[0x8d, 1];
fn expected() -> FlpcUnit {
    FlpcUnit::Server(ServerUnit::ServerSyncEnd(ServerSyncEnd::new(1)))
}
#[test]
fn literal_wire_decode_encode_and_conversion() {
    let unit = ServerSyncEnd::new(1);
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
