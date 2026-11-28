use flpc::{
    unit::{ChangePassword, FlpcUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[
    0x04, 0x05, 0x61, 0x6c, 0x69, 0x63, 0x65, 0x03, 0x6f, 0x6c, 0x64, 0x04, 0x6e, 0xc3, 0xab, 0x77,
];

#[test]
fn wire_vector_conversion_and_decode() {
    let unit = ChangePassword::new("alice".into(), "old".into(), "nëw".into());
    let expected = FlpcUnit::ChangePassword(ChangePassword::new(
        "alice".into(),
        "old".into(),
        "nëw".into(),
    ));
    let mut dst = vec![0xaa; FRAME.len() + 1];
    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(dst[FRAME.len()], 0xaa);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((expected.clone(), FRAME.len()))
    );
    let wrapped: FlpcUnit = ChangePassword::new("alice".into(), "old".into(), "nëw".into()).into();
    assert_eq!(wrapped, expected);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let adjacent = [FRAME, &[0x00]].concat();
    assert_eq!(decode(&adjacent).unwrap(), Some((expected, FRAME.len())));
}

#[test]
fn short_destination_is_unchanged() {
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        ChangePassword::new("alice".into(), "old".into(), "nëw".into()).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert!(dst.iter().all(|byte| *byte == 0xaa));
}

#[test]
fn credentials_redact_and_limit_each_field() {
    let unit = ChangePassword::new("alice".into(), "secret-old".into(), "secret-new".into());
    let debug = format!("{unit:?}");
    assert!(!debug.contains("secret-old"));
    assert!(!debug.contains("secret-new"));
    for unit in [
        ChangePassword::new("x".repeat(256), "p".into(), "n".into()),
        ChangePassword::new("u".into(), "x".repeat(256), "n".into()),
        ChangePassword::new("u".into(), "p".into(), "x".repeat(256)),
    ] {
        assert!(matches!(
            unit.encoded_len(),
            Err(EncodeError::StringTooLong)
        ));
    }
    for bad in [&[4, 1, 0xff][..], &[4, 0, 1, 0xff], &[4, 0, 0, 1, 0xff]] {
        assert!(decode(bad).is_err());
    }
}
