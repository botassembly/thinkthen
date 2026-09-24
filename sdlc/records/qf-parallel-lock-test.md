# Quick Fix qf-parallel-lock-test: hold both lock-test requests until both arrive

Status: landed. It closes `sdlc/issues/2026-09-24-parallel-lock-test-fails-under-load.md`. A fresh read-only Opus review is in `sdlc/records/qf-parallel-lock-test-review.md`.

## Result

- `parallel::different_cache_digests_do_not_share_a_lock` no longer replies after a 50 ms delay. Its listener holds each of the two requests until both have arrived, and the test asserts a peak of 2. The proof depends on counts, and a slow second worker no longer lowers the peak.
- `Gathering` moves from `tests/backend/annotate/scheduling.rs` into `tests/backend/harness/mod.rs`, and both tests share it. Its wait is bounded: a request that never gets company waits out a 10-second failsafe, and the test then fails on its peak. The failsafe stays under the tool's 30-second request timeout.
- No production code changed.

## The test gate of `2026-09-24-tests-earn-their-place.md`

- What it protects: two records with different cache digests run at once under `--jobs 2`.
- The regression that fails it: one lock shared across digests.
- Why no other test catches it: the other cache-lock tests prove that equal digests share a lock, and none of them proves that different digests stay apart.
- Test-only hooks: none. It drives the compiled binary against the loopback listener.

It is an outside-in behavior test, and it stays.

## Planted bug and load rounds

The plant sets `let path = locks.join("planted-shared-lock");` in `opened_lock` in `crates/thinkthen/src/engine/cache_lock.rs`, so every digest shares one lock. Load for the loaded rounds came from `stress-ng --cpu 16` plus a loop that ran the lib test binary.

| Round | Test | Tool | Load | Result |
| --- | --- | --- | --- | --- |
| 5 runs, no load | fixed | planted | 1.5 to 2.3 | 0 pass, 5 fail, each `left: 1, right: 2` |
| 20 runs, loaded | old, `origin/main` | correct | 12.3 to 13.2 | 20 pass |
| 20 runs, loaded | fixed | correct | 13.2 to 14.1 | 20 pass |
| 20 runs, loaded | fixed | planted | 14.1 to 18.1 | 0 pass, 20 fail, each `left: 1, right: 2` |

The old test did not fail in these 20 loaded runs. The issue saw it fail at a load of 10.27 during a full `sdlc/scripts/test`, when more ran beside it.

## The hang during the load rounds

The load loop ran the lib test binary over and over. One run left the subprocess `cli::interrupt::tests::unix::sigint_child --ignored` parked for 68 minutes until the coordinator killed it. In prefix mode that child waits in `loop { park() }` for a SIGINT its parent sends, and nothing else ends it. The lock test and `Gathering` do not run in that binary. That problem is filed as `sdlc/issues/2026-09-24-the-sigint-test-child-can-park-forever.md`. Every later test command in this fix ran under `timeout`.

## Ratchet

The ceiling rises from 48801 to 48812. The helper gains a constructor, and the lock test gains its hold. Moving the helper deleted its copy in the annotate test. The other holds in the backend tests use `Barrier` for a different shape of wait.

## Checks

Main moved to `7c8c3063` while this fix was open. The merge re-measured the ratchet at 48812/48812. The whole ladder ran with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and each rung under `timeout`, at the commit that adds this section, which is the commit that lands. The merge commit on main states its result. `sdlc/scripts/live` did not run.
