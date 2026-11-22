use flpc::{
    unit::{FlpcUnit, ServerLogin, ServerUnit},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[0x81, 0x5A];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = ServerLogin::new(0x5A);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ServerLogin(ServerLogin::new(0x5A))),
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
            FlpcUnit::Server(ServerUnit::ServerLogin(ServerLogin::new(0x5A))),
            FRAME.len()
        ))
    );
}
