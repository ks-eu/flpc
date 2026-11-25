use flpc::{
    unit::{ClientUnit, FlpcUnit, RequestSyncMessage},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x06\x01\x02\x03\x04\x05\x06\x07\x08\xff";

fn unit() -> RequestSyncMessage {
    RequestSyncMessage::new(0x0102_0304_0506_0708, 255)
}

#[test]
fn encodes_and_decodes_literal_bytes_with_client_conversion() {
    let value = unit();
    let mut dst = [0xaa; FRAME.len() + 1];
    assert_eq!(value.encoded_len().unwrap(), FRAME.len());
    assert_eq!(value.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(value.to_bytes().unwrap(), FRAME);
    let wrapped: FlpcUnit = value.clone().into();
    let expected = FlpcUnit::Client(ClientUnit::RequestSyncMessage(value));
    assert_eq!(wrapped, expected);
    assert_eq!(decode(FRAME).unwrap(), Some((expected, FRAME.len())));
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    assert_eq!(
        decode(&[FRAME, FRAME].concat()).unwrap(),
        Some((wrapped, FRAME.len()))
    );
}

#[test]
fn validates_counts_and_preserves_short_buffer() {
    assert_eq!(
        RequestSyncMessage::new(0, 1).to_bytes().unwrap(),
        &[6, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    );
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit().encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
    assert!(matches!(
        RequestSyncMessage::new(1, 0).encoded_len(),
        Err(EncodeError::InvalidCount)
    ));
    assert!(matches!(
        decode(&[6, 0, 0, 0, 0, 0, 0, 0, 1, 0]),
        Err(DecodeError::InvalidCount)
    ));
}
