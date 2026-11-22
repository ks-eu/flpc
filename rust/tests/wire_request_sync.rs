use flpc::{
    unit::{ClientUnit, FlpcUnit, RequestSync},
    wire::{Encode, EncodeError, decode},
};

// Opcode 06 carries first=0x0102030405060708 then last=0x1122334455667788.
const FRAME: &[u8] = b"\x06\x01\x02\x03\x04\x05\x06\x07\x08\x11\x22\x33\x44\x55\x66\x77\x88";

fn unit() -> RequestSync {
    RequestSync::new(0x0102_0304_0506_0708, 0x1122_3344_5566_7788)
}

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = unit();
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_all_bytes_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [
        FRAME,
        &[0x06, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]
    .concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((
            FlpcUnit::Client(ClientUnit::RequestSync(unit())),
            FRAME.len(),
        ))
    );
}

#[test]
fn round_trips_bounds_as_opaque_big_endian_values() {
    let unit = RequestSync::new(u64::MAX, 0);
    let bytes = [
        0x06, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    assert_eq!(unit.to_bytes().unwrap(), bytes);
    assert_eq!(
        decode(&bytes).unwrap(),
        Some((FlpcUnit::Client(ClientUnit::RequestSync(unit)), bytes.len(),))
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
