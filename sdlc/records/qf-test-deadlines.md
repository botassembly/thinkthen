# Quick Fix qf-test-deadlines: bound every test child wait and every wait for a signal

Status: landed. It closes `sdlc/issues/2026-09-24-the-sigint-test-child-can-park-forever.md` and `sdlc/issues/2026-09-24-a-spawned-test-run-has-no-time-limit.md`.

## Result

- `crates/thinkthen/src/test_deadline/wait.rs` holds one helper, `finish`. It closes the child's input, as the standard library's `wait` does. It drains a child's piped output, waits for the child, and kills it at 60 seconds. On expiry it returns a `TimedOut` error that names what ran, so the test fails with a message. The lib tests reach it as `crate::test_deadline::finish`. The `backend` test binary includes the same file through `#[path]`, so one copy serves both.
- `crate::test_deadline::park_for_signal` replaces each `loop { park() }` in a test child. The child parks for its parent's signal and panics after 30 seconds with `no signal arrived within 30 seconds`. The SIGINT child uses it in both modes, and the identity test's paused writer uses it too.
- `harness::spawn` now calls `finish`, and its error names the arguments. Every plain child wait in the `backend` binary now goes through `finish`: the cache-lock and cache-prune children, and the waits in `parallel`, `find`, `recording_durability`, `annotate/scheduling`, `interrupt`, `relate`, `table`, `scheduling`, `tag/matrix`, and `terminal`. The two waits that already had a 2-second promptness limit keep it, and waits that follow a `kill` stay as they are.
- The SIGINT parent waits through `finish` too. A child that never gets its signal ends by its own deadline. Its exit closes its output pipe and frees a parent blocked on reading it.
- No production code changed.

## Deadlines chosen

- A test child may run 60 seconds. The issue proposed it. The whole `backend` binary finishes in about 27 seconds, and the longest honest run waits out the tool's 30-second request timeout.
- A test child waits 30 seconds for its signal. The issue proposed it. An honest parent sends the signal within milliseconds of reading `ready` or `armed`.

## Review

A fresh read-only Opus review of `5dbf9649` returned ACCEPT with three fixes and one note. Its reply is in `sdlc/records/qf-test-deadlines-review.md`.

## Planted bugs

Each plant ran once on `origin/main` at `ba6864d4` and once on the fix, under `timeout`, at a load under 7.

| Plant | Code | Result |
| --- | --- | --- |
| The SIGINT parent skips its first `sigint(child.id())` | `origin/main` | hung. `timeout 90` killed it at 90 seconds, exit 124 |
| The same | fix | failed in 30.0 seconds. The child printed `no signal arrived within 30 seconds`, and the parent failed with `left: None, right: Some(2)` |
| `main` parks forever before `thinkthen::entry()`, run on `refused::record_verbs_name_their_own_required_question_kind` | `origin/main` | hung. `timeout 120` killed it at 120 seconds, exit 124 |
| The same | fix | failed in 60.0 seconds with `thinkthen filter @…/wrong-kind-question.json --lines ran past 60 seconds and was killed` |

The plants were reverted before any commit.

## The test gate of `2026-09-24-tests-earn-their-place.md`

This fix adds no test. It changes how existing tests wait, and the planted bugs above prove the change. A regression test of the helper would spawn a child that hangs for 60 seconds on every run, and the cost outweighs the proof.

## Left open

Other test binaries still wait on children with no limit. That is filed as `sdlc/issues/2026-09-24-other-test-binaries-wait-on-children-with-no-limit.md`.

## Ratchet

The ceiling rises from 50276 to 50378. The shared helper and the child-side park add about 75 lines. The review's input fix adds one. The imports and call sites add the rest. The deleted `loop { park() }` copies and `wait_with_output` calls paid back what they could. The two 2-second promptness loops in `scheduling` and `annotate/scheduling` check a different limit and stay.
