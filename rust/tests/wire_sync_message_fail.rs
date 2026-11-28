use flpc::{
    unit::{FlpcUnit, SyncMessageFail},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0xd0, 0x34];

#[test]
fn wire_vector_conversion_and_decode() {
    let unit = SyncMessageFail::new(0x34);
    let expected = FlpcUnit::SyncMessageFail(SyncMessageFail::new(0x34));
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
    let wrapped: FlpcUnit = SyncMessageFail::new(0x34).into();
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
        SyncMessageFail::new(0x34).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert!(dst.iter().all(|byte| *byte == 0xaa));
}
