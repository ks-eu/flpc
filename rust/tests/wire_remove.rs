use flpc::{
    unit::{AdminUnit, FlpcUnit, Ping, PingUnit, Remove},
    wire::{Encode, EncodeError, decode},
};

// Opcode 7E carries unsigned big-endian ID 0x89AB.
const FRAME: &[u8] = b"\x7e\x89\xab";

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = Remove::new(0x89ab);
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let wrapped: FlpcUnit = Remove::new(0x89ab).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_id_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x00]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((
            FlpcUnit::Admin(AdminUnit::Remove(Remove::new(0x89ab))),
            FRAME.len(),
        ))
    );
    assert_eq!(
        decode(&adjacent[FRAME.len()..]).unwrap(),
        Some((FlpcUnit::Ping(PingUnit::Ping(Ping::new())), 1))
    );
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        Remove::new(0x89ab).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}
