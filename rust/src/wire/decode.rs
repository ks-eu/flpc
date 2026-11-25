use super::DecodeError;
use crate::unit::{
    Add, AdminUnit, ChangeUsername, ChangedUsername, ClientUnit, FlpcUnit, Kick, Login, MakeAdmin,
    Message, Ping, PingUnit, PrivateMessage, Register, Remove, RemoveAdmin, RemoveSelf,
    RequestSync, RequestSyncPrivateMessage, RequestSyncUsername, ServerLogin, ServerMessage,
    ServerPrivateMessage, ServerRegister, ServerRemoveSelf, ServerUnit, SyncMessage,
    SyncPrivateMessage, UserJoined, UserLeft, UserRegistered, UserRemoved, WrongUnit,
};

/// Decodes the first unit and returns it with the byte count, including its opcode.
///
/// Returns `Ok(None)` for empty input or an incomplete unit with a supported opcode.
/// The caller retains the input and can decode subsequent units from the remaining bytes.
///
/// # Errors
///
/// Returns [`DecodeError::UnknownOpcode`] for an unassigned opcode
/// or [`DecodeError::InvalidUTF8`] for an invalid UTF-8 field.
pub fn decode(buf: &[u8]) -> Result<Option<(FlpcUnit, usize)>, DecodeError> {
    let Some((&opcode, mut remaining)) = buf.split_first() else {
        return Ok(None);
    };

    let unit = match opcode {
        // 0x00
        Ping::OPCODE => Some(FlpcUnit::Ping(PingUnit::Ping(Ping::new()))),
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
            Some(FlpcUnit::Client(ClientUnit::Register(unit)))
        }
        // 0x02
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
        // 0x03
        Message::OPCODE => {
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::Message(Message::new(
                message.into(),
            ))))
        }
        // 0x04
        PrivateMessage::OPCODE => {
            let Some(target) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(message) = read_long_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::PrivateMessage(
                PrivateMessage::new(u16::from_be_bytes(target), message.into()),
            )))
        }
        // 0x05
        ChangeUsername::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::ChangeUsername(
                ChangeUsername::new(username.into()),
            )))
        }
        // 0x06
        RequestSync::OPCODE => {
            let Some(first) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(last) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::RequestSync(RequestSync::new(
                u64::from_be_bytes(first),
                u64::from_be_bytes(last),
            ))))
        }
        // 0x07
        RemoveSelf::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::RemoveSelf(RemoveSelf::new(
                username.into(),
                password.into(),
            ))))
        }
        // 0x08
        RequestSyncUsername::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Client(ClientUnit::RequestSyncUsername(
                RequestSyncUsername::new(u16::from_be_bytes(id)),
            )))
        }
        // 0x09
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
            Some(FlpcUnit::Client(ClientUnit::RequestSyncPrivateMessage(
                RequestSyncPrivateMessage::new(
                    u16::from_be_bytes(target),
                    u64::from_be_bytes(timestamp),
                    count,
                ),
            )))
        }
        // 0x7B
        RemoveAdmin::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Admin(AdminUnit::RemoveAdmin(RemoveAdmin::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x7C
        MakeAdmin::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Admin(AdminUnit::MakeAdmin(MakeAdmin::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x7D
        Add::OPCODE => {
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            let Some(password) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Admin(AdminUnit::Add(Add::new(
                username.into(),
                password.into(),
            ))))
        }
        // 0x7E
        Remove::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Admin(AdminUnit::Remove(Remove::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x7F
        Kick::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Admin(AdminUnit::Kick(Kick::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x80
        ServerRegister::OPCODE => {
            let Some([outcome]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::ServerRegister(
                ServerRegister::new(outcome),
            )))
        }
        // 0x81
        ServerLogin::OPCODE => {
            let Some([outcome]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::ServerLogin(ServerLogin::new(
                outcome,
            ))))
        }
        // 0x82
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
            Some(FlpcUnit::Server(ServerUnit::ServerMessage(
                ServerMessage::new(
                    u16::from_be_bytes(id),
                    u64::from_be_bytes(timestamp),
                    message.into(),
                ),
            )))
        }
        // 0x83
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
            Some(FlpcUnit::Server(ServerUnit::ServerPrivateMessage(
                ServerPrivateMessage::new(
                    u16::from_be_bytes(id),
                    u16::from_be_bytes(target),
                    u64::from_be_bytes(timestamp),
                    message.into(),
                ),
            )))
        }
        // 0x84
        UserRegistered::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::UserRegistered(
                UserRegistered::new(u16::from_be_bytes(id), username.into()),
            )))
        }
        // 0x85
        UserRemoved::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::UserRemoved(UserRemoved::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x86
        UserJoined::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::UserJoined(UserJoined::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x87
        UserLeft::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::UserLeft(UserLeft::new(
                u16::from_be_bytes(id),
            ))))
        }
        // 0x88
        ChangedUsername::OPCODE => {
            let Some(id) = read_array(&mut remaining) else {
                return Ok(None);
            };
            let Some(username) = read_short_string(&mut remaining)? else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::ChangedUsername(
                ChangedUsername::new(u16::from_be_bytes(id), username.into()),
            )))
        }
        // 0x89
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
            Some(FlpcUnit::Server(ServerUnit::SyncMessage(SyncMessage::new(
                u16::from_be_bytes(id),
                u64::from_be_bytes(timestamp),
                message.into(),
            ))))
        }
        // 0x8A
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
            Some(FlpcUnit::Server(ServerUnit::SyncPrivateMessage(
                SyncPrivateMessage::new(
                    u16::from_be_bytes(id),
                    u16::from_be_bytes(target),
                    u64::from_be_bytes(timestamp),
                    message.into(),
                ),
            )))
        }
        // 0x8B
        WrongUnit::OPCODE => Some(FlpcUnit::Server(ServerUnit::WrongUnit(WrongUnit::new()))),
        // 0x8C
        ServerRemoveSelf::OPCODE => {
            let Some([result]) = read_array(&mut remaining) else {
                return Ok(None);
            };
            Some(FlpcUnit::Server(ServerUnit::ServerRemoveSelf(
                ServerRemoveSelf::new(result),
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
