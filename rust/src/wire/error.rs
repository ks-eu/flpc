//! Errors for encoding and decoding

pub enum EncodeError {
    StringTooLong,
    BufferTooSmall,
}

pub enum DecodeError {
    UnknownOpcode,
    InvalidUTF8,
} 
