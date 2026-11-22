use flpc::{
    unit::{ChangedUsername, FlpcUnit, ServerUnit},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// Independent bytes from README opcode 0x88 field layout.
const FRAME: &[u8] = &[0x88, 0x12, 0x34, 2, 0xC3, 0xA9];

#[test]
fn encoding_and_decoding_match_the_wire_vector() {
    let unit = ChangedUsername::new(0x1234, "é".into());
    let expected = FlpcUnit::Server(ServerUnit::ChangedUsername(ChangedUsername::new(
        0x1234,
        "é".into(),
    )));
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
}

#[test]
fn undersized_destination_stays_unchanged() {
    let unit = ChangedUsername::new(0x1234, "é".into());
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, vec![0xaa; FRAME.len() - 1]);
}

#[test]
fn username_accepts_255_bytes_and_rejects_256() {
    let max = ChangedUsername::new(0, "x".repeat(255));
    let bytes = max.to_bytes().unwrap();
    assert_eq!(bytes[3], 255);
    assert_eq!(bytes.len(), 259);

    let too_long = ChangedUsername::new(0, "x".repeat(256));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
}

#[test]
fn invalid_utf8_username_is_rejected() {
    assert!(matches!(
        decode(&[0x88, 0, 1, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
