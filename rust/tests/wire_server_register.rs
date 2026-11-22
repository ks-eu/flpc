use flpc::{
    unit::{FlpcUnit, ServerRegister, ServerUnit},
    wire::{Encode, decode},
};

const FRAME: &[u8] = &[0x80, 0xA5];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = ServerRegister::new(0xA5);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ServerRegister(ServerRegister::new(0xA5))),
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
            FlpcUnit::Server(ServerUnit::ServerRegister(ServerRegister::new(0xA5))),
            FRAME.len()
        ))
    );
}
