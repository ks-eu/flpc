use flpc::{
    unit::{ChangeUsername, ClientUnit, FlpcUnit},
    wire::{DecodeError, Encode, EncodeError, decode},
};

// Opcode 05 carries one one-byte-length UTF-8 username.
const FRAME: &[u8] = b"\x05\x02\xc3\xa9";

fn expected() -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::ChangeUsername(ChangeUsername::new("é".into())))
}

#[test]
fn encode_matches_independent_bytes_and_preserves_tail() {
    let unit = ChangeUsername::new("é".into());
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        ChangeUsername::new("é".into()).encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn decode_waits_for_complete_username_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x05, 0]].concat();
    assert_eq!(decode(&adjacent).unwrap(), Some((expected(), FRAME.len())));
}

#[test]
fn empty_and_255_byte_usernames_round_trip() {
    let full = "x".repeat(255);
    for (unit, bytes) in [
        (ChangeUsername::new(String::new()), &[0x05, 0][..]),
        (
            ChangeUsername::new(full.clone()),
            &[&[0x05, 0xff][..], full.as_bytes()].concat(),
        ),
    ] {
        assert_eq!(unit.to_bytes().unwrap(), bytes);
        assert_eq!(
            decode(bytes).unwrap(),
            Some((
                FlpcUnit::Client(ClientUnit::ChangeUsername(unit)),
                bytes.len(),
            ))
        );
    }
}

#[test]
fn oversized_username_is_rejected_before_writing() {
    let unit = ChangeUsername::new("é".repeat(128));
    let mut dst = [0xaa; 259];
    assert!(matches!(
        unit.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    assert!(matches!(
        unit.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 259]);
}

#[test]
fn invalid_utf8_is_a_decode_error() {
    assert!(matches!(
        decode(&[0x05, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}

#[test]
fn converts_to_flpc_unit_and_encodes() {
    let concrete = ChangeUsername::new("é".into());
    let wrapped: FlpcUnit = concrete.into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}
