use flpc::{
    unit::{FlpcUnit, ServerUnit, UserJoined},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[0x86, 0x12, 0x34];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = UserJoined::new(0x1234);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::UserJoined(UserJoined::new(0x1234))),
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
            FlpcUnit::Server(ServerUnit::UserJoined(UserJoined::new(0x1234))),
            FRAME.len()
        ))
    );
}
