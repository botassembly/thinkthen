# Punch list report — the languages lane

The library team's work against the architect's punch list (`2026-09-21`,
handoff from the architect's survey of `surfaces` at `9ef50a6`). This lane
owns items 1, 2, and 3 plus the parser preparation from the wait-list,
across the six language surfaces. The boundary held: a binding converts
arguments, manages host lifetimes and interrupts, calls Rust, and presents
the result — no second scheduler, no second answer rule, no second bulk
implementation, nothing published, no paid call. Snapshot: branch
`surfaces`, this lane's commits `c666251`, `0cd86bd`, and the ones named
below.

## Item 1 — probabilities from the one call (Ruby)

`decide_many_with_probabilities` made the bulk call and then a `details`
call a record. The native side now carries each judgment's probability
beside its answer from the same bulk call — a new `Bulk::DecideManyWithProbabilities`
crossing that shares the one `decide_many_opts` invocation, no extra
request.

- Fixed: commit `c666251`.
- Focused test: `libraries/ruby/tests/test_pairs_one_crossing.rb`, run on
  the wire by `libraries/ruby/check.sh`. Acceptance output: "one crossing:
  20 pairs beside their probabilities, 20 requests, none after the
  return" — the old shape would show 40. The 74-case conformance slice
  stays green through the same path (symbol keys re-keyed in the wrapper,
  one map, no engine call).

## Item 2 — the per-record scheduling loops, inventoried

Six loops schedule judgments a record today. Each is conversion-ready for
the engine's bulk entry point; none was converted here, because converting
any of them would mean inventing a second bulk implementation while the
engine entry is pending (the architect's boundary). A loop that only
converts returned values into a host column is not listed.

| # | surface | loop | where | conversion target |
| --- | --- | --- | --- | --- |
| 1 | Python | `recognize_stream` calls `engine.recognize_opts` per record | `libraries/python/src/lib.rs` (`for (place, text) in references`) | future engine `recognize_many(ask, texts)` |
| 2 | Python | `score` over an Arrow column calls `engine.score_opts` per record (also runs without the poll) | `libraries/python/src/lib.rs` (`for text in &texts`) | future engine `score_many` |
| 3 | R | `tt_recognize_column` maps `engine.recognize_opts` per record | `libraries/r/thinkthen/src/rust/src/lib.rs` | future engine `recognize_many` |
| 4 | R | `tt_choose` vapplies `tt_choose_one` a row | `libraries/r/thinkthen/R/thinkthen.R` | future engine `choose_many` |
| 5 | R | `tt_score` vapplies `tt_score_one` a row | same file | future engine `score_many` |
| 6 | R | `tt_tag` lapplies `tt_tag_one` a row | same file | future engine `tag_many` |

Ruby's `decide_many_with_probabilities` was the seventh and is fixed
above. TypeScript, Rust, and C carry no such loops: the TS bulk verbs and
the Rust wrappers call the engine's bulk entries directly, and the C door
takes one request per call with the host driving. Dated sections with the
same list live in `libraries/python/NOTES.md` and `libraries/r/NOTES.md`.

## Item 3 — canonical results and ownership

- **Answers are typed, not re-parsed.** Ruby's `recognize` and `relate`
  used to parse a serialized answer JSON; both now arrive as typed Ruby
  records built natively from the engine's own structures (commit
  `0cd86bd`; the `recognize_json`/`relate_json` methods and the
  `edges_json` import are gone). Python and TypeScript already carried
  typed accessors; the C door's JSON string is the ruled shape for a
  result of no fixed size.
- **Nested mutation isolation, lifetime, and ownership tests, added:**
  - Python — `libraries/python/tests/test_ownership.py` (4 tests): a
    returned frame survives its input's deletion and garbage collection
    and ignores a rebuilt input; the recognize records list is the host's
    own (mutating and dropping it changes nothing about the next answer);
    offsets count code points with an accent and an emoji; relation
    endpoints are typed ids. Wired into `libraries/python/check.sh`.
  - R — `libraries/r/ownership_check.R` (11 checks): the returned list is
    mutated at every depth and then the engine is asked again; values stay
    readable through `gc()` and a later call; repeated names carry
    distinct offsets that slice their own name; relation endpoints are the
    entities' own ids and name the right rows. Wired into
    `libraries/r/check.sh`.
  - TypeScript — `libraries/typescript/tests/ownership.test.mjs` (3
    tests): entities mutated at every depth leave the next answer
    untouched; `decide_many` answers are the host's own; endpoints are
    typed ids; UTF-16 slices hold on repeated names. Picked up by the
    existing `tests/*.test.mjs` glob.
  - Ruby — the mutation and lifetime tests live in
    `libraries/ruby/tests/test_surface.rb` (32 runs, 99 assertions, green),
    including the repeated-name ids case and the GC lifetime case.
- **Index units, each documented where the conversion happens and each
  conversion done once:** Python counts code points (docstring on the
  record); TypeScript counts UTF-16 units (`index.d.ts`); Ruby counts Ruby
  characters (docstring on `recognize`); R counts code points from one
  (`recognize_check.R` comments and the native conversion); C counts bytes
  (`door.rs` and the header); the database walkers are the databases
  lane's. The accent-and-emoji case is asserted in every host's own
  indexing.

## Wait-list preparation — the parser fixtures

The contract's second question-set parser will be replaced by the
production parser. The invalid cases are prepared now as synthetic
fixtures under `conformance/fixtures/parser-invalid/` (README states
"synthetic" on every file): bad versions, missing version, duplicate
names, unknown top-level keys, empty sets, empty names, `on`-group
collisions, and the file-order expectation. `contract/tests/parser_fixtures.rs`
locks today's behavior on all of them (8 tests green — the current parser
accepts every invalid case and sorts names, exactly as the survey
recorded) and carries two `#[ignore]`d tests that run at the swap and
require every invalid fixture to refuse with a usage error and names to
stay in file order.

## What this lane did NOT do, and why

- No second bulk implementation while the engine's entries are pending
  (item 2's loop list is the hand-off, not a workaround).
- No parser replacement — fixtures and locking tests only.
- No recognition policy changes, no relation-request redesign (the
  architect settles those in Rust).
- No C header changes: the missing capabilities (cancellation, deadlines,
  partial completion, null/length inputs, allocated results, concurrent
  callers) stay reported in `libraries/c/NOTES.md` until the accepted
  options/ownership design exists.
- Score's bare number and the highest-probability meaning of its detailed
  level, rank's index/probability pairs, and find's full candidate
  distribution are preserved as the contract implements them; nothing
  reinterpreted.

## The three lists

**Fixed on branch**

1. Ruby probabilities from the one call — `c666251`;
   `libraries/ruby/tests/test_pairs_one_crossing.rb` (wire, 20/20).
2. Ruby typed recognize/relate answers — `0cd86bd`;
   `libraries/ruby/tests/test_surface.rb` (32 runs; conformance slice
   green).
3. Python ownership and lifetime tests — `libraries/python/tests/test_ownership.py`
   (4 passed), wired into `check.sh`.
4. R ownership, lifetime, and repeated-name checks —
   `libraries/r/ownership_check.R` (11 checks), wired into `check.sh`.
5. TypeScript ownership and lifetime tests —
   `libraries/typescript/tests/ownership.test.mjs` (3 passed).
6. Parser fixtures and locking tests —
   `contract/tests/parser_fixtures.rs` (8 passed, 2 ignored with the
   production expectation named).

**Waiting for a named engine capability**

1. Python `recognize_stream` → `recognize_many(ask, texts)`.
2. Python `score` column loop → `score_many` (and the poll with it).
3. R `tt_recognize_column` → `recognize_many`.
4. R `tt_choose` / `tt_score` / `tt_tag` → `choose_many` / `score_many` /
   `tag_many`.
5. The production question-set parser (the fixtures and the two ignored
   tests run at the swap).
6. The C options/ownership design, before the ABI freezes.
7. Recognition boundary/overlap policy and the conflicting relation
   request designs — the architect's, in Rust.

**Still blocked**

1. Nothing in this lane's remit. The packaging-evidence item (clean
   installed-artifact checks, ABI floors, a self-contained R package,
   macOS, Node child-process inheritance) belongs to the separate
   packaging/databases lane and is not claimed here.

## The databases lane — items 4 and 5

This lane owns punch-list items 4 and 5 across the three database
extensions, in `databases/{sqlite,duckdb,postgresql}`. The boundary held:
a binding converts arguments, manages host lifetimes and interrupts,
calls Rust, and presents the result — no second scheduler, no second
answer rule, nothing published, no key, no paid call, and no container
left behind. The languages lane's sections above stand as written.

### Item 4 — the hidden counter reset, removed (SQLite)

Commit `7fbb568`. `thinkthen_usage('reset')` cleared the process
counters and the shim cache; the spelling now refuses with a usage error
naming the substitution ("the counters are cumulative, so take two
snapshots and subtract them"), the counters are cumulative only, and the
shim's answer map and hit counter are marked in the source as temporary,
deleted together at the engine swap.

Focused tests: `tests/null_suite.py` 23 of 23 (the warm check now reads
deltas — judges 3, serves 2 from the session map; a fresh sentence grows
`requests` by one and never lowers a counter; the reset spelling's
refusal names "cumulative" and "subtract"), and
`tests/conformance_driver.py` case 17 reads snapshot deltas and
supersedes its `after_reset` arm in place. The shared conformance file
keeps its recorded `after_reset` as history; no shared case changed.

### Item 5 — database authority made explicit

Each of the three READMEs gained an "Authority: who may do what" section
naming the six items: allowed question-file access, backend selection,
credential source, query execution, connection lifetime, and the
cancellation channel. Commits `34080d8` (SQLite), `98c5e81` (DuckDB),
`7c86878` (PostgreSQL).

**SQLite stays non-deterministic, proven.** `check.sh` now reads
`pragma_function_list`: eight functions, zero carrying
`SQLITE_DETERMINISTIC` (0x800 in the flags), and an index expression
over `thinkthen_decide` refuses with `unsafe use of thinkthen_decide()`.

**DuckDB's LOAD-time handler coexists with its host, proven.**
`tools/host_signal.py`, run by `check.sh` through the build's own venv
Python (3.12.3 with duckdb 1.5.5, the CLI's exact version), two child
processes so one arm's signal cannot color the other's evidence:

- the after-LOAD arm, the job-2 shape the punch list names: the host's
  own SIGINT handler fires, the process survives, and the extension
  still answers true;
- the chained arm, the default Python shape and the CLI's: a SIGINT
  1.0 s into a 3-million-record query stops it in 1.00 s
  (`thinkthen cancelled: the wait was cancelled`) and reaches the host's
  chained handler.

**Found by that proof, not fixed: the DuckDB token is one shot per
process.** Once the extension's own handler cancels the process-wide
token, every later call in that process answers `cancelled`; the proof
prints the reproduction as its `note` line. The CLI exits before it
matters, and a long-lived host — a kernel that catches the interrupt, a
service embedding the extension — is poisoned after one Ctrl-C. A fix
means a re-armable current token; a scalar function gets no statement
hook from DuckDB, so *when a fresh statement begins* is a real design
choice. The lane left the behavior measured and deliberately unasserted
and asks the architect for the fix shape rather than inventing one.

**PostgreSQL's credential refuses loudly, proven.** The ruled setting is
`thinkthen.api_key`; this engine build takes no key from the host (the
stand-in reads no key, and which channel delivers the setting to the
send is the database ADR's open question 4). A set, non-blank value now
refuses every call with a usage error naming the setting and the
substitute channel — never silently ignored, never echoing the value.
`check.sh` proves both arms in a third disposable container
(`laneb-pg-key`, its engine pointed at a free port nothing listens on, a
made-up value, no real key):

- `SET thinkthen.api_key = 'made-up-not-a-key'` then a call → SQLSTATE
  `22023`, the setting and `THINKTHEN_API_KEY` named, the value absent
  from the psql output and the server log;
- `RESET` then the same call → SQLSTATE `38000` with `the address
  refused the connection` — the control proving the refusal above is the
  setting's, and the missing-credential case answering loudly.

Two fixes found on the way, both in `7c86878`: the PostgreSQL message
shape (`surface()` formatted the contract's `Display`, which already
carries `usage: `, so every error read `thinkthen usage: usage: ...`;
now one prefix like the other two surfaces, pinned by
`the_kind_word_appears_once`), and the first refusal attempt's placement
inside `engine()`, which the batch closures call on their worker thread —
where `GucSetting::get` checks the active thread and panics. Cases 19,
69, 70, and 72 caught it as `the batch thread stopped`; the closures now
take the engine reference before `run_batch` spawns, so no worker thread
reads a setting, and the slice is back to 73 of 74 with the known case
17 divergence.

### The three lists — the databases lane

**Fixed on branch**

1. SQLite's hidden counter reset removed — `7fbb568`;
   `tests/null_suite.py` (23 of 23) and `tests/conformance_driver.py`
   case 17 (deltas; `after_reset` superseded in place).
2. SQLite's volatility proven and its authority named — `34080d8`;
   `check.sh`'s volatile section (`flagged|0`, `known|8`, the index
   expression's refusal).
3. DuckDB's host-SIGINT coexistence proven and its authority named —
   `98c5e81`; `tools/host_signal.py`, both arms green, run by
   `check.sh`.
4. PostgreSQL's configured credential refuses loudly and its authority
   named — `7c86878`; `check.sh`'s credential arm (22023 naming the
   setting and the channel; reset → 38000 refused address) plus
   `a_configured_key_refuses_but_a_blank_one_passes`.
5. PostgreSQL's doubled kind word in every error message — `7c86878`;
   `the_kind_word_appears_once`.
6. PostgreSQL's batch paths back on their feet after the refusal's first
   placement — `7c86878`; conformance cases 19, 69, 70, and 72 green
   again.

**Waiting for a named engine capability**

1. A key channel from the host. The engine takes no key input, so the
   ruled `thinkthen.api_key` setting refuses rather than delivering. When
   the engine accepts a key (or the database ADR answers open question
   4), the refusal becomes delivery.
2. A re-armable DuckDB cancel token, or a statement hook to re-arm on.
   The surface owns the token but cannot see a statement boundary, so the
   architect's ruling names the fix shape; until then the one-shot
   behavior is measured, documented, and printed by the proof.

**Still blocked**

1. Nothing in this lane's remit is blocked on another lane. The proofs
   run offline on loopback only; the batch forms' wire paths were
   exercised offline through the null backend, and each surface's wire
   suite skipped by design because this lane started no stub. The
   stand-in's `cache_answers` gap (case 17) remains the engine's to fix,
   not this lane's.
