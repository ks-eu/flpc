use flpc::{
    unit::{FlpcUnit, Ping, PingUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x00];

fn decoded() -> FlpcUnit {
    FlpcUnit::Ping(PingUnit::Ping(Ping::new()))
}

#[test]
fn encode_matches_the_readme_vector_and_preserves_the_tail() {
    let unit = Ping::new();
    let mut dst = [0xaa; 3];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_consumes_one_byte_and_leaves_adjacent_frames() {
    assert_eq!(decode(&[]).unwrap(), None);
    assert_eq!(decode(FRAME).unwrap(), Some((decoded(), FRAME.len())));

    let adjacent = [0x00, 0x00];
    assert_eq!(decode(&adjacent).unwrap(), Some((decoded(), 1)));
}

#[test]
fn encode_rejects_an_empty_destination() {
    assert!(matches!(
        Ping::new().encode_into(&mut []),
        Err(EncodeError::BufferTooSmall)
    ));
}
