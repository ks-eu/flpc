use flpc::{
    unit::{ClientUnit, FlpcUnit, Register},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

fn register(token: &str, username: &str, password: &str) -> Register {
    Register::new(token.into(), username.into(), password.into())
}

fn decoded(unit: Register) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Register(unit))
}

#[test]
fn encode_writes_only_the_reference_frame() {
    let unit = register("t", "é", "pw");
    let mut dst = [0xaa; FRAME.len() + 2];
    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_all_fields_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let frames = [FRAME, FRAME].concat();
    assert_eq!(
        decode(&frames).unwrap(),
        Some((decoded(register("t", "é", "pw")), FRAME.len()))
    );
}

#[test]
fn empty_and_full_length_fields_are_valid() {
    let full = "x".repeat(255);
    let mut full_frame = vec![1];
    for _ in 0..3 {
        full_frame.push(255);
        full_frame.extend_from_slice(full.as_bytes());
    }
    for (unit, bytes) in [
        (register("", "", ""), vec![1, 0, 0, 0]),
        (register(&full, &full, &full), full_frame),
    ] {
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(decode(&bytes).unwrap(), Some((decoded(unit), bytes.len())));
    }
}

#[test]
fn too_small_a_destination_is_preserved() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        register("t", "é", "pw").encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn each_field_rejects_256_utf8_bytes_before_writing() {
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
fn each_complete_field_rejects_invalid_utf8() {
    for input in [&[1, 1, 255][..], &[1, 0, 1, 255], &[1, 0, 0, 1, 255]] {
        assert!(matches!(decode(input), Err(DecodeError::InvalidUTF8)));
    }
}
