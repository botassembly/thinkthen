---
flow: build
priority: 112
opens: libraries/ruby sdlc/scripts sdlc/planning/libraries/ruby.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0112: Port the Ruby surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the Ruby binding from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-ruby` at `libraries/ruby`, in its own Cargo workspace. `require "thinkthen"` keeps the ten verbs as `ThinkThen` module methods, plus `decide_many`, `details`, `question`, `set`, `usage`, `with_tick`, the six error classes under one `ThinkThen::Error` base, and `ThinkThen::Cancel`. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` lists Ruby among the remaining surfaces.

Draft ADR 0047 (branch `ticket/0093-first-binding-crate`) fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0095 fixes the members this binding calls, and 0098 builds them. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ticket 0105 is the accepted template, and its design review (`sdlc/records/2026-09-24-design-review-0105-0106.md` on its branch) sets the bar this ticket answers. Ian can overturn every decision below.

## Design and decisions

1. **Magnus over the public Rust API.** Ruby does not build on ticket 0094's C door. It keeps its magnus extension over `thinkthen` for three reasons. First, the rows this ticket carries (R1-20, R2-15, R3-5, R4-2, R5-9) all test the tag's `cross`: the wait with the interpreter lock released, the unblock function, `rb_thread_check_ints` under `rb_protect`, and blocked signals on the call's thread. A Fiddle call releases the lock with no unblock function, so the port would rewrite that design in Ruby and re-open closed rows. Second, `thinkthen_call` returns JSON text only. Ruby would parse every result back and lose the typed batch path. Third, Fiddle leaves Ruby's default gems after 3.4, and the `ffi` gem is a new dependency. From 0094 this binding takes the FFI edge pattern of ADR 0047 item 7: one panic guard, one error table, and one `unsafe` module. It also takes 0094's closure of R7-1, since both link the same ureq resolver. Cost: the build needs Ruby's headers and libclang for rb-sys. The next section places both. Ian can overturn this in favor of the C door.
2. **Engine.** Every call uses `thinkthen::default_engine()`. The tag's `Native::Engine` value and its connector line leave. The environment variables that `Engine::from_env` reads configure the engine. Ruby has no `Engine` class.
3. **One crossing per call on a worker.** Ruby keeps the tag's worker-thread pattern. 0095 and ADR 0047 allow it. The binding does not use `CallOptions::interrupt`, because that check runs on the engine's calling thread and would have to take the interpreter lock there. That lock re-take is the R4-2 crash shape. Each verb spawns one worker that never touches Ruby. The worker owns `&'static Engine`, a clone of the question, the texts as `String` values, and a clone of the call's own `CancelToken`. It blocks asynchronous signals (`pthread_sigmask`), builds its own `CallOptions`, and runs the verb. For `decide_many`, `filter`, and `annotate` it builds and drains the `Batch` on its own thread, since `Batch` is neither `Send` nor `Sync` (0084). It stops at the first `Err`, drops the batch, and returns that error. A call returns all of its results or raises.
4. **Interrupts are prompt.** The Ruby thread waits in 50 ms slices with the lock released and an unblock function set. Between slices it holds the lock in ordinary frames and runs `rb_thread_check_ints` under `rb_protect`. A trapped signal that raises nothing and a spurious `Thread#wakeup` leave the call alone, as at the tag. A real raise (Ctrl-C, `Thread#raise`, `Thread#kill`, a raising trap) fires the call's own token and returns that raise at once. Each slice also reads the caller's `cancel:` token. A fired caller token fires the call's own token and raises `CancelledError` at once. The worker is then detached. Its sent attempts finish under 0073, no retry starts, and its result is dropped. The tag raised only after the in-flight request finished, and this port changes that. The 0105 review asked for prompt interrupts, and a detached send can hold one width permit for up to the request timeout. A host raise surfaces unchanged: Ctrl-C raises `Interrupt`, as at the tag. The worker ignores a failed send on the closed handoff and does not panic. When the handoff closes with no result, the Ruby thread raises `DefectError`. Ian can overturn the prompt return.
5. **The watchdog runs ticks only.** `with_tick` keeps the tag's watchdog thread, one row per in-flight call, and the tick on `Thread.current`. The caller-token relay moves into the crossing's slice (decision 4). A tick that raises fires the call's own token, and the call raises the tick's error once the crossing returns.
6. **Deadlines.** `deadline:` goes through `CallOptions::deadline_seconds` after the binding refuses a non-number and a boolean. `nil` and `-1` mean none. `0` is spent. Any other negative, NaN, infinity, or a budget above 4,294,967,295 seconds raises `UsageError` (ADR 0041). The tag's own conversion leaves.
7. **Inputs.** The binding reads a whole `Enumerable` into Rust before the first send. A `String` crosses unchanged. Any other value crosses as its JSON text. Its `to_s` form never crosses. `nil`, invalid UTF-8, and a NUL byte raise `UsageError` naming the item's index (G11).
8. **Questions.** `ThinkThen.question(...)` composes the question-file object and calls `Question::from_json`, so keywords and files give one digest. A `Range` threshold becomes the file's band string. `ThinkThen.set(path)` calls `QuestionSet::load`, and keywords go through `from_json`. A broken rule from keywords raises `UsageError`, and from a file raises `LocalError` (0095, Q16). Choose and tag with runtime labels go through `Question::choose_labels` and `tag_labels`. A scalar `choose` or `tag` calls `details` and reads `value()` with no added send (G5). `filter` on a banded question raises `UsageError` before any request.
9. **Results.** `decide` returns `true`, `false`, or `nil`. `decide_many` returns an `Array`. `decide_many_with_probabilities` returns `{answer:, probability:}` pairs from `Row::probability` in the same call. `score` returns the position. `score_with_level` reads the position and the nearest level from one `details` call. `tag` returns labels. `filter` returns the passing records. `rank` and `find` wrap each text in one binding `Evidence` type holding its index and keep the tag's `Ranked(index, record, probability)` and `Found(index, unit, probability)`. `find` returns `nil` fields when nothing is selected. `annotate` parses `AnnotatedRecord::value_json` per record and keeps the tag's symbol keys, its `on:` merge, and its failed marker. `details` returns `JSON.parse` of `Details::to_json`, equal to the command's `--details` document. `usage` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`.
10. **Recognize and relate.** Keyword forms compose the `recognize @FILE` object and the version-one relate file, then call `Recognize::from_json` and `Relate::from_json`. `Entity` becomes `(name, kind, start, end, strength)`. Offsets count Unicode scalar values, and a Ruby UTF-8 string indexes the same way. `Relation` and `Edge` become `(relation, source, target, probability)` with `Entity` ends. `relate` takes `[name, kind]` pairs, hashes with `name` and `kind`, or `Entity` values. The tag's `id`, `text`, record-number ends, and `kind_field` leave.
11. **Errors and panics.** One exhaustive `match` maps `ErrorKind::name` to the six classes. The binding's own refusals raise the same classes. One panic guard wraps the worker's body and turns a shim panic into `DefectError`. `thinkthen` already stops engine panics at its public methods (0086). All conversion back to Ruby runs under the tag's `protected` wrapper.
12. **Registration.** The native half registers its functions by hand under a private `ThinkThen::Native` module. A Ruby test compares the loaded module's public names with one pinned list.

## Where the check runs

The freeze record at the tag says the Ruby check did not run, because every step ran in Docker. Its lint step also failed on `libraries/ruby` alone, since Ruby was not on `PATH`. This port drops Docker. `check.sh` runs on this Linux machine (Ubuntu 24.04, glibc 2.39) as ordinary host processes, offline, from cached toolchains.

Observed by command on 2026-09-24: no `ruby` on `PATH` and no sudo without a password. Rust 1.93.1 is installed, matching the root `rust-toolchain.toml`. `clang`, `llvm-18-dev`, and `/usr/lib/llvm-18/lib/libclang.so.1` are present. So are `build-essential`, `libssl-dev`, `zlib1g-dev`, `libffi-dev`, `libreadline-dev`, and `libgmp-dev`. `libyaml-dev` is absent. The cargo cache holds magnus 0.7.1, magnus-macros 0.6.0, rb-sys 0.9.130, rb-sys-build 0.9.130, rb-sys-env 0.1.2, bindgen 0.72.1, and clang-sys 1.9.1. The retired builder image pins Ruby 3.4.11 with SHA-256 `f79c6e789ce4f30f77c88a40b23b93e2547c512a843dc17db27ab1f5cc66f4e4`.

Options:

- **(a) Recommended: a source-built Ruby in a user cache.** A new `libraries/ruby/setup-ruby.sh` runs once on a networked machine. It fetches `ruby-3.4.11.tar.xz` and the libyaml 0.2.5 source, checks each SHA-256, and builds Ruby with `--enable-shared --disable-install-doc --with-libyaml-source-dir=…`. It installs into `${XDG_CACHE_HOME:-$HOME/.cache}/thinkthen/ruby-3.4.11`. Psych needs libyaml for `gem build` and for installing the bundled minitest gem. Building libyaml from source avoids sudo. The script writes only under that folder, and no rung runs it. The builder pins the libyaml checksum from a second published source and records both in `NOTES.md`. Cost: one networked run on this machine and one new script. No Docker, sudo, or network in any gate.
- (b) Ubuntu's packaged Ruby through apt. Ubuntu 24.04 ships Ruby 3.2, below the gemspec's `>= 3.4` floor. The port would lower the floor and prove 3.2, and the install needs sudo. Rejected.
- (c) Keep the Docker builder image. It is still on this machine. This contradicts the no-Docker requirement and the rung's offline rule, and the freeze run could not use it. Rejected.

`check.sh` uses only the pinned prefix and never a `ruby` found on `PATH`. It puts the prefix's `bin` first on `PATH` for rb-sys. It sets `LIBCLANG_PATH` to the newest `/usr/lib/llvm-*/lib` holding `libclang.so.1`, and it puts the prefix's `lib` on `LD_LIBRARY_PATH` for the shim's unit tests. A missing prefix, a missing libclang, or a missing crate prints "not run" and never "pass" (R6-2). The line names the one command to run on a networked machine: `libraries/ruby/setup-ruby.sh` or `cargo fetch --locked --manifest-path libraries/ruby/Cargo.toml`.

## What moves from the tag

- `lib/thinkthen.rb` and `lib/thinkthen/version.rb`. The keyword builders, `text_of`, the record structs, `with_tick`, the watchdog, and the `on:` clash check stay. The caller-token relay moves into the crossing. The spec builders follow main's file forms.
- `src/lib.rs`: `cross`, `wait_for`, `wake`, `quiet_signals`, `protected`, and the error table stay. The stand-in imports, `EngineValue`, the connector, `details_hash`, `recognized_value`, `edges_value`, and `annotated_value` leave. The JSON methods replace the last four. The builder splits the file into one FFI module and plain Rust files.
- `thinkthen.gemspec`: `license = "MIT"`, `Gem::Platform::CURRENT`, the extension glob, and `required_ruby_version = [">= 3.4", "< 4"]`. The tag read the version from a `VERSION` file that main lacks. The gemspec now reads it from `crates/thinkthen/Cargo.toml`.
- `.cargo/config.toml` (the macOS `dynamic_lookup` flags), `README.md`, and `examples.json`, each re-derived against 0092's generic arm.
- `build.sh` and `check.sh`, rewritten for the host (next sections). `Dockerfile` and the `.runtimes/cargo` home leave.
- Tests that call the binding keep their assertions where main's shapes allow: `test_surface.rb`, `test_error_classes.rb`, `test_deadline_bounds.rb`, `test_annotate_union.rb`, `test_harmless_wakeups.rb`, `test_tick_gc.rb`, `test_flood.rb`, `test_fork.rb`, and the interrupt suites. `test_interrupt_fast.rb`, `test_interrupt_wire.rb`, `test_interrupt_bounded.rb`, `test_cancel.rb`, `test_cancel_fast.rb`, and `test_signal_no_resend.rb` regroup into two files on the 0092 held arm. `test_pairs_one_crossing.rb` moves onto the counted backend. `public_names.rb` becomes a test that asserts. `slide_sample.rb` keeps each drawn call that main's shapes allow, and the record lists each changed line.
- `tests/conformance.rb`, rewritten onto main's `conformance/cases.json`.

These retire: the `synthetic-partial` feature and the fixture build, `ENGINE_NULL`, the wire stub on port 8214, the in-test `CountingStub`, `scripts/check_public_names.py`, and the skip-table reader. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines. `sdlc/planning/libraries/ruby.md` is the design page, and this ticket updates it.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 15 `ruby` rows, and the port guide classes all 15 as binding. This ticket re-proves eleven. It retires four whose mechanism leaves, and it guards each retirement with a check. It also re-proves the Ruby halves of twelve cross-surface rows and two engine rows. Each re-proof runs against the real engine through the 0092 loopback backend unless marked as a unit test. "Counted" means the 0092 backend's `count` line. The default engine's width is W, 4 under 0077's fallback. The record plants each bug and shows its test turning red, then green once the bug is removed.

Every held-arm test ends on its own. A timer thread releases held replies at a fixed bound of 2 s, and check.sh runs each test file under `timeout 120`. A planted bug fails an assertion or the timeout. It never hangs the rung.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-20 | closed | A 200-record `decide_many` on the held arm. Once the count reads W, a second thread calls `Thread#raise`. `Interrupt` arrives within 150 ms. After release the count stays W. | The crossing ignores a raise and waits for the answer. The 150 ms assertion turns red. |
| R1-20 single half | closed | One `decide` on the held arm. `Process.kill("INT")` once the count reads 1. `Interrupt` arrives within 150 ms. After release the count stays 1. | Wait for the worker's answer before returning, the tag's rule. The 150 ms assertion turns red. |
| R2-8 | closed | Covered by R7-9's test and plant. | As R7-9. |
| R2-15 | closed | `Thread#wakeup` and a trapped `USR1` during a held 16-record batch leave it running, and it answers all 16 after release. Two threads share one caller token. A raise on one leaves the other answering, and the shared token stays unfired. | The unblock function fires the call's token, the tag's original bug. The batch raises `CancelledError`. A second plant passes the caller's token as the engine's cancel. The sibling cancels. |
| R3-5 | closed | 300 `decide` calls on the generic arm, each with `Thread#raise` at a random offset from 0 to 5 ms. The process survives. After `GC.start` the watchdog holds no row. | Move the row's removal out of `ensure`. Rows remain. |
| R3-15b | closed | `annotate(on:)` where a question lands on the second record's key raises `UsageError` naming the key, with zero counted requests. | Check only the first record's keys. The call sends. |
| R3-16 | closed | Two threads each set their own tick. Each tick runs only while its own thread's call is in flight. A `nil` record raises `UsageError` naming its index, with zero counted requests. The single-verb half is R1-20's. | A tick stored on the module. The other thread's tick runs. A second plant sends `nil` as `"null"`. The count is nonzero. |
| R4-2 | closed | A 2,000-record `decide_many` on the held arm under a raising `USR1` trap sent every 0.2 ms. Each of four rounds ends in the trap's caught `RuntimeError`, then releases. The file runs in a child process, and a crash or hang fails it. | Take the lock inside the released region through `rb_thread_call_with_gvl`, the crash shape. The tag's snapshot hung on 2 of 3 runs. If three runs of the plant stay green, the builder stops and records it. |
| R5-9 | closed | A 200-record `decide_many` on the held arm. `Process.kill("INT")` once the count reads W. `Interrupt` arrives within 150 ms, and the count stays W. | The wait blocks until the answer lands, with no slice and no unblock function. The 150 ms assertion turns red. |
| R5-14 | closed | A test lists the loaded module's constants, public singleton methods, and struct extras, and compares them with one pinned list. It exits nonzero on any difference. | Drop `private_class_method` for `_parse_question`. The test fails. |
| R7-3 | open (fixed on branch, unverified) | Every `cargo` call in `check.sh` and `build.sh` passes `--locked` and `--offline`. A check.sh step reads both scripts and fails on a call without them. | Drop `--offline` from `cargo test`. The step fails. |
| R7-9 | open | `test_tick_gc.rb` clears the caller's thread-local by name, then checks that the drop took. A finalizer watches the tick while a second thread runs `GC.start`. The batch is held for 20 ticks, then released. | The row holds the tick through a `WeakRef`, the mutant the tag's notes name. "the collector took the tick" turns red. |
| R4-8 | closed | Retires. No builder image remains. The host build uses the root `rust-toolchain.toml` and its components. | Guarded by R7-10's check. |
| R4-9, R5-13 | closed | Retire. The port has one production build and no fixture build to restore. A check.sh step fails when `libraries/ruby/Cargo.toml` declares any feature. | Add `synthetic-partial = []`. The step fails. |
| R7-10 | open | Retires with the Dockerfile. A check.sh step fails when `libraries/ruby` holds a `Dockerfile` or a `rust-toolchain` file. | Add a Dockerfile naming 1.92.0. The step fails. |
| R1-10 host half | engine | Rust unit test with libruby linked: the guard runs a closure that panics and returns the defect kind with the Ruby class name `DefectError`. | Remove the guard. The panic fails the test. |
| R1-11 host half | engine | `deadline: 1e300`, `Float::INFINITY`, and `Float::NAN` raise `UsageError` with zero counted requests. | Convert with `Duration::from_secs_f64` in the shim. The guard turns the panic into `DefectError`, and the class assertion turns red. |
| R2-10 Ruby half | partial | `nil` and `-1` mean none. `0` raises `DeadlineError` with zero counted requests. `-2` and `4294967296` raise `UsageError`. `true` and `"soon"` raise `UsageError`. | Treat any negative as none. `-2` runs. |
| R2-25 Ruby half | closed | All six classes descend from `ThinkThen::Error`. `text_of({note: "refund"})` equals `{"note":"refund"}` (unit test). | Build the text with `to_s`. The test pins the JSON and turns red. |
| R1-34 Ruby half | closed | The built gem's spec carries `MIT` and a platform other than `ruby`. | Remove the license. The gem check fails. |
| R4-19, R5-34, R3-29 Ruby halves | closed; open | R7-3's step covers every `cargo` call. No `apt-get`, `curl`, or `docker` call remains in any script the rung runs. | Add a `curl` call to `build.sh`. The step fails. |
| R3-28, R1-28 Ruby halves | partial; closed | `libraries/ruby` passes the ADR 0047 manifest, lock, lint-table, and profile checks, deny, and both ratchets. check.sh runs `cargo clippy --locked --offline --all-targets -- -D warnings`. | A git-sourced dependency. Deny's `sources` check fails. |
| R3-30, R5-32 Ruby halves | closed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with a reason, and the three counts sum to the file's count. | Skip one case silently. The sum check fails. |
| R4-18 Ruby half | partial | R7-9's test and plant. | As R7-9. |
| R6-12 Ruby half | closed | R5-14's test fails by exit code. check.sh reads exit codes and never scrapes lines. | R5-14's plant. check.sh exits nonzero. |
| R1-31 | waive | One kind table. A Rust unit test maps each `ErrorKind` to its class name. | Map `Deadline` to `BackendError`. |
| R2-31 | partial | One panic guard. A check.sh step counts one `catch_unwind` site in `src`. | Add a second guard. |

Not carried: R5-35's Ruby half retires with the lint container. `lint` no longer builds the Ruby crate. R1-29's Ruby half (the floating `ruby:3.4-trixie` tag) retires, since the Ruby source is pinned by checksum. R5-37's Ruby half (the Mac recipe) moves to the release ticket (queue item 11). This ticket runs nothing on a Mac. R5-42 is standin-only in the port guide. R2-27 is not a Ruby row. G9 waits on ADR 0047 item 5, and the Ruby page states whichever answer Ian gives.

R2-29 asks for rulings on record. The rulings that live only in the tag's `NOTES.md` land in a short Ruby section of ADR 0047. They cover the crossing that never takes the lock beneath the engine call, a host raise that surfaces unchanged, the prompt return, the watchdog's tick cadence, `nil` refused, and records as JSON.

## Other acceptance

- A caller's token cancels promptly during a send. The test cancels it from a second thread during a held single `decide` and gets `CancelledError` within 150 ms with one counted send.
- A pre-cancelled token sends nothing, single or bulk, and raises `CancelledError`.
- Red first: the ported tests fail against an empty `libraries/ruby` workspace for the stated reason, then pass.
- `cargo test --lib --locked --offline` in `libraries/ruby` passes with libruby linked. It covers the guard, the kind table, the closed handoff, and a worker that outlives its caller.
- The conformance runner runs every applicable case through the 0092 case arm with recomputed digests. It adds the rank and find arms that the tag's runner lacked. Case 18 (cancel mid-batch) uses the held arm. Case 17 asserts counter differences. No backend arm is added.
- `decide_many_with_probabilities` over 20 records counts exactly 20 requests (G4).
- A fork test warms the default engine, calls `Process.fork`, and gets an answer in the child. The parent's counters do not move (0096, Q15). The parent's read runs under a 10 s bound.
- Case 68 (an accent and an emoji) returns the same `start` and `end` in Ruby as in Rust.
- Only the FFI module allows `unsafe`, with a reason (ADR 0047 item 3).
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset. A secrecy test reads every raised message and `inspect` for the key and for the base URL's credentials.

## The check it adds to the gate ladder

- `libraries/ruby/check.sh` joins the surface registry as landed. The `surfaces` rung runs it with the 0092 backend. It builds once with `build.sh`: `cargo build --release --locked --offline`, a copy of the `.so` into `lib/thinkthen/`, and `gem build`. The tag's second build and restore step leave.
- Its steps, in order: the file checks for R7-10, R4-9, R7-3, and R2-31; the toolchain probe with its "not run" line; the build; clippy; `cargo test --lib`; each Ruby test file under `timeout 120`; conformance; the gem content check; the slide sample.
- `lint` runs on `libraries/ruby` without Ruby. It runs the ADR 0047 manifest, lock, lint-table, and profile checks and the registry check. It runs deny as `cargo deny --offline --manifest-path libraries/ruby/Cargo.toml check --config deny.toml advisories bans licenses sources` against the root `deny.toml`, with the planted git dependency. It runs `ratchet.mjs` on `libraries/ruby/ratchet.json` for Rust and `ratchet.rb.json` for Ruby.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, and `unexpected_cfgs` with one allowed cfg. It denies `expect_used`, `unwrap_used`, `indexing_slicing`, `panic`, and `allow_attributes_without_reason`. The tag's shim uses `expect` and `unwrap` in several places, and the port rewrites them. `#[magnus::init]` emits an exported symbol, so it lives in the FFI module. If code generated by magnus or rb-sys trips any forbid-level lint, the builder stops and records the case for an ADR 0047 amendment.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off; `magnus` `0.7.1` with `rb-sys`; `rb-sys` `0.9.130`; `libc` at the root lock's version. The build tree adds rb-sys-build 0.9.130, rb-sys-env 0.1.2, bindgen 0.72.1, clang-sys 1.9.1, and libloading 0.8.9. Each is in the tag's lock and this machine's cache. Their licenses (MIT, MIT or Apache-2.0, BSD-3-Clause, Apache-2.0, ISC) are all in the root `deny.toml` allow list. The binding lock resolves `thinkthen`'s own tree to the root lock's versions (ADR 0047 item 1).
- Toolchain: Ruby 3.4.11 and libyaml 0.2.5 from source, pinned by SHA-256 in `setup-ruby.sh`. Tests use only Ruby's standard library and the bundled minitest. The gem depends on no other gem.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, deny's result, and both checksums, and the review record says so (repo `CLAUDE.md`).

## Budgets

Measured at the tag: `src/lib.rs` 941 nonblank lines with its tests, `lib/thinkthen.rb` 426, 19 test files 1,582, `check.sh` and `build.sh` 180.

- Production Rust: at most four files and 850 nonblank lines, each file under 500. The JSON methods remove about 150 lines of hand conversion. The worker handoff and the caller-token read add about 60.
- Ruby library: `thinkthen.rb` and `version.rb` together at most 450 nonblank lines.
- Tests: at most 15 Ruby test files and 1,650 nonblank lines, the conformance runner at most 300 inside that, and at most 200 nonblank Rust unit-test lines. One helper, `tests/backend.rb`, starts the 0092 backend and speaks its `count` and `release` lines, and it replaces the tag's stub code.
- Scripts: `check.sh`, `build.sh`, and `setup-ruby.sh` together at most 240 nonblank lines. Gate changes under `sdlc/scripts` at most 30 nonblank lines.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/libraries/ruby.md`, and the ADR 0047 section, at most 240 net nonblank lines.
- Ratchet: `libraries/ruby/ratchet.json` and `ratchet.rb.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, or using `CallOptions::interrupt`.

## Exclusions

The C door route (decision 1 records why). Release gems, platform builds for other hosts, uploads, and a Mac run (queue item 11). Any change to `thinkthen`. An `Engine` class in Ruby. Async forms. Docker. Any live or paid call.

## Dependencies

After 0086 lands. Also after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`, `Row::probability`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface and this ticket follows 0094's FFI edge. 0092 and 0099 have landed. `setup-ruby.sh` runs once on this machine before the first check.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. Signals and raises cross the interpreter lock, and a wrong rule crashes the host VM or loses a user's Ctrl-C.

## Review

- Design review: pending.
- Code review: pending.
