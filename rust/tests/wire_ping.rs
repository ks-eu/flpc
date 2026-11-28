use flpc::{
    unit::{FlpcUnit, Ping, Register},
    wire::{Encode, EncodeError, decode},
};

const PING: &[u8] = &[0x00];
const REGISTER: &[u8] = b"\x01\x01t\x02\xc3\xa9\x02pw";

fn decoded() -> FlpcUnit {
    FlpcUnit::Ping(Ping::new())
}

#[test]
fn encode_matches_the_readme_vector_and_preserves_the_tail() {
    let unit = Ping::new();
    let mut dst = [0xaa; 3];

    assert_eq!(unit.encoded_len().unwrap(), PING.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), PING.len());
    assert_eq!(&dst[..PING.len()], PING);
    assert_eq!(&dst[PING.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), PING);

    let wrapped: FlpcUnit = Ping::new().into();
    assert_eq!(wrapped, decode(PING).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), PING);
}

#[test]
fn decode_consumes_one_byte_before_a_differently_sized_unit() {
    assert_eq!(decode(&[]).unwrap(), None);
    assert_eq!(decode(PING).unwrap(), Some((decoded(), PING.len())));

    let stream = [PING, PING, REGISTER].concat();
    let (first, consumed) = decode(&stream).unwrap().unwrap();
    assert_eq!(first, decoded());
    assert_eq!(consumed, PING.len());

    let (second, consumed) = decode(&stream[consumed..]).unwrap().unwrap();
    assert_eq!(second, decoded());
    assert_eq!(consumed, PING.len());

    assert_eq!(
        decode(&stream[2 * PING.len()..]).unwrap(),
        Some((
            FlpcUnit::Register(Register::new("t".into(), "é".into(), "pw".into(),)),
            REGISTER.len(),
        ))
    );
}

#[test]
fn encode_rejects_an_empty_destination() {
    assert!(matches!(
        Ping::new().encode_into(&mut []),
        Err(EncodeError::BufferTooSmall)
    ));
}
