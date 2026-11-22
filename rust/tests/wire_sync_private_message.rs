use flpc::{
    unit::{FlpcUnit, ServerUnit, SyncPrivateMessage},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[
    0x8A, 0x12, 0x34, 0x56, 0x78, 1, 2, 3, 4, 5, 6, 7, 8, 0, 2, 0xC3, 0xA9,
];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = SyncPrivateMessage::new(0x1234, 0x5678, 0x0102030405060708, "é".into());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::SyncPrivateMessage(SyncPrivateMessage::new(
                0x1234,
                0x5678,
                0x0102030405060708,
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
            FlpcUnit::Server(ServerUnit::SyncPrivateMessage(SyncPrivateMessage::new(
                0x1234,
                0x5678,
                0x0102030405060708,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
}
