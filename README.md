# FLPC

**FLPC** is a custom ultra-lightweight network communication protocol, designed for
locally hosted small chatrooms over TCP. This repository contains the Rust
implementation as a crate, with optional framing through `tokio-util`.

This is part of a hobby project, with the aim of building a chatroom server and
client TUI app in Rust that can be hosted locally. Ideally, someone can host
this server on an old laptop at home, and be able to have real-time
conversations with friends. With this in mind, it is important that the
protocol is a lightweight as possible. This is achieved by byte-encoding and
minimising header size as much as possible.

## Wire format

This section specifies the new opcode map. The Rust implementation is being
updated in stages. See [the detailed wire specification](docs/spec.md) for
each unit's payload layout and validation rules.

Each unit starts with a one-byte opcode. Multibyte integers use big-endian
byte order. IDs and targets are unsigned 16-bit integers. Timestamps are
unsigned 64-bit integers.

Strings carry an unsigned length prefix followed by that many UTF-8 bytes.
A zero length encodes an empty string.

| Field | Length prefix | Maximum UTF-8 bytes |
|---|---:|---:|
| Token, username, password, new password | 1 byte | 255 |
| Message, server response | 2 bytes | 65,535 |

### Units

```text
0x00 Ping
0x01 Register
0x02 RemoveSelf
0x03 Login
0x04 ChangePassword

0x10 Message
0x11 PrivateMessage
0x12 ChangeUsername

0x20 RequestSyncMessage
0x21 RequestSyncPrivateMessage
0x22 RequestSyncUsername

0x60 Add
0x61 Remove
0x62 Kick
0x63 MakeAdmin
0x64 RemoveAdmin

0x80 ServerResponse
0x81 UserRegistered
0x82 UserRemoved
0x83 UserJoined
0x84 UserLeft
0x85 ChangedUsername
0x86 UserAdmin
0x87 UserAdminRemoved

0x90 ServerRegister
0x91 ServerRemoveSelf
0x92 ServerLogin
0x93 ServerChangedPassword
0x94 WrongUnit
0x95 OnboardingEnd

0xA0 ServerMessage
0xA1 ServerPrivateMessage

0xB0 ServerMessageFail
0xB1 ServerChangedUsername
0xB2 UserNotOnline

0xC0 SyncMessage
0xC1 SyncPrivateMessage
0xC2 SyncUsername
0xC3 SyncEnd

0xD0 SyncMessageFail
0xD1 SyncUsernameFail

0xE0 ServerAdd
0xE1 ServerRemove
0xE2 ServerKick
0xE3 ServerMakeAdmin
0xE4 ServerRemoveAdmin
```

`WrongUnit` is a server response for an unexpected first unit from a client.

`RequestSyncMessage` and `RequestSyncPrivateMessage` request the most recent
`count` messages strictly before the supplied timestamp. Responses use
`SyncMessage` and `SyncPrivateMessage`, respectively. `SyncEnd` ends the batch
and reports the actual number returned, including zero for an empty batch.

**Wire compatibility.** The new map reassigns several existing opcodes.
Clients and servers must upgrade together because an older peer can interpret
a reassigned opcode as a different unit.

### Framing

The optional `tokio` feature exposes the shared `FlpcCodec` for
`tokio_util::codec::Framed`. Concrete units implement `Into<FlpcUnit>` for use
with the codec.

TCP may split one unit across reads or deliver multiple units in one read. The
Rust decoder reports one complete unit and its size, or asks for more bytes
if a supported unit is incomplete. An unassigned opcode, invalid UTF-8 in a
complete field, or a zero sync request count is a decoding error. There is no
outer length with which to skip an unrecognized unit.

For example, registering with token `t`, username `é`, and password `pw` yields `01 01 74 02 C3 A9 02 70 77`. The username starts with `02` because `é` occupies two UTF-8 bytes.

Sending the message `é` yields `10 00 02 C3 A9`. The two-byte big-endian prefix
`00 02` counts the UTF-8 bytes in `é`.

### Credentials

`Register`, `Login`, `RemoveSelf`, `ChangePassword`, and `Add` carry passwords
as plaintext UTF-8. The registration token in `Register` is also plaintext.
Rust `Debug` output displays passwords and the registration token as
`[redacted]`.
