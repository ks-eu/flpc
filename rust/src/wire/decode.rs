use super::DecodeError;
use crate::unit::{
    Add, ChangeUsername, ChangedUsername, FlpcUnit, Kick, Login, MakeAdmin, Message, Ping,
    PrivateMessage, Register, Remove, RemoveAdmin, RemoveSelf, RequestSyncMessage,
    RequestSyncPrivateMessage, RequestSyncUsername, ServerLogin, ServerMessage,
    ServerPrivateMessage, ServerRegister, ServerRemoveSelf, ServerSyncEnd, ServerSyncUsername,
    SyncMessage, SyncPrivateMessage, UserJoined, UserLeft, UserNotOnline, UserRegistered,
    UserRemoved, WrongUnit,
};

/// Decodes the first unit and returns it with the byte count, including its opcode.
///
/// Returns `Ok(None)` for empty input or an incomplete unit with a supported opcode.
/// The caller retains the input and can decode subsequent units from the remaining bytes.
///
/// # Errors
///
/// Returns [`DecodeError::UnknownOpcode`] for an unassigned opcode,
/// [`DecodeError::InvalidUTF8`] for an invalid UTF-8 field, or
/// [`DecodeError::InvalidCount`] for a zero synchronization request count.
pub fn decode(buf: &[u8]) -> Result<Option<(FlpcUnit, usize)>, DecodeError> {
    let Some((&opcode, mut remaining)) = buf.split_first() else {
        return Ok(None);
    };

    let unit = match opcode {
        // 0x00
        Ping::OPCODE => Some(FlpcUnit::Ping(Ping::new())),
        // 0x01
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
            Some(FlpcUnit::Register(unit))
        }
        // 0x03
        Login::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Login(Login::new(
                username.into(),
                password.into(),
            )))
        }
        // 0x10
        Message::OPCODE => {
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Message(Message::new(message.into())))
        }
        // 0x11
        PrivateMessage::OPCODE => {
            let Some(target) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::PrivateMessage(PrivateMessage::new(
                u16::from_be_bytes(target),
                message.into(),
            )))
        }
        // 0x12
        ChangeUsername::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::ChangeUsername(ChangeUsername::new(
                username.into(),
            )))
        }
        // 0x20
        RequestSyncMessage::OPCODE => {
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some([count]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            if count == 0 {
                return Err(DecodeError::InvalidCount);
            }
            Some(FlpcUnit::RequestSyncMessage(RequestSyncMessage::new(
                u64::from_be_bytes(timestamp),
                count,
            )))
        }
        // 0x02
        RemoveSelf::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::RemoveSelf(RemoveSelf::new(
                username.into(),
                password.into(),
            )))
        }
        // 0x22
        RequestSyncUsername::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::RequestSyncUsername(RequestSyncUsername::new(
                u16::from_be_bytes(id),
            )))
        }
        // 0x21
        RequestSyncPrivateMessage::OPCODE => {
            let Some(target) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some([count]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            if count == 0 {
                return Err(DecodeError::InvalidCount);
            }
            Some(FlpcUnit::RequestSyncPrivateMessage(
                RequestSyncPrivateMessage::new(
                    u16::from_be_bytes(target),
                    u64::from_be_bytes(timestamp),
                    count,
                ),
            ))
        }
        // 0x64
        RemoveAdmin::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::RemoveAdmin(RemoveAdmin::new(u16::from_be_bytes(
                id,
            ))))
        }
        // 0x63
        MakeAdmin::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::MakeAdmin(MakeAdmin::new(u16::from_be_bytes(id))))
        }
        // 0x60
        Add::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Add(Add::new(username.into(), password.into())))
        }
        // 0x61
        Remove::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Remove(Remove::new(u16::from_be_bytes(id))))
        }
        // 0x62
        Kick::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Kick(Kick::new(u16::from_be_bytes(id))))
        }
        // 0x90
        ServerRegister::OPCODE => {
            let Some([outcome]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerRegister(ServerRegister::new(outcome)))
        }
        // 0x92
        ServerLogin::OPCODE => {
            let Some([outcome]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerLogin(ServerLogin::new(outcome)))
        }
        // 0xA0
        ServerMessage::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerMessage(ServerMessage::new(
                u16::from_be_bytes(id),
                u64::from_be_bytes(timestamp),
                message.into(),
            )))
        }
        // 0xA1
        ServerPrivateMessage::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(target) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerPrivateMessage(ServerPrivateMessage::new(
                u16::from_be_bytes(id),
                u16::from_be_bytes(target),
                u64::from_be_bytes(timestamp),
                message.into(),
            )))
        }
        // 0x81
        UserRegistered::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::UserRegistered(UserRegistered::new(
                u16::from_be_bytes(id),
                username.into(),
            )))
        }
        // 0x82
        UserRemoved::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::UserRemoved(UserRemoved::new(u16::from_be_bytes(
                id,
            ))))
        }
        // 0x83
        UserJoined::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::UserJoined(UserJoined::new(u16::from_be_bytes(
                id,
            ))))
        }
        // 0x84
        UserLeft::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::UserLeft(UserLeft::new(u16::from_be_bytes(id))))
        }
        // 0x85
        ChangedUsername::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::ChangedUsername(ChangedUsername::new(
                u16::from_be_bytes(id),
                username.into(),
            )))
        }
        // 0xC0
        SyncMessage::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::SyncMessage(SyncMessage::new(
                u16::from_be_bytes(id),
                u64::from_be_bytes(timestamp),
                message.into(),
            )))
        }
        // 0xC1
        SyncPrivateMessage::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(target) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(timestamp) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::SyncPrivateMessage(SyncPrivateMessage::new(
                u16::from_be_bytes(id),
                u16::from_be_bytes(target),
                u64::from_be_bytes(timestamp),
                message.into(),
            )))
        }
        // 0x94
        WrongUnit::OPCODE => Some(FlpcUnit::WrongUnit(WrongUnit::new())),
        // 0x91
        ServerRemoveSelf::OPCODE => {
            let Some([result]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerRemoveSelf(ServerRemoveSelf::new(result)))
        }
        // 0xC3
        ServerSyncEnd::OPCODE => {
            let Some([count]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerSyncEnd(ServerSyncEnd::new(count)))
        }
        // 0xB2
        UserNotOnline::OPCODE => Some(FlpcUnit::UserNotOnline(UserNotOnline::new())),
        // 0xC2
        ServerSyncUsername::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::ServerSyncUsername(ServerSyncUsername::new(
                u16::from_be_bytes(id),
                username.into(),
            )))
        }
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

fn read_long_string<'a>(remaining: &mut &'a [u8]) -> Result<Option<&'a str>, DecodeError> {
    let Some(prefix) = read_array(remaining) else {
        return Ok(None);
    };
    read_string(remaining, usize::from(u16::from_be_bytes(prefix)))
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
