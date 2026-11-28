use flpc::{
    unit::{FlpcUnit, Register},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// README wire-format vector: opcode 01, token "t", username "é", password "pw".
const FRAME: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

fn register(token: &str, username: &str, password: &str) -> Register {
    Register::new(token.into(), username.into(), password.into())
}

fn decoded(unit: Register) -> FlpcUnit {
    FlpcUnit::Register(unit)
}

#[test]
fn encode_matches_the_readme_vector_and_preserves_the_tail() {
    let unit = register("t", "é", "pw");
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let wrapped: FlpcUnit = Register::new("t".into(), "é".into(), "pw".into()).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_three_complete_fields_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x01, 0, 0, 0]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((decoded(register("t", "é", "pw")), FRAME.len()))
    );
}

#[test]
fn empty_and_255_byte_fields_round_trip() {
    let full = "x".repeat(255);
    let full_frame = [
        &[0x01, 0xff][..],
        full.as_bytes(),
        &[0xff],
        full.as_bytes(),
        &[0xff],
        full.as_bytes(),
    ]
    .concat();

    for (unit, bytes) in [
        (register("", "", ""), &[0x01, 0, 0, 0][..]),
        (register(&full, &full, &full), &full_frame),
    ] {
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(decode(bytes).unwrap(), Some((decoded(unit), bytes.len())));
    }
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        register("t", "é", "pw").encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn each_256_byte_field_is_rejected_before_writing() {
    let long = "é".repeat(128);
    for unit in [
        register(&long, "u", "p"),
        register("t", &long, "p"),
        register("t", "u", &long),
    ] {
        let mut dst = [0xaa; 262];
        assert!(matches!(
            unit.encoded_len(),
            Err(EncodeError::StringTooLong)
        ));
        assert!(matches!(
            unit.encode_into(&mut dst),
            Err(EncodeError::StringTooLong)
        ));
        assert_eq!(dst, [0xaa; 262]);
    }
}

#[test]
fn invalid_utf8_in_any_complete_field_is_a_decode_error() {
    for input in [&[1, 1, 255][..], &[1, 0, 1, 255], &[1, 0, 0, 1, 255]] {
        assert!(matches!(decode(input), Err(DecodeError::InvalidUTF8)));
    }
}
