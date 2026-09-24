---
flow: build
priority: 107
opens: libraries/typescript sdlc/scripts sdlc/planning/libraries/javascript.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0107: Port the TypeScript surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the Node binding from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-typescript` at `libraries/typescript`, in its own Cargo workspace. `import * as tt from "thinkthen"` and `require("thinkthen")` keep the ten verbs, `decide_many`, `details`, `question`, `usage`, and `ThinkThenError` with `kind` and `retryable`. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` puts TypeScript after Python.

Draft ADR 0047 fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern. Ticket 0105 is the accepted template for a host binding, and this ticket copies its gate and review shape. Tickets 0095 and 0098 fix the members this binding calls. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ian can overturn every decision below.

## Design and decisions

1. **Over the Rust API, beside the C door.** The addon is a napi-rs crate that depends on `crates/thinkthen` by path. It does not load `thinkthen-c` (0094). ADR 0047 item 1 forbids a binding that depends on another binding, and item 7 forbids a shared helper crate. The C door builds only `cdylib` and `staticlib`, so no Rust crate can link it. Node has no built-in foreign-function loader. A loader package would be a runtime dependency, and `sdlc/planning/libraries/javascript.md` rules those out. 0094 still serves as the reference for the FFI edge: one panic guard, one error-kind table, and one module that allows `unsafe`. 0094's `deadline_millis` choice for a host that counts milliseconds carries over too.
2. **One door, JSON inside.** The tag's single `call(op, spec, payload, cancel, deadlineMs)` export stays. The payload crosses as one string, and the result comes back as one envelope string, `{"ok": value}` or `{"err": {kind, retryable, message}}`. `index.js` parses it once. `details`, `annotate`, `recognize`, and `relate` values come from 0095's `Details::to_json`, `AnnotatedRecord::value_json`, `Recognized::to_json`, and `Edge::to_json`. The remaining values (a decide word, a choice, a position, labels, kept indexes, `{index, probability}` pairs, a found index or null, and the four counters) are built with `serde_json` inside the envelope. The envelope is private to this package, so its bytes need not match the command.
3. **Engine.** Every call uses `thinkthen::default_engine()`, configured by the environment variables `Engine::from_env` reads. The addon adds no engine setting. TypeScript has no `Engine` class. A `worker_threads` worker that loads the addon shares the one process engine.
4. **Calls and options.** Each verb calls the matching `Engine` `_with` form inside `Task::compute` on a libuv worker, with one `CallOptions` built there. `signal` maps to `CallOptions::cancel` through `CancelHandle`. The handle wraps `CancelToken`. The binding passes no `CallOptions::interrupt` check. The JavaScript thread cannot run on the libuv worker, and the token already reaches every engine poll.
5. **Prompt abort.** An abort must settle the promise at once on every verb. The engine's token alone cannot do that during one blocking send, because a sent attempt finishes under 0073. Design: `invoke` races the native promise against the signal. On abort it fires the token, removes its listener, and rejects with `ThinkThenError` of kind `cancelled`, `retryable` false, the pinned sentence "the call was cancelled by its AbortSignal", and `cause` set to `signal.reason`. The native task finishes in the background, and its late envelope is dropped. A signal already aborted rejects before any crossing. Cost: about 15 lines of `index.js`. Until its send ends, a dropped task holds one libuv worker and one width permit. The engine's 30-second request timeout bounds both. No change to `thinkthen` is needed.
6. **Deadlines.** `deadlineMs` follows ADR 0041. `null`, a missing key, and `-1` mean no deadline. `0` is spent. Every other negative and a budget above 4,294,967,295 seconds are `usage`. `index.js` keeps the host rule: a bool, a string, an array, or any non-number is `usage` before the crossing. The addon takes an `f64` and refuses a NaN, an infinity, or a fraction with `usage` naming the exact value. A whole value goes through `CallOptions::deadline_millis`. That method owns the rest of the rule. New here: a fractional budget is refused. `Math.max(0, end - Date.now())` is always whole, and a `performance.now()` budget needs `Math.ceil`. The tag's own negative check in the addon leaves.
7. **Inputs.** A record list is an array of strings. A non-string item raises `usage` naming its index (G11). A string that fails `isWellFormed()` raises `usage` naming its index, because Node-API would replace a lone surrogate silently. The same checks apply to a single evidence text.
8. **Questions.** `tt.question(...)` and the verbs spell the question-file JSON and call `Question::from_json`, so parts and a file give one digest. A band `[low, high]` becomes the file's `"low:high"`. `annotate` takes a set path (`QuestionSet::load`), set JSON text, or a set object (`from_json`). A broken rule from arguments raises `usage`, and from a file raises `local` (0095, Q16). Scalar `choose` and `tag` call `details_with` and read `value()`, with no added send (G5). The last-object rule of 2026-09-21 stays: `options`, `labels`, `levels`, and `top` ride beside `signal` and `deadlineMs`, and any other key is `usage` naming it.
9. **Results.** `decide` resolves `true`, `false`, or `null`. `decide_many` resolves a list of those. `score` resolves the position, and the nearest level moves to `details`. `tag` resolves a list of labels. `filter` resolves the passing texts. `rank` wraps each text in one binding `Evidence` type holding its index and resolves `{index, record, probability}` objects, cut by `top`. `find` resolves `{index, unit, probability}`, or `null` when nothing is selected (0095, Q16). This aligns `find` with Python. `annotate` resolves one object per record from `value_json`, with `null` for unsure and the failed marker `{failed: {kind, cause}}`. `details` resolves `JSON.parse` of `Details::to_json`, equal to the command's `--details` document and to Python's `tt.details`. `usage` resolves `{requests_sent, cache_answers, input_tokens, output_tokens}`. A bulk call that meets an error rejects whole, and partial rows are dropped, as at the tag.
10. **Recognize and relate.** `recognize` spells the `recognize @FILE` object of `specification/question-file.md` from `kinds`, `relations`, `threshold`, and `relationThreshold`, and calls `Recognize::from_json`. `relate` spells the version-one relate file from `relations`, `either`, and `threshold` and calls `Relate::from_json`. `relate` takes `{name, kind}` objects or `[name, kind]` pairs as `Entity` values. The engine refuses more than 255 entities or an exact duplicate before any send. The wrapper converts every `start` and `end` from Unicode scalar values to UTF-16 code units (0095, Q13), in entities and in relation endpoints, so `text.slice(start, end)` is the name.
11. **Errors and panics.** The envelope's `kind` comes from `ErrorKind::name`, the one kind table. One panic guard in `compute` turns a binding panic into `defect`. `thinkthen` already stops engine panics at its public methods (0086).
12. **Files and build.** The crate moves from `addon/` to `libraries/typescript/Cargo.toml` with `src/` and `build.rs`, where 0093's checks scan. `build-addon.sh` runs `cargo build --release --locked --offline` with the `$HOME` remap and copies the library to `thinkthen.node`. `@napi-rs/cli` leaves the dev dependencies. The 308-line generated `loader.cjs` and `loader.d.ts` give way to a hand-written `loader.js` of at most 20 lines. `index.mjs` is written by hand, and the tag's name generator retires. A test compares the names exported by `index.js`, `index.mjs`, and `index.d.ts`. Platform-named binaries and packages belong to the release ticket (queue item 4).
13. **Node version.** `engines` becomes `>=22`, the only version the check runs. The tag claimed 18 untested.

## What moves from the tag

- `index.js`, reshaped by decisions 5 to 10. The annotate field mapper and the hand-built `details` object leave.
- `index.d.ts`, following the new shapes: `find` returns `Found | null`, `details` names the `thinkthen.result/1` fields, and `usage` names the four counters. `index.mjs` by hand. `README.md`, `LICENSE`, and `.gitignore`.
- `package.json`: `license` MIT, `private`, `exports` for `import` and `require`, and `files` naming `index.js`, `index.mjs`, `index.d.ts`, `loader.js`, `LICENSE`, and `thinkthen.node`. `typescript` pinned at exactly `7.0.2`. `package-lock.json` regenerated offline.
- `addon/src/lib.rs` becomes `src/`. `CancelHandle`, `CallTask`, the `Op` table, `guarded`, `deadline_of`, and `failure_envelope` keep their roles. `records`, `question`, `set`, and the spec readers now call the public API. The stand-in connector, `relate_checked`, and the `synthetic-partial` feature leave.
- `build-addon.sh` and `check.sh`, with the changes in the gate section.
- `examples.json` and `tests/examples.test.mjs`. Each expected value is re-derived against 0092's generic arm.
- Tests that call the binding keep their assertions: `verbs`, `errors`, `abort_listener`, `deadline_bounds`, `ownership`, `slide`, `fork` with its child and worker scripts, `form.json`, and `types.test.ts`. `recognize.test.mjs` keeps its offset, any-kind, and last-object cases. Its stand-in recording cases (the deck's `located_in` finding and the unrecorded-text refusal) retire. `bulk.test.mjs` moves onto the held and delay arms. Its connection-reuse test retires, because the engine owns the pool (ADR 0017) and no binding row carries it. `cancel_fast.test.mjs` retires with the null backend, and the held-arm abort test replaces it. The builder regroups by topic into at most twelve files.
- `tests/conformance.test.mjs`, rewritten onto main's `conformance/cases.json`. The branch skip table and its Python reader retire.

`NOTES.md` stays at the tag as history, and the port writes a new one of at most 100 lines. `DIVERGENCES.md` retires. Its ten items move, rewritten, into a "Differences from Python" section of `sdlc/planning/libraries/javascript.md`. This ticket also brings that page up to the 2026-09-21 call shape.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists five TypeScript rows, and this ticket carries all five. It also carries R2-27 whole, since 0105 left it here, and the TypeScript halves of fourteen cross-surface rows and two engine rows. Each re-proof runs against the real engine through a 0092 backend unless marked as a unit test. The record plants each bug below and shows its test turning red, then green once the bug is removed. "Counted" means the backend's `count` line. "Held" is 0092's held arm, and "delay" is 0106's `/arm/delay/<ms>`. `W` is `default_engine()`'s width. The builder reads it from 0086 and pins it in each test as a literal.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-25 | closed | On the generic arm, 2,000 calls sharing one `AbortSignal` leave `getEventListeners(signal, "abort").length` at 0. A pre-aborted signal adds none. An abort mid-call leaves none. | Drop the listener removal in `invoke`'s `finally`. The count reads 2,000. |
| R3-20 | closed | `tt.score("How urgent?", "x")` with no `levels` rejects with `usage`, the pinned sentence "score takes its levels in the last object: { levels }", and zero counted. | Let a bare score string fall through to `specOf`. The engine answers with another sentence, and the pinned assertion turns red. |
| R3-25 | open/waive | In a child with `UV_THREADPOOL_SIZE=4`, five single `decide` calls on the held arm reach exactly 4 counted and stay at 4 for 300 ms. The test then releases, all five resolve, and the count reads 5. With a pool of 8, five calls reach 5. The pool is the bound on threads, and the ADR 0047 section records it as a waiver Ian can overturn. | Run each call on its own spawned thread. The pool-of-4 child reaches 5, and the assertion turns red. |
| R3-26 | closed | `check.sh` corrupts `12-score-upper`, `17-annotate-mixed`, and `27-decide-many` in turn in a temporary copy of `cases.json`. Each run must exit nonzero and name its case. | The runner catches a case failure and still exits 0. The corruption step fails. |
| R4-13 | closed | `types.test.ts` compiles `tt.score("q", "t", { levels: [...] })` under `tsc --strict`. `README.md` and the `deadlineMs` comment in `index.d.ts` both carry the pinned sentence "No deadline is spelled null, left out, or -1." A `check.sh` step reads both. | Remove the bare-string overload. `tsc` fails. Separately, write "-1 is refused" in `README.md`. The step fails. |
| R2-27 shapes | partial/waive | `tt.details` deep-equals `JSON.parse` of the command's `--details` output for the same question, text, and backend. Case `19-find-none` resolves `null`. The "Differences from Python" section lists every remaining difference with its reason. | Return the tag's hand-built details object. The equality turns red. |
| R2-27 worker | partial/waive | The R3-25 test above. On `/arm/delay/100`, a 96-record `decide_many` keeps timer drift under 250 ms over thirty 10 ms ticks. | Export a synchronous door that blocks the JavaScript thread. Drift passes 1 s, and the test ends when the run ends. |
| R2-10 TypeScript half | partial | `-1`, `null`, and a missing key run with no deadline. `0` rejects with `deadline` and zero counted. `-2`, `0.5`, and `4294967296000` reject with `usage` and zero counted. `4294967295000` runs. | Treat any negative as none. `-2` runs. |
| R5-8 TypeScript half | closed | `true`, `false`, `"5"`, and `[]` as `deadlineMs` reject with `ThinkThenError` of kind `usage` and zero counted. | Drop the `typeof` check. Node-API raises a plain `Error`, and the kind assertion turns red. |
| R1-11 host half | engine | `NaN`, `Infinity`, `1e300`, and `Number.MAX_VALUE` reject with `usage` and zero counted. A Rust unit test calls `deadline_of(Some(f64::NAN))` directly and gets `usage`, so a native caller cannot bypass the wrapper. | Drop the finite-and-whole check. `NaN` casts to 0, and the call rejects with `deadline`. |
| R1-10 host half | engine | Rust unit test with no Node: `guarded` runs a closure that panics and returns a `defect` envelope with `retryable` false. A second guarded call then returns its value. Node-API registration is compiled out under `cfg(test)`. | Remove the guard. The panic fails the test. |
| R1-31 | waive | One kind table. A Rust unit test builds the envelope kind for each of the six `ErrorKind` variants and compares it with `ErrorKind::name`. | Map `Deadline` to `"backend"` in a local match. |
| R2-31 | partial | One panic guard. A `check.sh` step counts exactly one `catch_unwind` in `src`. | Add a second guard. |
| R1-34 TypeScript half | closed | `npm pack --dry-run --json --offline` lists `LICENSE`, and `package.json` carries `"license": "MIT"`. The stand-in's recordings no longer exist to ship. | Remove the license field. The pack check fails. |
| R1-30 and R3-31 TypeScript half | closed | The built `thinkthen.node` holds zero copies of `$HOME`. The pack list holds exactly the `files` entries. No test build exists to ship once `synthetic-partial` leaves. | Drop the remap from `build-addon.sh`. The count is nonzero. |
| R1-29, R3-29, and R4-19 TypeScript half | closed and partial | Every `cargo` call in `check.sh` and `build-addon.sh` passes `--locked` and `--offline`. `npm ci` passes `--offline`. Every dev dependency is an exact version. A `check.sh` step reads both scripts and `package.json` and fails on any gap. | Drop `--locked` from `build-addon.sh`. Separately, write `^7.0.2`. The step fails. |
| R3-30 and R5-32 TypeScript half | closed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with the reason, and the three counts sum to the file's count. A not-run case never prints as pass. | Skip one case silently. The sum check fails. |
| R1-28 and R3-28 TypeScript half | closed | `lint` runs on `libraries/typescript` as the gate section says, including Clippy with `-D warnings`. | Add a `dbg!` call in `src`. `lint` fails. |

Retired here: the TypeScript part of R7-12. `DIVERGENCES.md` leaves the tree, and its content moves as stated above. The standin-only rows retire with the stand-in, as the port guide lists.

Not closed here: G9 waits on ADR 0047 item 5, and `javascript.md` states whichever answer Ian gives.

R2-29 asks for rulings on record. The rulings that live only in the tag's `NOTES.md` and `DIVERGENCES.md` land in a short TypeScript section of ADR 0047. They are one error class with a `kind`, the `AbortSignal` as the cancel gesture, milliseconds as the deadline unit, calls on the libuv pool, and the question value as a function. Decisions 5, 6, and 12 above join them.

## Other acceptance

- Prompt abort, single. One `decide` waits on the held arm. The test aborts once the count reads 1. The promise rejects with `cancelled` and the pinned sentence within 100 ms. The test then releases, and after a 500 ms settle the count reads exactly 1. Plant: drop the race. The rejection waits for the release, and the 100 ms assertion turns red. A 1 s timer sends the release, so the test ends.
- Prompt abort, batch. A 200-text `decide_many` waits on the held arm. The test aborts once the count reads `W`. The promise rejects within 100 ms. The test then releases, and after a 500 ms settle the count reads exactly `W`. Plant: race without firing the token. After the release the batch keeps sending, and the count passes `W`.
- A deadline during a held send. One `decide` on the held arm with `deadlineMs: 300` rejects with `deadline` by 450 ms with one counted send.
- Case 18 of the branch (cancel mid-batch) is the batch abort test above. It uses 0092's existing held arm, and no backend arm is added.
- A dropped task does not leak into the next call. After the single abort test releases, a new `decide` on the same backend resolves.
- Case `41-offsets-past-an-accent-and-an-emoji` returns `start` and `end` in UTF-16 units, and `text.slice(start, end)` equals each name. Plant: skip the conversion. The emoji case turns red.
- Case `40-decide-counters` asserts differences in `usage()` around a call.
- A lone surrogate at index 2 of a record list rejects with `usage` naming index 2 and zero counted. Plant: drop the `isWellFormed` check. The call sends, and the count turns red.
- A child process started after the parent's first call answers its own call. A `worker_threads` worker that loads the addon answers, and the parent's `usage()` shows its send.
- The name test: `index.js`, `index.mjs`, and `index.d.ts` export the same fifteen names.
- The addon holds no hand-written `unsafe`. `#[napi]` items live in one module, `src/node.rs`. Only that module carries `#[allow(unsafe_code, reason = "…")]`, for the macro's output. A `check.sh` step finds no `unsafe` token in `src`.
- Red first: the ported tests fail against an empty `libraries/typescript` workspace for the stated reason, then pass.
- `cargo test --lib --locked --offline` in `libraries/typescript` passes with no Node present.
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset. A secrecy test reads every rejection's `message` and `String(error)` for the key and for the base URL's credentials.
- Tests that can end. Each held-arm test releases in `t.after`, so a failed assertion cannot leave a reply held. Each test's backend exits when the test closes its standard input. `node --test` runs with `--test-timeout=30000`, and `check.sh` wraps each `node` step in `timeout 300`.

## How the tests reach a backend

Each test file that reads `count` or sends `release` starts its own backend from 0092's binary. `check.sh` passes the binary's path in `THINKTHEN_TEST_BACKEND`. The root `test` rung has already built it with `--locked --offline`. A small helper, `tests/backend.mjs`, starts the backend, reads the port from its first line, and runs a child `node` with `THINKTHEN_BASE_URL` naming the arm. No test changes the environment in-process, because `default_engine()` reads it once. The conformance runner uses the case arm on the port the `surfaces` rung passes.

## Where the check runs

The tag's check ran on Linux with no Docker. The freeze record reads "pass: 62 of 68 node tests passed, 6 skipped", with the wire suite run. This port runs on the same Linux machine, where the gate ladder runs, with no Docker and no network. Observed there by command on 2026-09-24:

- Node 22.22.3 and npm 10.9.8 are on `PATH`.
- `npm ci --offline` on the tag's `package-lock.json` succeeded in a scratch copy from npm's cache. It installed `typescript` 7.0.2 and `@napi-rs/cli` 2.18.4. This port drops the second.
- The cargo cache holds `napi` 2.16.17, `napi-derive` 2.16.13, `napi-derive-backend` 1.0.75, `napi-build` 2.5.0, `napi-sys` 2.4.0, and `ctor` 0.2.9. These are the tag's lock versions. `serde_json` 1.0.151 matches the root lock.

A missing Node 22 or later, npm, or a cached package reports "not run" and never "pass" (R6-2). The line names the one fetch to run on a networked machine.

## The check it adds to the gate ladder

- `libraries/typescript/check.sh` joins the surface registry as landed. The `surfaces` rung runs it with the 0092 loopback port and `THINKTHEN_TEST_BACKEND`. Its steps: `npm ci --offline`, `build-addon.sh`, the Rust unit tests, `node --test` over `tests/`, the can-fail corruption loop, `tsc --noEmit --strict` over `types.test.ts`, the pack and home-path checks, and the flag, pin, guard-count, `unsafe`, and pinned-sentence steps.
- `check.sh` drops `THINKTHEN_NULL`, `ENGINE_NULL`, `ENGINE_WIDTH`, `THEN_TS_WIRE_URL`, the `synthetic-partial` build, and the wire stub on port 8212.
- `lint` runs on `libraries/typescript`: the ADR 0047 manifest, lock, lint-table, and profile checks from 0093; Clippy with `-D warnings`; deny as `cargo deny --offline --manifest-path libraries/typescript/Cargo.toml check --config deny.toml advisories bans licenses sources` against the root `deny.toml`, planted with a git-sourced dependency that `[sources] unknown-git = "deny"` refuses; `ratchet.mjs` on `libraries/typescript/ratchet.json` for Rust and on `ratchet.js.json`, `ratchet.mjs.json`, and `ratchet.ts.json` for the host files; and the registry check.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, `unexpected_cfgs`, and `unsafe_code`, and denies `expect_used`, `unwrap_used`, `indexing_slicing`, `panic`, and `allow_attributes_without_reason`. A local `allow` cannot lift a forbid. `src/node.rs` is public from the crate root, so its `pub` items are reachable. If napi-derive's output trips any forbid-level lint, the builder stops and records the case for an ADR 0047 amendment. Its generated `cfg` attributes name only `test` and `target_family`. The compiler already knows both.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off. `napi` `2.16.17` with default features off and `napi8`. `napi-derive` `2.16.13`. `napi-build` `2.5.0` as a build dependency. `serde_json` at the root lock's `1.0.151`. These are the tag's lock versions and the cached crates here.
- npm: `typescript` exactly `7.0.2`, a dev dependency for `tsc`. The package has no runtime dependency.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, deny's result, and `npm pack`'s list, and the review record says so (repo `CLAUDE.md`).

## Budgets

The tag measures, in nonblank lines: `lib.rs` 388, `index.js` 439, `index.d.ts` 232, `index.mjs` 25, the generated `loader.cjs` 308, the JavaScript and TypeScript tests 1,322 across 15 files, `conformance.test.mjs` 363 of those, and `check.sh` with `build-addon.sh` 95.

- Production Rust: at most four files and 650 nonblank lines, each file under 500.
- Package JavaScript and types: `index.js`, `index.mjs`, `loader.js`, and `index.d.ts` together at most 760 nonblank lines.
- Tests: at most twelve test files plus `tests/backend.mjs`, together at most 1,500 nonblank lines. The conformance runner counts inside that total and stays at or under 300. Rust unit tests at most 150 nonblank lines.
- Scripts: `check.sh` and `build-addon.sh` together at most 170 nonblank lines. Gate changes under `sdlc/scripts` at most 30 nonblank lines.
- Documentation: `README.md`, the new `NOTES.md`, `javascript.md`, and the ADR 0047 section, at most 240 net nonblank lines.
- Ratchet: the four files under `libraries/typescript` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen` or `conformance/`, or adding a verb form this ticket excludes.

## Exclusions

Async iterables, streams, and `filterBytes` from `javascript.md`. A column or data-frame form. Bun and Deno checks. Platform-named binaries, `optionalDependencies`, `npm publish`, and release archives (queue item 4). A browser or WebAssembly build. An `Engine` class. A synchronous variant. Any change to `thinkthen`. Any live or paid call.

## Dependencies

After 0086. Also after 0098 (`from_json` readers, `ErrorKind::name`, and the JSON methods), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), 0094 (the FFI edge reference and the plan's C-first order), 0105 (the host-binding template), and 0106 (the delay arm). 0092 and 0099 have landed.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 2; total 12. Final level: 3. The abort race leaves native work running after the promise settles. A wrong rule there either strands a caller or keeps a batch spending.

## Review

- Design review: pending.
- Code review: pending.
