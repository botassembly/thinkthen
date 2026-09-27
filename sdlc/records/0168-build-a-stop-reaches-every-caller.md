# 0168: Build a stop that reaches every caller

Status: building 2026-09-27. Owner: Codex. Ticket: `sdlc/tickets/0168-a-stop-reaches-every-caller.md`. Branch: `ticket/0168-a-stop-reaches-every-caller`, lane: `worktrees/thinkthen-codex-2`.

## Result

Python and Ruby now read the caller's token after each wait and before taking an answer or error. The existing cancellation path fires the worker's own token. Signal and Ruby interrupt handling keep their positions. The throttle row of `specification/settings.md` states that a sent request keeps its place until its attempt ends, so a later call can wait. `CHANGELOG.md` names the binding correction.

The four host-interrupt loops have no caller token that can be read back. Their same-tick answer has no durable contradiction, so they need no code change. The held throttle place is a real cost even though the experiment's detached-work counter is deferred: attempts enter process usage before send, and the throttle bounds work in flight. A cancellable socket read remains open as a possible full fix.

## Proof

- Python unit, old order: red. A queued `Ok(3)` returned success despite a fired caller token. The assertion saw `None` instead of `Some("the call was cancelled")`. The fixed order passes. Logs: `target/codex-logs/0168-python-unit-old-order.log` and `0168-python-unit-fixed.log` in this lane.
- Ruby held-reply test, old order: red in 10 of 10 runs. Every failure returned `["answer", true]` in place of `["ThinkThen::CancelledError", true]`. The fixed order passed 20 of 20 runs. Logs: `target/codex-logs/0168-ruby-old-order-10.log` and `0168-ruby-fixed-20.log` in this lane.
- Python held-reply test: pending extension build and 20-run fixed proof.
- Existing Python and Ruby stop regressions: pending targeted checks.
- Fresh read-only code review and combined full gate ladder: pending.

Only loopback conformance backend calls run. No live or paid call runs. Targeted heavy commands use `THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-heavy.lock` and `flock -o`.

## Ratchets

- Python Rust: 4710 to 4726, up 16 nonblank lines for the deterministic queued-answer regression test. The loop block only moved.
- Python source and tests: 2357 to 2378, up 21 for the held-reply API regression test.
- Ruby Rust: 804, unchanged. The loop block only moved.
- Ruby source and tests: 1635 to 1655, up 20 for the held-reply API regression test.
- Root Rust ratchet: unchanged.

Duplication audit: I checked the existing stop tests in `test_stopping.py` and `test_interrupt_single.rb`. They hold the reply past the next tick and cannot catch a reply that wakes the loop beside a fired token. The new tests reuse their backend and child helpers. I checked both wait loops for a shared helper; their interpreter and VM-lock waits differ, so two moved blocks stay local.

## Deferred

A token fired after the last read can race the return. A sent socket request still occupies a throttle place until its attempt ends. The settings sentence states that cost; cancellable socket reads and a detached-work counter remain outside this ticket. The host-interrupt loops remain as they are because they have no caller token to contradict their answer.
