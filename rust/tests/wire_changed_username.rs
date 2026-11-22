use flpc::{
    unit::{ChangedUsername, FlpcUnit, ServerUnit},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[0x88, 0x12, 0x34, 2, 0xC3, 0xA9];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = ChangedUsername::new(0x1234, "é".into());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ChangedUsername(ChangedUsername::new(
                0x1234,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let adjacent = [FRAME, FRAME].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ChangedUsername(ChangedUsername::new(
                0x1234,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
}
