use super::*;

mod join_tests {
    use super::*;

    #[test]
    fn test_encode_join() {
        let join = Join::new("Alice").unwrap();
        assert_eq!(
            join.encode().unwrap(),
            vec![0x01, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65]
        );
    }

    #[test]
    fn test_decode_join() {
        let bytes = vec![0x01, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];
        let join = Join::decode(Some(&bytes)).unwrap();

        assert_eq!(join.length, 5);
        assert_eq!(join.username, "Alice");
    }

    #[test]
    fn test_empty_username() {
        let join = Join::new("").unwrap();

        assert_eq!(join.length, 0);
        assert_eq!(join.username, "");
        assert_eq!(join.encode().unwrap(), vec![0x01, 0x00]);
    }

    #[test]
    fn test_maximum_username_length() {
        let username = "a".repeat(255);
        let join = Join::new(&username).unwrap();

        assert_eq!(join.length, 255);
        assert_eq!(join.encode().unwrap().len(), 257);
    }

    #[test]
    fn test_username_too_long() {
        let username = "a".repeat(256);

        assert!(matches!(
            Join::new(&username),
            Err(ProtocolError::UsernameTooLong)
        ));
    }

    #[test]
    fn test_utf8_username_length() {
        let join = Join::new("é").unwrap();

        assert_eq!(join.length, 2);
        assert_eq!(
            join.encode().unwrap(),
            vec![0x01, 0x02, 0xC3, 0xA9]
        );
    }

    #[test]
    fn test_decode_empty_input() {
        assert!(matches!(
            Join::decode(None),
            Err(ProtocolError::EmptyMessage)
        ));
    }

    #[test]
    fn test_decode_incomplete_header() {
        assert!(matches!(
            Join::decode(Some(&[0x01])),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_incomplete_username() {
        let bytes = vec![0x01, 0x05, 0x41, 0x6C];

        assert!(matches!(
            Join::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        let bytes = vec![0x02, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];

        assert!(matches!(
            Join::decode(Some(&bytes)),
            Err(ProtocolError::InvalidHeader)
        ));
    }

    #[test]
    fn test_decode_invalid_utf8() {
        let bytes = vec![0x01, 0x02, 0xFF, 0xFE];

        assert!(matches!(
            Join::decode(Some(&bytes)),
            Err(ProtocolError::UTF8Error)
        ));
    }

    #[test]
    fn test_length_mismatch() {
        let join = Join {
            length: 3,
            username: "Alice".to_string(),
        };

        assert!(matches!(
            join.encode(),
            Err(ProtocolError::LengthMismatch)
        ));
    }

    #[test]
    fn test_decode_trailing_bytes() {
        let bytes = vec![0x01, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65, 0xFF];
        let join = Join::decode(Some(&bytes)).unwrap();

        assert_eq!(join.username, "Alice");
    }
}

mod join_success_tests {
    use super::*;

    #[test]
    fn test_encode_join_success() {
        let message = JoinSuccess {};
        assert_eq!(message.encode().unwrap(), vec![0x02]);
    }

    #[test]
    fn test_decode_join_success() {
        let bytes = vec![0x02];
        let message = JoinSuccess::decode(Some(&bytes)).unwrap();

        assert_eq!(message.encode().unwrap(), bytes);
    }

    #[test]
    fn test_decode_incomplete_message() {
        assert!(matches!(
            JoinSuccess::decode(Some(&[])),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        assert!(matches!(
            JoinSuccess::decode(Some(&[0x01])),
            Err(ProtocolError::InvalidHeader)
        ));
    }
}

mod join_fail_tests {
    use super::*;

    #[test]
    fn test_encode_join_fail() {
        let message = JoinFail::new(0x01);
        assert_eq!(message.encode().unwrap(), vec![0x03, 0x01]);
    }

    #[test]
    fn test_decode_join_fail() {
        let bytes = vec![0x03, 0x01];
        let message = JoinFail::decode(Some(&bytes)).unwrap();

        assert_eq!(message.id, 0x01);
    }

    #[test]
    fn test_zero_code() {
        let message = JoinFail::new(0x00);
        assert_eq!(message.encode().unwrap(), vec![0x03, 0x00]);
    }

    #[test]
    fn test_maximum_code() {
        let message = JoinFail::new(0xFF);
        assert_eq!(message.encode().unwrap(), vec![0x03, 0xFF]);
    }

    #[test]
    fn test_decode_incomplete_message() {
        assert!(matches!(
            JoinFail::decode(Some(&[0x03])),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        assert!(matches!(
            JoinFail::decode(Some(&[0x02, 0x01])),
            Err(ProtocolError::InvalidHeader)
        ));
    }
}

mod message_tests {
    use super::*;

    #[test]
    fn test_encode_message() {
        let message = Message::new("Hello").unwrap();

        assert_eq!(
            message.encode().unwrap(),
            vec![0x04, 0x00, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F]
        );
    }

    #[test]
    fn test_decode_message() {
        let bytes = vec![0x04, 0x00, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
        let message = Message::decode(Some(&bytes)).unwrap();

        assert_eq!(message.length, 5);
        assert_eq!(message.msg, "Hello");
    }

    #[test]
    fn test_empty_message() {
        let message = Message::new("").unwrap();

        assert_eq!(message.length, 0);
        assert_eq!(message.encode().unwrap(), vec![0x04, 0x00, 0x00]);
    }

    #[test]
    fn test_maximum_message_length() {
        let msg = "a".repeat(65535);
        let message = Message::new(&msg).unwrap();

        assert_eq!(message.length, 65535);
        assert_eq!(message.encode().unwrap().len(), 65538);
    }

    #[test]
    fn test_message_too_long() {
        let msg = "a".repeat(65536);

        assert!(matches!(
            Message::new(&msg),
            Err(ProtocolError::MessageTooLong)
        ));
    }

    #[test]
    fn test_utf8_message_length() {
        let message = Message::new("é").unwrap();

        assert_eq!(message.length, 2);
        assert_eq!(
            message.encode().unwrap(),
            vec![0x04, 0x00, 0x02, 0xC3, 0xA9]
        );
    }

    #[test]
    fn test_decode_incomplete_header() {
        assert!(matches!(
            Message::decode(Some(&[0x04, 0x00])),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_incomplete_payload() {
        let bytes = vec![0x04, 0x00, 0x05, 0x48, 0x65];

        assert!(matches!(
            Message::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        assert!(matches!(
            Message::decode(Some(&[0x01, 0x00, 0x00])),
            Err(ProtocolError::InvalidHeader)
        ));
    }

    #[test]
    fn test_decode_invalid_utf8() {
        let bytes = vec![0x04, 0x00, 0x02, 0xFF, 0xFE];

        assert!(matches!(
            Message::decode(Some(&bytes)),
            Err(ProtocolError::UTF8Error)
        ));
    }

    #[test]
    fn test_length_mismatch() {
        let message = Message {
            length: 3,
            msg: "Hello".to_string(),
        };

        assert!(matches!(
            message.encode(),
            Err(ProtocolError::LengthMismatch)
        ));
    }
}

mod client_message_tests {
    use super::*;

    #[test]
    fn test_encode_client_message() {
        let message = ClientMessage::new(0x1234, "Hello").unwrap();

        assert_eq!(
            message.encode().unwrap(),
            vec![
                0x05, 0x12, 0x34, 0x00, 0x05,
                0x48, 0x65, 0x6C, 0x6C, 0x6F
            ]
        );
    }

    #[test]
    fn test_decode_client_message() {
        let bytes = vec![
            0x05, 0x12, 0x34, 0x00, 0x05,
            0x48, 0x65, 0x6C, 0x6C, 0x6F
        ];

        let message = ClientMessage::decode(Some(&bytes)).unwrap();

        assert_eq!(message.uuid, 0x1234);
        assert_eq!(message.length, 5);
        assert_eq!(message.msg, "Hello");
    }

    #[test]
    fn test_empty_message() {
        let message = ClientMessage::new(0x0001, "").unwrap();

        assert_eq!(
            message.encode().unwrap(),
            vec![0x05, 0x00, 0x01, 0x00, 0x00]
        );
    }

    #[test]
    fn test_maximum_message_length() {
        let msg = "a".repeat(65535);
        let message = ClientMessage::new(0x0001, &msg).unwrap();

        assert_eq!(message.length, 65535);
        assert_eq!(message.encode().unwrap().len(), 65540);
    }

    #[test]
    fn test_message_too_long() {
        let msg = "a".repeat(65536);

        assert!(matches!(
            ClientMessage::new(0x0001, &msg),
            Err(ProtocolError::MessageTooLong)
        ));
    }

    #[test]
    fn test_utf8_message_length() {
        let message = ClientMessage::new(0x0001, "é").unwrap();

        assert_eq!(message.length, 2);
        assert_eq!(
            message.encode().unwrap(),
            vec![0x05, 0x00, 0x01, 0x00, 0x02, 0xC3, 0xA9]
        );
    }

    #[test]
    fn test_from_message() {
        let message = Message::new("Hello").unwrap();
        let client_message = ClientMessage::from_message(message, 0x1234);

        assert_eq!(client_message.uuid, 0x1234);
        assert_eq!(client_message.length, 5);
        assert_eq!(client_message.msg, "Hello");
    }

    #[test]
    fn test_uuid_zero() {
        let message = ClientMessage::new(0x0000, "Hi").unwrap();

        assert_eq!(
            &message.encode().unwrap()[1..3],
            &[0x00, 0x00]
        );
    }

    #[test]
    fn test_uuid_maximum() {
        let message = ClientMessage::new(0xFFFF, "Hi").unwrap();

        assert_eq!(
            &message.encode().unwrap()[1..3],
            &[0xFF, 0xFF]
        );
    }

    #[test]
    fn test_decode_incomplete_header() {
        let bytes = vec![0x05, 0x12, 0x34, 0x00];

        assert!(matches!(
            ClientMessage::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_incomplete_payload() {
        let bytes = vec![0x05, 0x12, 0x34, 0x00, 0x05, 0x48, 0x65];

        assert!(matches!(
            ClientMessage::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        let bytes = vec![0x04, 0x12, 0x34, 0x00, 0x00];

        assert!(matches!(
            ClientMessage::decode(Some(&bytes)),
            Err(ProtocolError::InvalidHeader)
        ));
    }

    #[test]
    fn test_decode_invalid_utf8() {
        let bytes = vec![0x05, 0x00, 0x01, 0x00, 0x02, 0xFF, 0xFE];

        assert!(matches!(
            ClientMessage::decode(Some(&bytes)),
            Err(ProtocolError::UTF8Error)
        ));
    }

    #[test]
    fn test_length_mismatch() {
        let message = ClientMessage {
            uuid: 0x1234,
            length: 3,
            msg: "Hello".to_string(),
        };

        assert!(matches!(
            message.encode(),
            Err(ProtocolError::LengthMismatch)
        ));
    }
}

mod user_join_tests {
    use super::*;

    #[test]
    fn test_encode_user_join() {
        let message = UserJoin::new("Alice", 0x1234).unwrap();

        assert_eq!(
            message.encode().unwrap(),
            vec![0x06, 0x12, 0x34, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65]
        );
    }

    #[test]
    fn test_decode_user_join() {
        let bytes = vec![0x06, 0x12, 0x34, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];
        let message = UserJoin::decode(Some(&bytes)).unwrap();

        assert_eq!(message.uuid, 0x1234);
        assert_eq!(message.length, 5);
        assert_eq!(message.username, "Alice");
    }

    #[test]
    fn test_empty_username() {
        let message = UserJoin::new("", 0x0001).unwrap();

        assert_eq!(
            message.encode().unwrap(),
            vec![0x06, 0x00, 0x01, 0x00]
        );
    }

    #[test]
    fn test_maximum_username_length() {
        let username = "a".repeat(255);
        let message = UserJoin::new(&username, 0x0001).unwrap();

        assert_eq!(message.length, 255);
        assert_eq!(message.encode().unwrap().len(), 259);
    }

    #[test]
    fn test_username_too_long() {
        let username = "a".repeat(256);

        assert!(matches!(
            UserJoin::new(&username, 0x0001),
            Err(ProtocolError::UsernameTooLong)
        ));
    }

    #[test]
    fn test_utf8_username_length() {
        let message = UserJoin::new("é", 0x0001).unwrap();

        assert_eq!(message.length, 2);
        assert_eq!(
            message.encode().unwrap(),
            vec![0x06, 0x00, 0x01, 0x02, 0xC3, 0xA9]
        );
    }

    #[test]
    fn test_uuid_zero() {
        let message = UserJoin::new("Alice", 0x0000).unwrap();

        assert_eq!(
            &message.encode().unwrap()[1..3],
            &[0x00, 0x00]
        );
    }

    #[test]
    fn test_uuid_maximum() {
        let message = UserJoin::new("Alice", 0xFFFF).unwrap();

        assert_eq!(
            &message.encode().unwrap()[1..3],
            &[0xFF, 0xFF]
        );
    }

    #[test]
    fn test_decode_incomplete_header() {
        let bytes = vec![0x06, 0x12, 0x34];

        assert!(matches!(
            UserJoin::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_incomplete_payload() {
        let bytes = vec![0x06, 0x12, 0x34, 0x05, 0x41, 0x6C];

        assert!(matches!(
            UserJoin::decode(Some(&bytes)),
            Err(ProtocolError::IncompleteMessage)
        ));
    }

    #[test]
    fn test_decode_invalid_header() {
        let bytes = vec![0x05, 0x12, 0x34, 0x00];

        assert!(matches!(
            UserJoin::decode(Some(&bytes)),
            Err(ProtocolError::InvalidHeader)
        ));
    }

    #[test]
    fn test_decode_invalid_utf8() {
        let bytes = vec![0x06, 0x00, 0x01, 0x02, 0xFF, 0xFE];

        assert!(matches!(
            UserJoin::decode(Some(&bytes)),
            Err(ProtocolError::UTF8Error)
        ));
    }

    #[test]
    fn test_length_mismatch() {
        let message = UserJoin {
            uuid: 0x1234,
            length: 3,
            username: "Alice".to_string(),
        };

        assert!(matches!(
            message.encode(),
            Err(ProtocolError::LengthMismatch)
        ));
    }
}
