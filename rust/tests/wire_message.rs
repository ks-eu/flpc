use flpc::{
    unit::{ClientUnit, FlpcUnit, Message},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x03\x00\x05hello";

fn unit(message: &str) -> Message {
    Message::new(message.into())
}

fn decoded(message: &str) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::Message(unit(message)))
}

#[test]
fn encodes_independent_bytes_and_preserves_the_tail() {
    let value = unit("hello");
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(value.encoded_len().unwrap(), FRAME.len());
    assert_eq!(value.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(value.to_bytes().unwrap(), FRAME);
}

#[test]
fn decode_waits_for_the_full_payload_and_consumes_one_frame() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }

    let adjacent = [FRAME, &[0x00][..]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((decoded("hello"), FRAME.len()))
    );
}

#[test]
fn accepts_empty_and_maximum_length_messages() {
    let empty_frame = [0x03, 0x00, 0x00];
    let empty = unit("");
    assert_eq!(empty.to_bytes().unwrap(), empty_frame);
    assert_eq!(
        decode(&empty_frame).unwrap(),
        Some((decoded(""), empty_frame.len()))
    );

    let max = "x".repeat(65_535);
    let mut max_frame = vec![0x03, 0xff, 0xff];
    max_frame.extend_from_slice(max.as_bytes());
    let max_unit = unit(&max);
    assert_eq!(max_unit.encoded_len().unwrap(), max_frame.len());
    assert_eq!(max_unit.to_bytes().unwrap(), max_frame);
    assert_eq!(
        decode(&max_frame).unwrap(),
        Some((decoded(&max), max_frame.len()))
    );
}

#[test]
fn encodes_and_decodes_a_length_with_both_prefix_bytes_set() {
    let message = "é".repeat(129);
    assert_eq!(message.len(), 258);
    let mut frame = vec![0x03, 0x01, 0x02];
    frame.extend_from_slice(message.as_bytes());

    assert_eq!(unit(&message).to_bytes().unwrap(), frame);
    assert_eq!(
        decode(&frame).unwrap(),
        Some((decoded(&message), frame.len()))
    );
}

#[test]
fn rejects_messages_over_the_wire_limit_without_writing() {
    let too_long = "x".repeat(65_536);
    let value = unit(&too_long);
    assert!(matches!(
        value.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));

    let mut dst = [0xaa; 3];
    assert!(matches!(
        value.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 3]);
}

#[test]
fn undersized_destination_is_unchanged() {
    let mut dst = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit("hello").encode_into(&mut dst),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(dst, [0xaa; FRAME.len() - 1]);
}

#[test]
fn invalid_utf8_in_a_complete_payload_is_a_decode_error() {
    assert!(matches!(
        decode(&[0x03, 0x00, 0x01, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
