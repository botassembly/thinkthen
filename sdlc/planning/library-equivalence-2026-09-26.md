# Library and database equivalence audit, 2026-09-26

Written 2026-09-26 by Claude from a read-only audit of main at `eba3a72a`. Nothing was built, run, or sent. Each surface's code was read, and the docs served only as leads. Ian asked for two things. The language libraries should match the command wherever they can, and they should run fast. Where a match is impossible, he wants the reason. This page answers both and proposes tickets. It decides nothing. Ian can overturn every recommendation here, and each proposed ticket still needs its own review.

The command's ten functions are `decide`, `choose`, `tag`, `score`, `rank`, `filter`, `find`, `annotate`, `recognize` and `relate`. The surfaces are the ones `sdlc/surfaces.txt` lists as landed: Rust, the Rust `polars` feature, C, Python with its Polars and pandas doors, TypeScript, Ruby, R, DuckDB, SQLite and PostgreSQL.

## What main holds today

These facts shape every row below.

- The public builder fixes a 30-second timeout, 2 retries and no backend profile (`crates/thinkthen/src/public/settings.rs:233-238`). No surface can change them.
- A cache folder serves as both the record folder and the replay folder (`settings.rs:270-275`). No surface has record-only, replay-only, or strict replay.
- `cache_bytes` checks its value and does nothing else (`settings.rs:207-220`).
- Counters belong to one engine, and nothing saves them (`settings.rs:246`, `engine.rs:140-144`).
- `details` covers `decide`, `choose`, `tag` and `score` on one text (`engine.rs:285-310`). No call returns wall time or cost.
- `find` has no "none of these" switch (`bulk.rs:168-173`). A question set refuses a per-question `on` pointer (`set.rs:37-47`).
- The library asks `choose`, `score` and `tag` over many records only through `annotate` or the Polars feature. `recognize` takes one text (`recognize.rs:404-435`).
- `filter`, `decide_many` and `annotate` stream lazily under the engine's throttle (`batch.rs:1-6`, `batch.rs:94-124`). `rank` and `find` hold their whole input first (`bulk.rs:223-234`).
- The pool fix (ticket 0142), the usage-write fix (0141) and relate running at once (0143) have landed. The engine's batch planner has landed (0144). No surface batches yet, because B4 is not built.

## 1. The matrix

Each cell holds yes, no, partial, or n/a for not applicable. A bracketed tag names the evidence note under the table, and every no and every partial carries one. Paths in the notes are relative to the surface's folder unless they start with `crates/`, `conformance/` or `sdlc/`. The Python column covers Python lists and plain calls. The Python frames column covers the Polars and pandas doors.

| Capability | Command | Rust | Rust Polars | C | Python | Python frames | TypeScript | Ruby | R | DuckDB | SQLite | PostgreSQL |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `decide` | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes |
| `choose` | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes |
| `tag` | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes |
| `score` | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes | yes |
| `rank` | yes | yes | no [F2] | yes | yes | n/a | yes | yes | partial [F3] | yes | no [F4] | partial [F5] |
| `filter` | yes | yes | n/a | yes | yes | n/a | yes | yes | yes | yes | yes | partial [F5] |
| `find` | yes | partial [F6] | n/a | partial [F6] | partial [F6] | n/a | partial [F6] | partial [F6] | partial [F6] | no [F7] | no [F7] | no [F7] |
| `annotate` | yes | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] | partial [F8] |
| `recognize` | yes | partial [F9] | no [F2] | partial [F9] | partial [F9] | partial [F10] | partial [F9] | partial [F9] | yes | partial [F11] | partial [F12] | yes |
| `relate` | yes | yes | n/a | yes | yes | n/a | yes | yes | yes | yes | yes | yes |
| Question files | yes | yes | yes | partial [Q1] | yes | yes | partial [Q2] | partial [Q3] | partial [Q4] | partial [Q5] | yes | yes |
| Jobs and throttle | yes | yes | yes | no [S1] | yes | yes | yes | yes | yes | yes | yes | yes |
| Batch | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] | no [S2] |
| Cache | yes | yes | yes | partial [S3] | yes | yes | yes | yes | yes | partial [S4] | yes | partial [S4] |
| Record and replay | yes | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] | partial [S5] |
| Strict replay | yes | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] | no [S5] |
| Backend profile | yes | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] |
| Timeout | yes | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] |
| Retries | yes | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] | no [S6] |
| Model | yes | yes | yes | partial [S7] | yes | yes | yes | yes | yes | partial [S7] | partial [S7] | partial [S7] |
| Base URL | yes | yes | yes | partial [S8] | yes | yes | yes | yes | yes | partial [S8] | partial [S8] | partial [S8] |
| Run facts: tokens | partial [R1] | partial [R2] | no [R3] | partial [R2] | partial [R2] | no [R3] | partial [R2] | partial [R2] | partial [R2] | no [R4] | partial [R2] | partial [R2] |
| Run facts: cost | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] | no [R5] |
| Run facts: time | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] | no [R6] |
| Usage and status | yes | partial [U1] | partial [U1] | partial [U1] | partial [U1] | partial [U1] | partial [U1] | partial [U1] | partial [U1] | partial [U2] | partial [U2] | partial [U2] |
| Errors as host classes | yes | yes | yes | yes | yes | yes | partial [E1] | yes | yes | partial [E2] | partial [E3] | partial [E4] |
| Cancel and deadline | yes | yes | yes | yes | yes | yes | yes | yes | yes | partial [C1] | partial [C2] | yes |
| Fork safety | n/a | yes | yes | yes | yes | yes | n/a [K1] | partial [K2] | yes | yes | yes | yes |
| Streaming | yes | yes | n/a | no [T1] | no [T2] | n/a | no [T3] | no [T4] | no [T5] | partial [T6] | partial [T7] | no [T8] |
| `audit`, `diff`, `transform` | yes | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] | no [A1] |

### Evidence notes

**Functions**

- **F1, closed 2026-09-28.** Rust, Python, TypeScript and Ruby now expose `choose_many`, `score_many` and `tag_many`; C's JSON door accepts ordered `records` for all four judgments; R's existing verbs accept columns. Frames retain their series/column forms. The independent [register 78 closure review](../records/2026-09-28-library-sql-completion-reconciliation.md) maps each public route to its accepted functional proof. SQL scalar throughput remains a separate model; this closure adds no typed C symbol or new SQL scheduler.
- **F2.** The Rust Polars door has five methods: four series verbs and `annotate_frame` (`crates/thinkthen/src/public/frame.rs:42-110`). `decide_series` returns a Boolean with no probability, so a frame cannot sort by it. No recognize method exists.
- **F3.** `tt_rank` and `tt_find` read a built question through `.tt_text`, which drops its model and threshold without a word (`libraries/r/thinkthen/R/thinkthen.R:152-154`, `:205`, `:213`).
- **F4.** SQLite registers no rank function and no probability scalar (`databases/sqlite/src/scalars.rs:372-405`). Its conformance runner lists rank as not run (`tests/conformance.py:26-27`).
- **F5.** PostgreSQL relies on `ORDER BY thinkthen_probability(...)` and `WHERE thinkthen_decide(...)` (`databases/postgresql/README.md:3`, `src/lib.rs:97`). Its runner skips every filter and rank case (`tests/runner.py:26-31`, `:52-57`), so nothing proves the two match the command.
- **F6.** No library has a "none" switch for find, because the public API has none (`crates/thinkthen/src/public/bulk.rs:168-173`). Cases `18-find-second` and `19-find-none` are skipped on every library:
  - Rust: `conformance/consumer/consumer/tests/public/cases.rs:30-35`
  - C: `libraries/c/tests/door/cases.rs:28-34`
  - Python: `libraries/python/tests/conformance.py:32-33`
  - TypeScript: `libraries/typescript/tests/cases.mjs:14-15`
  - Ruby: `libraries/ruby/tests/conformance.rb:13-14`
  - R: `libraries/r/tests/conformance.R:21-22`
- **F7.** No SQL surface has a find function.
  - DuckDB: `databases/duckdb/tools/conformance.py:28`
  - SQLite: `databases/sqlite/tests/conformance.py:26-27`
  - PostgreSQL: `databases/postgresql/tests/runner.py:26`
- **F8.** A question set refuses a per-question `on` pointer (`crates/thinkthen/src/public/set.rs:37-47`), so every surface sends each record whole. Every runner skips `18-annotate-two-groups`. SQLite and PostgreSQL also have annotate over one text per call only: `databases/sqlite/src/scalars.rs:172-196` and `databases/postgresql/src/lib.rs:146-160`.
- **F9.** The library's recognize takes one text (`crates/thinkthen/src/public/recognize.rs:404-435`). The command reads many records. No many-text call exists in:
  - C: `libraries/c/src/ffi.rs:390-415`
  - Python: `src/engine.rs:415-431`
  - TypeScript: `index.js:273-283`
  - Ruby: `lib/thinkthen.rb:192-196`
- **F10.** Python's frame recognize refuses relations (`libraries/python/thinkthen/__init__.py:337-338`). It also calls the engine once per text in sequence (`src/frame.rs:355-368`).
- **F11.** DuckDB's `thinkthen_recognize` takes kind names with no descriptions (`databases/duckdb/src/scalars/calls.rs:150-158`). Relations come through a second function, `thinkthen_relations`.
- **F12.** SQLite's recognize table function refuses a spec with relations (`databases/sqlite/src/tables.rs:137-141`).

**Question files**

- **Q1.** C reads JSON text and never a path (`libraries/c/src/door.rs:45-51`). Case `30-local-question-file` is skipped (`tests/door/cases.rs:14`).
- **Q2.** TypeScript reads a file for annotate alone. `specOf` takes the question text or an object (`libraries/typescript/index.js:36-53`), and recognize and relate specs are built from options (`index.js:177-212`). Case 30 is skipped (`tests/cases.mjs:17`).
- **Q3.** Ruby loads a question set by path (`libraries/ruby/lib/thinkthen.rb:271`) and takes keywords elsewhere (`lib/thinkthen.rb:262-267`, `:192`, `:203`). Case 30 is skipped (`tests/conformance.rb:17`).
- **Q4.** R's single-question verbs cannot load a file (`libraries/r/thinkthen/R/thinkthen.R:125-142`). Annotate takes a path with no JSON-text form (`src/rust/src/ffi.rs:240`). Case 30 is skipped (`tests/conformance.R:19`).
- **Q5.** DuckDB refuses `@file` and JSON for choose, score and tag, which must be plain text (`databases/duckdb/src/questions.rs:133-141`).

**Settings**

- **S1.** C builds its engine with `Engine::from_env` (`libraries/c/src/ffi.rs:160-166`), and no variable sets the throttle (`DESIGN.md:13`). `max_requests` is unreachable too.
- **S2.** Nothing batches yet. The command waits on B4. Each surface waits on its ticket from B12a to B12f or B13a to B13e (`sdlc/issues/2026-09-26-batching-design.md:371`, `:380-390`).
- **S3.** C selects the cache folder only through `THINKTHEN_CACHE` (`include/thinkthen.h:115-122`). It turns the cache off only through the configuration file (`crates/thinkthen/src/public/settings.rs:90-95`).
- **S4.** DuckDB and PostgreSQL can name a folder but cannot turn the cache off.
  - DuckDB: `databases/duckdb/src/engines.rs:60-71`
  - PostgreSQL: `Plan` has no off field (`databases/postgresql/src/call.rs:98-103`)
- **S5.** The public builder sets one folder as both record and replay (`crates/thinkthen/src/public/settings.rs:270-275`), so a miss sends a live request. No surface has record-only, replay-only, or strict replay. Filed in `sdlc/issues/2026-09-26-libraries-cannot-replay-a-recording-strictly.md`.
- **S6.** The builder fixes these three (`crates/thinkthen/src/public/settings.rs:233-238`). Filed as item 1 of `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md`.
- **S7.** C and the SQL surfaces take a model only in the question JSON or the configuration file (`crates/thinkthen/src/public/settings.rs:107-108`). Filed as item 2 of the settings issue.
- **S8.** C and the SQL surfaces read the address from the environment alone. For SQL this is deliberate, by ticket 0109 decision 2 and ticket 0110 decision 5.

**Run facts**

- **R1.** `--details` gives per-row usage. `filter --details` omits dropped records, and no run total prints until B5's `--facts` lands (`sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md`, gap 5).
- **R2.** Tokens come only from a `details` call on one text, or from cumulative counters.
  - Rust: `crates/thinkthen/src/public/engine.rs:285-310`
  - C: `libraries/c/src/call.rs:41`, `:125-127`
  - Python: `src/engine.rs:326-327`
  - TypeScript: `src/door.rs:347-349`, and the typed `Details` omits `confidence` (`index.d.ts:74-98`)
  - Ruby: `lib/thinkthen.rb:162-165`
  - R: `thinkthen/R/thinkthen.R:342-346`, which keeps only the first text (`:344`)
  - SQLite: `databases/sqlite/src/scalars.rs:155-170`
  - PostgreSQL: `databases/postgresql/src/lib.rs:162-166`
- **R3.** Rust Polars and Python frames return values alone. Python refuses `details` on a column (`libraries/python/src/frame.rs:116-117`).
- **R4.** DuckDB's details struct holds no token usage, no URL, no `confidence`, and no distribution for choose, score and tag (`databases/duckdb/src/scalars/calls.rs:113-140`).
- **R5.** No surface computes cost. Only the `cost` transform does, from a price the user passes (`crates/thinkthen/transforms/cost.jq:6-19`).
- **R6.** No code records wall time or backend time (`sdlc/issues/2026-09-23-record-the-backends-own-time-for-each-call.md`).

**Usage and status**

- **U1.** Library counters live in memory for one engine and die with the process (`crates/thinkthen/src/public/settings.rs:246`). `thinkthen status` never sees them. Filed in item 1 of `sdlc/issues/2026-09-25-public-library-api-gaps.md` and in `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`.
- **U2.** SQL counters are per process, and nothing saves them. The process request total has the three leaks in that status issue. DuckDB's warm skips the total (`databases/duckdb/src/tables.rs:88-91`), and relate checks it with an empty list (`src/relate.rs:216`). PostgreSQL counts per backend and reads the total once per call (`databases/postgresql/src/call.rs:366-371`).

**Errors**

- **E1.** TypeScript throws one `ThinkThenError` class that carries `kind` and `retryable` (`libraries/typescript/index.js:9-27`). Python, Ruby and R each have one class per kind. A native throw outside the envelope, such as a thread-safe-function failure (`src/node.rs:105`), reaches JavaScript as a plain `Error`.
- **E2.** DuckDB raises one error type. The kind travels as the message prefix `thinkthen <kind>: `, and retryable as text (`databases/duckdb/src/errors.rs:11-33`).
- **E3.** SQLite maps backend, deadline and defect all to `SQLITE_ERROR` (`databases/sqlite/src/lib.rs:73-80`). The kind rides in the message prefix (`lib.rs:84-94`).
- **E4.** PostgreSQL gives cancelled and deadline the same SQLSTATE, 57014 (`databases/postgresql/src/call.rs:62-71`). Retryable appears only in the message (`call.rs:41-49`).

**Cancel and deadline**

- **C1.** DuckDB's query cancel, `con.interrupt()`, never reaches a scalar. Only SIGINT does (`databases/duckdb/README.md:68`, `src/signal/ffi.rs:32-114`). Details and recognize give each text a fresh deadline budget (`src/scalars/calls.rs:101`, `:173`).
- **C2.** SQLite's warm, recognize and relate take no deadline (`databases/sqlite/src/scalars.rs:295`, `src/tables.rs:169`, `:324`).

**Fork safety**

- **K1.** Node has no `fork(2)`. `child_process.fork` starts a fresh process, and a `Worker` shares the process engine and throttle (`libraries/typescript/tests/fork.test.mjs:11`, `:25`).
- **K2.** Ruby rebuilds the engine after a fork. Its watchdog rows from threads in flight in the parent are copied into the child's `@rows` (`libraries/ruby/lib/thinkthen.rb:75`, `:445`). Only a fork after a finished call is tested (`tests/test_fork.rb:10`).

**Streaming**

- **T1.** C collects every result before it returns (`libraries/c/src/door.rs:82-91`, `src/call.rs:141-143`, `:194-198`). `DESIGN.md:79-83` chooses this on purpose: every row or none.
- **T2.** Python reads its whole input and collects every row (`libraries/python/src/input.rs:111-136`, `src/engine.rs:196`, `:409`).
- **T3.** TypeScript collects the whole batch into one JSON string (`libraries/typescript/src/door.rs:299-301`, `:327-330`, `:376-379`). Ticket 0107 deferred async iteration.
- **T4.** Ruby calls `records.to_a` first, so a lazy or endless enumerator never streams (`libraries/ruby/lib/thinkthen.rb:116`, `:121`, `:127`, `:134`, `:173`).
- **T5.** R drains every engine iterator before it builds a value (`libraries/r/thinkthen/src/rust/src/calls.rs:178-183`, `:270-282`).
- **T6.** DuckDB answers one chunk of up to 2,048 rows before it writes any row of that chunk (`databases/duckdb/src/scalars/ffi.rs:97-100`). Relate and warm collect everything (`src/relate/ffi.rs:143-145`).
- **T7.** SQLite scalars go row by row, while warm buffers 256 rows a question (`databases/sqlite/src/scalars.rs:16`, `:271`). Its table functions collect every row before the first `next` (`src/ffi.rs:217-228`).
- **T8.** PostgreSQL's set-returning functions build a whole `Vec` (`databases/postgresql/src/lib.rs:362-370`, `src/relate.rs:347-360`).

**Command-only tools**

- **A1.** No library or SQL surface exposes `audit`, `diff` or `transform`. They are command subcommands over saved JSON lines (`specification/audit.md`, `diff.md`, `transform.md`).

## 2. Gaps that can close

Each proposed ticket below groups gaps that share a cause. A ticket marked **shared** changes the public API once and then carries every surface in the same ticket. The sizes are rough: small is under a day of agent work, medium is one to three days, and large is more.

### E1. Engine settings reach every surface. Shared, medium.

Closes S1, S3 to S7 and C2, the settings issue, and the strict-replay issue.

- Add these to `EngineBuilder`: `timeout`, `max_retries`, `profile`, `replay_only`, `record_only`, and a strict-replay mode whose miss is `Error::Local` and sends nothing. The engine facade already has every one (`crates/thinkthen/src/engine/facade.rs:60-72`). The builder only fixes them.
- Give each surface the same spelling it already uses for the cache:
  - C and the SQL surfaces: an engine-level model.
  - C: throttle and `max_requests` through a builder symbol or variables.
  - DuckDB and PostgreSQL: a cache-off setting.
  - SQLite: a deadline on warm, recognize and relate.
- Remove `cache_bytes` in the same ticket, following item 4 of the API gaps issue and Ian's ruling of 2026-09-25. Correct the READMEs that claim it has an effect: Python, Ruby, TypeScript and the three SQL surfaces.
- Update `specification/settings.md` in the same commit.
- Proof: one shared conformance case per setting, plus a strict-replay miss that counts zero loopback requests.
- This ticket also unblocks the site's offline language examples.

### E2. Many-record calls for every function. Shared, medium.

F1 is complete through the landed batching tickets and the independently reviewed register 78 closure. The original proposal below also covers F9, F10 and serial loops in section 4; their remaining criteria are separate from F1.

- Add `choose_many`, `score_many` and `tag_many` to the public API as thin calls over a one-question set, as `crates/thinkthen/src/public/frame.rs:196-218` already does.
- Add a `recognize_many` that runs texts under the engine's throttle. Batching tickets R7 and B12 then build on one call per surface.
- Carry the calls to C, Python lists, TypeScript and Ruby.
- Replace the serial loops:
  - R: `thinkthen/src/rust/src/relate.rs:76-80`
  - Python frames: `src/frame.rs:355-368`
  - DuckDB: `src/scalars/calls.rs:91-111`, `:160-178`
- Let a question set carry a per-question model, so R's named-model fallback (`calls.rs:381-405`) goes away.
- Run DuckDB's groups within one chunk at once (`src/scalars.rs:180-189`).
- Proof: a held-arm test on each surface counting the throttle in flight for choose and for recognize.

### E3. Find's none option and annotate's record parts. Shared, medium.

Closes F6 and F8, and item 5 of the API gaps issue.

- Add a none switch to find and per-question `on` parts to `QuestionSet`, then carry both to each library.
- For SQL, add a find aggregate, such as `thinkthen_find(question, text)` over a group, which sends the whole set as one request.
- Proof: cases `18-find-second`, `19-find-none` and `18-annotate-two-groups` leave every skip list.

### E4. Question files everywhere. Shared, small.

Closes Q1 to Q5.

- Accept a path or JSON text for every question, recognize spec, and relate file:
  - C: a load call or a `"file"` key. Either adds a symbol.
  - TypeScript and Ruby: every verb.
  - R: the single-question verbs.
  - DuckDB: choose, score and tag.
- Proof: case `30-local-question-file` leaves four skip lists.

### E5. Errors carry their kind as a host type. Shared, small.

Closes E1 and E4, and records E2 and E3.

- TypeScript: one subclass per kind. Wrap the native call so a stray throw becomes `defect`.
- PostgreSQL: give deadline its own SQLSTATE and put `retryable` in the error detail.
- DuckDB and SQLite: say in each README that the message prefix is the contract. A conformance check then parses it. Section 3 explains why they cannot do more.

### E6. Run facts on every call. Existing tickets, large.

Closes R1 to R6. B5 and B12a to B12f already hold this work, and B0 set the fields.

- Add wall time per request, from the 2026-09-23 issue.
- Add the missing fields to DuckDB's details struct: usage, URL, confidence and the distributions. The run-facts issue keeps this item out of the deferral.
- Expose `confidence` and `url` on the typed Rust and TypeScript views.
- Cost needs a price the user supplies in configuration. It needs a decision on output pricing before any code.
- SQL per-call facts stay deferred to their own design, as that issue says.

### E7. Batching on every surface. Existing tickets B12a to B13e, large.

This was the pre-build proposal. B13d/0217 and B13e/0219 have now landed; their accepted packed warm paths and the numeric model in section4 close original register73. The three historical suggestions below do not add new mandatory scalar work or supersede the accepted ticket contracts.

- SQLite and PostgreSQL batch only decide, through warm (`databases/sqlite/src/scalars.rs:317-327`, `databases/postgresql/src/warm.rs:148-153`). B13d and B13e should widen warm, or add an aggregate, to every kind.
- PostgreSQL's warm and SQLite's finalize run question groups one after another (`databases/postgresql/src/warm.rs:144-156`, `databases/sqlite/src/scalars.rs:363-365`). They should run the groups together.
- SQLite judges each warm chunk inside `step`, which stalls the scan (`scalars.rs:347-349`).

### E8. Usage per process, and status for every surface. Shared, medium, needs an ADR.

Closes U1 and U2, item 1 of the API gaps issue, and option 1 of the status issue.

- Count once per process, so PostgreSQL's sum goes away.
- Let the engine write the same count-only usage store the command writes, under the same best-effort rule.
- Close DuckDB's warm and relate leaks around the total.

### E9. Streaming results. Per-language design, medium to large.

Closes T2 to T5 and T8. The engine already streams (`crates/thinkthen/src/public/batch.rs`), so each door would hand rows through as they come:

- Python: a generator.
- Ruby: an `Enumerator` with lazy input.
- TypeScript: `for await`.
- PostgreSQL: a value-per-call set-returning function.

C's frozen all-or-nothing contract (T1) would need a new callback symbol and a DESIGN amendment, so this page leaves C out unless a user asks. R returns vectors by nature, and streaming serves it little.

### E10. Correctness sweep. Quick fixes, small.

Small defects this audit found that no issue holds yet:

- Ruby's `@rows` in a forked child (K2).
- R:
  - `tt_details` keeps only the first text (`thinkthen.R:344`).
  - `tt_rank` and `tt_find` drop a model (F3).
  - `tt_annotate` refuses NA rows and a missing `on` column (`src/rust/src/ffi.rs:71-74`, `R/thinkthen.R:235`).
  - A tibble loses its class (`R/thinkthen.R:237`).
- PostgreSQL:
  - The README says a throttle of 0 to 32 (`README.md:40`), and the builder refuses 0.
  - The wait loop ignores postmaster death (`src/call.rs:243-259`).
  - The runner's docstring says it has no skip list (`tests/runner.py:8`).
- C's header and DESIGN disagree on the frozen version (`include/thinkthen.h:39`, `DESIGN.md:124`).
- Several pages say "this process's totals" for per-engine counters (`crates/thinkthen/src/public/engine.rs:140`, `libraries/c/include/thinkthen.h:247-249`, `libraries/typescript/index.d.ts:190`).
- Ruby's README omits that a second throttle raises.

### E11. Concurrency proofs and the conformance gaps. Shared, small to medium.

The tests section 4 and section 5 name. Land E11 before E7, so batching starts from a measured baseline.

### Proposed order

1. E4 question files. Small, it stands alone, and it removes a skip on four surfaces.
2. E1 engine settings. It closes two open issues and unblocks the site's offline examples.
3. E11 concurrency proofs and conformance cases. They give B12 and B13 a baseline.
4. E2 many-record calls. The engine batches only what reaches it in one call, so this comes before E7.
5. E3 find none and annotate parts.
6. E10 correctness sweep and E5 errors, before 0.1.
7. E7 and E6, the batching tickets B12a to B13e with facts, in the order the batching design sets.
8. E8 usage per process and status. It needs an ADR first.
9. E9 streaming, once a user asks for it on a named language.

## 3. Gaps that cannot close, and why

- **A plain SQLite scalar cannot batch or run rows at once.** SQLite calls a scalar once for each row and waits for its value before it steps the next row. The extension API has no vector form. So one statement holds one request in flight, and B13e can batch only through warm or an aggregate (`databases/sqlite/src/worker.rs:30-62`, batching design section 6). What would change the answer: a table-valued function over a subquery, or a warm call before the scalar pass, which the README already recommends. Both change how a user writes the query.
- **A PostgreSQL scalar runs one row at a time for the same reason.** The executor calls a function per row and has no vectorized scalar interface (`databases/postgresql/src/call.rs:278-332`). The array form and warm reach the throttle. Parallel query could spread rows across workers. Each worker is a separate process with its own engine, throttle and request total, so the functions stay `parallel_restricted`. What would change the answer: a ruling that accepts a throttle per worker.
- **DuckDB's batches follow its vectors.** A batch closes at a chunk's edge of up to 2,048 rows, and a parallel scan hands rows over in another grouping. Recordings match the command's only at `--batch 1`, or at `SET threads = 1` for exact replay (batching design, open item 14). What would change the answer: a sink that gathers across chunks, as warm does. That would need a new query shape.
- **DuckDB's stable C API limits errors and cancel.**
  - A scalar can set only an error message, so the kind cannot become a host class. The message prefix is the contract.
  - A scalar cannot see the query's interrupt, so `con.interrupt()` never reaches it (`databases/duckdb/README.md:68`).
  - Scalar bind is unusable (`sdlc/issues/2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md`).
  - An aggregate has no client context and cannot read `SET` values. That is why warm skips the request total (`src/tables.rs:69-93`).
  - A table function cannot run SQL on the caller's connection.
  - What would change the answer: DuckDB adding these to its stable C API, or a move to the C++ extension API, which ties each build to one DuckDB version.
- **SQLite's error codes are a fixed set.** An extension may return only SQLite's own codes. Three kinds share `SQLITE_ERROR`, and the kind rides in the message. Nothing short of a SQLite change alters this.
- **SQL cannot name an address or a key.** This is a ruling, and SQL could technically carry them. SQL text lands in logs, query history and plans, and the key rule keeps a key out of all three (tickets 0109 and 0110, `databases/postgresql/README.md:87-89`). Ian can overturn it. The address is less sensitive than the key, and a session setting for the address alone would be safe.
- **SQL cannot return facts on every call.** A scalar returns one value a row. Facts need a second function or a details column, which the run-facts issue defers to its own design. A library can carry `facts` on every call. SQL can only offer them beside the answer.
- **R's host thread blocks for the whole call.** R runs its interpreter on one thread, so R does nothing else while a call works. The engine still reaches its throttle inside one vector call on its own threads (`libraries/r/thinkthen/src/rust/src/calls.rs:98-123`). R code that calls per row, through `sapply` or `rowwise()`, is serial by R's own design. What would change the answer: an asynchronous R API over `later` or `promises`. No user has asked for one.
- **Node has no fork.** Fork safety does not apply to TypeScript. A `Worker` shares the process engine and throttle, which is correct.
- **Each loaded copy of the engine has its own throttle.** A process that loads Python and DuckDB together runs two throttles and two sets of counters (ADR 0047 item 5, `libraries/rust/README.md`). The throttle is a static in each copy. What would change the answer: one shared library that every surface links, which ADR 0047 chose against.
- **Rank and find have no column form.** A rank sorts records, and a find picks one unit of a set. The Polars and pandas doors return one value per row, so they leave both functions to the list call.
- **`audit`, `diff` and `transform` should stay command tools.** No technical limit stops a library from calling them. They read and write saved JSON lines files, and any surface can write those. This page recommends against porting them, because the command serves every surface's saved rows. Ian can overturn this.

## 4. Performance

### Does each surface reach the engine's concurrency?

The engine gives each loaded copy one request throttle, 1 to 32. Landed batching routes pack the records supplied by one call under that throttle. SQLite and PostgreSQL ordinary scalar expressions remain serial; warm paths and PostgreSQL array calls submit several records through the shared bulk engine. The dated register73 closure below distinguishes the model from measured cases.

| Surface | Many-record calls reach the throttle | Where it serializes, copies, or blocks |
| --- | --- | --- |
| Command | Yes. `crates/thinkthen/tests/backend/parallel.rs:162`, `:317` | None known |
| Rust | Yes, for `filter`, `decide_many` and `annotate`. `crates/thinkthen/tests/public_batches.rs:82`, `:223` | `rank` and `find` hold their input first by design. Recognize runs one text a call |
| Rust Polars | Yes. `crates/thinkthen/tests/polars/throttle_equality.rs:51` | Copies the column into rows (`crates/thinkthen/src/public/frame.rs:120`, `:210`) |
| C | Yes, in one call (`src/door.rs:76-100`, `src/call.rs:141-143`, `:194-197`) | Blocks the calling C thread. The JSON door copies records (`src/call.rs:338-340`) and the reply (`src/ffi.rs:371-373`) |
| Python | Yes. The GIL is released in 50 ms ticks (`src/worker.rs:132`) | Holds the GIL while it copies input (`src/input.rs:121-136`) and builds output. Starts one OS thread a call (`src/worker.rs:90-97`). pandas 2 object columns copy whole (`src/frame.rs:71-81`). Frame recognize is serial (F10) |
| TypeScript | Yes, off the main thread (`src/node.rs:115-120`) | One OS thread a call with no cap. Stringifies input and parses output on the main thread (`index.js:25`, `:123-127`). Three copies of the input (`src/door.rs:455-465`) |
| Ruby | Yes. The GVL is released (`src/ffi.rs:161-169`) | One OS thread a call (`src/lib.rs:171-177`). Three copies of the input (`lib/thinkthen.rb:116-121`, `src/ffi.rs:198`) |
| R | Yes, for vector calls (`src/rust/src/calls.rs:171-199`) | The R thread blocks. Recognize is serial per text (`src/rust/src/relate.rs:76-80`), and so is the named-model fallback (`calls.rs:381-405`) |
| DuckDB | Yes, within one chunk group, and all threads share one engine (`src/engines.rs:55-99`) | Groups in a chunk run serially (`src/scalars.rs:180-189`). Details, recognize and relations run one text at a time (`src/scalars/calls.rs:91-111`, `:160-178`). Duplicates are removed within a chunk only |
| SQLite | Warm uses the configured request throttle, 1–32, within a packed question group; its held test observes 8 at throttle 8 | Ordinary scalar rows remain one call at a time. `src/scalars/warm.rs` packs warm input through `decide_many_with`; accepted0219 proves the installed path |
| PostgreSQL | Array and warm calls use the configured request throttle, 1–32, within a packed group | Ordinary scalar rows remain serial. `src/warm.rs` submits each group through `decide_many_with` under one deadline. Accepted0217 proves warm request bodies; its held-eight measurement is for arrays |

Register73's original numeric-documentation and warm-path criteria are complete after B13d/B13e and independent closure review. This does not change the scalar executor contracts described in section3. Other historical rows in this study need their own outcome checks before being treated as current limitations.

### Tests that prove concurrency today

- **Command:** `crates/thinkthen/tests/backend/parallel.rs:162` holds no more in flight than `--jobs`. `:317` opens one connection a job and reuses it. `:188` prints the same bytes at every job count.
- **Rust:**
  - `crates/thinkthen/tests/public_batches.rs:82`: one engine serves two threads under one throttle.
  - `public_batches.rs:223`: input is read at most one throttle ahead.
  - `crates/thinkthen/tests/polars/throttle_equality.rs:51`: a Polars column matches a slice.
  - `conformance/consumer/fork-probe/tests/fork.rs:74`: a child of a busy parent gets its own permits.
- **Python:**
  - `tests/test_engine.py:21`: exactly 8 in flight.
  - `tests/test_column_timing.py:53`, `:74`.
  - `tests/test_pandas.py:266`, `:294`.
  - `tests/test_stopping.py:107`.
- **TypeScript:** `tests/throttle.test.mjs:8`, `:26`, `tests/settings.test.mjs:76`, `tests/abort.test.mjs:44`.
- **Ruby:** `tests/test_engine_settings.rb:31` and `tests/test_interrupt_batch.rb:94`, which runs two Ruby threads with 16 in flight.
- **R:** `tests/interrupt.R:92-95` holds exactly 8. `:111-121` shows only more than one in flight for choose, score and tag.
- **DuckDB:**
  - `tools/signal_suite.py:112`: 8 in flight within one chunk.
  - `signal_suite.py:171`: several queries held at once.
  - `tools/relate_suite.py:184`.
- **SQLite:** `tests/test_settings.py:37-51`, warm at throttle 8.
- **PostgreSQL:** `check.sh:453-462`, array form at throttle 8, and `check.sh:490-505`.

No test proves these:

- C holds anything in flight at once.
- Two Python threads overlap, which is the GIL proof.
- Python choose, score or tag columns reach the throttle.
- DuckDB threads on a table over 2,048 rows share one throttle.
- Two SQLite connections share the process throttle.
- PostgreSQL warm reaches the throttle.
- R filter and annotate reach it.

No shared conformance case tests concurrency.

### Measurements for QA wave 3

Every measurement uses the loopback backend in `conformance/backend` and sends no live call. Each report names the build and the machine.

1. **Width efficiency.** Run 1,000 records of `decide_many`, or each surface's equivalent, on `/arm/delay/100/v1` at throttle 8, on every surface and on the command. The ideal is 12.5 seconds. Report wall time against that ideal and against the command.
2. **The three verbs over many records.** Run 1,000 records each of choose, score and tag on every surface. Before E2 this shows the serial surfaces. After E2 it proves the fix.
3. **Recognize over 100 texts.** Run R, Python frames and DuckDB against a Rust loop under the throttle. Count requests in flight with `wait N`.
4. **SQL rows.** Run SQLite and PostgreSQL decide over 1,000 rows three ways: cold scalar, warm then scalar, and PostgreSQL's array form. Run DuckDB at `SET threads = 1` and at 4 over 10,000 rows, and hold the arm to read in-flight counts.
5. **Per-call overhead.** Make 10,000 cached single calls on `/generic/v1` on each surface. This prices the thread started per call in Python, Ruby, R, SQLite and TypeScript.
6. **Copies.** Measure peak resident memory for a one-million-row column: Python Polars against pandas 2 object, a TypeScript array, a Ruby array, and R.
7. **Host responsiveness.** Measure TypeScript event-loop drift, which exists already. Measure a second Python thread's progress during a held call, and Ruby thread progress likewise.
8. **After B4.** Count requests over the 306-line fixture per surface under replay, against the command's count.

## 5. What the conformance cases cover and miss

`conformance/cases.json` holds 54 cases.

**What they cover:**

- Every function's bare value, and detailed request identity through four details fields: `answer`, `model`, `question_sha256` and `requests`.
- The six error kinds.
- The counters in case 40.
- Question-file loading in case 30.
- Recognize and relate.

**How many cases each surface runs:**

| Surface | Runs | Skipped cases |
| --- | --- | --- |
| Rust | 50 | find none, annotate parts, defect |
| C | 49 | Adds case 30 |
| TypeScript | 49 | Adds case 30 |
| Ruby | 49 | Adds case 30 |
| Python | about 50 | Re-runs cases over a Polars series and frame, and never over pandas (`libraries/python/tests/conformance.py:93`, `:116`, `:168`) |
| R | 46 | Also skips cases 20, 22, 23, 25 and 30 (`libraries/r/tests/conformance.R:14-23`) |
| DuckDB | All but the find and parts cases | The installed-release run also drops the defect case |
| SQLite | 38 | 16 not run, including rank, find and recognize relations (`databases/sqlite/NOTES.md:28`) |
| PostgreSQL | 43 | 11 not run, including every filter, rank and find case and case 23 (`databases/postgresql/tests/runner.py:26-31`, `:52-57`) |

**What they miss:**

- Every run-fact field beyond the four: `usage`, `requests_sent`, `cached`, `confidence` and `url`. DuckDB's smaller struct slipped through because of this.
- Settings. No case sets a throttle, cache folder, `max_requests`, model, or deadline value and checks its effect.
- Concurrency. No case states that a held backend sees the throttle in flight.
- Strict replay and record-only.
- Streaming. No case checks that a surface reads at most one throttle ahead.
- Fork.
- Batching. The batching design asks for shared cases on batch count, shares, and `--batch 1` bytes, and none exists yet.
- The backend profile cases. `conformance/backend-profiles.json` runs only in the command's tests (`crates/thinkthen/src/cli/conformance_tests/profile_cases.rs`). `record-values.json` runs only in the core.
- Recognize settings. R6's `keep`, `infixes` and `boundary` cases do not exist yet.

**Proposal, inside E11:**

- Add details-field checks for `usage`, `requests_sent`, `cached`, `confidence` and `url`.
- Add one settings case per engine setting.
- Add a `concurrency` case kind whose expectation is the in-flight count on the held arm at a named throttle. Each runner drives it with `wait N` and `count`.
- Add pandas re-runs to Python.
- Let PostgreSQL run filter and rank through `WHERE` and `ORDER BY`, as SQLite and DuckDB already do.
