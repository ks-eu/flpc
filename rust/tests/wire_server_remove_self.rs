use flpc::unit::{FlpcUnit, ServerRemoveSelf, ServerUnit};
use flpc::wire::{Encode, decode};
const FRAME: &[u8] = &[0x8c, 0xa5];
fn expected() -> FlpcUnit {
    FlpcUnit::Server(ServerUnit::ServerRemoveSelf(ServerRemoveSelf::new(0xa5)))
}
#[test]
fn literal_wire_decode_encode_and_conversion() {
    let unit = ServerRemoveSelf::new(0xa5);
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
