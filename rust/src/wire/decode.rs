use super::DecodeError;
use crate::unit::{
    Add, ChangeUsername, ChangedUsername, FlpcUnit, Kick, Login, MakeAdmin, Message, Ping,
    PrivateMessage, Register, Remove, RemoveAdmin, RequestSync, ServerLogin, ServerMessage,
    ServerPrivateMessage, ServerRegister, SyncMessage, SyncPrivateMessage, UserJoined, UserLeft,
    UserRegistered, UserRemoved,
};

/// Decodes the first unit and returns it with the byte count, including its opcode.
///
/// Returns `Ok(None)` for empty input or an incomplete unit with a supported opcode.
/// The caller retains the input and can decode subsequent units from the remaining bytes.
///
/// # Errors
///
/// Returns [`DecodeError::UnknownOpcode`] for an unassigned opcode,
/// [`DecodeError::UnsupportedOpcode`] for an assigned opcode whose codec is pending,
/// or [`DecodeError::InvalidUTF8`] for an invalid UTF-8 field.
pub fn decode(buf: &[u8]) -> Result<Option<(FlpcUnit, usize)>, DecodeError> {
    let Some(&opcode) = buf.first() else {
        return Ok(None);
    };

    match opcode {
        Ping::OPCODE
        | Register::OPCODE
        | Login::OPCODE
        | Message::OPCODE
        | PrivateMessage::OPCODE
        | ChangeUsername::OPCODE
        | RequestSync::OPCODE
        | RemoveAdmin::OPCODE
        | MakeAdmin::OPCODE
        | Add::OPCODE
        | Remove::OPCODE
        | Kick::OPCODE
        | ServerRegister::OPCODE
        | ServerLogin::OPCODE
        | ServerMessage::OPCODE
        | ServerPrivateMessage::OPCODE
        | UserRegistered::OPCODE
        | UserRemoved::OPCODE
        | UserJoined::OPCODE
        | UserLeft::OPCODE
        | ChangedUsername::OPCODE
        | SyncMessage::OPCODE
        | SyncPrivateMessage::OPCODE => Err(DecodeError::UnsupportedOpcode(opcode)),
        _ => Err(DecodeError::UnknownOpcode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_empty_and_unassigned_input() {
        assert!(matches!(decode(&[]), Ok(None)));
        assert!(matches!(decode(&[0xff]), Err(DecodeError::UnknownOpcode)));
    }
}
