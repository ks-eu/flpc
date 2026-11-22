# FLPC

**FLPC** is a custom ultra-lightweight network communication protocol, designed for
locally hosted small chatrooms over TCP. This repository contains the Rust
implementation as a crate, including integration with tokio-utils.

This is part of a hobby project, with the aim of building a chatroom server and
client TUI app in Rust that can be hosted locally. Ideally, someone can host
this server on an old laptop at home, and be able to have real-time
conversations with friends. With this in mind, it is important that the
protocol is a lightweight as possible. This is achieved by byte-encoding and
minimising header size as much as possible.

## Current Protocol
FLPC currently defines the following protocol messages:


