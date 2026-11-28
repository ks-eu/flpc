use flpc::{
    unit::{FlpcUnit, UserRegistered},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// Independent bytes from README opcode 0x84 field layout.
const FRAME: &[u8] = &[0x84, 0x12, 0x34, 2, 0xC3, 0xA9];

#[test]
fn encoding_and_decoding_match_the_wire_vector() {
    let unit = UserRegistered::new(0x1234, "é".into());
    let expected = FlpcUnit::UserRegistered(UserRegistered::new(0x1234, "é".into()));
    let mut dst = [0xaa; FRAME.len() + 2];
    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((expected.clone(), FRAME.len()))
    );

    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let adjacent = [FRAME, FRAME].concat();
    assert_eq!(decode(&adjacent).unwrap(), Some((expected, FRAME.len())));

    let wrapped: FlpcUnit = UserRegistered::new(0x1234, "é".into()).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn undersized_destination_stays_unchanged() {
    let unit = UserRegistered::new(0x1234, "é".into());
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, vec![0xaa; FRAME.len() - 1]);
}

#[test]
fn username_accepts_255_bytes_and_rejects_256() {
    let max = UserRegistered::new(0, "x".repeat(255));
    let bytes = max.to_bytes().unwrap();
    assert_eq!(bytes[3], 255);
    assert_eq!(bytes.len(), 259);

    let too_long = UserRegistered::new(0, "x".repeat(256));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
}

#[test]
fn invalid_utf8_username_is_rejected() {
    assert!(matches!(
        decode(&[0x84, 0, 1, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
