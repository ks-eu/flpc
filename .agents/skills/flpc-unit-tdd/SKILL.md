---
name: flpc-unit-tdd
description: Orchestrate four user-approved local commits for one named FLPC unit, starting with Register or using the completed Register reference.
---

# Implement one FLPC unit

Run in the main thread. Identify the concrete struct and its opcode in the
README's Wire format section. Record HEAD and the working tree and index status.
For Register, start from the README vector. For later units, use the approved
Register implementation and permanent tests as the reference.

Delegate one stage at a time on the shared checkout. Give the specialist the
unit, opcode, protocol section, permitted files, last approved commit, relevant
tests, and expected commands and results. Use the manifest path and checks in
AGENTS.md. Specialists can iterate within their stage. The main thread owns
reviews, staging, and local commits.

1. **Red tests** — Assign `flpc_test_author` both
   `rust/tests/wire_<unit>.rs` and `rust/tests/framed_<unit>.rs` (snake case).
   Gate the Framed target with `#![cfg(feature = "tokio")]`. Independently
   check send and receive against hand-authored bytes, using a raw duplex peer
   for Framed. Run each focused target with Cargo using
   `--no-default-features --test wire_<unit>` and
   `--features tokio --test framed_<unit>` respectively. Verify that failures
   arise from missing unit behavior. A missing Encode implementation may
   prevent the test binary from compiling. Resolve unrelated failures before
   accepting red. Submit both test files for one commit.
2. **Implementation** — Assign `flpc_implementer` the family wire module and
   decoder. Implement Encode for the concrete unit and dispatch its opcode in
   wire::decode. Run the focused tests and full core and Tokio suites. Keep
   approved assertions intact. Bring a contract conflict back for test review.
3. **Characterization** — Assign `flpc_characterizer` the unit's two test
   files. Replace the initial suite with lasting public wire and Framed
   regression tests. Retain independent expected bytes and account for each
   approved guarantee in the review. Consolidate duplicate cases and report
   observed discrepancies with the protocol before encoding them as expectations.
   Run both full suites.
4. **Documentation** — Assign a `worker` the unit's public API documentation,
   a runnable example, and any relevant README update. Run doctests and all
   AGENTS.md verification commands. The approved fourth commit finishes this
   unit. Begin another unit only when named.

## Review each stage

Show the diff or relevant excerpts, exact commands and results, files to stage,
and proposed commit message. Wait for the user's approval of this specific
stage. If revised, show the new diff for approval. After approval, stage only
the reviewed changes, inspect the staged diff, and preserve unrelated edits
including anything already staged. Commit locally, report the hash, then
delegate the next stage. Stage 1 deliberately commits the failing tests.
