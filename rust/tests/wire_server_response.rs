use flpc::{
    unit::{FlpcUnit, ServerResponse},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x80, 0x00, 0x06, 0x68, 0xc3, 0xa9, 0x6c, 0x6c, 0x6f];

#[test]
fn literal_bytes_match_encoding_and_decoding() {
    let unit = ServerResponse::new("héllo".into());
    let expected = FlpcUnit::ServerResponse(unit.clone());
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
fn utf8_length_and_maximum_are_byte_based() {
    let full = "x".repeat(65_535);
    let frame = ServerResponse::new(full.clone()).to_bytes().unwrap();
    assert_eq!(frame.len(), 65_538);
    assert_eq!(&frame[..3], &[0x80, 0xff, 0xff]);
    assert_eq!(
        decode(&frame).unwrap(),
        Some((
            FlpcUnit::ServerResponse(ServerResponse::new(full)),
            frame.len()
        ))
    );
    let mut dst = [0xaa; 65_539];
    let oversized = ServerResponse::new("é".repeat(32_768));
    assert!(matches!(
        oversized.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    assert!(matches!(
        oversized.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 65_539]);
    assert!(matches!(
        decode(&[0x80, 0, 1, 0xff]),
        Err(flpc::wire::DecodeError::InvalidUTF8)
    ));
    assert_eq!(
        ServerResponse::new(String::new()).to_bytes().unwrap(),
        [0x80, 0, 0]
    );
}
