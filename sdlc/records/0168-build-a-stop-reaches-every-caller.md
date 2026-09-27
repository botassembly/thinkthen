# 0168: Build a stop that reaches every caller

Status: done 2026-09-27. Fresh read-only code review accepted commit `91096c8a`, and the combined full ladder passed. Owner: Codex. Ticket: `sdlc/tickets/0168-a-stop-reaches-every-caller.md`. Branch: `ticket/0168-a-stop-reaches-every-caller`, lane: `worktrees/thinkthen-codex-2`.

## Result

Python and Ruby now read the caller's token after each wait and before taking an answer or error. The existing cancellation path fires the worker's own token. Signal and Ruby interrupt handling keep their positions. The throttle row of `specification/settings.md` states that a sent request keeps its place until its attempt ends, so a later call can wait. `CHANGELOG.md` names the binding correction.

The four host-interrupt loops have no caller token that can be read back. Their same-tick answer has no durable contradiction, so they need no code change. Register 76 is a non-issue: the held throttle place keeps detached sends inside the concurrency bound. The experiment's detached-work counter is unnecessary. The code and regression evidence are recorded in `sdlc/records/2026-09-27-register-76-disposition.md`. The page sentence is a clarification, not a fix for register 76.

## Proof

- Python unit, old order: red. A queued `Ok(3)` returned success despite a fired caller token. The assertion saw `None` instead of `Some("the call was cancelled")`. The fixed order passes. Logs: `target/codex-logs/0168-python-unit-old-order.log` and `0168-python-unit-fixed.log` in this lane.
- Ruby held-reply test, old order: red in 10 of 10 runs. Every failure returned `["answer", true]` in place of `["ThinkThen::CancelledError", true]`. The fixed order passed 20 of 20 runs. Logs: `target/codex-logs/0168-ruby-old-order-10.log` and `0168-ruby-fixed-20.log` in this lane.
- Python held-reply test, fixed order: green in 20 of 20 runs. Log: `target/codex-logs/0168-python-fixed-20.log` in this lane.
- Existing Python stop regressions: all 9 tests in `test_stopping.py` passed. Existing Ruby stop regressions: all 6 tests and 25 assertions in `test_interrupt_single.rb` passed. Logs: `target/codex-logs/0168-python-stopping.log` and `0168-ruby-stopping.log` in this lane.
- A fresh read-only Codex reviewer accepted the diff at `91096c8a`, checking token order, retained interrupts, test value and ratchets. The combined candidate `3b3918e7` passed `install`, `lint`, `test`, `spec` and `surfaces`, all exit 0. `target/codex-logs/database-stop-batch-gate-results.json` in Codex-1 names the exact results and `database-stop-batch-<rung>.log` paths. The Rust test summaries total 975 passed, 0 failed and 13 ignored. Spec ran 21 green demos and 0 red. Surfaces passed all nine bindings, eight package checks and release smoke.

Only loopback conformance backend calls ran. No live or paid call ran. Targeted builds used `THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-heavy.lock` and `flock -o`. The already-built binding tests ran directly while another lane held the build lock; they compiled nothing.

The full test child alone used a 4,096 open-file limit. The separately open loopback listener fixture issue records 1,275 descriptors and failure at the inherited 1,024 limit. The gate runner did not change the global limit or close that issue.

## Ratchets

- Python Rust: 4710 to 4726, up 16 nonblank lines for the deterministic queued-answer regression test. The loop block only moved.
- Python source and tests: 2357 to 2378, up 21 for the held-reply API regression test.
- Ruby Rust: 804, unchanged. The loop block only moved.
- Ruby source and tests: 1635 to 1655, up 20 for the held-reply API regression test.
- Root Rust ratchet: unchanged.

Duplication audit: I checked the existing stop tests in `test_stopping.py` and `test_interrupt_single.rb`. They hold the reply past the next tick and cannot catch a reply that wakes the loop beside a fired token. The new tests reuse their backend and child helpers. I checked both wait loops for a shared helper; their interpreter and VM-lock waits differ, so two moved blocks stay local.

## Deferred

A token fired after the last read can race the return. A sent socket request still occupies a throttle place until its attempt ends. The settings sentence states that cost; cancellable socket reads remain outside this ticket. The detached-work counter is a non-issue for the reasons above. The host-interrupt loops remain as they are because they have no caller token to contradict their answer.
