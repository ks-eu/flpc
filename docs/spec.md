# FLPC wire specification

FLPC units are concatenated on a TCP byte stream. Each unit begins with a
one-byte opcode followed immediately by the fields below, in order. All
multibyte integers are unsigned and use big-endian byte order.

## Field encoding

| Type | Wire encoding |
|---|---|
| `u8` | One unsigned byte |
| `u16` | Two unsigned bytes, most significant byte first |
| `u64` | Eight unsigned bytes, most significant byte first |
| `str8` | A `u8` length followed by 0–255 bytes of UTF-8 |
| `str16` | A `u16` length followed by 0–65,535 bytes of UTF-8 |

String lengths count bytes. A zero length encodes an empty string. User IDs
and targets are `u16` values. Timestamps are raw `u64` values, with epoch
and precision agreed between peers. Outcome, result, and reason fields
preserve all 256 byte values, with their meanings agreed between peers.

## Opcodes and payloads

The direction describes who sends the unit. An empty payload consists of the
opcode alone. All opcodes absent from the table, including `0xFF`, are
unassigned.

| Opcode | Unit | Sender | Fields after opcode |
|---|---|---|---|
| `0x00` | `Ping` | Either | Empty |
| `0x01` | `Register` | Client | token `str8`, username `str8`, password `str8` |
| `0x02` | `RemoveSelf` | Client | username `str8`, password `str8` |
| `0x03` | `Login` | Client | username `str8`, password `str8` |
| `0x04` | `ChangePassword` | Client | username `str8`, current password `str8`, new password `str8` |
| `0x10` | `Message` | Client | message `str16` |
| `0x11` | `PrivateMessage` | Client | target ID `u16`, message `str16` |
| `0x12` | `ChangeUsername` | Client | new username `str8` |
| `0x20` | `RequestSyncMessage` | Client | timestamp `u64`, count `u8` (1–255) |
| `0x21` | `RequestSyncPrivateMessage` | Client | target ID `u16`, timestamp `u64`, count `u8` (1–255) |
| `0x22` | `RequestSyncUsername` | Client | user ID `u16` |
| `0x60` | `Add` | Client | username `str8`, password `str8` |
| `0x61` | `Remove` | Client | user ID `u16` |
| `0x62` | `Kick` | Client | user ID `u16` |
| `0x63` | `MakeAdmin` | Client | user ID `u16` |
| `0x64` | `RemoveAdmin` | Client | user ID `u16` |
| `0x80` | `ServerResponse` | Server | response `str16` |
| `0x81` | `UserRegistered` | Server | user ID `u16`, username `str8` |
| `0x82` | `UserRemoved` | Server | user ID `u16` |
| `0x83` | `UserJoined` | Server | user ID `u16` |
| `0x84` | `UserLeft` | Server | user ID `u16` |
| `0x85` | `ChangedUsername` | Server | user ID `u16`, new username `str8` |
| `0x86` | `UserAdmin` | Server | user ID `u16` |
| `0x87` | `UserAdminRemoved` | Server | user ID `u16` |
| `0x90` | `ServerRegister` | Server | outcome `u8` |
| `0x91` | `ServerRemoveSelf` | Server | result `u8` |
| `0x92` | `ServerLogin` | Server | outcome `u8` |
| `0x93` | `ServerChangedPassword` | Server | outcome `u8` |
| `0x94` | `WrongUnit` | Server | Empty |
| `0x95` | `OnboardingEnd` | Server | Empty |
| `0xA0` | `ServerMessage` | Server | sender ID `u16`, timestamp `u64`, message `str16` |
| `0xA1` | `ServerPrivateMessage` | Server | sender ID `u16`, target ID `u16`, timestamp `u64`, message `str16` |
| `0xB0` | `ServerMessageFail` | Server | reason `u8` |
| `0xB1` | `ServerChangedUsername` | Server | outcome `u8` |
| `0xB2` | `UserNotOnline` | Server | Empty |
| `0xC0` | `SyncMessage` | Server | sender ID `u16`, timestamp `u64`, message `str16` |
| `0xC1` | `SyncPrivateMessage` | Server | sender ID `u16`, target ID `u16`, timestamp `u64`, message `str16` |
| `0xC2` | `SyncUsername` | Server | user ID `u16`, username `str8` |
| `0xC3` | `SyncEnd` | Server | count `u8` (0–255) |
| `0xD0` | `SyncMessageFail` | Server | reason `u8` |
| `0xD1` | `SyncUsernameFail` | Server | reason `u8` |
| `0xE0` | `ServerAdd` | Server | outcome `u8` |
| `0xE1` | `ServerRemove` | Server | outcome `u8` |
| `0xE2` | `ServerKick` | Server | outcome `u8` |
| `0xE3` | `ServerMakeAdmin` | Server | outcome `u8` |
| `0xE4` | `ServerRemoveAdmin` | Server | outcome `u8` |

`WrongUnit` indicates an unexpected first unit from the client.
`OnboardingEnd` marks the end of onboarding. `UserAdmin` announces that a
user has administrator privileges and `UserAdminRemoved` announces that
those privileges have been removed. `ServerChangedUsername` reports the
outcome of a username change request, while `ChangedUsername` announces a
user's new username.

### Synchronization

`RequestSyncMessage` and `RequestSyncPrivateMessage` request up to `count`
of the most recent messages strictly before the supplied timestamp. The server
returns the units as `SyncMessage` or `SyncPrivateMessage`, respectively.
`SyncEnd` terminates the message batch and gives the number of items actually
returned. An empty batch ends with `SyncEnd` with a count of zero.
`SyncUsername` supplies the user ID and current username requested by
`RequestSyncUsername`.

## Framing and validation

- Determine the length of a unit from its opcode and field lengths. TCP can
  split a unit at any byte or combine several units in one read.
- When decoding succeeds, return the first unit and the number of bytes it
  consumed, including the opcode. Leave any following bytes for the next
  decode. When more input is needed, retain the bytes already received.
- Reject unassigned opcodes, invalid UTF-8 in a complete field, and a zero
  count in either message synchronization request. There is no outer length
  for skipping an unknown unit. EOF at a unit boundary is clean. EOF partway
  through a unit is a framing error.
- Reject encoding a string over its byte limit or a synchronization request
  with a zero count. A slice destination must fit the complete unit. Preserve
  the destination on error and preserve trailing bytes on success.
- Passwords and the registration token are plaintext on the wire. Rust
  `Debug` output redacts the token and all password fields, including both
  passwords in `ChangePassword`.

## Example units

| Unit | Values | Complete bytes (hex) |
|---|---|---|
| `Ping` | Empty | `00` |
| `Register` | Token `t`, username `é`, password `pw` | `01 01 74 02 C3 A9 02 70 77` |
| `ChangePassword` | Username `é`, current password `p`, new password `pw` | `04 02 C3 A9 01 70 02 70 77` |
| `Message` | Message `é` | `10 00 02 C3 A9` |
| `ServerResponse` | Response `é` | `80 00 02 C3 A9` |
| `UserAdminRemoved` | User ID `0x1234` | `87 12 34` |
| `SyncEnd` | Zero items | `C3 00` |

## Compatibility

Peers must use the same opcode map. Some bytes in this map carried different
units in earlier versions, so clients and servers must upgrade together.
