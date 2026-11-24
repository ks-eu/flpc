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
    assert_eq!(&dst[..1], &[0x8B]);
    assert_eq!(&dst[1..], &[0xaa, 0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), &[0x8B]);

    assert!(matches!(
        unit.encode_into(&mut []),
        Err(EncodeError::BufferTooSmall)
    ));

    let wrapped: FlpcUnit = WrongUnit::new().into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_consumes_one_byte_before_the_next_unit() {
    let expected = FlpcUnit::Server(ServerUnit::WrongUnit(WrongUnit::new()));

    assert_eq!(decode(&[]).unwrap(), None);
    assert_eq!(decode(FRAME).unwrap(), Some((expected.clone(), 1)));
    assert_eq!(decode(&[0x8B, 0x00]).unwrap(), Some((expected, 1)));
}
