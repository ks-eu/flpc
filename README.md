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

The Rust crate currently encodes and decodes
`Ping`, `Register`, `Login`, `Message`, `PrivateMessage`, `ChangeUsername`,
`RequestSync`, `RemoveAdmin`, `MakeAdmin`, `Add`, `Remove`, and `Kick`, plus
`ServerRegister`, `ServerLogin`, and `ServerMessage`.
The optional `tokio` feature exposes the shared `FlpcCodec` for
`tokio_util::codec::Framed`.

Each unit starts with a one-byte opcode. Its fields follow in the order shown below. All multibyte integers use big-endian byte order. IDs and targets are unsigned 16-bit integers. Timestamps and sync bounds are unsigned 64-bit integers.

Each string has an unsigned length prefix followed by that many UTF-8 bytes. The field determines the prefix width. An empty string has a length of zero.

| Field | Length prefix | Maximum UTF-8 bytes |
|---|---:|---:|
| Token, username, password | 1 byte | 255 |
| Message | 2 bytes | 65,535 |

### Units

| Opcode (hex) | Unit | Fields after opcode |
|---|---|---|
| `00` | `Ping` | None |
| `01` | `Register` | token, username, password |
| `02` | `Login` | username, password |
| `03` | `Message` | message |
| `04` | `PrivateMessage` | target, message |
| `05` | `ChangeUsername` | username |
| `06` | `RequestSync` | first, last |
| `7B` | `RemoveAdmin` | id |
| `7C` | `MakeAdmin` | id |
| `7D` | `Add` | username, password |
| `7E` | `Remove` | id |
| `7F` | `Kick` | id |
| `80` | `ServerRegister` | outcome (1 byte) |
| `81` | `ServerLogin` | outcome (1 byte) |
| `82` | `ServerMessage` | id, timestamp, message |
| `83` | `ServerPrivateMessage` | id, target, timestamp, message |
| `84` | `UserRegistered` | id, username |
| `85` | `UserRemoved` | id |
| `86` | `UserJoined` | id |
| `87` | `UserLeft` | id |
| `88` | `ChangedUsername` | id, username |
| `89` | `SyncMessage` | id, timestamp, message |
| `8A` | `SyncPrivateMessage` | id, target, timestamp, message |

The outcome byte is opaque to the wire codec. The timestamp epoch and precision and the meaning and inclusivity of sync bounds still need to be specified.

### Framing

TCP may split one unit across reads or deliver multiple units in one read. The Rust decoder reports one complete unit and its size, or asks for more bytes if a supported unit is incomplete. An unassigned opcode or invalid UTF-8 in a complete field is a decoding error. There is no outer length with which to skip an unrecognized unit.

For example, registering with token `t`, username `é`, and password `pw` yields `01 01 74 02 C3 A9 02 70 77`. The username starts with `02` because `é` occupies two UTF-8 bytes.

Sending the message `é` yields `03 00 02 C3 A9`. The two-byte big-endian prefix
`00 02` counts the UTF-8 bytes in `é`.
