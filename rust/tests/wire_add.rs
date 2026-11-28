use flpc::{
    unit::{Add, FlpcUnit},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// Opcode 7D carries username "Zoë" and password "p" as short UTF-8 fields.
const FRAME: &[u8] = b"\x7d\x04Zo\xc3\xab\x01p";

fn expected() -> FlpcUnit {
    FlpcUnit::Add(Add::new("Zoë".into(), "p".into()))
}

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = Add::new("Zoë".into(), "p".into());
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let wrapped: FlpcUnit = Add::new("Zoë".into(), "p".into()).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_both_fields_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x7d, 1, b'u', 1, b'p']].concat();
    assert_eq!(decode(&adjacent).unwrap(), Some((expected(), FRAME.len())));
    assert_eq!(
        decode(&adjacent[FRAME.len()..]).unwrap(),
        Some((FlpcUnit::Add(Add::new("u".into(), "p".into())), 5,))
    );
}

#[test]
fn empty_and_255_byte_fields_round_trip() {
    let full = "x".repeat(255);
    for (unit, bytes) in [
        (Add::new(String::new(), String::new()), &[0x7d, 0, 0][..]),
        (
            Add::new(full.clone(), String::new()),
            &[&[0x7d, 0xff][..], full.as_bytes(), &[0]].concat(),
        ),
        (
            Add::new(String::new(), full.clone()),
            &[&[0x7d, 0, 0xff][..], full.as_bytes()].concat(),
        ),
    ] {
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(
            decode(bytes).unwrap(),
            Some((FlpcUnit::Add(unit), bytes.len()))
        );
    }
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        Add::new("Zoë".into(), "p".into()).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn either_oversized_field_is_rejected_before_writing() {
    let long = "é".repeat(128);
    for unit in [
        Add::new(long.clone(), "p".into()),
        Add::new("u".into(), long),
    ] {
        let mut dst = [0xaa; 260];
        assert!(matches!(
            unit.encoded_len(),
            Err(EncodeError::StringTooLong)
        ));
        assert!(matches!(
            unit.encode_into(&mut dst),
            Err(EncodeError::StringTooLong)
        ));
        assert_eq!(dst, [0xaa; 260]);
    }
}

#[test]
fn invalid_utf8_in_either_field_is_a_decode_error() {
    assert!(matches!(
        decode(&[0x7d, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
    assert!(matches!(
        decode(&[0x7d, 0, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
