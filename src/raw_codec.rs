use crate::header::{
    Encode, Join, JoinSuccess, JoinFail, Message, ClientMessage, UserJoin,
};
use crate::error::ProtocolError;

pub enum ProtocolMessage {
    Join(Join),
    JoinSuccess(JoinSuccess),
    JoinFail(JoinFail),
    Message(Message),
    ClientMessage(ClientMessage),
    UserJoin(UserJoin),
}

impl Encode for ProtocolMessage {
    fn encode(&self) -> Result<Vec<u8>, ProtocolError> {
        match self {
            Self::Join(msg) => msg.encode(),
            Self::JoinSuccess(msg) => msg.encode(),
            Self::JoinFail(msg) => msg.encode(),
            Self::Message(msg) => msg.encode(),
            Self::ClientMessage(msg) => msg.encode(),
            Self::UserJoin(msg) => msg.encode(),
        }
    }
}


