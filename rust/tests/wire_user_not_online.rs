use flpc::{
    unit::{FlpcUnit, UserNotOnline},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x8e];

fn unit() -> UserNotOnline {
    UserNotOnline::new()
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
    assert_eq!(wrapped, FlpcUnit::UserNotOnline(self::unit()));
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decodes_one_frame_and_waits_for_all_fields() {
    let expected = FlpcUnit::UserNotOnline(unit());
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
