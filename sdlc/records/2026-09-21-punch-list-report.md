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
