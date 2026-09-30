# Area 4: Cancellation and deadlines across surfaces

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

One stop flag, an optional whole-call deadline, a caller's token and a host interrupt check end a call without another send, and the command turns SIGINT and SIGTERM into exit 130 and 143 with a stop line.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/`, `databases/` or `libraries/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, core | `engine/mod.rs` (334, holds `Cancel`, `Deadline`, `Width`), `cli/interrupt.rs` (335), `engine/workers.rs` (171), `cli/failure/after_signal.rs` (142), `cli/failure/stopped.rs` (59), the deadline and check wiring in `public/options.rs` (about 80) |
| Code, host glue sampled | `databases/duckdb/src/signal.rs` and `signal/` (520), `databases/sqlite/src/worker.rs` (287), `databases/postgresql/src/ffi.rs` (329), `libraries/python/src/worker.rs` (537), `libraries/r/thinkthen/src/rust/src/calls/worker.rs` (210). The other 16 surfaces were not measured |
| Code total | about 1,040 core lines and about 1,880 lines of host glue in five of 21 surfaces |
| Tests | About 64 tests. Unit: `cli/interrupt/tests.rs` (7), `engine/deadline_tests.rs` (11), `engine/host_signal_tests.rs` (5), `cli/edge/deadline_tests.rs` (2), inline in `after_signal.rs` (1). Integration: `crates/thinkthen/tests/backend/interrupt.rs` (10) and `interrupt/facts_flush.rs` (1), `tests/backend/timeout.rs` (3), `tests/public_controls.rs` (9) with `public_controls/call_facts.rs` (11), `fired.rs` (2) and `stopped.rs` (1), `tests/polars/deadline.rs` (1). Per-language cancel checks in `libraries/*` and `databases/*` (not counted) |
| Contract | `specification/channels.md` (exit 130 and 143), `specification/records.md` "Failure", `specification/result.md` "The run facts line", `specification/settings.md` row Deadline and cancel and row Throttle, `specification/backends.md` "The request"; ADR 0041, 0042, 0043, 0083 (plus 0052 and 0101 in part); tickets 0078, 0084 to 0086, 0352 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 1,040 core lines, plus about 1,880 in five host glue files sampled from 21 surfaces, so over 2,500 in all |
| States and concurrency | 5 | A signal carrier thread that blocks and unblocks SIGINT and SIGTERM around a command (`cli/interrupt.rs:194-328`), a three-step `signal_hook` registration order (`:17-22`, `:336-356`), a default handler emulated at exit so a shell sees the signal (`:99-126`, `:352-356`), workers that mask every host signal (`engine/workers.rs:168-180`), a host check that runs only on its calling thread and only while no worker is in a send (`engine/mod.rs:221-258`), and PostgreSQL interrupt flags (`databases/postgresql/src/ffi.rs:1-5`) |
| Rules and refusals | 3 | About 24. One spelling of no deadline (-1), other negatives refused, zero is spent, a computed budget clamps to zero, an unrepresentable budget means no deadline, a stop is seen before a spent deadline, a host check never runs during a send or under the usage lock, a 50 ms poll, exit 130 and 143 by the signal number, one stop line with three clauses, four fixed defect sentences, backend causes replaced by the signal's stop, Polars deadline per morsel |
| Surfaces touched | 5 | 22 of 22. `sdlc/surfaces.txt` lists 21, and each has a cancel or deadline door or takes the engine's |
| Settings | 3 | Five rows: Deadline and cancel, Timeout, Retries, Throttle, Run facts |
| Contract weight | 4 | Five spec pages with sections and six ADRs (0041, 0042, 0043, 0083, 0052, 0101) |
| Churn and debt | 5 | 122 commits on the paths since 2026-09-23, 34 since 2026-09-28 (most on `engine/mod.rs` and `cli/failure*`, which carry other work). Five closed load-flake issues on cancel and deadline tests in the window (`sdlc/issues/closed/2026-09-30-postgresql-single-cancel-wall-clock-limit-under-load.md`, `…relate-host-interrupt-test-fails-under-load.md`, `…postgresql-check-keeps-wall-clock-limits-under-load.md`, `…ordered-output-test-races-the-next-request-under-load.md`, `…duckdb-split-denials-case-failed-once-under-load.md`). Ticket 0352 is in progress |

Mean 4.1, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | Behavior matches the contract on the paths I checked. The deadline error keeps its budget and cancellation wins over a spent deadline (`crates/thinkthen/src/engine/deadline_tests.rs:36`, `:71`). A spent deadline opens no connection (`:180`). A retry wait past the budget ends as the deadline with one send (`:318`). A signal replaces only backend causes and keeps local failures and counts (`cli/failure/after_signal.rs:9-49`, test `:104`). The stop cause names match `specification/result.md:157` (`cli/failure/facts.rs:34-58`). Exit 143 follows the signal number (`cli/interrupt.rs:110-126`). Drift is small. Three `#[allow(dead_code)]` carry reasons that are now false: they say the command passes no deadline and tickets 0084 to 0086 will expose it (`engine/mod.rs:26-29`, `:158-161`, `:178-181`), yet `public/options.rs:325`, `:356` and `:437` use all three. The reference page of the site still lists exit codes 0 to 6 and 70 only (`sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md`, item 1) |
| Reliability | C | Failure paths are tested with send counts: a token fired before the final check sends nothing (`engine/http/tests.rs:211`), cancellation after a reservation refunds both charges (`:248`), a deadline while width is held reserves no send (`:304`). Routine tests still use wall-clock limits. `engine/deadline_tests.rs:233` asserts a return in under 1 s, `:291` and `:331` under 10 s, `tests/public_controls.rs:293-314` under 5 s, `tests/polars/deadline.rs:40-52` sets a 1 s budget against a 1.1 s sleep, and `tests/backend/interrupt.rs:41-46` and `:149-159` poll for 4 to 5 s. Five flake issues on these tests closed in the window, and ticket 0352 (`sdlc/tickets/0352-no-wall-clock-in-routine-tests.md`) is not landed. The host glue has been touched by 34 commits since 2026-09-28 |
| Maintainability | C | `Cancel` has 15 fields and mixes four concerns: stop flag and deadline, caller token and host check, send budgets and counts, and call facts with the attempt sink (`engine/mod.rs:40-59`). Its `impl` is split over `engine/mod.rs`, `engine/send_budget.rs:48-138` and `engine/call_facts.rs:122-140`, so no file owns it. Three stale lint suppressions (`engine/mod.rs:26`, `:158`, `:178`), and one more on `Engine::usage` (`engine/facade.rs`, reason "ticket 0086 exposes this snapshot"). Test files sit near the cap: `tests/backend/interrupt.rs` 490, `tests/public_controls/call_facts.rs` 494. Each language binding carries its own cancel glue, so a change to the stop rule is edited across 21 surfaces |

## Strengths

- The deadline has one spelling and one conversion, which the contract owns (`sdlc/planning/adr/0041-one-deadline-spelling-across-every-surface.md`, "Decision"; clamp in `engine/mod.rs:261-276`).
- The stop is observed at every wait, every 50 ms, through one function (`engine/mod.rs:311-324`, `engine/limits.rs:158-178`, `engine/backoff.rs:134-174`). A stopped waiter holds no slot.
- The final pre-send check reads no host callback while the usage lock is held (`engine/mod.rs:209-219`, `engine/http.rs:275-279`), which keeps a host interrupt from blocking a mark.
- The signal path ends the process by the signal it received, so a shell reports 130 or 143 (`cli/interrupt.rs:99-126`, `:352-356`).
- A host panic in the interrupt check fires the stop, then resumes unwinding, so every worker stops before the scope joins (`engine/mod.rs:221-242`).

## Cleanup

1. **Correct the site reference page's exit codes and key rule.** Where: `site/src/pages/reference.astro` (marketing owns it), tracked in `sdlc/issues/2026-09-30-reference-page-exit-codes-and-key-rule-drift.md`. Why: the published table omits 7, 130 and 143, and says 7 and 8 are unused while `specification/channels.md:65-67` defines 7. A user reading it expects the wrong exit code after Ctrl-C. Size: S. Blocks 0.1: yes.
2. **Finish ticket 0352 for the cancel and deadline tests.** Where: `engine/deadline_tests.rs:233`, `:291`, `:331`, `tests/public_controls.rs:293-314`, `tests/polars/deadline.rs:40-52`, `tests/backend/interrupt.rs:41-46`, `:149-159`. Why: five load flakes closed in the window and the same wall-clock bounds remain. Replace each bound with a signal or a recorded event, as `Cancel::observed` already does for waits. Size: M. Blocks 0.1: no.
3. **Remove the three stale dead-code suppressions.** Where: `engine/mod.rs:26-29`, `:158-161`, `:178-181`, and the one on `Engine::usage` in `engine/facade.rs`. Why: their reasons name tickets that have landed, and the functions are used from `public/options.rs:325`, `:356`, `:437`. I did not build, so that removal compiles without a warning is unconfirmed. Size: S. Blocks 0.1: no.
4. **Split `Cancel` into a stop and a call context.** Where: `engine/mod.rs:40-59`, `engine/send_budget.rs:48-138`, `engine/call_facts.rs:122-140`. Why: stop, deadline, token and host check belong together; budgets, facts and the attempt sink are a second value the stop carries for convenience. Areas 3 and 5 depend on the same split. Size: L. Blocks 0.1: no.
5. **Split the two test files at the cap.** Where: `crates/thinkthen/tests/backend/interrupt.rs` (490), `crates/thinkthen/tests/public_controls/call_facts.rs` (494). Why: the next case fails the 500-line cap. Size: S. Blocks 0.1: no.
6. **List the per-surface cancel proof.** Where: `libraries/*/` and `databases/*/` checks, none named in one place. Why: I could not tell which of the 21 surfaces prove a cancel and a deadline with a send count. A table in `sdlc/surfaces.txt` or the surfaces README would show any gap. Size: M. Blocks 0.1: no.

## Confidence: medium

What was read: `Cancel` and `Deadline` in full, `cli/interrupt.rs` in full, `engine/workers.rs` in part, `after_signal.rs`, `cli/failure/facts.rs`, `engine/http.rs` in full, ADR 0041, `specification/channels.md` exit rows, `specification/result.md` run facts and stop causes, and the names, bounds and clock use of every test file listed above.

Not checked: ADR 0042, 0043 and 0083 bodies, and ticket 0078, were not read in full. `cli/interrupt/tests.rs`, `engine/host_signal_tests.rs` and the host glue of 21 surfaces were only sampled, so per-language cancel behavior is unconfirmed. No test was run, so the flake evidence is from closed issue titles and the timing code, not from a measured rate. Whether the dead-code removal compiles is inferred.
