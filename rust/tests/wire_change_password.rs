use flpc::{
    unit::{ChangePassword, FlpcUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[
    0x04, 0x05, 0x61, 0x6c, 0x69, 0x63, 0x65, 0x03, 0x6f, 0x6c, 0x64, 0x04, 0x6e, 0xc3, 0xab, 0x77,
];

#[test]
fn literal_bytes_match_encoding_and_decoding() {
    let unit = ChangePassword::new("alice".into(), "old".into(), "nëw".into());
    let expected = FlpcUnit::ChangePassword(unit.clone());
    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());

    let mut dst = [0xaa; FRAME.len() + 1];
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(dst[FRAME.len()], 0xaa);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let mut short = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut short),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(short, [0xaa; FRAME.len() - 1]);

    let wrapped: FlpcUnit = unit.into();
    assert_eq!(wrapped, expected);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x00]].concat();
    for input in [FRAME, adjacent.as_slice()] {
        assert_eq!(
            decode(input).unwrap(),
            Some((expected.clone(), FRAME.len()))
        );
    }
}

#[test]
fn credentials_redact_and_limit_each_field() {
    let unit = ChangePassword::new("alice".into(), "secret-old".into(), "secret-new".into());
    let wrapped: FlpcUnit = unit.clone().into();
    for debug in [format!("{unit:?}"), format!("{wrapped:?}")] {
        assert!(debug.contains(r#", password: "[redacted]","#));
        assert!(debug.contains(r#"new_password: "[redacted]""#));
        assert!(!debug.contains("secret-old"));
        assert!(!debug.contains("secret-new"));
    }
    for unit in [
        ChangePassword::new("é".repeat(127) + "x", "p".into(), "n".into()),
        ChangePassword::new("u".into(), "é".repeat(127) + "x", "n".into()),
        ChangePassword::new("u".into(), "p".into(), "é".repeat(127) + "x"),
    ] {
        assert!(unit.to_bytes().is_ok());
    }
    for unit in [
        ChangePassword::new("é".repeat(128), "p".into(), "n".into()),
        ChangePassword::new("u".into(), "é".repeat(128), "n".into()),
        ChangePassword::new("u".into(), "p".into(), "é".repeat(128)),
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
    for bad in [&[4, 1, 0xff][..], &[4, 0, 1, 0xff], &[4, 0, 0, 1, 0xff]] {
        assert!(matches!(
            decode(bad),
            Err(flpc::wire::DecodeError::InvalidUTF8)
        ));
    }
}
