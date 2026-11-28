use flpc::{
    unit::{FlpcUnit, ServerPrivateMessage},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// Independent bytes from README opcode 0x83 field layout.
const FRAME: &[u8] = &[
    0x83, 0x12, 0x34, 0x56, 0x78, 1, 2, 3, 4, 5, 6, 7, 8, 0, 2, 0xC3, 0xA9,
];

#[test]
fn encoding_and_decoding_match_the_wire_vector() {
    let unit = ServerPrivateMessage::new(0x1234, 0x5678, 0x0102030405060708, "é".into());
    let expected = FlpcUnit::ServerPrivateMessage(ServerPrivateMessage::new(
        0x1234,
        0x5678,
        0x0102030405060708,
        "é".into(),
    ));
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

    let wrapped: FlpcUnit =
        ServerPrivateMessage::new(0x1234, 0x5678, 0x0102030405060708, "é".into()).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn undersized_destination_stays_unchanged() {
    let unit = ServerPrivateMessage::new(0x1234, 0x5678, 0x0102030405060708, "é".into());
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, vec![0xaa; FRAME.len() - 1]);
}

#[test]
fn message_accepts_65535_bytes_and_rejects_65536() {
    let max = ServerPrivateMessage::new(0, 0, 0, "x".repeat(65_535));
    let bytes = max.to_bytes().unwrap();
    assert_eq!(&bytes[13..15], &[0xff, 0xff]);

    let too_long = ServerPrivateMessage::new(0, 0, 0, "x".repeat(65_536));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
}

#[test]
fn invalid_utf8_message_is_rejected() {
    let mut invalid = FRAME.to_vec();
    *invalid.last_mut().unwrap() = 0xff;
    assert!(matches!(decode(&invalid), Err(DecodeError::InvalidUTF8)));
}
