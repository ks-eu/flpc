use crate::error::ProtocolError;

#[derive(Debug)]
pub struct Join {
    length: u8,
    username: String,
}

#[derive(Debug)]
pub struct JoinSuccess {}

#[derive(Debug)]
pub struct JoinFail {
    id: u8,
}

#[derive(Debug)]
pub struct Message {
    length: u16,
    msg: String,
}

#[derive(Debug)]
pub struct ClientMessage {
    uuid: u16,
    length: u16,
    msg: String,
}

#[derive(Debug)]
pub struct UserJoin {
    uuid: u16,
    length: u8,
    username: String,
}

pub trait Encode {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError>;
}

pub trait Decode: Sized {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError>;
}

impl Join {
    pub fn new(username: &str) -> Result<Self, ProtocolError> {
        if username.len() > 255 {
            return Err(ProtocolError::UsernameTooLong);
        }

        Ok(Self {
            length: username.len() as u8,
            username: username.to_string(),
        })
    }
}

impl Encode for Join {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.username.len() != self.length as usize {
            return Err(ProtocolError::LengthMismatch);
        }
        if self.username.len() > 255 {
            return Err(ProtocolError::UsernameTooLong);
        }

        let mut buffer = Vec::<u8>::new();
        buffer.push(0x01);
        buffer.push(self.length);
        buffer.extend_from_slice(self.username.as_bytes());

        Ok(buffer)
    }
}

impl Decode for Join {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 2 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x01 {
            return Err(ProtocolError::InvalidHeader);
        }

        let length = bytes[1] as usize;
        if bytes.len() < 2 + length {
            return Err(ProtocolError::IncompleteMessage);
        }
        
        let byte_username = &bytes[2..(2+length)];
        
        Ok(Self {
            length: length as u8,
            username: match String::from_utf8(byte_username.to_vec()) {
                Ok(username) => username,
                Err(_) => return Err(ProtocolError::UTF8Error),
            }
        })
    }
}

impl Encode for JoinSuccess {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(vec![0x02])
    }
}

impl Decode for JoinSuccess {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 1 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x02 {
            return Err(ProtocolError::InvalidHeader);
        }

        Ok(Self {})
    }
}

impl JoinFail {
    pub fn new(code: u8) -> Self {
        Self {
            id: code,
        }
    }
}

impl Encode for JoinFail {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError>  {
        Ok(vec![0x03, self.id])
    }
}


impl Decode for JoinFail {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 2 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x03 {
            return Err(ProtocolError::InvalidHeader);
        }

        Ok(Self {
            id: bytes[1],
        })
    }
}

impl Message {
    pub fn new(msg: &str) -> Result<Self, ProtocolError> {
        if msg.len() > 65535 {
            return Err(ProtocolError::MessageTooLong);
        }

        Ok(Self {
            length: msg.len() as u16,
            msg: msg.to_string(),
        })
    }
}

impl Encode for Message {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.msg.len() != self.length as usize {
            return Err(ProtocolError::LengthMismatch);
        }
        if self.msg.len() > 65535 {
            return Err(ProtocolError::MessageTooLong);
        }

        let mut buffer = Vec::<u8>::new();
        buffer.push(0x04);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.msg.as_bytes());

        Ok(buffer)
    }
}

impl Decode for Message {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 3 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x04 {
            return Err(ProtocolError::InvalidHeader);
        }

        let length = u16::from_be_bytes([bytes[1], bytes[2]]) as usize;

        if bytes.len() < 3 + length {
            return Err(ProtocolError::IncompleteMessage);
        }

        let byte_msg = &bytes[3..(3 + length)];

        Ok(Self {
            length: length as u16,
            msg: match String::from_utf8(byte_msg.to_vec()) {
                Ok(msg) => msg,
                Err(_) => return Err(ProtocolError::UTF8Error),
            },
        })
    }
}

impl ClientMessage {
    pub fn new(uuid: u16, msg: &str) -> Result<Self, ProtocolError> {
        if msg.len() > 65535 {
            return Err(ProtocolError::MessageTooLong);
        }
        
        Ok(Self {
            uuid: uuid,
            length: msg.len() as u16,
            msg: msg.to_string(),
        })
    }

    pub fn from_message(message: Message, uuid: u16) -> Self {
        Self {
            uuid: uuid,
            length: message.length,
            msg: message.msg,
        }
    }
}

impl Encode for ClientMessage {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.msg.len() != self.length as usize {
            return Err(ProtocolError::LengthMismatch);
        }
        if self.msg.len() > 65535 {
            return Err(ProtocolError::MessageTooLong);
        }

        let mut buffer = Vec::<u8>::new();
        buffer.push(0x05);
        buffer.extend_from_slice(&self.uuid.to_be_bytes());
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.extend_from_slice(&self.msg.as_bytes());

        Ok(buffer)
    }
}

impl Decode for ClientMessage {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 5 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x05 {
            return Err(ProtocolError::InvalidHeader);
        }

        let uuid = u16::from_be_bytes([bytes[1], bytes[2]]);
        let length = u16::from_be_bytes([bytes[3], bytes[4]]) as usize;

        if bytes.len() < 5 + length {
            return Err(ProtocolError::IncompleteMessage);
        }

        let byte_msg = &bytes[5..(5 + length)];

        Ok(Self {
            uuid,
            length: length as u16,
            msg: match String::from_utf8(byte_msg.to_vec()) {
                Ok(msg) => msg,
                Err(_) => return Err(ProtocolError::UTF8Error),
            },
        })
    }
}

impl UserJoin {
    pub fn new(username: &str, uuid: u16) -> Result<Self, ProtocolError> {
        if username.len() > 255 {
            return Err(ProtocolError::UsernameTooLong);
        }

        Ok(Self {
            uuid: uuid,
            length: username.len() as u8,
            username: username.to_string(),
        })
    }
}

impl Encode for UserJoin {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        if self.username.len() != self.length as usize {
            return Err(ProtocolError::LengthMismatch);
        }
        if self.username.len() > 255 {
            return Err(ProtocolError::UsernameTooLong);
        }

        let mut buffer = Vec::<u8>::new();
        buffer.push(0x06);
        buffer.extend_from_slice(&self.uuid.to_be_bytes());
        buffer.push(self.length);
        buffer.extend_from_slice(&self.username.as_bytes());

        Ok(buffer)
    }
}

impl Decode for UserJoin {
    fn decode(bytes: Option<&[u8]>) -> Result<Self, ProtocolError> {
        let bytes = match bytes {
            Some(bytes) => bytes,
            None => return Err(ProtocolError::EmptyMessage),
        };

        if bytes.len() < 4 {
            return Err(ProtocolError::IncompleteMessage);
        }

        if bytes[0] != 0x06 {
            return Err(ProtocolError::InvalidHeader);
        }

        let uuid = u16::from_be_bytes([bytes[1], bytes[2]]);
        let length = bytes[3] as usize;

        if bytes.len() < 4 + length {
            return Err(ProtocolError::IncompleteMessage);
        }

        let byte_username = &bytes[4..(4 + length)];

        Ok(Self {
            uuid,
            length: length as u8,
            username: match String::from_utf8(byte_username.to_vec()) {
                Ok(username) => username,
                Err(_) => return Err(ProtocolError::UTF8Error),
            },
        })
    }
}

#[cfg(test)]
#[path = "tests/header.rs"]
mod tests;
