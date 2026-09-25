# 0094: Build the C interface

Status 2026-09-25: landed in the surface batch (`sdlc/records/surface-batch-integration.md`). The one-time churn run closed R7-1 on 2026-09-25; see "Churn run" below. The 2026-09-24 pause below is history.

## Done

- The branch merged `ticket/0093-first-binding-crate` and `ticket/0098-binding-members`, the latter last at `9c0fc2cb`, which brought Rust 1.95.0. The crate names `rust-version = "1.95.0"` to match the root. 0094 needs no 0098 member beyond what 0098 builds, and it changes none.
- `libraries/c` holds the crate `thinkthen-c`, in its own workspace under ADR 0047. Its library is named `thinkthen_c`, its crate types are `cdylib` and `staticlib`, and `build.rs` sets the soname `libthinkthen.so.0`.
- `include/thinkthen.h` follows the ticket's header table. It keeps all 19 functions.
- `src/` splits the door into `ffi.rs` (the only file that allows `unsafe`), `call.rs` (the envelope and the serializer), `door.rs` (the typed doors), and `failures.rs` (the per-thread, per-engine failure table, its thread-exit cleanup, and the panic guard).
- `examples/slide.c` keeps the drawn lines. `examples/functions.c` makes one call per function in the new grammar.
- `tests/c/` holds the C rows: `nulls.c`, `opts.c`, `threads.c` (R1-9), `engines.c` (R2-16), `atexit.c` (R2-7), `fork.c`, and `driver.c`. `tests/door/main.rs` runs them under AddressSanitizer and checks the soname and the exported symbols (R2-26). `tests/door/cases.rs` runs the shared cases through the driver. `tests/door/bytes.rs` holds the door's bare values to the command's bytes.
- `DESIGN.md`, `README.md`, and `check.sh` are written. The ADR 0037 amendment is written. `sdlc/planning/libraries/c.md` points at DESIGN.md. `sdlc/surfaces.txt` names `libraries/c` landed. An issue records the site's stale C samples.
- Measured sizes: production is 1,071 nonblank Rust lines in `src/` plus 11 in `build.rs`, 1,082 in all, under the 1,100 budget. That count includes the unit tests in `failures.rs`. The envelope and serializer file `call.rs` is 222 lines, under 350. `ratchet.json` holds 1,789 and `ratchet.c.json` holds 636, the measured totals.

## Resumed 2026-09-25 in the surface batch

Status: built and checked, not reviewed. The crate compiles, Clippy is clean, and `libraries/c/check.sh` passes with 7 tests (2 unit, 5 door) and the slide diff. `sdlc/scripts/surfaces --registry` passes, and `policy.py` finds no binding failure for `libraries/c`.

Fixes made to reach green:
- `ffi.rs` dropped a `clippy::too_many_arguments` expectation that nothing met. Two test functions split to stay under 90 lines.
- The header symbol reader in `tests/door/main.rs` read no names, since `name(void` kept the parenthesis inside the word. It now takes the identifier before each `(`.
- `opts.c` compared 45 bytes of a 44-byte audit prefix, so it compared the prefix's NUL. It now uses the prefix's length.
- The filter and byte tests rewrote `{"decide":` only when the question was written without spaces. The shared cases are pretty-printed, so the rewrite now skips whitespace.
- The driver built a fresh engine for each request, so the usage counters of `40-decide-counters` never moved. One engine now serves every request to one base until an `env` request or another base comes. The counter requests run first, while the cache is still empty.
- The byte test sent every case to one cache folder, and a folder answers one backend address. Each case now names its own cache.
- `check.sh` linked the slide against `libthinkthen_c.so`, whose soname `libthinkthen.so.0` then failed to load. It now lays the library out as the archive does.
- `22-local-fault` joins the skipped cases. Main's engine opens the cache while it is built, so a broken cache folder makes `thinkthen_engine_new` return NULL, and the header gives a null engine no failure kind. DESIGN's table row says so.
- `examples/functions.txt` is pinned from the generic arm. `opts.c`'s score `0.1` holds.
- "width" became "throttle" in DESIGN, README, and the ADR 0037 amendment, matching the public `EngineBuilder::throttle`.

Lock: `libraries/c/Cargo.lock` gained `sha2` under the crate's own entry. `policy.py`'s check finds thinkthen's tree at the root lock's versions, and `cargo deny` passes.

Planted bugs, each run and reverted, each turning its test red:

| Row | Plant | Red test |
|---|---|---|
| R1-9 | `fail` clears the table before it inserts, so one thread's failure frees another's | `threads.c`: "the first read is not this thread's own failure" |
| R2-16 | every engine shares one static table | `engines.c`: "the first engine keeps its own failure", "a new engine starts with no failure" |
| R2-7 | `fail` reaches thread-local storage through `with` in place of `try_with` | `atexit.c`: the thread-local access panic, then "the failing call after teardown" |
| R2-26 | `build.rs` names the soname `libthinkthen_c.so` | the soname and symbol test |
| R2-26 | the details flag reads key presence | `opts.c`: "details false answers the bare value" |
| R3-24 | the thread-exit hook leaves its entries | the unit test finds 200 entries |

The first R2-7 plant stayed green. The old `atexit.c` failed only after teardown, when the thread's storage had never been built, so `with` built it fresh. `atexit.c` now fails once on a first engine before exit, then on a second engine in the handler.

Sizes: `src/` is 1,142 nonblank lines and `build.rs` 11, 1,153 in all. rustfmt's wrapping added most of the growth over the earlier 1,082. The unit test module in `failures.rs` holds 55 of those lines, so production code without it is 1,098, under the 1,100 budget by 2. `call.rs` is 248 lines, under 350. `ratchet.json` holds 2,067 and `ratchet.c.json` 664, the measured totals.

## Code review fixes, 2026-09-25

A fresh review of `9dae61b8` found no memory-safety bug and no leak. It returned eleven findings. Finding 1, the churn run, stays with the coordinator. The rest are fixed:

- A failed `thinkthen_engine_new` now keeps its failure in a per-thread slot, reached only through `try_with`. With a null engine, `thinkthen_error_code`, `_message`, and `_retryable` read that slot, and they keep their old answers when it is empty. A built engine clears the slot. The header names the local kind for an unreadable cache or configuration, and its failure rule gains one sentence. Shared case `22-local-fault` now runs through the door and returns `THINKTHEN_ELOCAL`. The driver prints a null engine's code and message in place of stopping.
- `ffi.rs` fell from 517 to 439 nonblank lines. One `plain!` macro writes the five plain spellings. One `typed` helper holds the shared shape of the four typed `_opts` bodies. `door::capped` and `door::entities` hold relate's 255 cap and its record mapping for both doors. The C `Judgment` and `Door` types moved to `lib.rs`, and `outs` moved to `door.rs`.
- The test-only `entries` method left `failures.rs`. The unit test reads the table directly.
- The fork child asks about new text, and the sanitizer test pins the backend count at 2.
- A new driver row prints the retry signal after a failure. The 503 arm pins `2 1`, and the 401 arm pins `2 0`.
- `run()` in `tests/door/main.rs` fails when the key appears on standard output or standard error.
- The collision plant, the library renamed to `thinkthen`, did not turn the old check red. `cargo build` never warns about that collision; only `cargo doc` does, and a stale `libthinkthen_c.so` from an earlier build hid the rename. The archive now copies the files that the build reports under the target name `thinkthen_c`. With the plant, the soname test fails with "the build reported the door's library under its own name".
- The README's build line now uses the soname layout that `check.sh` uses.
- Comments now say `try_with`, "the calling thread's", and `tests/door/`.
- Each unsafe block has a `// SAFETY:` line, and `held`, `string`, `text`, `texts`, `hand_over`, and `typed` each have a `# Safety` section.

The sizes after the review:

- `ffi.rs`: 439 nonblank lines.
- Production code in `src/` plus `build.rs`: 1,160 lines, or 1,097 without the unit test module in `failures.rs`.
- Ratchets: `ratchet.json` holds 2,119, and `ratchet.c.json` holds 678.

## Re-review, 2026-09-25

The re-review of `135c7e0e` accepted it, with one finding: nothing tested the header's promise that a built engine clears the thread's slot. After `22-local-fault`, the cases test now points `THINKTHEN_CACHE` at a good folder, builds, and has a new driver row `nullcode` print `thinkthen_error_code(NULL)`. The row must print `1`. A planted bug that clears the slot only on failure makes it print `4` and turns the test red. As a small extra, a panic while `thinkthen_engine_new` builds now puts the defect kind in the slot, so an old failure does not stay behind. Production code is 1,099 lines without the unit test module. The ratchets rise to 2,137 Rust and 685 C lines.

## Not done

- Churn (R7-1, G3): the tag-side count and the 300-run count were never taken. Ian's 2026-09-24 ruling replaced them with one C-door run. The run is below.
- The ladder has not run. The integration step runs it once for all surfaces.
- Code review: ACCEPT. The batch integration landed the work, and the churn run is below.

## Churn run, 2026-09-25

Ian ruled on 2026-09-24 that the churn probe is a one-time measurement. It never runs in the ladder, `check.sh`, or a review. This run is that measurement. It ran once, on branch `ticket/0094-churn-probe` cut from main at `07622d49`.

Setup:

- The door library came from `cargo build --release --locked --offline --lib -j 4` in `libraries/c`. It was laid out as `libthinkthen.so` with the soname link, as `check.sh` does.
- `probes/c-churn/churn.c` compiled unchanged with `cc -O1 -pthread` against `include/thinkthen.h`.
- The run used `NT=32 ITERS=20000`, the probe's 70 engines, and the probe's refused address `http://127.0.0.1:9/v1`. That makes 640,000 decide calls, plus one final call.
- It ran under `flock -o /run/user/1000/thinkthen-heavy.lock`, with the real key removed from the environment. The engine refuses a call with no key before it sends. So the probe's environment held a dummy loopback key, `sk-churn-loopback`, and a scratch `THINKTHEN_CACHE`. Port 9 refused every connection, and nothing left the machine.
- A sampler read `/proc` every half second. It would have stopped the run by process id above a one-minute load of 10. It never fired.

Result:

| Measure | Value |
|---|---|
| Crashes | 0 of 1 run. Exit status 0, no signal, empty standard error |
| Final line | `done rc=2 the backend refused the connection` |
| Wall time | 128.2 s (user 33.3 s, system 114.1 s) |
| Peak resident memory | 6,312 kB (`/usr/bin/time -v`) |
| Sampled resident memory | 5,376 kB at start, 5,864 kB at 10 s, 5,884 to 5,892 kB from 40 s to 112 s. Growth after warm-up stayed under 30 kB |
| Threads | 33 to 35 while the workers ran: 32 workers, the main thread, and short-lived resolver threads |
| Open file descriptors | 67 to 70 while the workers ran, flat. 13 as the process ended |
| One-minute load | 1.90 at start, peak 2.55 |

Against the bar: the C door passed Ian's one-run bar with zero crashes, flat memory, and flat descriptor counts. R7-1 closes on that bar.

Gaps kept in view:

- The ticket's original bar asked for up to 300 tag-side runs with a crash, and 300 door runs, 8 at a time, at the load where the crash appeared (300 to 360). None of that ran. Ian's ruling put this machine's health first.
- This run held the load near 2.5. It shows no leak and no crash under steady churn. It does not reproduce the heavy-load conditions of the tag crash.

Ian can overturn this closure and ask for a heavy-load run on a machine that can take it.
