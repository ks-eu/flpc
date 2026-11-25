use flpc::{
    unit::{ClientUnit, FlpcUnit, RemoveSelf},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x07\x02Zo\x02pw";

fn unit(username: &str, password: &str) -> RemoveSelf {
    RemoveSelf::new(username.into(), password.into())
}

#[test]
fn encodes_independent_bytes_and_converts_into_client_unit() {
    let value = unit("Zo", "pw");
    assert_eq!(value.to_bytes().unwrap(), FRAME);
    let wrapped: FlpcUnit = value.into();
    assert_eq!(
        wrapped,
        FlpcUnit::Client(ClientUnit::RemoveSelf(unit("Zo", "pw")))
    );
}

#[test]
fn decodes_prefixes_and_adjacent_frames() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None);
    }
    assert_eq!(
        decode(&[FRAME, FRAME].concat()).unwrap(),
        Some((
            FlpcUnit::Client(ClientUnit::RemoveSelf(unit("Zo", "pw"))),
            FRAME.len()
        ))
    );
}

#[test]
fn validates_short_string_limits_and_preserves_short_buffer() {
    assert_eq!(unit("", "").to_bytes().unwrap(), &[7, 0, 0]);
    assert_eq!(unit(&"x".repeat(255), "p").to_bytes().unwrap()[1], 255);
    assert!(matches!(
        unit(&"x".repeat(256), "p").encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    let mut dst = [0xaa; 1];
    assert!(matches!(
        unit("u", "p").encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa]);
    assert!(matches!(
        decode(&[7, 1, 255]),
        Err(DecodeError::InvalidUTF8)
    ));
    assert!(!format!("{:?}", unit("u", "secret")).contains("secret"));
}
