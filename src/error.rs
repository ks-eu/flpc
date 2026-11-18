use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProtocolError {
    UsernameTooLong,
    LengthMismatch,
    MessageTooLong,
    InvalidHeader,
    IncompleteMessage,
    EmptyMessage,
    UTF8Error,
    UnknownHeader,
    IoError(std::io::Error),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::UsernameTooLong => {
                "username exceeds the protocol length limit"
            }
            Self::LengthMismatch => {
                "declared length differs from payload length"
            }
            Self::MessageTooLong => "message exceeds the protocol length limit",
            Self::InvalidHeader => "invalid header for this message type",
            Self::IncompleteMessage => "incomplete message",
            Self::EmptyMessage => "empty message",
            Self::UTF8Error => "message contains invalid UTF-8",
            Self::UnknownHeader => "unknown message header",
            Self::IoError(error) => {
                return write!(formatter, "protocol I/O error: {error}");
            }
        };

        formatter.write_str(message)
    }
}

impl Error for ProtocolError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IoError(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProtocolError {
    fn from(error: std::io::Error) -> Self {
        ProtocolError::IoError(error)
    }
}
