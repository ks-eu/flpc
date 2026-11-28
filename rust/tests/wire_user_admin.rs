use flpc::{
    unit::{FlpcUnit, UserAdmin},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x86, 0x12, 0x34];

#[test]
fn literal_bytes_match_encoding_and_decoding() {
    let unit = UserAdmin::new(0x1234);
    let expected = FlpcUnit::UserAdmin(unit.clone());
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
