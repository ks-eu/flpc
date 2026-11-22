use flpc::{
    unit::{FlpcUnit, ServerMessage, ServerUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x82, 0x12, 0x34, 1, 2, 3, 4, 5, 6, 7, 8, 0, 2, 0xC3, 0xA9];

#[test]
fn wire_bytes_and_decode_match_protocol() {
    let unit = ServerMessage::new(0x1234, 0x0102030405060708, "é".into());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ServerMessage(ServerMessage::new(
                0x1234,
                0x0102030405060708,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let adjacent = [FRAME, FRAME].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((
            FlpcUnit::Server(ServerUnit::ServerMessage(ServerMessage::new(
                0x1234,
                0x0102030405060708,
                "é".into()
            ))),
            FRAME.len()
        ))
    );
}

#[test]
fn message_length_prefix_accepts_65535_bytes_and_rejects_more() {
    let max = ServerMessage::new(0, 0, "x".repeat(65_535));
    let bytes = max.to_bytes().unwrap();
    assert_eq!(&bytes[11..13], &[0xff, 0xff]);
    assert_eq!(bytes.len(), 65_548);

    let too_long = ServerMessage::new(0, 0, "x".repeat(65_536));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
}
