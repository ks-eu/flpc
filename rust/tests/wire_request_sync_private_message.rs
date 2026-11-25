use flpc::unit::{ClientUnit, FlpcUnit, RequestSyncPrivateMessage};
use flpc::wire::{Encode, decode};
const FRAME: &[u8] = &[0x09, 0x12, 0x34, 1, 2, 3, 4, 5, 6, 7, 8, 1];
fn expected() -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::RequestSyncPrivateMessage(
        RequestSyncPrivateMessage::new(0x1234, 0x0102030405060708, 1),
    ))
}
#[test]
fn literal_wire_decode_encode_and_conversion() {
    let unit = RequestSyncPrivateMessage::new(0x1234, 0x0102030405060708, 1);
    let wrapped: FlpcUnit = unit.clone().into();
    assert_eq!(wrapped, expected());
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(decode(FRAME).unwrap(), Some((expected(), FRAME.len())));
    for n in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..n]).unwrap(), None);
    }
    assert_eq!(
        decode(&[FRAME, FRAME].concat()).unwrap(),
        Some((expected(), FRAME.len()))
    );
}
#[test]
fn invalid_zero_count_is_rejected() {
    use flpc::wire::{DecodeError, EncodeError};
    let u = RequestSyncPrivateMessage::new(1, 2, 0);
    assert!(matches!(u.encoded_len(), Err(EncodeError::InvalidCount)));
    assert!(matches!(
        decode(&[9, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0]),
        Err(DecodeError::InvalidCount)
    ));
    assert_eq!(
        RequestSyncPrivateMessage::new(1, 2, 255)
            .to_bytes()
            .unwrap()[11],
        255
    );
}
