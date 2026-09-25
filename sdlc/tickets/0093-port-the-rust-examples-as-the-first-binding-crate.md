---
flow: build
priority: 93
opens: CLAUDE.md Cargo.toml Cargo.lock libraries/rust sdlc/scripts sdlc/ratchet.json sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0093: Port the Rust examples as the first binding crate

Status: landed on main 2026-09-24 after code review ACCEPT; the ladder passed on `3deda370`, and lint and surfaces passed again after merging onto 0098. See `sdlc/records/0093-build-first-binding-crate.md`. Owner: Claude.

## Outcome and authority

Port the Rust examples surface onto the public API, and use it to set the pattern every later surface copies. Draft ADR 0047 governs, and it lands with this ticket after review. Queue item 10 of `sdlc/planning/one-line-plan-2026-09-24.md`.

This ticket runs before C (0094), which swaps the plan's items 9 and 10 for this one surface. Rust examples have no FFI and no host toolchain. They prove the workspace, lint, ratchet, and surface-rung pattern apart from ABI work, so C carries only its door. C still comes next, before every other surface. Ian can overturn the swap.

## Work

- `libraries/rust` becomes its own Cargo workspace of examples and tests over `thinkthen`, per ADR 0047 item 1. The branch forwarder that re-exported the stand-in contract retires. The slide and examples keep their intent. Tests built on `thinkthen_standin::testkit` are rewritten whole.
- The root `Cargo.toml` adds `"libraries"` and `"databases"` to the `exclude` list beside 0086's `conformance/consumer`, and sets `default-members = ["crates/thinkthen"]` (ADR 0047 item 2).
- Checks run from `lint` scan `libraries/*/Cargo.toml` and `databases/*/Cargo.toml` directly: `publish = false`; the only `thinkthen` dependency is the path form with default features off; no dependency on another binding; a lint table equal to the root table except `unsafe_code = "deny"` (ADR 0047 item 3); a `[profile.release]` equal to the root one; and a lock whose versions for `thinkthen`'s own normal dependency tree equal the root lock's (ADR 0047 item 1). The check compares only the versions `thinkthen` resolves to, since a lock may also hold a second major version for a host crate.
- `sdlc/scripts/ratchet.mjs` gains the optional config-path argument (ADR 0047 item 4). `lint` runs it on `libraries/rust/ratchet.json`, and deny covers the binding's lock.
- `policy.py` refuses `#[ignore]` and any binding test body that returns before its first assertion.
- One surface registry file lists the nine surfaces and their state. The surface rung `sdlc/scripts/surfaces` runs after `spec` and runs each landed surface's `check.sh` with the 0092 loopback port, reports a missing toolchain as "not run", and refuses a listed surface whose check is missing (R6-2). `lint` runs its registry check, which refuses a listed surface with no `check.sh`.
- The repo `CLAUDE.md` ladder line names `surfaces` as the fifth rung.

## Acceptance

- A bare `cargo test` in `libraries/rust` passes with no skip-only test (R2-28, the row this surface owns). R2-32 (the serde_cbor and paste dependency audit) moves to the PostgreSQL and R tickets.
- Planted members each fail `lint`: one with `publish = true`, one with default features on, one that depends on another binding, one with `unsafe` outside an FFI module, one with `overflow-checks = false`, one lock with another ureq version, and one ratchet file whose ceiling misses the count.
- R2-28 planted bug: a test that prints "skipped" and returns fails `lint`.
- A registry entry with no check fails the surface rung. A plain root `cargo build` compiles `thinkthen` alone.
- Code stays under 600 nonblank Rust lines and 300 nonblank script lines, counting the ratchet argument, the policy checks, the registry check, and the `surfaces` rung. Re-score before crossing either. No dependency is added to `thinkthen`.

## Dependencies

After 0086, 0098, and 0092. Before 0094 and every other surface ticket.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 1; state and timing 0; reach 3; proof 2; cost of error 1; total 7. Final level: 2. The pattern reaches every later surface.

## Review

- Design review: the 2026-09-24 reviews (`sdlc/records/2026-09-24-spine-review-contract.md`, `sdlc/records/2026-09-24-spine-review-engine.md`) found the workspace, lint, library-name, ratchet, and ruling gaps in ADR 0047 and one misassigned row. All applied. The re-reviews (`sdlc/records/2026-09-24-rereview-contract.md`, `sdlc/records/2026-09-24-rereview-engine.md`) found the ratchet reader, release profile, lock versions, rung name, and R2-28 planted bug; all applied. The confirmation (same file) found the planted R2-28 test passed the early-return rule; the rule now refuses any return before the first assertion. The final check accepted it.
- Code review: a fresh Claude (Opus) session rejected the first pass on one finding: host-language ratchet files ran nowhere. It accepted `f52f1ee4`. The record is `sdlc/records/0093-build-first-binding-crate.md`.
