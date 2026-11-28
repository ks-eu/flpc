use flpc::{
    unit::{FlpcUnit, ServerRegister},
    wire::{Encode, EncodeError, decode},
};

// Independent bytes from README opcode 0x90 field layout.
const FRAME: &[u8] = &[0x90, 0xA5];

#[test]
fn encoding_and_decoding_match_the_wire_vector() {
    let unit = ServerRegister::new(0xA5);
    let expected = FlpcUnit::ServerRegister(ServerRegister::new(0xA5));
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
    let unit = ServerRegister::new(0xA5);
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, vec![0xaa; FRAME.len() - 1]);
}
