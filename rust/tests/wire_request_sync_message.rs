use flpc::{
    unit::{FlpcUnit, RequestSyncMessage},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x06, 1, 2, 3, 4, 5, 6, 7, 8, 255];

fn unit() -> RequestSyncMessage {
    RequestSyncMessage::new(0x0102_0304_0506_0708, 255)
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
    assert_eq!(wrapped, FlpcUnit::RequestSyncMessage(self::unit()));
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decodes_one_frame_and_waits_for_all_fields() {
    let expected = FlpcUnit::RequestSyncMessage(unit());
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
fn validates_request_count_before_writing() {
    let invalid = RequestSyncMessage::new(0x0102_0304_0506_0708, 0);
    let mut dst = [0xaa; FRAME.len()];
    assert!(matches!(
        invalid.encoded_len(),
        Err(EncodeError::InvalidCount)
    ));
    assert!(matches!(
        invalid.encode_into(&mut dst),
        Err(EncodeError::InvalidCount)
    ));
    assert_eq!(dst, [0xaa; FRAME.len()]);
    assert!(matches!(
        decode(&[0x06, 1, 2, 3, 4, 5, 6, 7, 8, 0]),
        Err(DecodeError::InvalidCount)
    ));

    let limit = RequestSyncMessage::new(0x0102_0304_0506_0708, 1);
    let bytes = &[0x06, 1, 2, 3, 4, 5, 6, 7, 8, 1];
    assert_eq!(limit.to_bytes().unwrap(), bytes);
    assert_eq!(
        decode(bytes).unwrap(),
        Some((FlpcUnit::RequestSyncMessage(limit), bytes.len()))
    );
}
