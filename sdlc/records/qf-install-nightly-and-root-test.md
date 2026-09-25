# Quick Fix qf-install-nightly-and-root-test: name the dated nightly, and fail the cache entry in a way root cannot bypass

Status: prepared on `ticket/qf-install-nightly-and-root-test` from main `2c98d63`, for the queue owner to review and land. Ian can overturn either change.

A second agent reviewed the ceiling rise, the test, and the scripts, read-only, and accepted with nits. It traced the owner's and the waiter's failure through `recorder.rs` and `cache_lock.rs`, the digest-to-entry name, the `.locks` read order, `git grep '+nightly'`, `gate.yml`, and the ratchet count. Its two corrections to this record are applied. The GitHub workflow never installs cargo-public-api or a nightly, so its rung 0 stops at that check before and after this change. That gap predates this fix.

## Cause

- The install rung checked `rustc +nightly` and advised `rustup toolchain install nightly-2026-08-23 && rustup toolchain link nightly …`. The rustc in `nightly-2026-08-23` reports `(c54751567 2026-08-22)`, so the advised toolchain failed the check. rustup 1.29.1 also refuses `nightly` as a custom toolchain name, so the link step failed.
- `cache_locking::a_failed_owner_keeps_the_empty_lock_name_and_a_waiter_sends_nothing` made the cache folder mode `0500` to fail the entry install. Root ignores that mode. In a container that runs as root, the owner installed its entry and exited 0: `left: Some(0)`, `right: Some(5)` at the owner's exit-code assertion, on every run.

## Change

- `sdlc/scripts/install` names the toolchain `nightly-2026-08-24` and calls `rustc +nightly-2026-08-24`. The check stays strict. It still requires that rustc to report the commit date `2026-08-23`. The advice is `rustup toolchain install nightly-2026-08-24`, with no link step.
- `sdlc/scripts/inventory` calls `cargo +nightly-2026-08-24 public-api`. `git grep -n '+nightly'` now finds only the dated name in scripts. `sdlc/records/0086-build-public-rust-api.md` still says `+nightly`. It is a dated record and stays as written.
- The test no longer changes modes. After the owner's request arrives, it reads the owner's lock under `.locks`, whose name is the digest, and creates a directory at `<digest>.json`. The owner's hard link then meets an existing path, its read of that path fails, and it exits 5 with the fixed storage error. The waiter starts after the directory exists. Its first entry read fails the same way, so it exits 5 before it takes the lock or sends. The directory stays until the waiter joins, so the old window between restoring the mode and the waiter creating its private file no longer exists. The test still expects exit 5 from both, one request, one empty `0600` lock, and a `0700` lock folder. No skip, and no test-only hook or flag.
- `sdlc/ratchet.json` rises from 61768 to 61769. The test replaced six mode lines with seven: the lock read, the entry path, the directory, and a one-line comment saying why a mode does not work. A shorter form that reused the later lock listing grew to the same count and read worse, so it was dropped. Dropping the old mode restore took away the only cleanup line; `folder()` clears the path at the next run.

## The test gate of `2026-09-24-tests-earn-their-place.md`

- What it protects: a cache owner whose install fails exits 5 and keeps one empty private lock name, and a waiter that finds the entry unreadable refuses before it sends.
- The regression that fails it: an install failure that is swallowed, or a lock removed without a valid entry. A waiter that treated an unreadable entry as a miss would send and fail the request count.
- Why no other test catches it: the other cache-lock tests cover backend and decode failure, process death, and a successful fill. None fails the install itself and checks the retained lock. `recordings::an_entry_that_cannot_be_read_is_not_reported_as_a_miss` covers the unreadable entry under replay only and counts no sends.
- What it no longer covers: the old waiter blocked on the retained lock and then failed to create its private file in the mode `0500` folder, which showed that it proves it can write before it sends. The new waiter refuses at its first entry read and never waits. Root can create that file anywhere, and the waiter's file name holds its process id, so no filesystem change blocks it ahead of time. The second reviewer offered one way back: start the waiter first and wait for `process_has_file` to show the owner's lock open, then create the directory. The waiter would then refuse on its re-read under the lock. That makes the test Linux-only, because `process_has_file` reads `/proc`, so this change leaves the choice to the queue owner.
- Test-only hooks: none. It drives the compiled binary against the loopback listener and changes only the filesystem.

## Proof

| Run | User | Result |
| --- | --- | --- |
| old form, `origin/main` | root | fail: `left: Some(0)`, `right: Some(5)` |
| old form, `origin/main` | uid 1001 | pass |
| new form | root | pass, 5 runs of 5 |
| new form | uid 1001 | pass |
| new form without the directory | root | fail: `left: Some(0)`, `right: Some(5)`, the same assertion as the old form |

The uid 1001 runs used the compiled `backend` test binary through `runuser`.

The install check, with `RUSTUP_AUTO_INSTALL=0` so rustup would not fetch a missing name:

| `nightly-2026-08-24` | Result |
| --- | --- |
| absent | exit 1, both lines of advice printed |
| the 2026-08-23 build installed under that name | exit 1 |
| the real toolchain | exit 0 |

With auto-install on, as rustup ships, the dated name installs itself the first time the check calls it.

## Checks

In a root cloud container, 4 cores, rustc 1.95.0, `THINKTHEN_API_KEY` unset:

- `sdlc/scripts/install` exited 0.
- `sdlc/scripts/lint` exited 0, including ratchet 61769/61769 and `inventory: 353 declared items checked, 4 plants refused` on `+nightly-2026-08-24`. It first needed every landed surface's crates fetched, because install fetches only the root workspace.
- `sdlc/scripts/test` exited 1. All 887 cargo tests passed, with 13 ignored. The last step, `sdlc/live-test`, failed with `expected status 1, got 0` when it runs `live` against a ledger made mode `0400`. That is the same root cause, and this Quick Fix leaves it alone because it is live-ledger code. As uid 1001 it passed: `live-test: all cases passed`.
- `sdlc/scripts/surfaces`: rust passed. polars failed once on its 5 percent timing check (`series 2.615128892s and slice 3.445184571s`) and passed on the one rerun. c failed both runs: `bytes::the_doors_bare_values_are_the_commands_bytes` and `cases::a_retried_status_is_retryable_and_a_refused_key_is_not` panicked at `libraries/c/tests/door/main.rs:129` with `Text file busy` when starting the program. This change does not touch `libraries/c`. Not run for lack of a host toolchain: typescript, ruby, r, sqlite, postgresql.
- `sdlc/scripts/spec` and `sdlc/scripts/live` did not run.
