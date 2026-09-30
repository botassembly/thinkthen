# Quick Fix qf-batch-cancel: a cancelled batch keeps sending

Status: no bug found. No code changed, and this branch stays unmerged. The engine sends nothing new after a cancel. The leak came from a stale TypeScript addon build. Owner: Claude. Base: `origin/main` at `2a09f9d5`.

## Evidence it started from

Ticket 0107's record, finding 1, at `sdlc/records/0107-build-typescript-surface.md` in the 0107 worktree. The binding passes a `CancelToken` through `CallOptions::cancel` into `decide_many` and keeps pulling the batch. After an abort the promise rejects within 100 ms. One second after the held arm's release, the backend had read these counts:

| Throttle | Records | Sent |
| --- | --- | --- |
| 2 | 200 | 38 |
| 4 | 20 | 20 |
| 8 | 200 | 138 |

ADR 0017 says a cancelled call sends no new request. The expected count is the number in flight.

## What was found

The engine has no hole on this path. The counts came from an addon whose worker thread did not hold the handle's token.

1. A Rust test at the public boundary did not reproduce it. It used the loopback conformance backend's held arm, `throttle(4)`, a cache folder, and 200 owned `String` records. It ran `decide_many_with` on a spawned thread, as the binding does, and fired the token from the test thread once 4 sends were held. It released the arm 2 s later and read the count 1 s after that. The count read 4, and the batch ended with 4 rows and `Cancelled`.
2. The TypeScript test and a three-row probe were run against addons built from the same sources. The probe copied the test's `aborted` helper and read the count 1 s after release.

| Addon | Test "an abort settles a held batch at once" | Throttle 2 over 200 | Throttle 4 over 20 | Throttle 8 over 200 |
| --- | --- | --- | --- | --- |
| 0107's `thinkthen.node`, as found | fails, 40 not 4 | 34 | 20 | 144 |
| Built by `build-addon.sh` from the 0107 binding source and main's engine | passes, 3 of 3 | 2 | 4 | 8 |
| The same build with one plant: the worker gets `CancelToken::new()` in place of the handle's clone | not run | 38 | 20 | 137 |
| 0107's `thinkthen.node` after its 11:13 rebuild | not run | 2 | 4 | 8 |

The planted row matches finding 1's counts. The failing addon's `__napi__call` is 0xffa bytes. The clean build's is 0xfaa and the planted build's is 0xff2. Every engine symbol has the same size in the failing and clean addons. The ticket's plant table lists "`detach()` fires no token", with count 37. The likely cause is a plant that stayed compiled into the addon after its source was restored. The 0107 addon has since been rebuilt, and its counts now equal the throttle.

## The other calls and the deadline

Code reading, with one run for the deadline:

- `filter`, `decide_many`, `rank`, and `annotate` all run through `public/batch.rs`. `Stream::pull` reads the token on the calling thread at every 50 ms tick and fires the call's flag. The scheduler refuses new records once the flag is set. Each worker checks the flag before its request, and `http.rs` checks it again before each attempt.
- `decide`, `choose`, `score`, `tag`, `details`, `find`, `recognize`, and `relate` run through `Stop::run`. Their requests run one at a time. The calling thread checks the token before each request in `ask_prepared`, and it polls between sends in `workers::on_worker`.
- A whole-call deadline lives in the shared stop, so workers read it directly. A 700 ms deadline on the same held batch ended the batch with `Deadline` before the release, and the count stayed at 4.

None of them shares the reported cause, because the cause was not in the engine.

## Test

No test was added. The four-question gate rejects one here. The credible regressions already fail existing rows in `crates/thinkthen/tests/public_controls.rs`:

- `a_stop_during_a_batch_or_a_cache_lock_wait_sends_nothing_new` stops a held batch through `Stop::interrupted` and pins the count at 4. The token takes the same path.
- `a_stop_at_the_throttle_gate_sends_nothing_new_and_sent_work_finishes` fires a token from another thread and pins the count.

A new row would test the same contract again, and it has no red run on the old code.

## Checks

With `THINKTHEN_API_KEY` unset on unchanged main plus this record. `sdlc/scripts/live` did not run.

- `install`: exit 0.
- `lint`: exit 0.
- `test`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`.

`sdlc/ratchet.json` is unchanged at 61,721. No Rust source changed.

## Deferred

- The TypeScript builder has retracted finding 1 after a clean rebuild. Ticket 0107's record and its kept failing test are that owner's to correct.
- 0107's check can run a stale addon. A plant restored with its old modification time is not rebuilt by Cargo. A clean build before the node tests, or a check that the addon is newer than every binding source, would close that gap. The 0107 owner decides.
