# FLPC

**FLPC** is a custom ultra-lightweight application layer protocol, designed for
locally hosted small chatrooms over TCP. This repository contains the Rust
implementation as a crate, including integration with tokio-utils.

## Aims

This is part of a hobby project, with the aim of building a chatroom server and
client TUI app in Rust that can be hosted locally. Ideally, someone can host
this server on an old laptop at home, and be able to have real-time
conversations with friends. With this in mind, it is important that the
protocol is a lightweight as possible. This is achieved by byte-encoding and
minimising header size as much as possible.

The current goal for flpc is to support chatrooms with:
- message history
- user authentication
- private messaging
These goals have the added need of working on old, slow hardware as efficiently
as possible.

Some potential future goals are:
- multiple message channels
- more complex formats (audio, image, video)
- decorated text (bold, colours)
These features could be implemented as optional extras, which can be enabled
or disabled based on need.

## Current Protocol
FLPC currently defines the following protocol messages:

| Message | Description |
|---|---|
| `Join` | Client requests to join the server. |
| `JoinSuccess` | Server confirms a successful join. |
| `JoinFail` | Server rejects a join request. |
| `Message` | Server-to-client message. |
| `ClientMessage` | Client-to-server message. |
| `UserJoin` | Indicates that a user joined. |
| `UserLeft` | Indicates that a user left. |
