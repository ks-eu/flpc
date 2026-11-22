use super::DecodeError;
use crate::unit::{
    Add, ChangeUsername, ChangedUsername, ClientUnit, FlpcUnit, Kick, Login, MakeAdmin, Message,
    Ping, PingUnit, PrivateMessage, Register, Remove, RemoveAdmin, RequestSync, ServerLogin,
    ServerMessage, ServerPrivateMessage, ServerRegister, SyncMessage, SyncPrivateMessage,
    UserJoined, UserLeft, UserRegistered, UserRemoved,
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
    let Some((&opcode, mut remaining)) = buf.split_first() else {
        return Ok(None);
    };

    let unit = match opcode {
        Ping::OPCODE => Some(FlpcUnit::Ping(PingUnit::Ping(Ping::new()))),
        Register::OPCODE => {
            let mut fields = [""; 3];
            for field in &mut fields {
                let Some(value) = read_short_string(&mut remaining)? else {
                    return Ok(None);
                };
                *field = value;
            }
            let [token, username, password] = fields;
            let unit = Register::new(token.into(), username.into(), password.into());
            Some(FlpcUnit::Client(ClientUnit::Register(unit)))
        }
        Login::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::Login(Login::new(
                username.into(),
                password.into(),
            ))))
        }
        Message::OPCODE
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
        | SyncPrivateMessage::OPCODE => return Err(DecodeError::UnsupportedOpcode(opcode)),
        _ => return Err(DecodeError::UnknownOpcode),
    };
    Ok(unit.map(|unit| (unit, buf.len() - remaining.len())))
}

fn read_array<const N: usize>(remaining: &mut &[u8]) -> Option<[u8; N]> {
    let (bytes, rest) = remaining.split_first_chunk()?;
    *remaining = rest;
    Some(*bytes)
}

fn read_string<'a>(remaining: &mut &'a [u8], len: usize) -> Result<Option<&'a str>, DecodeError> {
    let Some((bytes, rest)) = remaining.split_at_checked(len) else {
        return Ok(None);
    };
    *remaining = rest;
    std::str::from_utf8(bytes)
        .map(Some)
        .map_err(|_| DecodeError::InvalidUTF8)
}

fn read_short_string<'a>(remaining: &mut &'a [u8]) -> Result<Option<&'a str>, DecodeError> {
    let Some([len]) = read_array(remaining) else {
        return Ok(None);
    };
    read_string(remaining, usize::from(len))
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
