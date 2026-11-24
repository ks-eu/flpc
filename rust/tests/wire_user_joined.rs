use flpc::{
    unit::{FlpcUnit, ServerUnit, UserJoined},
    wire::{Encode, EncodeError, decode},
};

// Independent bytes from README opcode 0x86 field layout.
const FRAME: &[u8] = &[0x86, 0x12, 0x34];

#[test]
fn encoding_and_decoding_match_the_wire_vector() {
    let unit = UserJoined::new(0x1234);
    let expected = FlpcUnit::Server(ServerUnit::UserJoined(UserJoined::new(0x1234)));
    let mut dst = [0xaa; FRAME.len() + 2];
    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((expected.clone(), FRAME.len()))
    );

    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
    let adjacent = [FRAME, FRAME].concat();
    assert_eq!(decode(&adjacent).unwrap(), Some((expected, FRAME.len())));
}

#[test]
fn undersized_destination_stays_unchanged() {
    let unit = UserJoined::new(0x1234);
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, vec![0xaa; FRAME.len() - 1]);
}

#[test]
fn converts_to_flpc_unit_and_encodes() {
    let concrete = UserJoined::new(0x1234);
    let wrapped: FlpcUnit = concrete.into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}
