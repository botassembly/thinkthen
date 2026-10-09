# 0509 — preserve-started-deadline-budget

Status: COMPLETE.

Milestone: 0.2

Reviews: revision 4bd15dd4a, accept

Reviews: revision 70f5b1115, accept

Landed: 0f05a78

## Outcome

Composed Request calls preserve the caller's configured deadline budget in errors while enforcing one unchanged absolute deadline.

## Evidence

- Starts from: SQLite 0499's unchanged installed held-call regression expects a configured 200 ms deadline; Request reports the reduced remaining duration. CallOptions::started converts Due::After into Due::At and loses the original budget.
- Keeps: Deadline error class, counted sends, cancellation, deadline_at semantics, clearing and zero-deadline refusal. Repeated started calls never extend the deadline. No SQL formatting workaround or weakened assertion.
- Changes: `crates/thinkthen/src/public/options.rs`, `crates/thinkthen/tests/public_controls.rs` and the measured source ceiling in `sdlc/ratchet.json`. Reuse the existing private Deadline without changing the engine. Extend the existing counted outside-in deadline cases and keep one record under `sdlc/records/0509-preserve-started-deadline-budget.md`.
- Proof: Run the unchanged installed SQLite held-call test against a rebuilt artifact with 0499 integrated; exercise started relative deadlines and repeated starts, absolute deadlines, clearing and zero sends. Run policy and affected tests before fresh code review; full lint, tests and specifications qualify the landing.
- Defers: New deadline controls, SQL-specific diagnostics, release operations and unrelated binding migrations.

## Progress

- 2026-10-08 started
