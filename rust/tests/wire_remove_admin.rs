use flpc::{
    unit::{FlpcUnit, RemoveAdmin},
    wire::{Encode, EncodeError, decode},
};

// Opcode 64 carries unsigned big-endian ID 0x1234.
const FRAME: &[u8] = b"\x64\x12\x34";

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = RemoveAdmin::new(0x1234);
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);

    let wrapped: FlpcUnit = RemoveAdmin::new(0x1234).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_id_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x64, 0, 0]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((FlpcUnit::RemoveAdmin(RemoveAdmin::new(0x1234)), FRAME.len(),))
    );
}

#[test]
fn round_trips_id_boundaries() {
    for (id, bytes) in [(0, &[0x64, 0, 0][..]), (u16::MAX, &[0x64, 0xff, 0xff][..])] {
        let unit = RemoveAdmin::new(id);
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(
            decode(bytes).unwrap(),
            Some((FlpcUnit::RemoveAdmin(unit), bytes.len(),))
        );
    }
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        RemoveAdmin::new(0x1234).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}
