---
flow: build
priority: 86
opens: Cargo.toml Cargo.lock conformance/consumer probes sdlc/scripts/test crates/thinkthen/Cargo.toml crates/thinkthen/src/lib.rs crates/thinkthen/src/public crates/thinkthen/tests sdlc/scripts/package sdlc/scripts/install sdlc/scripts/lint sdlc/scripts/policy.py sdlc/ratchet.json README.md sdlc/planning/libraries/rust.md
---

# 0086: Expose the public Rust API

Status: landed on main 2026-09-24 after code review ACCEPT at `5102516f`; see `sdlc/records/0086-build-public-rust-api.md`. The churn run stopped at 152 public-API runs and 24 stand-in runs under Ian's one-time churn ruling, and Ian can overturn that shortfall. Owner: Claude.

## Outcome and authority

Expose the public contract frozen by ticket 0084 over the real host-neutral facade built by ticket 0085. One blocking `thinkthen` crate exports `Engine`, the typed question and question-set builders, typed descriptions, call options, cancellation and deadlines, the six error kinds, typed results, all ten engine methods (`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`, `recognize`, and `relate`), and only the conveniences approved by 0084. The public layer delegates to the 0085 facade. It does not copy question parsing, planning, scheduling, transport, retry, cache, recording, counter, recognition, or relation logic.

ADR 0017 and `sdlc/planning/libraries/rust.md` govern the implementation. Ticket 0084 owns the exact public names, signatures, builder states, result types, and convenience inventory. From ticket 0095 this ticket takes only `CallOptions::interrupt` (over 0097's private check), `deadline_seconds`, and `deadline_millis`. Ticket 0098 adds the other 0095 members after this ticket. Ticket 0085 owns the private typed execution contract. This ticket implements those records without reopening them. A contradiction stops for an 0084 or 0095 amendment instead of creating a second shape in code. Ian can overturn the public contract through that durable record.

## Public boundary

Keep one package and one published name, `thinkthen`. The library and command remain targets of that package, and the command remains behind the default `cli` feature. Add no `thinkthen-core`, `thinkthen-api`, helper package, derive package, proc-macro target, async mirror, runtime, or C export. The public root re-exports the frozen user types and functions. Private engine modules remain unreachable to an external crate.

The API stays blocking. `Engine` construction validates settings and creates no wire traffic. Explicit warming exists only if 0084 froze it. Every explicit engine call reaches the 0085 facade. Any approved module-level convenience uses one lazily initialized process-wide engine and never constructs an engine per call. Convenience calls and explicit-engine calls share results, errors, cache behavior, counters, the process width gate, fork repair, cancellation, and deadline behavior.

Implement the frozen builders directly in this crate. `Description` supports the ruled typed fields and additional ordered keys. Question, question-set, recognition, and relation builders preserve the production grammar and canonical bytes. Builder steps that can fail return `Result` at the failing step. The declarative `choices!` macro and `Choice` trait live in `thinkthen`; the macro emits the frozen inherent helpers and trait implementation. The first API has no derive and no generated user type. Structured annotate forms use the explicit builder fixed by the typed-builder ruling.

Call options carry only the frozen caller controls. Public cancellation is a clonable token safe to set from another thread. A deadline is an absolute instant derived from the caller's budget at call time. They are the cancel token, the deadline forms, and 0095's interrupt check, which delegates to 0097's private check. No host signal hook appears. A past deadline or already-cancelled token sends nothing. The six public kinds remain `usage`, `backend`, `local`, `cancelled`, `deadline`, and `defect`, with the frozen retryable signal and structured source data. No public signature returns a string error or `Box<dyn Error>`.

Return the frozen typed values and detailed results without converting through command JSON. Preserve `Answer::Unsure`, ordered bulk results, partial per-question failures, request digests, sends, cache state, model, usage, entity strength, relation probability, and `source`/`target` exactly as 0084 defines them.

## Scope and exclusions

Allowed: the public Rust modules and re-exports; thin conversions to and from the 0085 facade where the frozen public types require them; rustdoc and checked examples; external-crate compile-pass and compile-fail fixtures; public inventory enforcement; focused loopback and replay tests; the existing package, policy, install, lint, and ratchet checks; source-package metadata and Rust-library documentation needed to package this crate from source.

Source-package metadata may add the accurate library-aware description, README, repository, homepage, documentation URL, keywords, and categories required by the accepted package check. Keep the version at its pre-release value and keep `publish = false`. Package the existing source crate only. Do not build or describe release archives, prebuilt binaries, shared or static libraries, headers, installers, taps, download scripts, checksums, signatures, uploads, registry ownership, or publication.

Excluded: C and every other language or database surface; the `surfaces` branch and its integration; FFI symbols; `unsafe` anywhere except the one test-only fork call in `fork-probe` below; command behavior, output, diagnostics, or exit codes except a mechanical call-through needed to keep the shared 0085 facade single; new result semantics; a second parser, scheduler, transport, cache, or counter; async APIs; derive or procedural macros; a second crate; live or paid calls; release workflows and GitHub Actions changes.

## Deterministic acceptance

- The external consumer lives at `conformance/consumer`. It is its own Cargo workspace, and the root `Cargo.toml` excludes it, so it is not a root member (0093 keeps that entry when it adds its own). It has two members. `consumer` uses the root lint table, including `unsafe_code = "forbid"`. `fork-probe` is a test-only crate whose lint table equals the root table except `unsafe_code = "deny"`. Its one module carries `#[allow(unsafe_code, reason = "…")]` and holds the single fork call. Neither is published. Because the root excludes it, the `test` rung runs `cargo test --locked --offline --manifest-path conformance/consumer/Cargo.toml`, so the real-fork proofs, the G10 cases, and the consumer examples gate every change. The workspace's lock pins the root lock's version of every shared package.
- The external consumer crate builds offline against the packaged path with default features disabled and exercises every frozen public type, builder, result, engine method, and approved convenience. The same examples compile as doctests. The command parser and command-only modules do not compile in that consumer graph.
- `cargo tree -p thinkthen -e normal,build --no-default-features` and the packaged manifest prove one library target, no proc-macro target, and no `clap`, `csv-core`, `signal-hook`, or other CLI-only normal dependency. The Unix library graph keeps `nix` with feature `signal`. Workspace metadata is not asserted, since 0092 adds a test-only member. `cargo package` strips versionless path dev-dependencies, so only the unpacked library build is checked. The default-feature command build stays green.
- Compile-fail fixtures are generated from one table file and pin the 0084 type-state refusals, including a banded question passed to `filter`, a wrong question kind, an incomplete builder, and access to private engine internals. Diagnostics are matched by stable substance rather than compiler line decoration. Compile-pass fixtures cover string and built-question inputs, `choices!`, typed descriptions, question sets, all ten methods, and every approved convenience.
- Compile-time assertions prove `Engine` and every frozen shared handle are `Send + Sync`. A loopback test shares one engine between at least two caller threads, observes the one process-wide width rule, receives ordered typed results, and leaves no engine worker alive after both calls return.
- Every applicable shared conformance success and failure case runs through the public Rust layer with no key and no outside network. Explicit-engine and approved convenience paths produce equal values, details, error kinds, retryability, request counts, cache counts, and digests. The convenience path reuses one lazy engine; a counted local server proves repeated calls do not rebuild transport state.
- Every bulk conformance case produces identical ordered results from the frozen slice form and from an iterator that yields the same borrowed texts. A guarded iterator proves bounded pull-ahead at the configured width, and a long finite stand-in run proves memory does not grow with input length. Partial results and stable ties retain input order on both paths.
- Public calls preserve 0085's controls. Counted listener tests cover cancellation before a call, cancellation during gate wait, cancellation during a batch, a past deadline, a deadline during retry wait and blocking send, conflicting explicit width, and a child call after fork. Nothing starts after observed cancellation or deadline, spent calls send zero, already-sent work finishes under the accepted contract, and counters and cache events match the real attempts.
- No engine panic crosses a public engine method or approved convenience. A private test seam injects a panic below the public door and receives the frozen `defect` error while the process continues. A panic from the caller's own interrupt check fires the cancel token, joins every worker, then resumes its payload. The package rung proves a release library uses `panic = "unwind"`; the existing dedicated command release build remains `panic = "abort"`. No fault hook becomes public.
- A checked public API inventory matches the 0084 allowlist plus the three 0095 members above and fails on an added, removed, or signature-changed public item. It includes macro and trait exports and excludes private engine paths and the internal doctest harness. The check runs from `lint`; a planted unexpected export proves the check fails. Breaking the inventory requires an ADR and an intentional baseline update.
- `cargo package --locked --offline --allow-dirty` finishes without missing-metadata warnings. The package list contains the one crate's required source, license, README, and tests, and no sibling crate, worktree file, secret, recording credential, build output, release archive, installer, or surface package. Unpack the generated crate into a temporary directory and build its library with `--no-default-features` offline.
- Error-index rows from `sdlc/issues/closed/2026-09-23-surfaces-branch-error-index.md`: R1-10 (the panic test above), R1-11 and the deadline half of R4-12 (`deadline_after(Duration::MAX)` returns `Usage` and panics nowhere; `deadline_seconds` and `deadline_millis` match 0095's table case by case, including that the last deadline call wins and `-1` clears an earlier one), the width half of R4-12 (`width(0)` and `width(33)` return `Usage`), and R4-24. The R4-24 test spawns a child process with `THINKTHEN_BASE_URL` pointing at one loopback listener. The child builds an engine with a `base_url` naming a second listener and calls it. The second listener counts one send and the first counts none, so no test changes the environment in-process.
- Engine churn (G3, R7-1): one Rust probe committed under `probes/` runs the same load twice: 32 threads, 70 engines, a timeout on each, a refused port, and 20,000 iterations, in 8 parallel runs under heavy load. The first run builds against a temporary checkout of tag `surfaces-wave7-final` and drives the stand-in's Rust API, the second drives the public API on main, and each runs 300 times. The record keeps both counts. If the tag run shows no crash in 300 runs, the record says so, G3 and R7-1 stay open for ticket 0094's C probe, and this ticket claims no fix. A crash on main stops the ticket with an issue carrying the cores. A short smoke churn stays in `test`.
- The external consumer crate runs every shared case through 0092's loopback backend, and a loopback reply produces each `FailureCause` (G10). 0092's held-reply arm proves the deadline kind here, since the command has no deadline option.
- Real-fork proofs for ticket 0096 run from `conformance/consumer` through `fork-probe`'s one fork call: a warm parent, a busy parent holding every permit and reply, and a warm cache. `default_engine` initialized in a parent answers in the forked child, and an `Engine` clone shares counters with its source (Q15).
- Planted-bug proof: the record plants one bug per carried row and shows its test turning red. R1-10: drop the unwind guard. R1-11: use an unchecked `Instant + Duration`. R4-24: read the environment URL at send time. R4-12: accept width 33.
- Focused tests fail for the absent public surface before implementation and pass afterward. Run `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, followed by `git diff --check`. No live endpoint or paid call runs.

## Budgets

Touch at most sixteen production Rust files and add at most 1,300 nonblank production Rust lines. Touch at most fourteen Rust test, fixture, and example files and add at most 1,900 nonblank lines there. The compile-fail table counts as one file. The consumer workspace counts by its real files: at most six Rust files and 600 nonblank lines, inside the totals above. Package and inventory enforcement may add at most 260 nonblank script or baseline lines. Documentation and manifests may touch at most six files and add at most 220 net nonblank lines. Keep every Rust file below the existing 500-nonblank-line ceiling.

Add no runtime dependency. The one permitted dependency change is the compile-test development dependency already selected by the 0084 contract, if 0084 requires one; otherwise use the repository's direct compiler probes and add none. Any tool used for public inventory is pinned by the install rung and is not a crate runtime dependency. The ratchet increase equals the measured Rust increase. The implementation record names why each added block earns its lines and where duplicate facade, parser, and result conversion code was removed first.

Re-scored 2026-09-24 after code review; the queue owner approved these measured numbers. Net nonblank lines against main:

- Production Rust: 32 files and 4,329 lines, against 16 and 1,300. The 0084 inventory freezes about 60 types and 326 items, each with the docs the lints require. The 26 free-function pairs and the result getters alone exceed the old estimate. Review found no second parser, scheduler, or cache. The review cuts removed the duplicate refusal helpers, the unused `cache_bytes` field, and the throwaway set build. Its doc and comment fixes added some of those lines back.
- Tests: 24 crate test files and 1,045 lines, plus the consumer's 6 Rust files and 1,091 lines, against 14 files and 1,900 lines together. Fourteen crate files gained only the one-line `cli` gate. The consumer carries every shared case, the G10 causes, and the real-fork proofs.
- Scripts and baseline: 272 lines against 260. The inventory check is most of it.
- A macro for the free-function pairs would save about 150 lines. It is refused, because it hides the public docs.

Ian can overturn this re-score. The larger surface is the 0084 contract itself, so a smaller budget means a smaller 0084.

Stop and re-score before crossing a budget, adding a normal dependency, exposing a name absent from 0084, changing the 0085 facade, weakening a control or panic guarantee, collecting an unbounded input, or touching another surface.

## Dependencies

Ticket 0086 starts only after tickets 0084, 0095, 0085, 0097, 0096, 0078, 0091, 0092, and 0099 land. 0099 puts branch ADR 0041 on main. Ticket 0084 freezes the exact compile contract and public allowlist. Ticket 0085 supplies one private host-neutral facade over the real engine for all ten functions and proves its conformance, ordering, partial results, controls, cache, counters, and no-send rules. Ticket 0078 completes host signal ownership, and ticket 0096 completes fork recovery, at the private boundary. Through 0085, this ticket also depends on landed 0080 and 0081 result shapes and the preceding cancellation, deadline, and process-wide width controls. Do not substitute the `surfaces` stand-in contract for any landed predecessor.

The existing one-crate boundary from 0055, shared cases from 0052, partial-result contract from 0054, structured descriptions from 0069, package checks, ADR 0017, and the typed-builder ruling remain direct authorities. Ticket 0086 precedes 0098 (binding members), 0093 (first binding crate), 0094 (C), every other surface ticket, installed-artifact work, and publication.

## Complexity

Contract 4; state and timing 3; reach 4; proof 4; cost of error 4; total 19. Minimum floor: level 4 for a semver-bearing public API over shared concurrency, cancellation, fork, panic, and package boundaries. Final level: 4. Reasons: a wrong signature becomes a lasting source contract, an accidental export widens the promise, a missed unwind can terminate a host, and a convenience that rebuilds or bypasses the real engine changes cost and correctness. Claude builds (Opus subagent). A fresh Claude session reviews design and code and reconciles every exported item against 0084.

Re-score if implementation changes public semantics, requires a new dependency or crate, or reaches into C, another language, release artifacts, or publication.

## Amended 2026-09-24

0084 adds `EngineBuilder::from_env() -> Result<EngineBuilder, Error>`, and `Engine::from_env` becomes `EngineBuilder::from_env()?.build()`. The inventory check counts the new member. The command keeps its own reader in `cli/edge.rs`, so the tests below compare the two readers.

What is read, and when:

- `from_env` reads what `Environment::read` reads: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, the XDG cache home, and the XDG config file (model, cache on or off, `cache_bytes`). It never reads a `THINKTHEN_TEST_*` variable.
- `from_env` captures every value when it is called, the key included. The setters and `build` read no environment. A builder seeded at one time and built later uses the seed-time values.
- `from_env` also resolves the default cache folder, once. `default_cache()` selects that captured folder, so a seeded builder never resolves the XDG path again. `Engine::builder()` keeps 0084's own resolution at `build`.
- A malformed variable or config field returns `Error::Usage` from `from_env`, naming the variable or file and never a key value. A config file that exists but cannot be read returns `Error::Local`.
- A missing cache home (no `HOME`) does not fail `from_env`. A host such as a database service account calls `from_env()?.no_cache()` or `.cache_at(..)`. `build` fails with `Usage` only if the default cache is still selected, with the command's sentence: "no default cache folder is available; set THINKTHEN_CACHE or use no_cache".
- The seed leaves width omitted until `width()` is called, as 0077 requires of `Engine::from_env`.
- The builder holds settings only. `from_env` opens no file, creates no counter or usage handle, registers no width, and installs no handler. Registration and process state start at `build`, inside 0096's `ProcessState` accessor. A builder seeded in a parent and built in a forked child carries the parent's snapshot and gets fresh child state.

Tests, each in a child process with the environment set, like the R4-24 test. Every child starts from a cleared environment, so the real key is removed. A child sets a fake key only beside a loopback `THINKTHEN_BASE_URL`, and the listener counts zero wherever no send is expected. This follows the paid-backend rule in `sdlc/planning/surfaces-port-guide.md`.

- Oracle equality: with `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and a fixture config under `XDG_CONFIG_HOME`, an engine from `EngineBuilder::from_env()?.build()` equals one from `Engine::builder()` given each value explicitly. The comparison covers settings through a private `cfg(test)` settings seam and request digests on a counted listener. Before the digest arm makes any call, it asserts through the seam that the address is the loopback. The same environment's command plan (`plan_document`) names the same address, model, and cache folder.
- Width stays omitted after seeding, and the seam shows `None`. In its own child process, `from_env()?.width(8)?.build()` registers 8, and after an explicit 4 it fails with 0077's conflict diagnostic.
- Overrides after the seed take effect: `max_requests`, `cache_at`, `no_cache`, `default_cache`, and `cache_bytes`, each observed through the settings seam, and cache placement also through the folder. With environment key A, `api_key(B)` makes the listener see B. `base_url(second)` moves the digest and the send to the second listener; this carries R4-24 onto the path surfaces use. `model` overrides the config model.
- A malformed variable returns `Usage` naming it. An unreadable config returns `Local`. With `HOME` unset, `from_env()?.no_cache().build()` succeeds, and `from_env()?.build()` fails with the sentence above.
- Secrecy: a sentinel key set both by the variable and by `api_key` never appears in `{:?}` or `{:#?}` of the builder or the engine, nor in the `Display` or `Debug` of an error returned after the key was set.
- No effects: seeding and building send nothing on the listener, and seeding creates no file, counter, or width registration.
- Planted bug: a seed that skips `THINKTHEN_CACHE`, and the settings-seam arm of the oracle equality test turns red. The loopback listener counts zero during the plant run.

`sdlc/scripts/package` gains `cargo test --locked --offline --package thinkthen --no-default-features`, the one library-only run it lacks.

Ticket 0085 needs no change. The facade takes typed settings and reads no environment, and the public layer owns environment capture.

Amended 2026-09-24: 0078 made `nix` (feature `signal`) a Unix library dependency and `signal-hook` CLI-only. The library-graph check follows it, and 0086 adds no dependency.

## Builder note from 0097, 2026-09-24

The 0097 code review (`sdlc/records/0097-run-the-host-interrupt-check.md` on `ticket/0097-interrupt-check`) left five duties to this ticket:

1. `Batch::next` calls the private stop poll, `Cancel::stop_or_remaining`, on the calling thread at each tick. `Batch` is neither `Send` nor `Sync`, so the check's thread ID holds while the batch stays on the thread that built it.
2. Each public call attaches the check with `Cancel::with_check` at call entry, on the calling thread. That call records the thread ID. A check attached anywhere else never runs.
3. The door tells a check panic from an engine defect before it maps a panic to `Defect`. 0097 resumes the host's payload unchanged and leaves no marker. One answer: the public wrapper catches the panic around the host closure, stores the payload, returns `true`, and resumes the payload after the call joins. This ticket picks one answer and records it.
4. A `true` check fires the shared stop flag. When the host passes its own cancel token, `is_cancelled()` then reads `true`, and every sibling call on that token stops. This ticket states that in the public docs, or gives each call its own flag.
5. When the public interrupt tests land, the facade rows in `engine/facade_tests/interrupt_tests.rs` move to `CallOptions::interrupt` or are deleted, so one contract is tested at one layer.

## Builder note from 0096, 2026-09-24

The 0096 code review (finding F6, recorded in `sdlc/records/0096-build-fork-recovery.md` on `ticket/0096-fork-recovery`) leaves this to the real-fork proofs here. `cache_lock` releases a digest lock or a shared folder lock by closing its file and never calls `unlock`. A forked child keeps copies of the parent's open lock files for requests in flight. Under flock(2) such a lock stays held until every copy is closed or one copy unlocks. So a busy parent's lock outlives the parent's release until the child exits, and other waiters for that digest, or `cache prune`, keep waiting. The busy-parent real-fork proof may meet it. The fix is an explicit `File::unlock` in the lock types before close. This ticket fixes it or files it as a follow-up. This note changes no design.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-engine.md`) found two contradictions, a churn test too weak to catch the crash, and a budget too small. The panic exemption, the dependency-tree check, the churn probe, and the split into 0097 and 0098 answer them. The re-review (`sdlc/records/2026-09-24-rereview-engine.md`) found the fork call, the R4-24 test, and the churn comparison unbuildable as written; all rewritten. The confirmation (same file) asked which rung runs the consumer; `test` does. The final check accepted it.
- Code review: pending.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The tag `surfaces-wave7-frozen-2026-09-24b` holds the Rust stand-in in `libraries/rust/src/lib.rs`, `libraries/rust/Cargo.toml`, and `libraries/rust/NOTES.md`, whose finding 3 records the unused `Settings.address`. It holds the stand-in's tests in `libraries/rust/tests/defect.rs`, `deadline_fast.rs`, and `verbs.rs`, the churn probe `standin/tests/churn_probe.rs`, and `sdlc/issues/2026-09-23-the-c-door-churn-still-crashes-in-a-new-threads-start.md`. Those folders match `surfaces-wave7-final`, the tag this ticket names. `crates/thinkthen/src/lib.rs` at the tag exports only `cli::entry`. Local experiment 205's `rust/NOTES.md` covers one crate, labels without a derive, and panic unwinding. Local experiment 211's `FINDINGS.md` covers the blocking engine, fork, deadlines, and error kinds. Local experiment 218's `wave2/PLAN.md` records the unchecked `Duration` crash. The engine spine review and re-review in `sdlc/records/` accepted the boundary. `probes/` and a private experiment repository: none found.
- Keeps: One blocking `thinkthen` crate with the command behind the `cli` feature. Every call goes through the 0085 facade with no second parser, scheduler, cache, or counter. The six error kinds, `Answer::Unsure`, ordered and partial results, and the command's behavior stay.
- Changes: The crate exports 0084's frozen types, builders, the ten methods, and the approved conveniences. It adds the three 0095 members and `EngineBuilder::from_env`, an inventory check in `lint`, and a library-only package test.
- Proof: An external consumer in `conformance/consumer` runs from the `test` rung with compile-fail and compile-pass fixtures, `cargo tree` and package checks, and counted-listener tests. The churn probe, the real-fork proofs, the `from_env` equality test, and the planted bugs in "Deterministic acceptance" complete it.
- Defers: C and every other surface, async, derive macros, release, and publication. The other 0095 members go to 0098. Experiment 205's hidden engine module and async-first surface are superseded by ADR 0017's blocking engine and stay out.

## Owner's ruling, 2026-09-24

Under the tests-earn-their-place rule, and Ian can overturn it:

1. The two test-only seams go: the private `cfg(test)` settings seam and the private panic seam below the public door. Each behavior is driven through the public API or the command instead, as 0096 passed the process ID as an argument. If one behavior truly has no real boundary, the record names it and keeps the smallest seam with its reason.
2. The public inventory check is a contract check only. It reads the frozen declarations in the landed 0084 ticket and the 0086 part of the 0095 block, the single source, and compares them with the built API. No test file holds a hand-copied list. The record notes this change.

Amended 2026-09-24: the ADR 0017 amendment of that date on main renames the width setting to the throttle. `EngineBuilder::width` becomes `EngineBuilder::throttle`, and an omitted width is an omitted throttle. This ticket builds the public name and rewords the conflict message as the amendment gives it.
