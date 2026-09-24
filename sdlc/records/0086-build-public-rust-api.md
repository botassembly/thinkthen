# 0086: Build the public Rust API

Status: built, code review pending. Owner: Claude.

Branch `ticket/0086-public-rust-api`. The build merges main `665975c5`. Ian can overturn every choice this record marks as decided.

## Result

`crates/thinkthen/src/public/` holds the public layer, and `lib.rs` re-exports it. The crate exports 0084's frozen types, builders, results, the ten methods, and the approved conveniences. It also exports the three 0095 members named by the ticket and `EngineBuilder::from_env`. Every call goes through the 0085 facade. The layer holds no second parser, scheduler, cache, counter, or HTTP client.

- `engine.rs` holds `Engine`, the sealed question traits, `Evidence`, and the single-evidence calls. `bulk.rs` holds `filter`, `decide_many`, `annotate`, `rank`, and `find`.
- `batch.rs` runs the facade's ordered scheduler on one thread. The calling thread pulls records only when the scheduler asks. So input is read at most one throttle ahead of the returned rows, and the caller's interrupt check runs on the calling thread at each 50 ms tick (0097 duties 1 and 2). Dropping a batch fires its stop and joins every worker.
- `options.rs` holds `CallOptions`, `CancelToken`, and the private `Stop`. `guarded` turns any engine panic into `Error::Defect`.
- `question.rs`, `builders.rs`, `set.rs`, `recognize.rs`, and `relate.rs` build the same core values the file parsers build. A relate builder writes a relate file and parses it with `RelateSpec::parse`. A recognize builder uses `RecognizeSpec::from_parts`.
- `settings.rs` holds `EngineBuilder`. `from_env` captures everything once, as the 2026-09-24 amendment says.

### Choices made here

- 0097 duty 3: the public wrapper catches a panic from the host's check, stores it, reads it as `true`, and resumes the payload after the call joins. The tests compare the resumed payload with the original.
- 0097 duty 4: each call gets its own stop flag. A `true` check stops that call alone and leaves a caller's shared `CancelToken` clear. The docs say "cancels this call alone", and `public_controls` pins it. Ian can overturn this for the shared-token reading.
- Library defaults: a 30 s timeout, 2 retries, a 1 s retry wait, and no durable usage counters. `Engine::builder()` reads no environment.
- The throttle: `EngineBuilder::throttle` replaces `width` under the ADR 0017 amendment. The messages read "throttle N is already active for this process; use throttle N or drop the throttle argument" and "a throttle is a whole number from 1 through 32".
- `Counters` reads zero when the state read fails, since `Engine::usage` returns `Counters` in 0084.

### Facade and core changes

- The facade's key reader is now `KeyReader`, an `Arc` closure in place of a plain `fn`. A builder's `api_key` then reaches the engine without an environment read.
- `engine::facade::Engine` derives `Clone` and holds its state in an `Arc<Guarded<State>>`. So `with_model` makes an engine for a question's own model that shares the counters, pool, and throttle. It uses struct-update syntax, so the contract test "only the one accessor reads the retained state" still holds. That test now also reads `facade/annotate.rs`, where the annotate plan moved from the command. The unused `Annotation` re-export is gone.
- `cli/config.rs` moved to `src/config.rs`, since `from_env` reads the same file. `default_kinds` moved from `cli/recognize/config.rs` into core.
- `core::QuestionSet::from_parts` builds a set from typed members with the root `on` and the empty and duplicate checks, and it shares `check_name` with the parser.
- `cache_lock`: `CacheLock` and `FolderGate` call `File::unlock` on drop. This closes 0096's F6 (see Tests).
- `choices!` also emits a constant check. A duplicate label compiled cleanly in an outside crate before, because a lint raised in another crate's macro stays silent. 0084 says duplicate labels fail compilation. The compile table's row failed before this fix.

## Inventory check

The owner's ruling made the check a contract check. `sdlc/scripts/inventory` runs from `lint`. It reads the rust block under "Normative public inventory" in ticket 0084 and the 0095 members that this ticket's "takes only" sentence names. It also reads the ADR 0017 rename sentence and 0084's "The crate root exports `choices!`" sentence. No file holds a copied list. It runs `cargo +nightly public-api -ss`, normalizes both sides, and compares them. It strips paths and parameter names, turns `<I as IntoIterator>::Item` into `I::Item`, and reads `Self` as the impl type. Extra trait impls may only be `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, or `StructuralPartialEq`, as 0084's prose allows. No builder, `Error`, or `ErrorDetail` may be `Clone` or `Copy`. Every public type must be `Debug`. 326 declared items match.

Each lint run plants four faults into the real listing: an added export, a removed item, a changed signature, and an extra trait. The check refuses each one. A real source plant of `pub fn planted() {}` in `public/mod.rs` failed the check with "not in the contract: fn planted()".

To match the contract exactly, getters lost `const` wherever 0084 writes a plain `fn`. Three builders lost `Default`, `ErrorDetail` lost `Display`, and the typed question and builder types got a `Debug` with no `C: Debug` bound. The install rung pins `cargo-public-api` 0.52.0 and the nightly of 2026-08-23. The crate does not depend on either.

## Tests

The owner's ruling removed both test-only seams. Each row runs through the public API, the command, or a child process.

- `tests/compile_contract.rs`: one table of eight rows compiled as an outside crate with default features off. The pass row covers string and built questions, `choices!`, typed descriptions, question sets, all ten methods, and every convenience. Fail rows cover a band passed to `filter`, a choose question passed to `decide`, a tag question passed to `choose`, an unfinished builder, the private `engine` module, a private field, and a duplicate `choices!` label. Each is matched by error code and phrase.
- `tests/public_env.rs`: child processes from a cleared environment. They cover R4-24, the oracle equality of a seeded engine and an explicit one, the throttle after seeding, the overrides, a malformed variable, an unreadable config, `HOME` unset, secrecy of a sentinel key, and no effects from seeding.
- `tests/public_controls.rs`: the deadline table (R1-11, 0095's rows, last call wins, `-1` clears) and the throttle table (R4-12). It also covers:
  - a cancel before a call and a past deadline, both of which send zero
  - stops at the throttle gate, in a retry wait, in a held send, during a batch, and during a cache-lock wait
  - a panicking check resuming its payload
  - counters and cache answers matching the real attempts
- `tests/public_batches.rs`:
  - `Send + Sync` assertions
  - two threads sharing one engine under throttle 2 with no worker left
  - slice and iterator equality for every bulk form
  - bounded pull-ahead
  - flat memory over 20,000 records
  - a smoke churn
- `public::options::tests`: R1-10 has no real seam. No input makes the engine panic, and the ruling removed the fault hook. The one unit test drives `guarded`, the door every public call passes.
- `engine/facade_tests/interrupt_tests.rs` is deleted, since its rows moved to `CallOptions::interrupt` tests (0097 duty 5).
- `conformance/consumer` is its own workspace, which the root excludes and the `test` rung runs. `consumer` runs 48 of the 52 shared cases through the public API on the loopback backend. It also covers each error kind at a real boundary and the six `FailureCause`s on the malformed arm (G10). It proves the deadline kind on the held arm and convenience equality in a child process. `fork-probe` holds the one fork call. Its proofs cover a warm parent, a busy parent holding every permit and reply, a warm cache, `default_engine` answering in a forked child, and a clone sharing counters (Q15). `policy.py` checks that the consumer lock pins only versions the root lock pins and that both lint tables equal the root table. A planted `sha2` 0.11.9 failed it.
- F6: `a_parents_released_digest_lock_frees_its_waiter_while_the_child_lives` waited 2.8 s, until the child exited, before the unlock fix. It passes after.

## Planted bugs

Each plant ran alone under the heavy lock on the branch after `28f49ea3`, and was reverted.

| Plant | Result |
| --- | --- |
| R1-10: `guarded` calls straight through | red: `a_panic_below_the_door_is_a_defect_and_the_next_call_runs` |
| R1-11: `deadline_after` adds `Instant::now() + value` unchecked | red: `deadline_numbers_follow_the_host_table_and_the_last_call_wins` panics in `std::time` |
| R4-24: the send reads `THINKTHEN_BASE_URL` at send time | red: `r4_24_the_engine_base_url_outranks_the_environment_base` |
| R4-12: `throttle(33)` accepted | red: `a_throttle_outside_one_through_thirty_two_is_refused_at_the_step` |
| `from_env` skips `THINKTHEN_CACHE` | red: `a_seeded_engine_equals_one_given_each_value_and_the_command_plan`. The listener counted zero |
| F6: no unlock on drop | red: `a_parents_released_digest_lock_frees_its_waiter_while_the_child_lives` |
| An unexpected public export | red: `inventory: not in the contract: fn planted()` |

## Churn probe

`probes/churn-0086` runs 32 threads over 70 engines, each thread making 20,000 calls to a refused loopback port, 8 runs at a time at nice 19. The ticket asked for 300 runs of each side. One batch of 8 public runs pushed the load to 120, so the 300-run target would take about 6 hours for the public side and 2 hours for the stand-in. The coordinator's load limit stopped the runs early.

- The stand-in at tag `surfaces-wave7-final` ran 24 times with no crash. G3 and R7-1 stay open for ticket 0094's C probe, and 0086 claims no fix.
- The public API ran 48 times. 47 were clean, and none ended in SIGSEGV or an abort. One run ended with exit 101, a panic on the probe's main thread, at load near 60 with the machine in swap. Its output was not kept. The probe's main thread spawns scoped threads and calls `expect`, and a failed thread start panics there. That cause is likely but unproven, so the run stays open. The next 32 runs, with output capture and backtraces, were clean.

## What the ticket did not foresee

- Find has no `none` switch in 0084, so cases `18-find-second` and `19-find-none` cannot run through the public API. `18-annotate-two-groups` reads record parts through per-question `on`, which the public API refuses. `25-defect-fault` has no outside boundary.
- `EngineBuilder::cache_bytes` changes nothing the engine does, since the facade's storage has no cap. Only its refusal of 0 is testable.
- The command's dry-run plan names the address and model but not the cache folder. The oracle test proves cache placement through the folder and a cache hit.
- A cache folder belongs to one backend address, so the busy-parent child needs its own folder.
- `fork-probe` names `libc` and `nix` (feature `process`) directly. Both were already in the root lock, so no package entered it.
- The loopback backend costs about 20 to 40 ms a request. It writes the head and the body separately with Nagle's algorithm on, which is the likely cause, not yet confirmed. The memory test uses cache hits to stay short. This belongs to the backend as an issue.
- The public API has no transport timeout setting, so the churn probe gives each call a deadline instead.

## Budgets

The ticket's budgets were exceeded, and this is the re-score. Measured nonblank lines against the merge base with main:

- Production Rust: 31 files touched, 4,326 net lines, against 16 files and 1,300 lines. The public layer is thirteen files, with every file under 500 lines. The rest are the facade, config, and core moves above. The inventory would not have fit in fewer files, because 0084 freezes about 60 types and 326 items, each with docs.
- Tests: 11 files, 1,011 net lines after deleting the 325-line facade interrupt file. The consumer workspace adds 6 Rust files and 1,067 lines against 600. Together they total 17 files against 14 and 2,078 lines against 1,900. The consumer carries every shared case, the G10 causes, and the five real-fork proofs.
- Scripts: the inventory check is 190 nonblank lines. The package, install, lint, and policy additions add about 75. The total is about 265 against 260.
- Documentation and manifests: `README.md`, `rust.md`, three crate manifests, the consumer workspace manifest, and the root manifest.

## Ratchet

The ceiling rises from main's 54,155 to 60,306, an increase of 6,151 lines. Before adding lines, I deleted the command's copy of the annotate plan, now shared through `facade/annotate.rs`. I also deleted the command's recognize kind list, now in core, and the facade interrupt tests the public tests replace. The public layer calls the facade and the core parsers and repeats none of their rules.

## Ladder

Recorded in the landing record once the code review accepts a commit.
