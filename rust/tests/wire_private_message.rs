use flpc::{
    unit::{ClientUnit, FlpcUnit, Ping, PingUnit, PrivateMessage},
    wire::{DecodeError, Encode, EncodeError, decode},
};

const FRAME: &[u8] = b"\x04\x12\x34\x00\x02\xc3\xa9";

fn private_message(target: u16, message: &str) -> PrivateMessage {
    PrivateMessage::new(target, message.into())
}

fn decoded(target: u16, message: &str) -> FlpcUnit {
    FlpcUnit::Client(ClientUnit::PrivateMessage(private_message(target, message)))
}

#[test]
fn encodes_independent_bytes_and_decodes_one_unit() {
    let unit = private_message(0x1234, "é");
    let mut dst = [0xaa; FRAME.len() + 2];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(&dst[FRAME.len()..], &[0xaa; 2]);
    assert_eq!(unit.to_bytes().unwrap(), FRAME);
    assert_eq!(
        decode(FRAME).unwrap(),
        Some((decoded(0x1234, "é"), FRAME.len()))
    );

    let adjacent = [FRAME, &[0x00][..]].concat();
    assert_eq!(
        decode(&adjacent).unwrap(),
        Some((decoded(0x1234, "é"), FRAME.len()))
    );
    assert_eq!(
        decode(&adjacent[FRAME.len()..]).unwrap(),
        Some((FlpcUnit::Ping(PingUnit::Ping(Ping::new())), 1))
    );
}

#[test]
fn waits_for_target_length_and_payload_bytes() {
    for end in 0..FRAME.len() {
        assert_eq!(decode(&FRAME[..end]).unwrap(), None, "prefix {end}");
    }
}

#[test]
fn supports_empty_and_maximum_messages() {
    let empty = [0x04, 0x00, 0x01, 0x00, 0x00];
    assert_eq!(private_message(1, "").to_bytes().unwrap(), empty);
    assert_eq!(decode(&empty).unwrap(), Some((decoded(1, ""), empty.len())));

    let max = "x".repeat(65_535);
    let mut frame = vec![0x04, 0xff, 0xff, 0xff, 0xff];
    frame.extend_from_slice(max.as_bytes());
    let unit = private_message(u16::MAX, &max);
    assert_eq!(unit.encoded_len().unwrap(), frame.len());
    assert_eq!(unit.to_bytes().unwrap(), frame);
    assert_eq!(
        decode(&frame).unwrap(),
        Some((decoded(u16::MAX, &max), frame.len()))
    );
}

#[test]
fn encode_errors_leave_the_destination_unchanged() {
    let mut short = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        private_message(0x1234, "é").encode_into(&mut short),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(short, [0xaa; FRAME.len() - 1]);

    let too_long = private_message(1, &"x".repeat(65_536));
    assert!(matches!(
        too_long.encoded_len(),
        Err(EncodeError::StringTooLong)
    ));
    let mut dst = [0xaa; 5];
    assert!(matches!(
        too_long.encode_into(&mut dst),
        Err(EncodeError::StringTooLong)
    ));
    assert_eq!(dst, [0xaa; 5]);
}

#[test]
fn rejects_invalid_utf8_in_a_complete_message() {
    assert!(matches!(
        decode(&[0x04, 0, 1, 0, 1, 0xff]),
        Err(DecodeError::InvalidUTF8)
    ));
}
