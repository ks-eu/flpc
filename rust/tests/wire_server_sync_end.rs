use flpc::{
    unit::{FlpcUnit, ServerSyncEnd, ServerUnit},
    wire::{Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[0x8d, 1];

fn unit() -> ServerSyncEnd {
    ServerSyncEnd::new(1)
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
    assert_eq!(
        wrapped,
        FlpcUnit::Server(ServerUnit::ServerSyncEnd(self::unit()))
    );
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn decodes_one_frame_and_waits_for_all_fields() {
    let expected = FlpcUnit::Server(ServerUnit::ServerSyncEnd(unit()));
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
fn zero_count_ends_an_empty_batch() {
    for count in [0, 255] {
        let unit = ServerSyncEnd::new(count);
        let bytes = [0x8d, count];
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(
            decode(&bytes).unwrap(),
            Some((
                FlpcUnit::Server(ServerUnit::ServerSyncEnd(unit)),
                bytes.len()
            ))
        );
    }
}
