use flpc::{
    unit::{FlpcUnit, ServerSyncUsername},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0xc2, 0x12, 0x34, 2, 0xc3, 0xa9];

fn unit() -> ServerSyncUsername {
    ServerSyncUsername::new(0x1234, "é".into())
}

#[test]
fn encodes_literal_frame_without_touching_tail() {
    let unit = unit();
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let wrapped: FlpcUnit = unit.into();
    assert_eq!(wrapped, FlpcUnit::ServerSyncUsername(self::unit()));
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decodes_one_frame_and_waits_for_all_fields() {
    let expected = FlpcUnit::ServerSyncUsername(unit());
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((expected.clone(), FRAME.len()))
    );
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    assert_eq!(
        decode(&[FRAME, &[0x00]].concat()).unwrap(),
        Some((expected, FRAME.len()))
    );
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit().encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn username_limits_are_measured_in_utf8_bytes() {
    let max = format!("{}x", "é".repeat(127));
    for (username, prefix) in [("", 0), (max.as_str(), 255)] {
        let unit = ServerSyncUsername::new(0x1234, username.into());
        let bytes = [&[0xc2, 0x12, 0x34, prefix], username.as_bytes()].concat();
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(
            decode(&bytes).unwrap(),
            Some((FlpcUnit::ServerSyncUsername(unit), bytes.len()))
        );
    }

    let too_long = ServerSyncUsername::new(0x1234, "é".repeat(128));
    let mut dst = [0xaa; 260];
    assert!(matches!(
        too_long.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 260]);
    assert!(matches!(
        decode(&[0xc2, 0x12, 0x34, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
