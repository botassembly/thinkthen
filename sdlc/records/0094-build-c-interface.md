# 0094: Build the C interface

Status 2026-09-24: paused, not done. Ian asked for the work to wrap up, so the builder stopped at a clean point and pushed the work in progress to `ticket/0094-c-interface`. Nothing here is reviewed or landed.

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

## Not done

- Churn (R7-1, G3): not run. `probes/c-churn/churn.c` is committed. The runner planned for it runs 8 at a time under the heavy lock. The tag's library build was queued and then stopped at the wrap-up request, so no count exists on either side. Ian ruled on 2026-09-24 that the churn probe is a one-time measurement. 0094 runs the C-door churn once to close R7-1, only when the load is low and under the heavy lock. It never runs in the ladder, `check.sh`, or a review.
- The ladder has not run. The integration step runs it once for all surfaces.
- Code review: findings 2 through 11 fixed as above; the re-review is pending.
