---
flow: build
priority: 92
opens: Cargo.toml Cargo.lock deny.toml conformance crates/thinkthen/Cargo.toml crates/thinkthen/tests/backend sdlc/scripts sdlc/ratchet.json
---

# 0092: Serve the shared cases from a loopback backend

Status: built 2026-09-24, code review pending. Design accepted 2026-09-24 after re-review. Owner: Claude.

## Outcome and authority

Give every binding one offline backend that its tests can start from any language. It replaces the branch's `ENGINE_NULL` and `THINKTHEN_NULL` null backends and its `synthetic-partial` feature, which the real engine lacks (gap G10, question 10 of `sdlc/planning/surfaces-port-guide.md`). A binding points `base_url` at it, so the real transport path runs. No public fault hook or null backend enters `thinkthen`. Ian can overturn the tool's form and the generic arm.

## Shape

- One unpublished crate, `conformance/backend`, with a library target and a thin binary. The library holds the listener and canned replies from `crates/thinkthen/tests/backend/harness`, made `pub`. The binary only parses arguments and runs the library.
- `spawn` and `process_has_file` stay in `tests/backend/harness`, because `env!("CARGO_BIN_EXE_thinkthen")` resolves only inside the `thinkthen` package's own tests. The harness module re-exports the rest from the new library, so the 49 test files that import it do not churn.
- `thinkthen` gains a path dev-dependency on `conformance/backend`. A second reviewer checks it under the repo's dependency rule.
- It becomes a second root workspace member. This ticket rules that a test-only, unpublished root member is allowed, under policy, deny, and the ratchet. Draft ADR 0047 repeats the rule.
- It binds `127.0.0.1` only, on an ephemeral port, and prints the port on its first line. It exits when its standard input closes. A `count` line on standard input prints the request count so far, and the final count prints on exit.
- The URL path picks the arm, for example `http://127.0.0.1:P/arm/reset/v1`. The engine appends `/systemone` to any base, so every binding can pick an arm with no header.
- Case arm (path `/case/…`): an exact request body gets its case's response from `conformance/cases.json`. An unknown body gets a distinct 5xx and a line on standard error, so drift fails loud.
- Generic arm (path `/generic/…`): any well-formed request gets an answer by one fixed rule. The first option or yes gets probability 0.9, and the rest share the remainder in declared order. Branch suites that ran arbitrary questions on the null backend keep their coverage.
- Fault arms: reset after the body is read (0089), 429 and 503 with a retry delay, a held reply released by a line on standard input, and a malformed reply for each of the six `FailureCause` values.

## Request identity

Case digests hash the backend URL, and every case is recorded against the canonical URL. A runner against this backend recomputes each expected digest for the served URL, the way `tests/backend/recognize.rs` swaps in `$URL`.

## Acceptance

- The command runner passes every success case in `cases.json` (01 through 19 on main today, plus 0091's additions from 26 upward) and every wire fault case against this backend, with recomputed digests. The wire faults are the backend fault cases and one case per fault arm: reset after the body, 429, 503, held reply, and each of the six malformed replies. The injection cases 20 through 25 stay in-process.
- An unknown body on the case arm returns the distinct 5xx. A planted fall-through to the generic arm turns that test red. The held-reply deadline proof runs in 0086, since the command has no deadline option.
- Each fault arm yields its expected kind and cause through the command. The generic arm answers every verb.
- A test proves it binds 127.0.0.1 only and leaves no process behind after standard input closes.
- Production code in `thinkthen` is unchanged. The crate stays under 450 nonblank Rust lines net of the moved harness. No dependency beyond what the harness uses.

## Dependencies

`opens` adds `Cargo.lock` and `deny.toml` for the dev-dependency.


After 0089, which adds 97 lines to the harness, and after 0091. Before 0076's tests are written, so they build on the moved harness once. Before 0085, 0086, and every surface ticket.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 1; state and timing 2; reach 2; proof 2; cost of error 1; total 8. Final level: 2.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-controls.md`) found broken digests, no way to name an arm, an arm the command cannot run, and a missing order. All applied, and the coordinator added the generic arm. The re-review (`sdlc/records/2026-09-24-rereview-near.md`) found the harness cannot move whole into a binary crate and the loopback cases unnamed; both applied. Confirmation accepted it.
- Code review: pending.
