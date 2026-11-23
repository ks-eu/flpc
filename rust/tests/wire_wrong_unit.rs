use flpc::{
    unit::{FlpcUnit, ServerUnit, WrongUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x8B];

#[test]
fn encode_matches_opcode_and_preserves_tail() {
    let unit = WrongUnit::new();
    let mut dst = [0xaa; 3];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_consumes_one_byte_before_the_next_unit() {
    let expected = FlpcUnit::Server(ServerUnit::WrongUnit(WrongUnit::new()));

    assert_eq!(decode(&[]).unwrap(), None);
    assert_eq!(decode(FRAME).unwrap(), Some((expected.clone(), FRAME.len())));
    assert_eq!(
        decode(&[0x8B, 0x00]).unwrap(),
        Some((expected, FRAME.len()))
    );
}

#[test]
fn encode_rejects_an_empty_destination() {
    assert!(matches!(
        WrongUnit::new().encode_into(&mut []),
        Err(EncodeError::BufferTooSmall)
    ));
}
