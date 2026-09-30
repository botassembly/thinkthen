# Area 7: The C interface and the 13 languages on it

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

The C door is one shared library with a header of 32 exported functions, a JSON door that carries any request, typed doors for the hot paths, and per-thread failure facts. Thirteen languages call it through eleven binding folders: PHP, C#, Java, Kotlin and Scala (one JVM folder), Dart, Swift, Zig, Go, C++, Ada, Objective-C and COBOL.

Paths are under `libraries/` unless they start with `sdlc/`, `specification/` or `conformance/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, door | `c/src/` (about 2,160 with its unit tests: `ffi.rs` 473, `call.rs` 375, `failures.rs` 274, `door.rs` 238, `plan.rs` 115, `settings.rs` 100, `lib.rs` 35, plus `ffi/`, `call/`, `failures/`), `c/include/thinkthen.h` (394), `c/localize.sh` (50), `c/build.rs`, `c/DESIGN.md` (103) |
| Code, ports | About 5,300 together: PHP 201, C# 297, JVM 543, Dart 926, Swift about 270 (its header copy is ignored by Git), Zig 246, Go 548, C++ 393, Ada 843, Objective-C 349, COBOL 677 |
| Tests | C door: 34 Rust tests, 11 C programs in `c/tests/c/` (atexit, cancel, engines, fork, nulls, opts, plan, settings, threads, typed_facts, driver), 2,589 lines under `c/tests/door/`. Ports: Go has 14 tests in `go/thinkthen_test.go` (690). The other ports keep a `check.sh` plus a Python matrix and a `public_types.py` or `type_cases.py` per port (81 to 108 lines each). Ten loopback `backend.py` copies |
| Ratchets | 33 ceilings: two for the C crate (`c/ratchet.json` 4,751 equals the measured total, `c/ratchet.c.json` 1,245) and 31 across the ports (for example Dart 2,343, Go 1,470 and 656, COBOL 1,335 and 723) |
| Contract | `specification/types.md` "Inputs", "Answers and failures", "Offsets", "Parity"; `specification/result.md`; `specification/question-file.md` (`doorRequest`); `sdlc/planning/libraries/c.md`; ADRs 0037, 0047, 0082, 0101, 0106, 0111, 0112, 0113; `libraries/BINDING-AUTHOR.md` |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 5 | About 7,850 nonblank lines: door 2,160, header 394, ports 5,300 |
| States and concurrency | 4 | A per-thread, per-engine failure table that unregisters on thread exit (`c/src/failures.rs`), a cancel token fired from any thread, deadlines, an `atexit` path reached through `try_with`, a fork test (`c/tests/c/fork.c`), and Go's OS-thread pin around each call and error copy (`go/thinkthen.go:164-215`). No file locks |
| Rules and refusals | 4 | About 40. Six error codes, ten verbs, six envelope keys (`c/src/call.rs:40-50`), five deadline rules, six null and length rules, seven ownership rules, the 255-record relate cap, and the 1 MiB JSON bound on the JVM |
| Surfaces touched | 4 | 12 of 22: the door and the 11 binding folders. Every other surface is indirect |
| Settings | 5 | `thinkthen_engine_new_with` reads a closed object of 12 keys: `base_url`, `model`, `throttle`, `max_requests`, `max_request_bytes`, `cache`, `timeout`, `max_retries`, `profile`, `batch`, `record`, `replay` (`c/DESIGN.md` header table) |
| Contract weight | 4 | 4 spec pages and 8 ADRs, 12 in all |
| Churn and debt | 5 | 116 commits on `libraries/c` since 2026-09-23, about 16 of them fixes or review answers. The door's row writing was rebuilt on 2026-09-30 (ticket 0346). Open issues: the Zig linker workaround, the C door cases test over the cap, and, on main, find returns no probability |

Mean 4.4, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | C | The door follows its design page closely on ownership, nulls and cancellation (`c/DESIGN.md` sections 1 to 7, pinned by `c/tests/c/nulls.c`, `cancel.c`, `threads.c`, `atexit.c`). One primary-path divergence: `find` returns only the chosen unit, never its probability. The envelope lets `details` apply to four verbs only (`c/src/call.rs:48`), and the `find` arm writes `found.value().selected()` alone (`c/src/call.rs:273-289`). `specification/find.md:41` says `--details` holds every probability, `conformance/cases.json` case `18-find-second` expects `operation.probabilities`, and the result schema already defines a find details type. The door test compares only the unit (`c/tests/door/cases.rs:250-257`), so no door test or port proves the probability. This confirms Release QA's report on the pinned code. Minor drift: the ABI freeze list names "19 prior and eight typed" symbols (`c/DESIGN.md:134`) while the header exports 32 (`sdlc/scripts/check-c-exports.py` compares against the header) |
| Reliability | B | Failure paths are counted: `nulls.c` pins each code and message and checks the backend saw no request, `threads.c` and `engines.c` pin per-thread and per-engine failures, `atexit.c` must exit 0, and a unit test checks 200 exited threads leave no table entry (`c/src/failures/tests.rs`). One guard wraps every export (`c/src/failures.rs:282`). Code rewritten in the last 7 days caps the grade at B. Known limits are documented, not hidden: PHP and COBOL cannot fire a token during a blocking call, and Zig 0.15.2 needs an LLD workaround (`sdlc/issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md`) |
| Maintainability | C | `c/tests/door/cases.rs` holds 768 lines over the 500 cap, and no gate checks it (`sdlc/issues/2026-09-30-c-door-cases-test-over-the-file-cap.md`). `c/src/ffi.rs` (473) and `c/tests/door/settings.rs` (487) sit near the cap. Five copies of the same 19-line export check live in `dart`, `cobol`, `objective-c`, `ada` (`checks/exports.py`) and `cpp` (`fixtures/exports.py`), beside the shared `sdlc/scripts/check-c-exports.py`. Ten ports carry their own loopback `backend.py` (98 to 159 lines), and these reply over HTTP/1.0 without `Connection: close`, the pattern behind `sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`. The header copies Swift once kept are gone from Git: the Swift copy is ignored (`.gitignore:5`) and Objective-C keeps none. `thinkthen_question_file` has no port wrapper (only the C crate and the header name it), so its reason to exist is unconfirmed |

## Strengths

- One guard and one error table serve all 32 exports, and a panic becomes a defect code with fixed text rather than unwinding into the host (`c/src/failures.rs:282-307`).
- The unsafe code sits in three `ffi.rs` files with reasons on each allow (`c/src/ffi.rs:6-9`, `c/src/ffi/texts/ffi.rs:3`, `c/src/ffi/typed_facts/ffi.rs:3`).
- The release archive hides every name except `thinkthen_` functions, so a host's own SQLite or Rust runtime cannot clash (`c/localize.sh:1-50`, with the export check at `sdlc/scripts/check-c-exports.py`).
- Every port compares its exports with the header at each rebuild and never pins a count (`libraries/objective-c/checks/exports.py:13-19`).
- The Go wrapper pins the OS thread around failure reads, guards closed engines, and maps a context deadline to code 3 and a cancel to code 5 (`go/thinkthen.go:164-215`).

## Cleanup

1. **Return find's probabilities through the door.** Where: `c/src/call.rs:48` and `:273-289`, `c/tests/door/cases.rs:250-257`, plus each port's find example. Why: the spec and the shared case promise every candidate probability, and the door drops it. Allow `details` on `find`, emit the schema's find details, compare `operation.probabilities` in the door runner, then add one port check per language. Size: M, because the bindings follow the fixture. Blocks 0.1: yes, if Ian confirms the probability is promised on every surface (unconfirmed; the issue on main sets the same condition).
2. **Split `cases.rs` by case family and extend the file cap to binding Rust.** Where: `c/tests/door/cases.rs` (768), `sdlc/scripts/policy.py`. Why: the file is over the cap and nothing catches growth. Size: S. Blocks 0.1: no.
3. **Replace the five `exports.py` copies with the shared checker.** Where: `dart/checks/exports.py`, `cobol/checks/exports.py`, `objective-c/checks/exports.py`, `ada/checks/exports.py`, `cpp/fixtures/exports.py`. Why: one rule in six places. Size: S. Blocks 0.1: no.
4. **Send `Connection: close` from every binding's loopback backend, or share one backend.** Where: the ten `backend.py` files under `libraries/*`. Why: the HTTP/1.0 reuse flake affects any port whose engine sends twice on one connection (the DuckDB suite hit it 11 times in 200 runs, per the issue). Size: M. Blocks 0.1: no.
5. **Update the ABI freeze paragraph.** Where: `c/DESIGN.md:132-140`. Why: it counts 27 names while the header exports 32, and says nothing about `thinkthen_question_file`, `thinkthen_plan_json` or `thinkthen_engine_new_with` being frozen. Size: S. Blocks 0.1: no.
6. **Decide whether `thinkthen_question_file` stays exported.** Where: `c/include/thinkthen.h`, `c/src/ffi/texts/ffi.rs`. Why: no port wraps it (unconfirmed that no host needs it). Wrap it once or drop it before the ABI freezes at 0.1. Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: the header, `c/DESIGN.md`, `c/localize.sh`, `c/check.sh`, the door's `call.rs` and `ffi.rs` export list, `failures.rs` guard, `c/tests/door/cases.rs` around find, the whole Go port's entry points and error handling, the ratchet files, the shared conformance README, and the issues and ticket text named in the brief. Ports other than Go were sampled by size, export check, find usage and README.

Not checked: no test or build ran, so the find gap comes from reading the door and its tests. The ports' own matrices, the Swift, Zig and JVM wrappers line by line, and the per-language test counts were not read. Whether any host needs `thinkthen_question_file` is unknown. The release QA grid was read only as a one-paragraph issue on main.
