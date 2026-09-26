# Two gate failures in a root cloud container

Status: Open. Items 1 and 3 are open. Item 2 is closed.

Filed on 2026-09-25 from Quick Fix `qf-install-nightly-and-root-test` (see its record). A root cloud container with 4 cores found both. Neither failed on the Beelink as uid 1000 at first. On 2026-09-25 the C failure (item 2) also hit the Beelink as uid 1000, in the surfaces rung at the landing of ticket 0130, and passed on the rung before it. Neither touches a key or the live ledger.

## 1. `sdlc/live-test` passes a write it expects to fail, as root

The test rung's last step, `sdlc/live-test`, fails with `expected status 1, got 0`. It makes a throwaway ledger read-only with `chmod 400`, then expects the write to fail. Root ignores the mode, so the write succeeds. As uid 1001 in the same container, every case passes.

Fix: block the write in a way root cannot ignore, as the Quick Fix did for the cache test. For example, put a directory at the path the write needs. Or refuse to run the gate as root with one plain sentence. This is live-ledger code, so the fix needs a ticket that names it.

## 2. The C surface fails with `Text file busy`

Status: Closed on 2026-09-26 by Quick Fix `qf-flaky-gate-tests` (record `sdlc/records/qf-flaky-gate-tests.md`). Item 1 stays open.

`sdlc/scripts/surfaces` failed the C surface on both runs. Two tests hit `ETXTBSY` when they launched a program they had just built, at `libraries/c/tests/door/main.rs:129`. A parallel test that still holds the new file open for writing causes this: a fork in another thread inherits the write descriptor before `exec`. It is a known Linux race, and a 4-core machine sees it more often.

Fix: retry the launch a few times on `ETXTBSY`. Or build every test program once before any test runs. Or run those tests on one thread.

## 3. The interrupt spawn-failure test fails as root

Added on 2026-09-26 by ticket 0133. `a_carrier_that_cannot_spawn_is_a_defect_and_sends_nothing` in `crates/thinkthen/tests/backend/interrupt.rs` runs `thinkthen decide` under `prlimit --nproc=1:`. The limit makes the SIGINT carrier thread fail to spawn, and the test pins exit 70 and the defect sentence. The kernel ignores `RLIMIT_NPROC` for a process that holds `CAP_SYS_RESOURCE` or `CAP_SYS_ADMIN`. Root holds both by default. There the thread spawns, the command runs on, and the test fails with `RLIMIT_NPROC must bind this user; root ignores it`.

Fix: the same choice as item 1. Refuse to run the gate as root with one plain sentence, or run the rungs as a user without those capabilities.

## Done when

- The test rung passes as root, or refuses to run as root with one plain sentence. That covers items 1 and 3.
- The C surface passes 20 runs in a row on a 4-core machine.
