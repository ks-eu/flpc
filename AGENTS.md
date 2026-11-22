# FLPC development

- The Rust crate lives in `rust/`. Run Cargo commands from the repository root with `--manifest-path rust/Cargo.toml`.
- Read the README's [wire format](README.md#wire-format) for byte layouts, field limits, and the Register reference vector.
- Implement `Encode` in the unit family's `wire` module and dispatch incoming units in `wire/decode.rs`. Keep wire I/O on standard-library byte slices. The optional Tokio adapter handles `BytesMut` and shares one `FlpcCodec` across units.
- Run `$flpc-unit-tdd Register` in the main thread to build the reference. After Register's documentation commit, use `$flpc-unit-tdd <Unit>` for each named unit. The skill defines delegation, review, and commit order.

## Verification

```sh
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo test --manifest-path rust/Cargo.toml --no-default-features
cargo test --manifest-path rust/Cargo.toml --features tokio
RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path rust/Cargo.toml --features tokio --no-deps
```
