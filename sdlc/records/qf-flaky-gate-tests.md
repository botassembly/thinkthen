# Quick Fix qf-flaky-gate-tests: three gate tests that failed on a busy machine

Status: built, awaiting review and landing. It closes item 2 of `sdlc/issues/2026-09-25-two-gate-failures-in-a-root-container.md` and both of `sdlc/issues/closed/2026-09-25-postgresql-warm-rows-timing-fails-on-a-busy-machine.md` and `sdlc/issues/closed/2026-09-25-polars-throttle-equality-fails-on-a-busy-machine.md`. Item 1 of the first issue stays open.

## Result

### The C door compiles each program once

The cause was a shared output path. Three tests compile `tests/c/driver.c` to the same binary, `c-driver` under `CARGO_TARGET_TMPDIR`. While one test launched `c-driver`, another test's linker was rewriting it. The launch then failed in one of three ways, by the moment it hit: `ETXTBSY` while the linker wrote the file, `EACCES` before the linker set the execute bit, or `ENOENT` after the linker removed the old file.

The usual explanation does not apply here. A forked child can inherit a write descriptor only from the test process. The write descriptor lives in `ld`, a grandchild process, and it closes before `cc` returns.

`compile` in `libraries/c/tests/door/main.rs` now keeps a map from source to binary behind a `Mutex`. The first caller links the binary while holding the lock. Every later caller gets the same path back. Each binary is written once, before any test launches it.

A bounded retry on `ETXTBSY` was rejected. It misses `EACCES` and `ENOENT`, and it can launch a binary the linker has only half written.

### Polars: the held arm judges the throttle

`crates/thinkthen/tests/polars/throttle_equality.rs` no longer reads the clock. The 5 percent comparison and the 2.5 to 6 s window are gone. The delay arm still checks that a Series and a slice give equal answers and send 200 requests each. The held arm now measures the slice as well as the Series. Each must hold exactly 8 requests in flight, and still 8 after 300 ms. The Series must match the slice. A held arm counts sends, so a slow scheduler cannot change the result.

### PostgreSQL: counts and a ratio replace the 30 s cap

`twenty_thousand_warm_rows` in `databases/postgresql/check.sh` now runs the 20,000-row warm twice. The first pass must count 20,000 sends. The second pass must add none. The second pass must also take at most half the time of the first. Both passes run in the same step, so machine load slows both. The step prints both times. Each pass runs under a 180 s hang limit, up from 60 s. That limit is a hang guard only. The 20,001-row refusal is unchanged. `databases/postgresql/NOTES.md` says the 30 s bound is retired.

The measured ratio of first pass to second ran from 4.3 to 7.4 over 21 runs, well above the bound of 2.

## The test gate of `2026-09-24-tests-earn-their-place.md`

C door compile:
- What it protects: every C test launches a complete binary.
- The regression that fails it: a test relinks a shared binary while another test launches it.
- Why no other test catches it: this is the harness for every C test.
- Test-only hooks: none.

Polars throttle:
- What it protects: a Series call runs at the throttle a slice call runs at.
- The regression that fails it: a Series decided at another throttle, such as one row at a time.
- Why no other test catches it: the other Polars tests check answers, not the throttle.
- Test-only hooks: none. It uses the loopback backend's held arm.

PostgreSQL warm rows:
- What it protects: a 20,000-row warm, the cap, sends each row once and fills a cache that a second pass reads without sending. It also protects a cheap cache read per row.
- The regressions that fail it: a warm that sends again on the second pass, or one that rereads the cache for each row.
- Why no other test catches it: `warm_then_decide_sends_nothing` covers 2,000 rows and no per-row cost.
- Test-only hooks: none.

## Planted regressions

The plant scripts sit outside the repository. Each file was restored and touched after its plant. The diff holds no plant text.

| Plant | Test | Result |
| --- | --- | --- |
| `compile` from `origin/main`, with no map, 16 threads | C door, 20 runs | RED: 12 of 20 failed at the launch, 11 with `EACCES` and 1 with `ENOENT` |
| The same, pinned to 4 cores with `taskset -c 0-3` | C door, 20 runs | RED: 16 of 20 failed, 10 with `Text file busy` |
| `decide_series` decides one row per `decide_many_with` call | Polars throttle | RED: `decide_series in flight on the held arm`, left `(1, 1)`, right `(8, 8)` |
| `thinkthen_warm` appends the server process id to each row | PostgreSQL warm rows | RED: `want: 20000`, `got: 40000` |
| `thinkthen_warm` lists the `THINKTHEN_CACHE` folder once per row | PostgreSQL warm rows | RED: `the cached pass took 174431 ms, over half of the cold pass's 12131 ms` |

## Repeat runs

Each run was a separate `cargo test` or `check.sh` process. Cargo commands ran under the heavy lock with `THINKTHEN_API_KEY` unset. The Polars runs set the fake key and closed loopback address that `libraries/polars/check.sh` sets.

| Test | Runs | Load, one minute | Result |
| --- | --- | --- | --- |
| C door, 16 threads | 20 | about 1.4 | 20 pass |
| C door, pinned to 4 cores | 20 | 11.4 at the start, from other work | 20 pass |
| Polars throttle | 20 | about 2.5 | 20 pass, about 8 s each |
| PostgreSQL warm rows (`STEPS=twenty_thousand_warm_rows`) | 20 | 2 to 4 | 20 pass, first pass 11.7 to 18.8 s, second 2.3 to 2.7 s |
| Polars throttle | 3 | 3.6 to 8.3 | 3 pass |
| PostgreSQL warm rows | 1 | 3.9 to 6.9 | pass, 12.4 s and 2.4 s |

The pinned C run started at a load of 11.4 from other builders. No Polars or PostgreSQL run caught a load of 10. A run under generated load was refused by this machine's permission rules, so the Polars and PostgreSQL loaded rows reach a load of 8 at most. The Polars test no longer reads the clock. The PostgreSQL ratio has more than twice its bound of headroom.

## Ratchet

- Root `sdlc/ratchet.json`: lowered from 65801 to 65791. The Polars test lost its clock checks.
- `libraries/c/ratchet.json`: raised from 2139 to 2147. The map, its lock, the early return, the insert, and one comment line add 8 lines. I looked in `tests/door` for code to remove first. The three `driver.c` compile calls stay, because each test must work when run alone.
- `databases/postgresql/ratchet.json` counts `src` only and stays at 1808.

## Checks

Recorded below after the final run on the merged branch.
