---
name: flpc-unit-tdd
description: Orchestrate four verified local commits per FLPC unit, for a named unit or a rollout in opcode order.
---

# Implement FLPC units

Run in the main thread. Identify the concrete struct and its opcode in the
README's Wire format section. Record HEAD and the working tree and index status.
For Register, start from the README vector. For later units, use the committed
Register implementation and permanent tests as the reference.

Apply and commit one stage at a time on the shared checkout. For an all-unit
rollout, prepare repeated stage edits in scratch, then apply and verify them
in order for each unit. Give the specialist the unit, opcode, protocol section,
permitted files, last verified commit, relevant tests, and expected results. Use the manifest path and checks in
AGENTS.md. Specialists can iterate within their stage. The main thread owns
reviews, staging, and local commits.

Use the model and reasoning effort pinned in the specialist's
`.codex/agents/` file. When the spawn tool exposes those settings as fixed,
select the role directly. Otherwise pass the settings explicitly. For the
documentation worker, pass the project defaults from `.codex/config.toml`.
Reuse a compatible idle specialist. If the runtime offers a close operation,
close finished specialists before replacing them. On thread exhaustion, run
one ephemeral `codex exec --sandbox workspace-write` worker at a time with
explicit `--model` and `model_reasoning_effort` settings. Wait for it to exit
before starting another. Include the full stage brief and file ownership. Use
only `gpt-6.2-sol` at medium or `gpt-6-luna` at high. If a pinned role is unavailable, use a worker with the role's
instructions and an explicit approved pairing. Stop if neither pairing
is available. Record the selected model and effort.

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
   the red-test assertions intact. Resolve conflicts against the README.
3. **Characterization** — Assign `flpc_characterizer` the unit's two test
   files. Replace the initial suite with lasting public wire and Framed
   regression tests. Retain independent expected bytes and the guarantees
   covered by the initial tests. Consolidate duplicate cases and check any
   discrepancies against the protocol before recording them as expectations.
   Run both full suites.
4. **Documentation** — Assign a `worker` the unit's public API documentation,
   a runnable example, and any relevant README update. Run doctests and all
   AGENTS.md verification commands. The fourth commit finishes this unit. During a
   rollout continue with the next unsupported opcode.

## Verify and commit automatically

Inspect the diff and test results for each stage. Stage only the verified
changes, check the staged diff, and preserve unrelated edits including
anything already staged. Commit locally and record the hash. Continue to the
next stage without a user approval prompt. Stage 1 deliberately commits
failing tests. Stop the rollout for a real protocol ambiguity or a failure
that cannot be corrected within the current stage.
