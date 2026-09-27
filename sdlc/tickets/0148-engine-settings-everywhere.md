---
flow: build
priority: 148
opens: crates/thinkthen/src/public/settings.rs crates/thinkthen/src/public/engine.rs crates/thinkthen/src/public/error.rs crates/thinkthen/src/public/relate.rs crates/thinkthen/src/public/frame.rs crates/thinkthen/tests/settings_cases.rs crates/thinkthen/tests/public_env.rs conformance/settings.json conformance/README.md conformance/consumer libraries/python libraries/typescript libraries/ruby libraries/r libraries/c libraries/rust/README.md libraries/polars/README.md databases/duckdb databases/sqlite databases/postgresql specification/settings.md sdlc/scripts/settings sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/planning/adr/0037-the-c-door-serves-every-language-that-can-call-c.md sdlc/tickets/0084-freeze-the-public-rust-contract.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0148: Every library gets the command's engine settings

Status: done 2026-09-27 after fresh Codex semantic review, correction and fresh acceptance at 9117961d. Owner: Codex under Ian's handover.

Review route: Ian handed this accepted ticket to Codex; fresh read-only Codex reviewers checked its implementation and final correction.

## Outcome and authority

A Python user writes `tt.Engine(replay="recordings/", timeout=60)` and gets what `thinkthen decide --replay recordings/ --timeout 60` gives. Every answer comes from the folder, and a miss fails with nothing sent. The same holds in Rust, TypeScript, Ruby, R and C. The site's language examples can then run offline as tests.

Ian's goal 3 for 0.1: every language library matches the command wherever it can, and runs fast. Ian's ruling 5 of 2026-09-25: no setting may do nothing, so `cache_bytes` leaves the library, ticket 0084 and every binding, and the settings are swept for any other setting with no effect.

This ticket merges row E1 of `sdlc/planning/library-equivalence-2026-09-26.md` with tickets L1, L2 and L3 of `sdlc/planning/backlog-0-1-2026-09-26.md`.

**The split.** One ticket over all eleven surfaces is too large. This ticket carries the engine, Rust, Python, TypeScript, Ruby, R and C. It also removes `cache_bytes` from all three SQL surfaces, because the Rust setter they call goes away. Ticket 0149 carries the new settings to DuckDB, SQLite and PostgreSQL. The section "Ticket 0149, the SQL settings" fixes its rules, so that ticket starts from settled decisions.

## What happens today

- `EngineBuilder::build` fixes a 30-second timeout, 2 retries and no backend profile (`crates/thinkthen/src/public/settings.rs:233-238`). No library can change them.
- `storage` sets one folder as both the record folder and the replay folder (`settings.rs:270-275`). A cache miss sends a live request. No library can record only or replay only.
- `cache_bytes` checks its value and returns the builder unchanged (`settings.rs:207-220`). Python, TypeScript, Ruby, R, DuckDB, SQLite and PostgreSQL each pass a value through it. The three SQL READMEs say it caps the cache.
- C builds its engine with `Engine::from_env` alone (`libraries/c/src/ffi.rs:160-166`). It cannot set the address, the model, the throttle, the request limit or the cache.
- `relate` in the library passes no profile to its planner (`crates/thinkthen/src/public/relate.rs:270`).

The engine below the builder already takes every one of these. `Settings` in `crates/thinkthen/src/engine/facade.rs:60-72` carries the profile, the timeout, the retries and separate record and replay folders. With a replay folder and no record folder, `Recorder` answers from the folder alone and a miss is `ReplayMiss` (`crates/thinkthen/src/engine/recorder.rs:201`, `:331`). That is the command's `--replay` path today. So this ticket changes the builder and the bindings. It changes no engine code.

## Design

### The Rust builder

`EngineBuilder` gains five setters and loses one. Each setter checks its value at once, as the others do. `build` reads the profile file and applies the rest.

- `timeout(self, value: Duration) -> Result<Self, Error>`. Zero is `Error::Usage`: `a timeout is a time above zero`.
- `max_retries(self, value: u32) -> Self`. Every value is valid, as `--max-retries` takes any whole number.
- `profile(self, path: impl AsRef<Path>) -> Result<Self, Error>`. An empty path is `Error::Usage`: `a profile file is a path, not empty`. `build` reads the file. An unreadable file is `Error::Local`: `the profile file could not be read`. A file the parser refuses is `Error::Local`: `the profile file {error}`, with the parser's own sentence, as the command prints it without the path. The command exits 5 on both, so both are local.
- `record(self, path: impl AsRef<Path>) -> Result<Self, Error>`. It acts as `--record DIR`. Every request is sent, and each exchange is written into the folder. An empty path is `Error::Usage`: `a recording folder is a path, not empty`.
- `replay(self, path: impl AsRef<Path>) -> Result<Self, Error>`. It acts as `--replay DIR`. Answers come from the folder alone. A miss is `Error::Local`: `the replay folder holds no reply for this request`. Nothing is sent, and no key is read. An empty path is refused with the recording sentence.
- `cache_bytes` is removed.

`record` and `replay` sit beside the existing cache choice, and `build` applies the command's folder rules from `crates/thinkthen/src/cli/asking/folders.rs`:

- With neither set, the cache choice works as today.
- With either set, the default cache and `THINKTHEN_CACHE` are suppressed, as `recording.md` line 18 says of the command.
- `no_cache` may stand beside either, as `--no-cache` may.
- `cache_at` beside either is `Error::Usage` at `build`: `a cache folder is record and replay on one folder, so it stands beside neither`. This is the command's `--cache` refusal in library words.
- `record` and `replay` naming two folders is `Error::Usage` at `build`: `record and replay name two different folders, and one engine keeps one`.
- `record` and `replay` naming one folder replay an entry the folder holds and record the rest, as the command's pair does.

The storage these build is the command's `Folders` value, field for field. `private_default` is false for every named folder. `cache_answers` is false whenever `record` or `replay` is set, including the same-folder pair, as `cli/asking/folders.rs:30-35` sets it and `recording.md` line 24 says: explicit replay and an explicit record and replay pair count no cache answers. Only `cache_at` sets it true.

The public `Engine` keeps the profile it was built with, beside `most`. `relate` passes it to `facade::relations` in place of `None`. Every other call already reaches the profile through the facade engine.

### The bindings

Each binding starts from `EngineBuilder::from_env()`, as today, and calls one setter for each given value. A binding checks only its host's types. Every range, conflict and file rule comes from the builder, so each refusal has one sentence on every surface.

| Setting | Rust | Python `tt.Engine` | TypeScript `new tt.Engine` | Ruby `Engine.new` | R `tt_engine` | C `thinkthen_engine_new_with` |
| --- | --- | --- | --- | --- | --- | --- |
| Address | `base_url` | `base_url=` | `baseUrl` | `base_url:` | `base_url =` | `"base_url"`, new |
| Model | `model` | `model=` | `model` | `model:` | `model =` | `"model"`, new |
| Throttle | `throttle` | `throttle=` | `throttle` | `throttle:` | `throttle =` | `"throttle"`, new |
| Request limit | `max_requests` | `max_requests=` | `maxRequests` | `max_requests:` | `max_requests =` | `"max_requests"`, new |
| Cache | `default_cache`, `cache_at`, `no_cache` | `cache=` | `cache` | `cache:` | `cache =` | `"cache"`, new: `false` or a folder |
| Record | `record`, new | `record=`, new | `record`, new | `record:`, new | `record =`, new | `"record"`, new |
| Replay | `replay`, new | `replay=`, new | `replay`, new | `replay:`, new | `replay =`, new | `"replay"`, new |
| Timeout | `timeout`, new | `timeout=`, new | `timeoutSeconds`, new | `timeout:`, new | `timeout =`, new | `"timeout"`, new |
| Retries | `max_retries`, new | `max_retries=`, new | `maxRetries`, new | `max_retries:`, new | `max_retries =`, new | `"max_retries"`, new |
| Backend profile | `profile`, new | `profile=`, new | `profile`, new | `profile:`, new | `profile =`, new | `"profile"`, new |
| Key | `api_key` | none | none | none | none | none |

- Python takes keyword arguments. `record=`, `replay=` and `profile=` take a `str` or an `os.PathLike`, as `cache=` does. `timeout=` and `max_retries=` take an `int`. A `bool` or a `float` raises `UsageError`, as `throttle=` does today.
- TypeScript takes the options object it already takes. `timeoutSeconds` says its unit in its name, because a JavaScript reader expects milliseconds. The `EngineOptions` interface in `index.d.ts` gains each key.
- Ruby takes keywords. It passes them to the native side as one options hash in place of today's seven positional arguments.
- R takes arguments on `tt_engine`. `named()` lists the new ones, and names each folder as `<folder>` as it does the cache.
- C gains one symbol: `thinkthen_engine *thinkthen_engine_new_with(const char *settings_json)`. The argument is one JSON object whose keys are the Rust builder's setter names, as the table shows. A null pointer or `{}` builds what `thinkthen_engine_new` builds. A refusal returns null, and the calling thread's error slot names it, as a failed `thinkthen_engine_new` does today. `thinkthen_engine_new` becomes a call of the new symbol with `{}`. The C door already speaks JSON for requests, so a JSON settings object adds one symbol and no struct layout.

The C settings object follows these rules. Each refusal returns null with `THINKTHEN_EUSAGE` in the thread's slot unless the builder names another kind.

- The text is UTF-8. Bytes that are not UTF-8 are refused before any parse.
- The text is one JSON object. Any other JSON value, or text that is not JSON, is refused.
- A key given twice is refused. The parser rejects duplicates and does not keep the last value.
- The keys are the ten in the table and no others. `"api_key"` is an unknown key and is refused, because ADR 0017 section 5 keeps the key in `THINKTHEN_API_KEY` on every surface but Rust.
- `base_url`, `model`, `record`, `replay` and `profile` take a string.
- `cache` takes `false` or a folder string. `true` is refused. The environment's cache is the absence of the key.
- `throttle`, `timeout`, `max_retries` and `max_requests` take a whole JSON number with no fraction and no sign, in the range of the setter's type. `8.0`, `-1` and `1e2` are refused. The builder then applies its own range, so `throttle` 33 and `timeout` 0 carry the builder's sentences.
- `max_requests` also takes `null`, which means no limit, as `EngineBuilder::max_requests(None)` does. Every other key refuses `null`.
- The key stays in `THINKTHEN_API_KEY` alone on every surface but Rust, by ADR 0017 section 5.
- The Rust Polars door and the Python Polars and pandas doors take the engine of their language. They need no change.

### The shared settings cases

`conformance/settings.json`, schema `thinkthen.settings-cases/1`, holds one case for each setting this ticket carries. Each case names its setting by the Rust builder's setter name, a loopback arm, and one or more steps. A step gives the settings, the verb, the question, the text or records, the expected bare value, detail field or error kind, and the loopback count after the step. `$FOLDER` stands for one fresh folder the runner makes for the case. A profile is given as its JSON object. A library runner writes it to a file and passes the path.

| Case | Arm | Steps and expectation |
| --- | --- | --- |
| `timeout-ends-a-slow-attempt` | `/arm/delay/2000/v1` | `timeout` 1, decide: error `backend`, count 1 |
| `max-retries-zero-sends-once` | `/arm/503/v1` | `max_retries` 0, decide: error `backend`, count 1 |
| `profile-refuses-before-sending` | `/generic/v1` | A profile with `max_evidence_bytes` 4, decide `refund now`: error `usage`, count 0 |
| `model-names-the-request` | `/generic/v1` | `model` `other-model`, details: `model` is `other-model`, count 1 |
| `record-sends-every-time` | `/generic/v1` | `record` `$FOLDER`, decide twice: `true` each time, count 2. The folder holds one entry |
| `replay-answers-from-the-folder-alone` | `/generic/v1` | `cache` `$FOLDER`, decide: count 1. Then `replay` `$FOLDER`, the same decide: `true`, count 1. Then `replay` `$FOLDER`, another text: error `local`, count 1 |
| `max-requests-refuses-past-the-limit` | `/generic/v1` | `max_requests` 2, `decide_many` over 3 records: error `usage`, count 2 |
| `cache-off-sends-again` | `/generic/v1` | `cache` false, decide twice: count 2. The runner points `THINKTHEN_CACHE` at a fresh folder first, so a dropped setting would answer the second call from the cache |

Each runner maps a setter name to its host's spelling. The command's runner maps it to the flag. That map is the surface's column of the table above, so a wrong cell fails a case. The generic arm already echoes the request's model, and the delay and 503 arms already exist. The loopback backend needs no change.

- The command runs every case but `max-requests-refuses-past-the-limit`, which has no flag. Its runner is the new `crates/thinkthen/tests/settings_cases.rs`. It runs the compiled command with `--timeout`, `--max-retries`, `--profile`, `--model`, `--record`, `--replay`, `--cache` and `--no-cache`.
- Rust runs every case from `conformance/consumer/consumer/tests/public/settings.rs`, a new module of the existing public test binary.
- Python, TypeScript, Ruby, R and C each run every case from their conformance runner, beside the cases of `cases.json`.
- The SQL runners do not read the file until ticket 0149.

The throttle has no case here. A throttle case needs an in-flight count on the held arm, which is the `concurrency` case kind that row E11 of the equivalence page proposes. C's throttle is proved by the C door test below until then.

### Removing `cache_bytes`

The setter, its doc, and every caller go. On each surface the spelling goes with it:

- Rust: `EngineBuilder::cache_bytes`, the doctest in `crates/thinkthen/src/public/frame.rs:33`, and the `seed().cache_bytes(0)` row of `crates/thinkthen/tests/public_env.rs:212` with its expected line.
- Python: the keyword, `CACHE_BYTES`, the `.pyi` line and the README word.
- TypeScript: `cacheBytes` in `door.rs`, `index.d.ts`, the README and `settings.test.mjs`.
- Ruby: the keyword, the native argument and the README word.
- R: the argument, the Rust field and its `named()` entry.
- DuckDB: `SET thinkthen_cache_bytes`, its `Asked` field, README line 39, and the zero step in `databases/duckdb/tools/settings_suite.py:91`.
- SQLite: `thinkthen_cache_bytes(n)`, its settings field, README lines 50 and 56, and its two tests.
- PostgreSQL: `thinkthen.cache_bytes`, `CACHE_BYTES_ZERO`, the `Plan` field, README line 44, the unit tests and the `check.sh` step.

The configuration file's `cache_bytes` stays. It is the command's prune target, and `cache prune` reads it (`crates/thinkthen/src/cli/cache.rs:18`).

A server configuration that still sets `thinkthen.cache_bytes` gets PostgreSQL's warning for an unknown setting under a reserved prefix, and the server starts. The README says so in one line.

The SQLite README tells a reader to raise `thinkthen_cache_bytes` before a warm pass. Nothing trims the cache on its own (`recording.md` line 32), so a warm pass never evicts its own answers, and the advice goes. The same sentence in ADR 0017 section 5 is corrected by this ticket's amendment.

### The settings sweep

Ian's ruling asks for a sweep of every setting for the same fault. The design pass read every engine setting on the six libraries and the three SQL surfaces. It found one more setting with no effect: a question file's calibration `profile` key. The library accepts it and then drops it. Every library result passes no calibration name to its digest and carries no warning (`crates/thinkthen/src/public/results.rs:209-210`, `:226`). So a question file with `profile` gives a different `question_sha256` in a library than in the command, which passes it (`crates/thinkthen/src/result_json.rs:35`).

The fix lives in `public/question.rs` and `public/results.rs`, which ticket 0146 holds open. This ticket files it as an issue in its landing commit and does not fix it. The build repeats the sweep on the final diff and records the list it checked.

### Ticket 0149, the SQL settings

These rules bind ticket 0149. Ian can overturn each.

- **The address and the key stay out of SQL.** Tickets 0109 decision 2 and 0110 decision 5 rule this, and `databases/postgresql/README.md` line 94 states it. SQL text lands in logs, query history and plans. The equivalence page's "cannot close" section records this as a ruling, and it notes that an address alone would be safe to set. Ian can overturn it.
- Each SQL surface gains `model`, `timeout`, `max_retries`, `profile`, `record` and `replay`, spelled as it spells its throttle: `SET thinkthen_timeout` in DuckDB, `thinkthen.timeout` in PostgreSQL, and `thinkthen_timeout(n)` in SQLite. The timeout is whole seconds. PostgreSQL registers it with the unit `s`.
- The profile is JSON text in SQL, not a path, so that no new file rule enters any SQL surface. Ticket 0149 adds `EngineBuilder::profile_json` for it.
- A record or replay folder follows that surface's rule for a cache folder: DuckDB's absolute path and its probe through the caller's file system, PostgreSQL's superuser setting, and SQLite's plain path.
- DuckDB and PostgreSQL turn the cache off with the value `off`. A cache folder set from SQL is an absolute path, so the word cannot name a folder. SQLite keeps `thinkthen_cache(NULL)`.
- SQLite's `thinkthen_warm`, `thinkthen_recognize` and `thinkthen_relate` take a last deadline argument in milliseconds, as its scalars do.
- Each SQL runner runs `conformance/settings.json` but for the throttle cases it cannot express.

Ticket 0149 then closes the two issues this one settles for the libraries.

### How this makes B12a smaller

The batching design's B12a gives the Rust library `batch`, `context` and `.facts`. B12a to B13e carry them to each surface. This ticket leaves `batch` to them and gives them four things.

1. The profile is already on the builder and on the public `Engine`. ADR 0048 items 2 and 6 close batches at a profile's limits, so B12a reads the profile the engine holds and adds no setter for it.
2. Each binding's settings code already maps a list of names to builder setters. An engine-level `batch` is one more row in that map on each surface, with no new plumbing.
3. `conformance/settings.json` and a runner for it exist on every library. B12a's shared cases for batch count, shares and `--batch 1` bytes become new entries there, with no new case kind.
4. `replay` on every library lets B12a to B12f prove their `--batch 1` bytes against a recording with nothing sent on a miss, as ticket 0146 proves the command.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **One ticket for the libraries, one for SQL.** The libraries share one design: a host-native options form over the builder. The SQL surfaces each need their own path, permission and session rules. The split keeps each diff reviewable. This ticket removes `cache_bytes` from SQL as well, because the setter they call goes away.
2. **Strict replay is `replay`, with no separate strict switch.** The command's `--replay` is already strict, and the replay issue asks for "a replay-only setting that matches `--replay`". A lenient replay is the cache.
3. **The builder applies the command's folder rules.** A binding passes each value through, and the builder refuses a conflict at `build`. Every surface then refuses the same pairs with the same sentence, and the rule lives in one place. The existing three cache setters keep their last-one-wins order.
4. **Defaults are the command's.** Timeout 30 seconds, 2 retries, no profile, no recording folder. No environment variable or configuration key is added for them, because the command reads none. The defaults' missing reasons stay listed on the settings page.
5. **The timeout is whole seconds on every surface but Rust.** That is what `--timeout` takes. Rust takes a `Duration`, its own time type, and refuses zero. TypeScript names the unit, `timeoutSeconds`.
6. **The libraries take a profile as a path, as `--profile` does.** SQL takes JSON text in ticket 0149.
7. **Precedence follows the settings page.** An engine setting sits in the environment tier and overrides the variable it was seeded from. Timeout, retries, profile, record and replay have no other tier, so the engine value wins or the default holds. The model is the one exception. A question's `model` outranks the engine's model, by ADR 0017 section 5 and the builder's doc: "Name this model when a question names none". The settings page's model line gains the engine tier below the question file. Changing the code to follow the tier rule instead would break every library caller who sets a model on a question.
8. **C gets a JSON settings constructor.** DESIGN section 8 makes a new symbol a minor bump. No C version has shipped, and 0.1 is itself a minor bump over 0.0.1. The DESIGN table grows to 20 symbols, and its deferred "checked throttle constructor" is done. ADR 0037 gains a dated amendment.
9. **One shared case per setting, run by every surface.** The case file names settings by the Rust setter name, and each runner's map is its column of the settings table. Where a library's own test asserts the same effect of `model`, `max_requests` or `cache` false, the build deletes that assertion in the same commit and names it in the record. The shared case replaces it, so the contract is tested once on each surface.
10. **The calibration `profile` gap is filed, not fixed.** Its fix needs two files ticket 0146 holds.
11. **ADR 0017 section 5 and ticket 0084 gain dated amendments.** Section 5's table gains timeout, retries, profile, record and replay, and loses the cache cap row for libraries and SQL. Ticket 0084's frozen inventory drops `cache_bytes` and adds the five setters. No release has shipped, so no user breaks.

## Edge cases

The Rust rows run in `conformance/consumer/consumer/tests/public/settings.rs` as one table.

| Input | Expected behavior |
| --- | --- |
| `timeout(Duration::ZERO)` | `Usage`, `a timeout is a time above zero`, at the setter |
| `timeout` 1 s against the 2-second delay arm | `Backend`, one request, never sent again |
| `max_retries(0)` against the 503 arm | `Backend`, one request. The default sends three |
| `profile("")` | `Usage`, `a profile file is a path, not empty` |
| `profile` naming a missing file | `Local` at `build`, `the profile file could not be read`. Nothing sent |
| `profile` naming a file with an unknown key | `Local` at `build`, the parser's sentence after `the profile file `. Nothing sent |
| A text over the profile's `max_evidence_bytes` | `Usage`, the command's profile sentence. Nothing sent |
| `relate` under a profile | The same requests or refusal as the command's `relate --profile` over the same entities |
| `record(F)`, the same decide twice | Two requests. `F` holds one entry |
| `replay(F)` on a text `F` holds | The answer. No request, and no key needed |
| `replay(F)` on a text `F` lacks | `Local`, `the replay folder holds no reply for this request`. No request |
| `replay(F)` where `F` does not exist | The same miss. `F` is not created |
| `replay(F)` where `F` belongs to another address | `Local`, the recording-folder mismatch sentence. No request |
| `record(F)` where `F` is a file | `Local` at `build`, `the recording folder names a file` |
| `record(F)` and `replay(F)` | One request, then none. The engine's `cache_answers` counter stays 0 |
| `record(F)` and `replay(G)` | `Usage` at `build`, `record and replay name two different folders, and one engine keeps one` |
| `cache_at(F)` and `replay(F)` | `Usage` at `build`, `a cache folder is record and replay on one folder, so it stands beside neither` |
| `no_cache()` and `replay(F)` | Replays `F` |
| `THINKTHEN_CACHE=C` and `replay(F)` | Replays `F`. `C` stays empty |
| `cache_at(F)` then `no_cache()` | No cache, as today |

The binding rows run in each surface's own settings test.

| Input | Expected behavior |
| --- | --- |
| Python `timeout=True`, `timeout=1.5`, `max_retries=-1` | `UsageError`, nothing sent |
| Python `replay=pathlib.Path(F)` | Replays `F` |
| TypeScript `{ timeoutSeconds: '30' }`, `{ maxRetries: 1.5 }` | `ThinkThenError` of kind `usage` |
| Ruby `timeout: "30"`, `replay: 7` | `UsageError` |
| R `timeout = 2.5`, `replay = NA` | The usage error naming the argument |
| C `thinkthen_engine_new_with(NULL)` and `("{}")` | The environment's engine, as `thinkthen_engine_new` |
| C `"[]"`, `"{\"timeout\":\"30\"}"`, `"{\"nope\":1}"`, text that is not JSON | Null. `thinkthen_error_code(NULL)` is `THINKTHEN_EUSAGE`, and the message names the key or the shape |
| C `"{\"api_key\":\"k\"}"` | Null, `THINKTHEN_EUSAGE`, the unknown-key sentence. This row guards ADR 0017 section 5 |
| C `"{\"cache\":true}"` | Null, `THINKTHEN_EUSAGE` |
| C `"{\"max_requests\":null}"` | An engine with no request limit |
| C `"{\"model\":null}"` | Null, `THINKTHEN_EUSAGE` |
| C `"{\"timeout\":1,\"timeout\":2}"` | Null, `THINKTHEN_EUSAGE`, naming the repeated key |
| C bytes that are not UTF-8 | Null, `THINKTHEN_EUSAGE` |
| C `"{\"throttle\":8.0}"`, `"{\"max_retries\":-1}"` | Null, `THINKTHEN_EUSAGE` |
| C `"{\"throttle\":8}"` after another engine took throttle 4 | Null, with the throttle sentence |

## Proof

Every test runs against the loopback backend in `conformance/backend` or a local folder, sends nothing live, and reads no key but a fake one where a live loopback send needs it.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| The shared settings cases, Rust | Every case in `conformance/settings.json` through the public builder | (a) `build` keeps the fixed 30-second timeout: the timeout case answers `true`. (b) `build` keeps 2 retries: the retry case counts 3. (c) `build` keeps `profile: None`: the profile case sends. (d) `replay` sets the folder as both record and replay, today's `storage`: the replay miss sends, count 2 |
| The shared settings cases, the command | The same file through the compiled command | (a) The runner's map sends `--cache` for `replay`: the miss sends. This proves the case file itself tells `--replay` from `--cache` |
| The shared settings cases, Python | The same file through `tt.Engine` | (a) Map `replay=` to `cache_at`: the miss sends. (b) Drop the `timeout=` setter call: the case answers `true` |
| The shared settings cases, TypeScript | The same file through `new tt.Engine` | (a) Map `replay` to `cache_at`. (b) Drop `maxRetries`: count 3 |
| The shared settings cases, Ruby | The same file through `Engine.new` | (a) Map `replay:` to `cache_at`. (b) Drop `profile:`: the profile case sends |
| The shared settings cases, R | The same file through `tt_engine` | (a) Map `replay =` to `cache_at`. (b) Drop `record =`: the record case writes no entry and counts 1 |
| The shared settings cases, C | The same file through `thinkthen_engine_new_with` | (a) Map `"replay"` to `cache_at`. (b) Drop `"model"`: the details name `jev-latest`. (c) Drop `"max_requests"`: the three records all send |
| `the_builder_follows_the_command_folder_rules`, Rust | The first edge table | (a) Let `cache_at` stand beside `replay`: no refusal. (b) Keep the default cache under `replay`: `C` gains an entry. (c) Create a missing replay folder: `F` exists after the miss |
| `relate_follows_the_engine_profile`, Rust | The library's `relate` and the command's `relate --profile` over the same entities and profile file give the same loopback count, or the same error kind with a count of 0 | (a) Pass `None` again at `public/relate.rs:270`: the counts differ |
| `a_strict_replay_reads_no_key`, Rust | A child process with `THINKTHEN_API_KEY` unset replays a hit from `F`, then misses. The hit answers, the miss is `Local`, and the loopback counts 0 | (a) Read the key at `build` for any engine with a folder: the child fails with the no-key usage error |
| The C settings table, `libraries/c/tests/door` | The C rows of the second edge table, and a held-arm run of `decide_many` over 8 records under `{"throttle":2}` that holds exactly 2 in flight | (a) Ignore `"throttle"`: the arm holds 4. (b) Accept an unknown key: the `nope` and `api_key` rows build an engine. (c) Keep the last of two equal keys: the repeated-key row builds an engine |
| The binding edge rows, Python, TypeScript, Ruby and R | Each surface's rows of the second edge table, in its existing settings test | (a) Drop the host type check for `timeout`: `True` or `1.5` builds an engine |

The existing check `the_library_carries_its_soname_and_exactly_the_header_symbols` in `libraries/c/tests/door/main.rs` compares the header with the exported symbols. It covers the new symbol once the header declares it, and needs no change.

The four questions, answered once for the shared cases and once for the rest:

- **What they protect.** Each engine setting reaches the engine on every library with the command's effect. A strict replay sends nothing and reads no key on a miss. The builder refuses the command's folder conflicts. `relate` obeys the profile.
- **What regression fails them.** A binding that drops or mis-maps a setting, most likely `replay` mapped onto the cache, which sends on every miss. A builder that ignores a value it stored. A binding that trusts a host value of the wrong type.
- **Why no existing test catches it.** For timeout, retries, profile, record and replay: none of these settings exists on a library today, and no shared case sets any engine setting (equivalence page section 5, "What they miss"). The command's own tests prove its flags and never reach a binding.
- **Why no existing test catches it, for model, request limit and cache off.** These three exist on Rust, Python, TypeScript, Ruby and R, and each of those surfaces tests them in its own settings test with its own inputs. C has none of the three, so no test reaches C. No test runs one input across every surface, so nothing shows the surfaces agree. By decision 9, the build deletes each library's own assertion of the same effect, and the shared case becomes the one test of each on each surface.
- **Does it need a test-only hook.** No. Each test sets the public spelling, counts at the real loopback listener, and reads the real folder.

By Ian's 2026-09-26 ruling that every setting is documented, `sdlc/scripts/settings` also checks the library and SQL cells both ways, with one plant each way, in about 40 lines. A cell reading `not on this surface` stays unchecked. If the direction from map to cell overruns, the builder files it as an issue.

`cache_bytes` gets no test of its absence. The build records that `rg -i 'cache_?bytes'` over `crates/thinkthen/src/public`, `crates/thinkthen/tests`, `libraries` and `databases` finds nothing. The tests that passed it lose those lines.

## Pages, comments, and issues

- `specification/settings.md`: the rows for Address, Model, Backend profile, Timeout, Retries, Throttle, Request limit, Answer cache, Recording and Prune target. Each library cell gains its spelling from the design table. The SQL cells of the new settings keep `not on this surface` and name ticket 0149. The Prune target row loses its library and SQL cells, and its Default cell loses the "no effect" note. The precedence section's model line gains the engine tier, by decision 7. The Timeout and Retries Default cells drop "Fixed at 30 on the libraries and SQL" and say "Fixed on SQL until ticket 0149".
- ADR 0017: a dated amendment to section 5, by decision 11.
- ADR 0037: a dated amendment for `thinkthen_engine_new_with`.
- Ticket 0084: a dated amendment to the frozen inventory.
- `libraries/c/DESIGN.md`: the symbol table, section 8's count, and the deferred throttle line. `include/thinkthen.h` documents the new symbol and its keys.
- The READMEs of Rust, Polars, Python, TypeScript, Ruby, R and C name the new settings in one sentence each. The DuckDB, SQLite and PostgreSQL READMEs lose their `cache_bytes` lines.
- `conformance/README.md` gains one paragraph for `settings.json`.
- A new issue, `sdlc/issues/2026-09-26-the-library-drops-a-question-files-calibration-profile.md`, from the sweep.
- `2026-09-26-settings-some-surfaces-cannot-reach.md`: the lander marks items 1 and 2 settled for the libraries and C, and item 3 settled. The SQL part stays open for ticket 0149.
- `2026-09-26-libraries-cannot-replay-a-recording-strictly.md`: the lander marks it settled for the libraries. The SQL part stays open for ticket 0149.
- `2026-09-25-public-library-api-gaps.md`: the lander marks item 4 settled by this ticket, as item 8 is marked.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `crates/thinkthen/src`: at most 120 added and at most 60 net, doc lines included.
- Library production code, headers and type files, across Python, TypeScript, Ruby, R and C: at most 280 added.
- Database production code: net negative. Nothing added but the README line on the retired PostgreSQL setting.
- `conformance/settings.json`: at most 140 lines.
- Settings runners: at most 70 added on each of the seven surfaces.
- The Rust edge work: `the_builder_follows_the_command_folder_rules`, `relate_follows_the_engine_profile`, `a_strict_replay_reads_no_key`, and the child process the cache-off case needs: at most 150 added.
- Binding edge rows and the C door test: at most 120 added in all.
- Pages: at most 20 changed rows in `settings.md`, at most 6 lines for each README, at most 25 for the ADR 0017 amendment, at most 15 each for ADR 0037, ticket 0084 and `DESIGN.md`, at most 10 for `conformance/README.md`, and one issue of at most 25.
- `sdlc/ratchet.json` moves to the measured total in the commit that adds the code. After 0146 lands, the build merges `origin/main` and measures again, so the ceiling holds both tickets' lines and neither one's number is kept by hand. The commit says what grew. The builder looks first for duplicated settings code across the bindings to delete.
- No dependency. The public surface widens, so the `surfaces` rung runs.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green.
3. Stop if the design needs a change in `crates/thinkthen/src/engine`. The facade already takes every setting, and a need to change it means this design is wrong.
4. Stop if ticket 0146 has not landed on main when the build starts. Merge `origin/main` first. Then stop before editing any file 0146 changed but `public_env.rs`, `settings.md` and `sdlc/ratchet.json`: `public/question.rs`, `public/results.rs`, `public/batch.rs`, `public/bulk.rs`, `engine/facade.rs`, `cli/`, any other existing file under `crates/thinkthen/tests`, or any specification page but `settings.md`. In `public_env.rs`, remove only the `cache_bytes` row and its expected line. The new file `crates/thinkthen/tests/settings_cases.rs` uses `crates/thinkthen/tests/support` as it is and adds nothing there. In `settings.md`, touch only the rows and the one precedence line named above. After 0146 lands, merge its `batch` row and precedence line, and do not rewrite them.
5. Stop if the sweep finds a setting with no effect beyond `cache_bytes` and the calibration `profile`. Report it for a ruling on scope.
6. Stop if a case in `conformance/settings.json` behaves differently on the command and on Rust. That is an equivalence bug, and it needs its own decision.
7. Stop if the timeout case takes more than 5 seconds on any surface, or flakes once in three runs. Report the arm's timing. Do not lengthen the delay to pass.
8. Never run `sdlc/scripts/live`. Unset `THINKTHEN_API_KEY` for every rung.

## Scope and exclusions

Excluded: the SQL settings, which ticket 0149 carries. `batch` and `context`, which B12a to B13e carry. The throttle's shared case, which E11 carries. The calibration `profile` fix, filed as an issue. Run facts, usage per process and streaming. `site/`, which the marketing lead owns. The replay issue came from the site, and this ticket lets its language examples run offline. The site schedules that change.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The change widens the public surface on six languages, raises the ceiling and changes the frozen 0084 inventory, so the code review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 2; proof 1; cost of error 2; total 7. Final level: 2. The risk is a replay that sends on a miss, which costs money in an offline example. Plant (d) on Rust and plant (a) on every binding guard it. The reach is wide, and each surface's change is a few mapped names.

## Deferred gaps

- Ticket 0149, the SQL settings, with the rules above.
- The build waits for ticket 0146 to land, by the coordinator's ruling. 0146 opens `crates/thinkthen/tests`, and this ticket edits `public_env.rs` there.
- The throttle's shared case, with E11's `concurrency` case kind.
- A library's calibration `profile`: the digest and `profile_warning`. Filed as an issue. A running profile on a library enforces limits and gives no calibration warning until that fix.
- Dry run on a library. The command's `--dry-run` has no library spelling, and no user has asked.
- Each surface's own throttle test, which E11's `concurrency` case would replace. The duplicate `model`, `max_requests` and cache-off assertions leave in this ticket, by decision 9.
- Environment variables for timeout, retries or the profile on C. The JSON constructor covers C, and the command reads none.

## What Ian can overturn

- Decision 1: the split into a library ticket and ticket 0149 for SQL.
- Decision 2: strict replay as `replay` alone, with no separate switch.
- Decision 5: whole seconds for the timeout, and `timeoutSeconds` in TypeScript.
- Decision 7: the engine model below the question's model, as an exception to the tier rule.
- Decision 8: a JSON settings constructor for C in place of one symbol per setting.
- The SQL rulings carried forward: the address and the key stay out of SQL by tickets 0109 and 0110. The equivalence page notes an address setting alone would be safe.
- Ticket 0149's `off` word for a cache and its JSON-text profile.

## Closes

This ticket closes no issue whole. It settles:

- `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md`, items 1 and 2 for the libraries and C, and item 3.
- `sdlc/issues/2026-09-26-libraries-cannot-replay-a-recording-strictly.md`, for the libraries and C.
- `sdlc/issues/2026-09-25-public-library-api-gaps.md`, item 4.

Ticket 0149 closes the first two issues.

## Evidence

- Starts from: The equivalence audit `sdlc/planning/library-equivalence-2026-09-26.md`, rows S1, S3 to S7 and C2, proposal E1, section 3 "Gaps that cannot close" on SQL addresses and keys, and section 5 on missing settings cases. The backlog page `sdlc/planning/backlog-0-1-2026-09-26.md`, tickets L1, L2 and L3. Ian's ruling 5 of 2026-09-25 in `sdlc/planning/issue-backlog-2026-09-25.md`. The three issues under "Closes". The code at `origin/main` `f42d0b1d`: `public/settings.rs:207-275`, `engine/facade.rs:60-72`, `engine/recorder.rs:78-110`, `:201`, `:331`, `cli/asking/folders.rs`, `public/relate.rs:270`, `public/results.rs:209-226`, and each binding's settings code. Ticket 0146's `opens` list and its settings changes, read from `ticket/0146-command-batches-decide-filter-rank`. The batching design's B12a to B13e rows. No experiment ran. The command's `--replay` tests already prove the engine path this ticket exposes.
- Keeps: Every command flag, request, output byte and exit code. Every existing library setting and its spelling. The builder's environment seed. The throttle rule of ADR 0017's 2026-09-24 amendment. The key in `THINKTHEN_API_KEY` alone on every surface but Rust. The address and key rulings for SQL. The configuration file's `cache_bytes` as the prune target. The Polars and pandas doors unchanged. Every engine module.
- Changes: `EngineBuilder` gains `timeout`, `max_retries`, `profile`, `record` and `replay`, and loses `cache_bytes`. `build` applies the command's folder rules. `relate` obeys the engine's profile. Python, TypeScript, Ruby and R gain the five settings. C gains `thinkthen_engine_new_with` with every engine setting. `cache_bytes` leaves every binding and SQL surface. `conformance/settings.json` holds one case for each of eight settings, run by the command and six libraries. `settings.md`, ADR 0017, ADR 0037, ticket 0084, the C DESIGN and ten READMEs follow. An issue files the calibration `profile` gap.
- Proof: The eight shared settings cases on the command, Rust, Python, TypeScript, Ruby, R and C, each with planted mis-mappings, the replay-onto-cache plant first. The Rust folder-rule table, the relate profile test against the command, and a no-key strict replay in a child process. The C settings table with a held-arm throttle count. The binding type rows. A grep for `cache_bytes` recorded in the build record.
- Defers: The SQL settings, in ticket 0149 with its rules fixed here. The throttle's shared case, with E11. The calibration `profile` fix, after 0146. A library dry run. Per-surface throttle tests for E11. C environment variables for the new settings.

## Amendment, 2026-09-27: older recording folders and exact file scope

This amendment carries `sdlc/issues/2026-09-26-ticket-0148-owes-old-recording-folders-and-the-retired-layout-sentence.md`. It preserves the accepted settings design. The command already replays an older folder with digest entries but no `.thinkthen-backend.json` read-only. The library's current error calls such a folder a "retired layout," although the entries remain usable. Main `8ff1f0fe` still has that sentence in `public/error.rs:198`. `sdlc/scripts/settings` is already called by the spec rung and needs the two-way library and SQL cell check promised above; both paths now appear in `opens`.

- `replay(F)` alone accepts an existing unmarked recording folder read-only. A hit returns its saved answer, a miss remains local, and neither path creates a backend marker, writes an entry, reads a key, or sends a request. `record(F)`, `cache_at(F)`, and the same-folder record and replay pair remain write-capable and refuse an unmarked folder that holds a digest entry. No engine change is needed: `engine/recorder/identity.rs` already returns success for a missing marker when `writing` is false.
- Replace `public/error.rs`'s "retired layout" sentence for `RecordingFolderLegacy` with `the recording folder predates backend binding; replay it read-only or choose a new folder`. This is the command's actionable sentence without flag syntax. No other public error mapping changes.
- Add one Rust public-boundary regression that copies `demos/27-test-with-no-network/recording` into a temporary folder and replays it through `EngineBuilder::replay`. Keep the copied entry bytes and saved backend address unchanged. With no key set, assert its saved answer, then ask for evidence absent from this legacy folder and require `Error::Local` with `the replay folder holds no reply for this request`. Assert no marker or entry change after both calls. Reuse that copy for a write-capable `cache_at` refusal and assert the exact replacement sentence. Keep the regression in the new `conformance/consumer/consumer/tests/public/settings.rs` module, imported by its existing `main.rs` parent. The fixture is keyed to its saved remote address, so an unrelated loopback listener cannot count this test’s requests. The existing shared settings replay case provides the meaningful zero-send count at its own loopback address on every binding.
- Close the named issue only after those three behaviors pass. The Rust edge-work budget rises from 150 to at most 185 added nonblank lines to allow the fixture regression and its setup. The source budget of 120 added and 60 net remains, because the error wording replaces one match arm. All other budgets remain.

Evidence: `specification/recording.md:40` promises unmarked replay, `engine/recorder/identity.rs:36-50` implements the read-only marker rule, `cli/failure/recording.rs:56-60` gives the command sentence, and demo 27 has one digest entry and no marker. No live or paid call is needed. The fix does not include the separate calibration-profile issue or SQL settings in ticket 0149. Ian can overturn this amendment's choice of library wording or extra test budget.


## Accepted implementation-budget amendment, 2026-09-27

The coordinator accepts these measured limits after a fresh independent read-only Sol Medium budget review (`target/codex-reviews/0148-budget/REVIEW.md`). This changes no behavior, dependency or file scope. It is separate from Batch J, which does not amend existing tickets. Replace only the following original ceilings: core source remains at most 120 added and becomes at most 90 net; binding production becomes at most 330 added; shared runners become command 115, Rust consumer 100 and C 110 added, with 70 retained for each other surface; Rust edge work becomes at most 240 added. Every other budget and stop rule remains.

Measured work was core 110 added and 85 net, binding production 316 added, runners 112/94/105, and Rust edges 169 before the required folder-rule table. Growth pays for the existing five settings, folder/profile validation, duplicate-key JSON handling at the C door, and outside-in shared-case proof. The reviewer inspected deletion opportunities and found no obvious reduction sufficient for the former production caps. The 240 Rust-edge cap leaves a bounded allowance for the required folder table and shared setup. Keep the C boundary and held-arm rows and all required plants. Remove the empty Ruby `impl Ask {}`. Fresh code review must specifically check Ruby bulk draining and the removed binding-level limit precheck; this budget approval is not semantic approval. Measure every category again after the remaining proof.
