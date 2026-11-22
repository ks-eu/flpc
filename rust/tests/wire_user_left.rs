use flpc::{
    unit::{FlpcUnit, ServerUnit, UserLeft},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[0x87, 0x12, 0x34];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = UserLeft::new(0x1234);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::UserLeft(UserLeft::new(0x1234))),
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
            FlpcUnit::Server(ServerUnit::UserLeft(UserLeft::new(0x1234))),
            FRAME.len()
        ))
    );
}
