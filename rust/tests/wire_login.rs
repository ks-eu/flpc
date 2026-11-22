use flpc::{
    unit::{ClientUnit, FlpcUnit, Login},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = &[
    0x02, 0x04, 0x5a, 0x6f, 0xc3, 0xab, 0x05, 0x70, 0xc4, 0x81, 0x73, 0x73,
];

fn unit() -> Login {
    Login::new("Zoë".into(), "pāss".into())
}

fn decoded(unit: Login) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Login(unit))
}

#[test]
fn encode_and_decode_match_the_independent_wire_vector() {
    let value = unit();
    let mut dst = vec![0xaa; FRAME.len() + 2];
    assert_eq!(value.encoded_len().unwrap(), FRAME.len());
    assert_eq!(value.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(value.to_bytes().unwrap(), FRAME);
    assert_eq!(decode(FRAME).unwrap(), Some((decoded(unit()), FRAME.len())));
    let adjacent = [FRAME, &[0x00][..]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((decoded(unit()), FRAME.len()))
    );
}

#[test]
fn every_proper_prefix_waits_for_more_bytes() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
}

#[test]
fn short_destination_fails_without_mutation() {
    let mut dst = vec![0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit().encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert!(dst.iter().all(|byte| *byte == 0xaa));
}

#[test]
fn utf8_lengths_and_field_limits_are_measured_in_bytes() {
    let empty = [&FRAME[..1], &[0; 2][..]].concat();
    let empty_unit = Login::new(String::new(), String::new());
    assert_eq!(empty_unit.to_bytes().unwrap(), empty);
    assert_eq!(
        decode(&empty).unwrap(),
        Some((decoded(empty_unit), empty.len()))
    );

    let max = "é".repeat(127) + "x";
    let max_frame = [
        FRAME[..1].to_vec(),
        vec![0xff],
        max.as_bytes().to_vec(),
        vec![0xff],
        max.as_bytes().to_vec(),
    ]
    .concat();
    let max_unit = Login::new(max.clone(), max.clone());
    assert_eq!(max_unit.to_bytes().unwrap(), max_frame);
    assert_eq!(
        decode(&max_frame).unwrap(),
        Some((decoded(max_unit), max_frame.len()))
    );
    let too_long = "é".repeat(128);
    for oversized in [
        Login::new(too_long.clone(), String::from("x")),
        Login::new(String::from("x"), too_long.clone()),
    ] {
        assert!(matches!(
            oversized.encoded_len(),
            Err(EncodeError::StringTooLong)
        ));
        let mut untouched = vec![0xaa; 800];
        assert!(matches!(
            oversized.encode_into(&mut untouched),
            Err(EncodeError::StringTooLong)
        ));
        assert!(untouched.iter().all(|byte| *byte == 0xaa));
    }
}

#[test]
fn invalid_utf8_is_a_decode_error() {
    let invalid = [&FRAME[..1], &[1, 255][..]].concat();
    assert!(matches!(decode(&invalid), Err(DecodeError::InvalidUTF8)));
    let invalid = [&FRAME[..1], &[0, 1, 255][..]].concat();
    assert!(matches!(decode(&invalid), Err(DecodeError::InvalidUTF8)));
}
