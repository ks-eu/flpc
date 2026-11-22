use flpc::{
    unit::{FlpcUnit, ServerUnit, UserRegistered},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x84, 0x12, 0x34, 2, 0xC3, 0xA9];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = UserRegistered::new(0x1234, "é".into());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::UserRegistered(UserRegistered::new(
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
            FlpcUnit::Server(ServerUnit::UserRegistered(UserRegistered::new(
                0x1234,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
}

#[test]
fn username_length_prefix_accepts_255_bytes() {
    let unit = UserRegistered::new(0, "x".repeat(255));
    let bytes = unit.to_bytes().unwrap();
    assert_eq!(bytes[3], 255);
    assert_eq!(bytes.len(), 259);

    let too_long = UserRegistered::new(0, "x".repeat(256));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
}
