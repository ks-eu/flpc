use flpc::{
    unit::{ClientUnit, FlpcUnit, RemoveSelf},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x07\x04Zo\xc3\xab\x03p\xc3\xa4";

fn unit(username: &str, password: &str) -> RemoveSelf {
    RemoveSelf::new(username.into(), password.into())
}

fn wrapped(username: &str, password: &str) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::RemoveSelf(unit(username, password)))
}

#[test]
fn encode_and_conversion_match_independent_bytes() {
    let value = unit("Zoë", "pä");
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(value.encoded_len().unwrap(), FRAME.len());
    assert_eq!(value.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(value.to_bytes().unwrap(), FRAME);
    let converted: FlpcUnit = value.into();
    assert_eq!(converted, wrapped("Zoë", "pä"));
}

#[test]
fn decode_handles_truncation_adjacency_and_utf8_errors() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    assert_eq!(
        decode(&[FRAME, FRAME].concat()).unwrap(),
        Some((wrapped("Zoë", "pä"), FRAME.len()))
    );
    for bytes in [&[7, 1, 0xff, 0][..], &[7, 0, 1, 0xff]] {
        assert!(matches!(decode(bytes), Err(DecodeError::InvalidUTF8)));
    }
}

#[test]
fn short_string_limits_and_encode_errors_preserve_buffer() {
    assert_eq!(unit("", "").to_bytes().unwrap(), &[0x07, 0, 0]);
    for value in [unit(&"x".repeat(255), "p"), unit("u", &"x".repeat(255))] {
        assert!(value.to_bytes().is_ok());
    }
    for value in [unit(&"x".repeat(256), "p"), unit("u", &"x".repeat(256))] {
        let mut dst = [0xaa; 520];
        assert!(matches!(
            value.encode_into(&mut dst),
            Err(EncodeError::StringTooLong)
        ));
        assert_eq!(dst, [0xaa; 520]);
    }
    let mut dst = [0xaa; 2];
    assert!(matches!(
        unit("u", "p").encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; 2]);
}

#[test]
fn debug_redacts_password_in_concrete_and_wrapped_units() {
    for value in [
        format!("{:?}", unit("u", "secret")),
        format!("{:?}", wrapped("u", "secret")),
    ] {
        assert!(!value.contains("secret"));
        assert!(value.contains("[redacted]"));
    }
}
