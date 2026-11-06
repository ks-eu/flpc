use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::header::{
    Join,
    JoinSuccess,
    JoinFail,
    Message,
    ClientMessage,
    UserJoin,
};

use crate::error::ProtocolError;
use crate::tokio::{FLPCodec, FLPCMsg};

mod encoder {
    use super::*;

    mod valid_messages {
        use super::*;

        #[test]
        fn test_encode_join() {
            let message = Join::new("Alice").unwrap();
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::Join(message), &mut buffer).unwrap();

            assert_eq!(&buffer[..], &[0x01, 0x05, b'A', b'l', b'i', b'c', b'e']);
        }

        #[test]
        fn test_encode_join_success() {
            let message = JoinSuccess {};
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::JoinSuccess(message), &mut buffer).unwrap();

            assert_eq!(&buffer[..], &[0x02]);
        }

        #[test]
        fn test_encode_join_fail() {
            let message = JoinFail::new(0x01);
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::JoinFail(message), &mut buffer).unwrap();

            assert_eq!(&buffer[..], &[0x03, 0x01]);
        }

        #[test]
        fn test_encode_message() {
            let message = Message::new("Hello").unwrap();
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::Message(message), &mut buffer).unwrap();

            assert_eq!(&buffer[..], &[0x04, 0x00, 0x05, b'H', b'e', b'l', b'l', b'o']);
        }

        #[test]
        fn test_encode_client_message() {
            let message = ClientMessage::new(0x1234, "Hello").unwrap();
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::ClientMessage(message), &mut buffer).unwrap();

            assert_eq!(
                &buffer[..],
                &[0x05, 0x12, 0x34, 0x00, 0x05, b'H', b'e', b'l', b'l', b'o']
            );
        }

        #[test]
        fn test_encode_user_join() {
            let message = UserJoin::new("Alice", 0x1234).unwrap();
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::new();

            codec.encode(FLPCMsg::UserJoin(message), &mut buffer).unwrap();

            assert_eq!(
                &buffer[..],
                &[0x06, 0x12, 0x34, 0x05, b'A', b'l', b'i', b'c', b'e']
            );
        }
    }

    mod invalid_messages {
        use super::*;

        #[test]
        fn test_join_username_too_long() {
            let username = "A".repeat(256);

            assert!(matches!(
                Join::new(&username),
                Err(ProtocolError::UsernameTooLong)
            ));
        }

        #[test]
        fn test_message_too_long() {
            let message = "A".repeat(65536);

            assert!(matches!(
                Message::new(&message),
                Err(ProtocolError::MessageTooLong)
            ));
        }

        #[test]
        fn test_client_message_too_long() {
            let message = "A".repeat(65536);

            assert!(matches!(
                ClientMessage::new(0x1234, &message),
                Err(ProtocolError::MessageTooLong)
            ));
        }

        #[test]
        fn test_user_join_username_too_long() {
            let username = "A".repeat(256);

            assert!(matches!(
                UserJoin::new(&username, 0x1234),
                Err(ProtocolError::UsernameTooLong)
            ));
        }
    }
}

mod decoder {
    use super::*;

    mod complete_messages {
        use super::*;

        #[test]
        fn test_decode_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x01, 0x01, b'A'][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::Join(_))));
            assert!(buffer.is_empty());
        }

        #[test]
        fn test_decode_join_success() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x02][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::JoinSuccess(_))));
            assert!(buffer.is_empty());
        }

        #[test]
        fn test_decode_join_fail() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x03, 0x01][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::JoinFail(_))));
            assert!(buffer.is_empty());
        }

        #[test]
        fn test_decode_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x04, 0x00, 0x02, b'H', b'i'][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::Message(_))));
            assert!(buffer.is_empty());
        }

        #[test]
        fn test_decode_client_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x05, 0x12, 0x34, 0x00, 0x02, b'H', b'i'][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::ClientMessage(_))));
            assert!(buffer.is_empty());
        }

        #[test]
        fn test_decode_user_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x06, 0x12, 0x34, 0x01, b'A'][..]);

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::UserJoin(_))));
            assert!(buffer.is_empty());
        }
    }

    mod incomplete_messages {
        use super::*;

        #[test]
        fn test_incomplete_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x01, 0x03, b'A'][..]);

            assert!(codec.decode(&mut buffer).unwrap().is_none());
            assert_eq!(&buffer[..], &[0x01, 0x03, b'A']);
        }

        #[test]
        fn test_incomplete_join_fail() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x03][..]);

            assert!(codec.decode(&mut buffer).unwrap().is_none());
            assert_eq!(&buffer[..], &[0x03]);
        }

        #[test]
        fn test_incomplete_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x04, 0x00, 0x03, b'H', b'i'][..]);

            assert!(codec.decode(&mut buffer).unwrap().is_none());
            assert_eq!(&buffer[..], &[0x04, 0x00, 0x03, b'H', b'i']);
        }

        #[test]
        fn test_incomplete_client_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x05, 0x12, 0x34, 0x00, 0x02, b'H'][..]);

            assert!(codec.decode(&mut buffer).unwrap().is_none());
            assert_eq!(&buffer[..], &[0x05, 0x12, 0x34, 0x00, 0x02, b'H']);
        }

        #[test]
        fn test_incomplete_user_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x06, 0x12, 0x34, 0x02, b'A'][..]);

            assert!(codec.decode(&mut buffer).unwrap().is_none());
            assert_eq!(&buffer[..], &[0x06, 0x12, 0x34, 0x02, b'A']);
        }
    }

    mod invalid_messages {
        use super::*;

        #[test]
        fn test_unknown_header() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0xFF][..]);

            assert!(matches!(
                codec.decode(&mut buffer),
                Err(ProtocolError::UnknownHeader)
            ));
        }

        #[test]
        fn test_invalid_utf8_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x01, 0x01, 0xFF][..]);

            assert!(matches!(
                codec.decode(&mut buffer),
                Err(ProtocolError::UTF8Error)
            ));
        }

        #[test]
        fn test_invalid_utf8_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x04, 0x00, 0x01, 0xFF][..]);

            assert!(matches!(
                codec.decode(&mut buffer),
                Err(ProtocolError::UTF8Error)
            ));
        }

        #[test]
        fn test_invalid_utf8_client_message() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x05, 0x12, 0x34, 0x00, 0x01, 0xFF][..]);

            assert!(matches!(
                codec.decode(&mut buffer),
                Err(ProtocolError::UTF8Error)
            ));
        }

        #[test]
        fn test_invalid_utf8_user_join() {
            let mut codec = FLPCodec::new();
            let mut buffer = BytesMut::from(&[0x06, 0x12, 0x34, 0x01, 0xFF][..]);

            assert!(matches!(
                codec.decode(&mut buffer),
                Err(ProtocolError::UTF8Error)
            ));
        }
    }

    mod multiple_messages {
        use super::*;

        #[test]
        fn test_decode_multiple_messages() {
            let mut codec = FLPCodec::new();

            let mut buffer = BytesMut::from(
                &[
                    0x02,
                    0x01, 0x01, b'A',
                    0x04, 0x00, 0x02, b'H', b'i',
                ][..]
            );

            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::JoinSuccess(_))));
            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::Join(_))));
            assert!(matches!(codec.decode(&mut buffer).unwrap(), Some(FLPCMsg::Message(_))));

            assert!(buffer.is_empty());
        }
    }
}
