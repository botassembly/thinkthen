---
flow: build
priority: 97
opens: crates/thinkthen/src/engine crates/thinkthen/tests sdlc/ratchet.json
---

# 0097: Run the host interrupt check

Status: COMPLETE.

Opened as: 2026-10-11. record `0097-run-the-host-interrupt-check.md`. Owner: Claude.

Split out of 0085 on 2026-09-24 (`sdlc/records/2026-09-24-spine-review-engine.md`, finding F1). It lands after 0085 and before 0086.

## Outcome and authority

Build the private half of ticket 0095's `CallOptions::interrupt` (gap G1 of `sdlc/planning/surfaces-port-guide.md`). A host passes a check. The engine runs it on the calling thread while the call waits, and a `true` return stops the call like its cancel token. Ian accepted this as queue item 7 of `sdlc/planning/one-line-plan-2026-09-24.md`. The proposed ADR 0017 amendment on the 0084 branch records it. Ticket 0086 exposes the public option. The command passes no check, so command behavior does not change.

## Design

- The facade's private call options gain an optional `&(dyn Fn() -> bool + Sync)`. The check is `Sync` because call options stay `Send + Sync` for the workers that share them. A host whose handle is not `Sync`, such as a SQLite database pointer, wraps it in its FFI module.
- The check enters only through the one cancel-and-deadline poll function that 0085 keeps. `annotate_schedule.rs` holds 500 nonblank lines on main, so no call site is added there. The check runs once before the first send, then at every 50 ms poll while the call waits on the width gate, a retry wait, a recording or lock wait, or bulk results. A `Batch` runs it inside `next`.
- It runs only on the calling thread, never on a worker, and never during one blocking send, since a sent attempt finishes under 0073.
- A `true` return fires the call's cancel token at that moment. Nothing new starts, sent attempts finish, and the call returns `Cancelled` with the existing stop metadata.
- A panic inside the check fires the call's cancel token, joins every worker, then resumes the payload unchanged. It is not an engine defect.

## Acceptance

- A counted loopback listener and a recorded thread ID prove the check runs only on the calling thread, before the first send and at each poll during a held width gate, a retry wait, a held recording or lock wait, and a held bulk reply.
- After the check returns `true`, the listener sees no new request, sent attempts finish, the call returns `Cancelled` with stop metadata, and every worker has joined.
- No check runs during one held single send.
- A panicking check resumes the same payload after every worker joins, and the listener sees no new request after the panic.
- With no check set, every existing cancellation, deadline, and width test stays green.
- Planted-bug proof: the record plants four bugs and shows a test turning red on each: a check call on a worker thread, a panic swallowed into `Defect`, a panic path that joins without cancelling, and a check that runs only once.
- Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass. The coordinator runs `install`, `lint`, `test`, and `spec` in order. No test opens a non-loopback socket.

## Scope

At most six production files and 180 nonblank production lines; at most 400 test lines. No dependency, no public item, no signal handler.

## Dependencies

After 0085. Before 0086. The error-index rows marked `*` in the port guide (R1-13, R1-20, R1-21, R1-22, R1-24, R2-24, R4-23, R5-9) depend on it. Their surface tickets re-run them.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Review

- Design review: `sdlc/records/2026-09-24-spine-review-engine.md`, then `sdlc/records/2026-09-24-rereview-contract.md` (cancel before join, the two waits, one entry point, the `Sync` reason). All applied; Confirmation accepted it.
- Code review: `sdlc/records/0097-code-review.md`. Findings F1 to F4 were fixed at `54ad43a2`, and the re-review accepted `d8666c50`.
