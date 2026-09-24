---
flow: build
priority: 93
opens: Cargo.toml libraries/rust sdlc/scripts sdlc/ratchet.json sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0093: Port the Rust examples as the first binding crate

Status: draft, revised after design review; needs re-review. Owner: Claude.

## Outcome and authority

Port the Rust examples surface onto the public API, and use it to set the pattern every later surface copies. Draft ADR 0047 governs, and it lands with this ticket after review. Queue item 10 of `sdlc/planning/one-line-plan-2026-09-24.md`.

This ticket runs before C (0094), which swaps the plan's items 9 and 10 for this one surface. Rust examples have no FFI and no host toolchain. They prove the workspace, lint, ratchet, and surface-rung pattern apart from ABI work, so C carries only its door. C still comes next, before every other surface. Ian can overturn the swap.

## Work

- `libraries/rust` becomes its own Cargo workspace of examples and tests over `thinkthen`, per ADR 0047 item 1. The branch forwarder that re-exported the stand-in contract retires. The slide and examples keep their intent. Tests built on `thinkthen_standin::testkit` are rewritten whole.
- The root `Cargo.toml` gains `exclude = ["libraries", "databases"]` and `default-members = ["crates/thinkthen"]` (ADR 0047 item 2).
- Checks run from `lint` scan `libraries/*/Cargo.toml` and `databases/*/Cargo.toml` directly: `publish = false`; the only `thinkthen` dependency is the path form with default features off; no dependency on another binding; `unsafe_code = "deny"` in the binding's lint table (ADR 0047 item 3).
- The binding gets its own `ratchet.json` read by the shared reader (ADR 0047 item 4), and deny covers its lock.
- One surface registry file lists the nine surfaces and their state. A surface rung runs each landed surface's `check.sh` with the 0092 loopback port, reports a missing toolchain as "not run", and refuses a listed surface whose check is missing (R6-2).

## Acceptance

- A bare `cargo test` in `libraries/rust` passes with no skip-only test (R2-28, the row this surface owns). R2-32 (the serde_cbor and paste dependency audit) moves to the PostgreSQL and R tickets.
- Planted members each fail `lint`: one with `publish = true`, one with default features on, one that depends on another binding, and one with `unsafe` outside an FFI module.
- A registry entry with no check fails the surface rung. A plain root `cargo build` compiles `thinkthen` alone.
- Code stays under 600 nonblank Rust lines and 150 script lines. No dependency is added to `thinkthen`.

## Dependencies

After 0086, 0098, and 0092. Before 0094 and every other surface ticket.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 1; state and timing 0; reach 3; proof 2; cost of error 1; total 7. Final level: 2. The pattern reaches every later surface.

## Review

- Design review: the 2026-09-24 reviews (`sdlc/records/2026-09-24-spine-review-contract.md`, `sdlc/records/2026-09-24-spine-review-engine.md`) found the workspace, lint, library-name, ratchet, and ruling gaps in ADR 0047 and one misassigned row. All applied; re-review pending, with ADR 0047.
- Code review: pending.
