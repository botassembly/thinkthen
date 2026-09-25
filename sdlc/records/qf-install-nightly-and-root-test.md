# Quick Fix qf-install-nightly-and-root-test: name the dated nightly, and fail the cache entry in a way root cannot bypass

Status: prepared on `ticket/qf-install-nightly-and-root-test` from main `2c98d63`, with main `c8ca9a6` merged in, for the queue owner to review and land. Ian can overturn either change.

A second agent reviewed the first push, `902214c`, read-only, and accepted it with nits. It traced the owner's and the waiter's failure through `recorder.rs` and `cache_lock.rs`, and checked the digest-to-entry name, the `.locks` read order, `git grep '+nightly'`, `gate.yml`, and the ratchet count. It found that the waiter in `902214c` refused at its first entry read and never waited on the retained lock. The queue owner's feedback asked for the wait back, with a planted fault to prove it. This version restores it. The GitHub workflow never installs cargo-public-api or a nightly, so its rung 0 stops at that check before and after this change. Release ticket 0128 covers that gap.

## Cause

- The install rung checked `rustc +nightly` and advised `rustup toolchain install nightly-2026-08-23 && rustup toolchain link nightly …`. The rustc in `nightly-2026-08-23` reports `(c54751567 2026-08-22)`, so the advised toolchain failed the check. rustup 1.29.1 also refuses `nightly` as a custom toolchain name, so the link step failed.
- `cache_locking::a_failed_owner_keeps_the_empty_lock_name_and_a_waiter_sends_nothing` made the cache folder mode `0500` to fail the entry install. Root ignores that mode. In a container that runs as root, the owner installed its entry and exited 0: `left: Some(0)`, `right: Some(5)` at the owner's exit-code assertion, on every run.

## Change

- `sdlc/scripts/install` names the toolchain `nightly-2026-08-24` and calls `rustc +nightly-2026-08-24`. The check stays strict. It still requires that rustc to report the commit date `2026-08-23`. The advice is `rustup toolchain install nightly-2026-08-24`, with no link step.
- `sdlc/scripts/inventory` calls `cargo +nightly-2026-08-24 public-api`. `git grep -n '+nightly'` now finds only the dated name in scripts. `sdlc/records/0086-build-public-rust-api.md` still says `+nightly`. It is a dated record and stays as written.
- The test no longer changes modes. Its listener holds the owner's answer behind a barrier, not a 200 ms delay. After the owner's request arrives, the test reads the owner's lock under `.locks`, whose name is the digest. It starts the waiter and waits until `/proc` shows the waiter holding that lock file open. Only then does it create a directory at `<digest>.json` and release the owner's answer. The owner's hard link meets an existing path, its read of that path fails, and it exits 5 with the fixed storage error. The waiter then takes the retained lock, re-reads the entry, fails the same way, and exits 5 before it sends. The test still expects exit 5 from both, one request, one empty `0600` lock, and a `0700` lock folder. No skip, and no test-only hook or flag.
- The wait for an open lock moves out of `waiter_blocks_on_the_owners_original_inode_before_install_and_unlink` into `wait_until_open`, which both tests call. `process_has_file` reads `/proc`, so the failed-owner test is now Linux-only, like the test it shares the helper with. It was `cfg(unix)`.
- `sdlc/ratchet.json` falls from 61768 to 61764. The shared helper deleted more lines than the fix added.

## The test gate of `2026-09-24-tests-earn-their-place.md`

- What it protects: a cache owner whose install fails exits 5 and keeps one empty private lock name. A waiter blocked on that lock wakes, re-reads the entry, and refuses before it sends.
- The regression that fails it: an install failure that is swallowed, a lock removed without a valid entry, or a waiter that skips its re-read under the lock and sends.
- Why no other test catches it: the other cache-lock tests cover backend and decode failure, process death, and a successful fill. None fails the install itself, checks the retained lock, and has a waiter wake on it. `recordings::an_entry_that_cannot_be_read_is_not_reported_as_a_miss` covers the unreadable entry under replay only and counts no sends.
- Test-only hooks: none. It drives the compiled binary against the loopback listener and changes only the filesystem.

## Proof

| Run | User | Result |
| --- | --- | --- |
| old form, `origin/main` | root | fail: `left: Some(0)`, `right: Some(5)` at the owner's exit code |
| old form, `origin/main` | uid 1001 | pass |
| new form | root | pass, 5 runs of 5 |
| new form, and the inode-wait test that shares its helper | uid 1001 | pass |
| new form without the directory | root | fail: `left: Some(0)`, `right: Some(5)` at the owner's exit code, as the old form failed |
| planted fault, new form | root | fail: `left: Some(4)`, `right: Some(5)` at the waiter's exit code |
| planted fault, `902214c` form | root | pass, so the fault went unseen |
| new form with the wait skipped | root | pass. The waiter refuses at its first entry read, which is also correct. The wait is what exposes the re-read fault above |

The planted fault sits in `prepare_in` in `crates/thinkthen/src/engine/recorder.rs`. After the waiter takes the digest lock, it treats the entry as missing instead of reading it again. So it skips the re-read that stops a waiter before it sends. The `902214c` waiter refused at its first read and never reached this path. The uid 1001 runs used the compiled `backend` test binary through `runuser`.

The install check, with `RUSTUP_AUTO_INSTALL=0` so rustup would not fetch a missing name:

| `nightly-2026-08-24` | Result |
| --- | --- |
| absent | exit 1, both lines of advice printed |
| the 2026-08-23 build installed under that name | exit 1 |
| the real toolchain | exit 0 |

With auto-install on, as rustup ships, the dated name installs itself the first time the check calls it.

## Checks

In a root cloud container, 4 cores, rustc 1.95.0, `THINKTHEN_API_KEY` unset, on the merge of main `c8ca9a6`:

- `sdlc/scripts/install` exited 0.
- `sdlc/scripts/lint`: exited 0, with ratchet 61764/61764 and `inventory: 353 declared items checked, 4 plants refused` on `+nightly-2026-08-24`. The first run after the merge stopped because this container had not fetched the crates of surfaces that landed since, such as `libraries/python`. The second stopped on clippy's `expect_used` in the new helper, which now returns `io::Result`. The third passed.
- `sdlc/scripts/test`: exited 1. All 887 cargo tests passed, with 13 ignored. The last step, `sdlc/live-test`, failed with `expected status 1, got 0`, because root writes the ledger it makes mode `0400`. It passed as uid 1001 on `902214c`.
- `sdlc/scripts/surfaces` ran once, on `902214c` before the merge. rust passed. polars failed once on its 5 percent timing check and passed on one rerun. c failed twice with `Text file busy` at `libraries/c/tests/door/main.rs:129`. The queue owner files the c failure and the root-only `sdlc/live-test` failure.
- `sdlc/scripts/spec` and `sdlc/scripts/live` did not run in the container. `live` never runs from an agent.

On the Beelink at landing, as uid 1000 with `THINKTHEN_API_KEY` unset and `nightly-2026-08-24` installed, the queue owner ran install, lint, spec, and the `cache_locking` tests once each. install exited 0. lint exited 0, with the ratchet at 61764 and `inventory: 353 declared items checked, 4 plants refused`. `cargo test --test backend cache_locking` passed 8 of 8. spec exited 0, with 21 demos green and 0 red.
