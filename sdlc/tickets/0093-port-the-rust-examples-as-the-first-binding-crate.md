---
flow: build
priority: 93
opens: Cargo.toml libraries/rust sdlc/scripts sdlc/ratchet.json sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0093: Port the Rust examples as the first binding crate

Status: draft, design not reviewed. Owner: Claude.

## Outcome and authority

Port the Rust examples surface onto the 0086 public API, and use it to set the workspace pattern every later surface copies. Draft ADR 0047 governs, and it lands with this ticket after review. Queue item 10 of `sdlc/planning/one-line-plan-2026-09-24.md`. Rust goes first because it has no FFI, so the pattern is proven apart from host glue.

## Work

- `libraries/rust` becomes an unpublished workspace crate of examples and tests over `thinkthen`. The branch forwarder that re-exported the stand-in contract retires. The slide and examples keep their intent, and tests built on `thinkthen_standin::testkit` are rewritten whole.
- Add the workspace rules of ADR 0047 as checks run from `lint`: every member except `crates/thinkthen` has `publish = false`; its only `thinkthen` dependency is the path form with default features off; no member depends on another binding.
- Bring the member under main's lint, deny, policy, and ratchet (error-index rows R1-28 and R3-28). The ratchet counts ported code.
- Add one surface registry file listing the nine surfaces and their state, and a rung that runs each landed surface's check and refuses a listed surface whose check is missing (R6-2).
- The examples run against the 0092 loopback backend with no key and no network.

## Acceptance

- A bare `cargo test` in `libraries/rust` passes with no skip-only test (R2-28). R2-32 re-runs against the real engine.
- A planted member with `publish = true`, one with default features on, and one that depends on another binding each fail `lint`.
- A registry entry with no check fails the surface rung.
- Code stays under 600 nonblank Rust lines and 150 script lines. No dependency is added to `thinkthen`.

## Dependencies

After 0086 and 0092. Before 0094 and every other surface ticket.

## Complexity

Contract 1; state and timing 0; reach 3; proof 2; cost of error 1; total 7. Final level: 2. The pattern reaches every later surface.

## Review

- Design review: pending, with draft ADR 0047.
- Code review: pending.
