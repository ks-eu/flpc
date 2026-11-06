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

impl From<std::io::Error> for ProtocolError {
    fn from(error: std::io::Error) -> Self {
        ProtocolError::IoError(error)
    }
}
