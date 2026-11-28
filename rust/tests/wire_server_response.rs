use flpc::{
    unit::{FlpcUnit, ServerResponse},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x80, 0x00, 0x06, 0x68, 0xc3, 0xa9, 0x6c, 0x6c, 0x6f];

#[test]
fn wire_vector_conversion_and_decode() {
    let unit = ServerResponse::new("héllo".into());
    let expected = FlpcUnit::ServerResponse(ServerResponse::new("héllo".into()));
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
    let wrapped: FlpcUnit = ServerResponse::new("héllo".into()).into();
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
        ServerResponse::new("héllo".into()).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert!(dst.iter().all(|byte| *byte == 0xaa));
}

#[test]
fn utf8_length_and_maximum_are_byte_based() {
    let full = "x".repeat(65_535);
    assert_eq!(
        ServerResponse::new(full.clone()).encoded_len().unwrap(),
        65_538
    );
    assert!(matches!(
        ServerResponse::new(format!("{full}x")).encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    assert!(decode(&[0x80, 0, 1, 0xff]).is_err());
}
