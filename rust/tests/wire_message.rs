use flpc::{
    unit::{ClientUnit, FlpcUnit, Message, Ping, PingUnit},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x03\x00\x05hello";

fn message(value: &str) -> Message {
    Message::new(value.into())
}

fn decoded(value: &str) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Message(message(value)))
}

#[test]
fn encodes_independent_bytes_and_decodes_one_unit() {
    let value = message("hello");
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(value.encoded_len().unwrap(), FRAME.len());
    assert_eq!(value.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(value.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((decoded("hello"), FRAME.len()))
    );

    let adjacent = [FRAME, &[0x00][..]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((decoded("hello"), FRAME.len()))
    );
    assert_eq!(
        decode(&adjacent[FRAME.len()..]).unwrap(),
        Some((FlpcUnit::Ping(PingUnit::Ping(Ping::new())), 1))
    );

    let wrapped: FlpcUnit = Message::new("hello".into()).into();
    assert_eq!(wrapped, decode(FRAME).unwrap().unwrap().0);
    assert_eq!(wrapped.to_bytes().unwrap(), FRAME);
}

#[test]
fn waits_for_each_prefix_and_payload_byte() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
}

#[test]
fn measures_utf8_in_bytes_and_accepts_empty_and_maximum_messages() {
    let empty = [0x03, 0x00, 0x00];
    assert_eq!(message("").to_bytes().unwrap(), empty);
    assert_eq!(decode(&empty).unwrap(), Some((decoded(""), empty.len())));

    let value = "é".repeat(129);
    let mut frame = vec![0x03, 0x01, 0x02];
    frame.extend_from_slice(value.as_bytes());
    assert_eq!(message(&value).to_bytes().unwrap(), frame);
    assert_eq!(
        decode(&frame).unwrap(),
        Some((decoded(&value), frame.len()))
    );

    let max = "x".repeat(65_535);
    let mut max_frame = vec![0x03, 0xff, 0xff];
    max_frame.extend_from_slice(max.as_bytes());
    let max_message = message(&max);
    assert_eq!(max_message.encoded_len().unwrap(), max_frame.len());
    assert_eq!(max_message.to_bytes().unwrap(), max_frame);
    assert_eq!(
        decode(&max_frame).unwrap(),
        Some((decoded(&max), max_frame.len()))
    );
}

#[test]
fn encode_errors_leave_the_destination_unchanged() {
    let mut short = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        message("hello").encode_into(&mut short),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(short, [0xaa; FRAME.len() - 1]);

    let too_long = message(&"x".repeat(65_536));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    let mut dst = [0xaa; 3];
    assert!(matches!(
        too_long.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 3]);
}

#[test]
fn rejects_invalid_utf8_in_a_complete_message() {
    assert!(matches!(
        decode(&[0x03, 0x00, 0x01, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
