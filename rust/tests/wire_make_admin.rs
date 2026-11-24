use flpc::{
    unit::{AdminUnit, FlpcUnit, MakeAdmin, RemoveAdmin},
    wire::{Encode, EncodeError, decode},
};

// Opcode 7C carries unsigned big-endian ID 0x4567.
const FRAME: &[u8] = b"\x7c\x45\x67";

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = MakeAdmin::new(0x4567);
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_id_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x7b, 0x89, 0xab]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((
            FlpcUnit::Admin(AdminUnit::MakeAdmin(MakeAdmin::new(0x4567))),
            FRAME.len(),
        ))
    );
    assert_eq!(
        decode(&adjacent[FRAME.len()..]).unwrap(),
        Some((
            FlpcUnit::Admin(AdminUnit::RemoveAdmin(RemoveAdmin::new(0x89ab))),
            3,
        ))
    );
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        MakeAdmin::new(0x4567).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn converts_to_flpc_unit_and_encodes() {
    let concrete = MakeAdmin::new(0x4567);
    let wrapped: FlpcUnit = concrete.into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}
