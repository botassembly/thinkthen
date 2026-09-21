# NOTES

Running log. Commands and output as they happened.

## 2026-09-21 — the DuckDB surface lands

The recipe is experiment 207's, read from `experiments/207-thinkthen-db/duckdb/`: the extension template on the `duckdb` crate `~1.10505.0` with `loadable-extension`, `vscalar`, and `vscalar-arrow`; `TARGET_DUCKDB_VERSION=v1.5.5` in the Makefile (the version-pin trap: the machine CLI is v1.1.3 and cannot load what the unstable C API builds). The stock v1.5.5 CLI fetched at user level into this folder (removal: `rm -rf duckdb-bin`; nothing outside the folder, no rc change):

```
$ curl -sSL -o duckdb.zip https://github.com/duckdb/duckdb/releases/download/v1.5.5/duckdb_cli-linux-amd64.zip
$ ./duckdb-bin/duckdb --version
v1.5.5 (Variegata) d8cdaa33fd
```

`make configure && make release` produced `build/release/thinkthen.duckdb_extension` (4.7 MB). The crate binds `thinkthen-contract` and `thinkthen-standin` by path and calls only the contract's doors; `crates/`, `contract/`, and `standin/` untouched.

**First smoke, null backend:**

```
$ ENGINE_NULL=1 ./duckdb-bin/duckdb -unsigned -c "LOAD '...'; SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today') AS yes, thinkthen_decide('Is this a complaint?', 'thanks for the help') AS no, thinkthen_probability('Is this a complaint?', 'I demand a refund today');"
│ true    │ false   │   0.97 │
```

**Two bugs the suites caught.** The details struct's string children leaked stale memory into unwritten rows (`'level': Is this a complaint?hipping label`) — the write path set no NULL where a field did not apply; every child now writes or nulls its row. And `thinkthen_details` over a score question failed outright ("the answer carries no probability"), because a score reply carries a level distribution and no yes-probability: the audit door is decide-shaped. The ruled shape taken: one struct for every verb, probability/answer/sends NULL on the non-decide kinds, the question's own model and digest carrying the audit, the score's nearest level in `level`. Both fixes verified in the null suite.

**Wire, stub on 8217** (`experiments/205-thinkthen-libs/shared/target/release/stub-backend`, `STUB_PORT=8217`):

```
SELECT thinkthen_decide('@tools_fixture.json', 'I demand a refund today');  -> true   (the @file carries threshold 0.9)
SELECT thinkthen_warm('Is this a complaint?', t) FROM (SELECT unnest(['I demand a refund today','refund my shipping']) t);  -> 2
SELECT thinkthen_decide('Is this a complaint?', NULL);  -> NULL
dead address (port 9):  Invalid Input Error: thinkthen backend: io: Connection refused ... the address refused the connection
malformed evidence:     Invalid Input Error: thinkthen backend: HTTP 422: ...
```

The stub's counters after the wire suite: `{"connections":9,"max_in_flight":4,"requests":7}` — three sends for the WHERE, four for the warm, nothing extra for the repeats.

**The slide, as drawn.** `tools/run_slide.sh` extracts the DuckDB block from the marketing `surfaces.md` verbatim into `tools/slide.sql` (nothing retyped), builds the `tickets.parquet` fixture with the same CLI, and runs the block in the stock v1.5.5 CLI with the LOAD riding an init file. Offline:

```
WHERE thinkthen_decide('Is this a complaint?', body);   -> rows 1 and 2 (the refund bodies)
the choose/score statement ordered all four rows, team column NULL on every row
```

**Finding for the slide's owner, reported not hidden.** The sample's `thinkthen_choose('Which team owns this?', body, ['billing','shipping','account'])` reads NULL against the offline stand-in — its rules lift an option only when the option's own text carries a keyword, and none of the three does — and the statement cannot run on the loopback stub at all, because the stub answers one probability per request and a choice question needs a distribution (`thinkthen backend: the answer to q1 is not the shape the question asked for`). This is the recorded stub limitation behind the five shaped-to-contract conformance exchanges. The sample's shape is proven; its values await a backend that distinguishes options. Same class as the Ruby score finding the orchestrator filed in the marketing repo.

**Conformance slice, offline:** 16 ok, 3 diverge, 0 failed. The divergences, with reasons: `09-usage-filter-band` (SQL spells filter as WHERE, and a band under WHERE reads as NULL by rule 6; the refusal belongs to the surface check, not a function), `17-usage-and-cache` (the stand-in has no disk cache; the named gap the other surfaces carry too), `18-cancel-mid-batch` (the CLI's Ctrl-C owns cancel here; the SIGINT-at-LOAD chaining is the planning page's proven behavior).

**Full check, `./check.sh`:** build, 17/17 null suite, conformance slice, the slide, and the wire suite (WHERE over parquet on the wire, warm, counters) — all green with the stub up.

**Installs and removals this lane made:** the v1.5.5 CLI binary into `duckdb-bin/` (`rm -rf duckdb-bin` removes it), `make configure`'s python venv inside `configure/` (removed with `make clean_all` or `rm -rf configure`), cargo build outputs in `target/` and `build/`. No container was needed. No sudo, no key, no paid call, nothing published.

**Runtime-installer guard, checked after the CLI fetch and again at close:**

```
$ grep -c deno ~/.zshrc
0
```

**What was not run (unchecked):** the community-repository signing path (unsigned local LOAD only); `thinkthen_usage` under concurrent CLI processes; the 2,048-row width bench (207's D1 stands; the chunk path here is the same grouping); macOS packaging (the rehearsal's lane owns it).

## 2026-09-21 — recognize, relations, and relate on this surface

The brief: `repos/thinkthen/sdlc/issues/2026-09-21-update-for-the-library-team-recognize-and-relate.md`, the deck's `recognize-surfaces.md` as the acceptance test, the recordings in `experiments/225-recognize-harvest-package` (through the conformance file's cases), no paid call. Three functions landed:

- **`thinkthen_recognize(body, kinds)`** — a scalar returning `LIST(STRUCT(text, kind, start, end, strength))`; `unnest()` makes rows. The deck's line runs as drawn.
- **`thinkthen_relations(body, spec)`** (beta) — a scalar returning `LIST(STRUCT(name, source_text, source_kind, target_text, target_kind, probability))` from the question file's `recognize` section.
- **`thinkthen_relate(query, rules)`** — a table function returning rows `(name, source, target, probability)`; the query's first column is the record id, the second its text; a `LIST` of rule names, or one `'@file'`/`'{...}'` entry, is the second argument. The 255-record guard rides in the contract's `relate_checked`, so the surface inherits it.

### Findings for the deck's owner, pinned with the exact errors

**1. The relate call cannot run as drawn.** The deck draws

```sql
SELECT * FROM thinkthen_relate((SELECT id, body FROM alerts), ['caused_by']);
```

The stock CLI answers:

```
Binder Error: Table function cannot contain subqueries
```

The binder refuses a subquery argument for every function but a table-in-out function (`bind_table_function.cpp`, "Only table-in-out functions can have subquery parameters"), and the stable C API registers no table-in-out function and no `TABLE`-typed parameter (checked against the v1.5.5 headers). The working call, the shape PostgreSQL's row takes, is the query as a string:

```sql
SELECT * FROM thinkthen_relate('SELECT id, body FROM alerts', ['caused_by']);
```

**2. The relations call cannot run as drawn either.** The deck draws `SELECT * FROM thinkthen_relations(body, '@names.json')`. A bare `body` has no FROM to resolve against, and the C API table functions take literal parameters only — the correlated form `FROM tickets t, thinkthen_relations(t.body, '@names.json')` answers `does not support lateral join column parameters`. The working call is the scalar list shape, the same as `thinkthen_recognize`:

```sql
SELECT t.id, unnest(thinkthen_relations(t.body, '@names.json')) AS r FROM tickets t;
```

**3. A kept connection is the engine-layer note.** The `get_database` pointer the extension access hands out points at a `DatabaseWrapper` owned by the load state (`extension_load.cpp`), so using it after init fails — the first run of the query form answered `thinkthen backend: relate could not open a connection for its query`. The surface now opens one connection at load and keeps it for the process lifetime, serialized by a mutex; the build team's engine note takes the same rule for any extension that runs a query of its own.

### Offsets, proven in DuckDB's own indexing

DuckDB counts characters, not bytes: `length('é😀x')` is 3, `'é😀x'[2:2]` is `😀`, and `'Le café 😀 Maria Chen arrived.'[11:20]` is `Maria Chen`. The recordings' code points are therefore DuckDB's own unit, and the conformance driver asserts `body[start + 1 : end] = text` for every entity of every case — the accent-and-emoji case included — not just the named one.

### The acceptance run, and the join proof

`tools/run_recognize.sh` extracts the three calls verbatim from the deck page and runs them in the stock CLI. The recognize line runs as drawn; the other two print their pinned binder errors and their working replacements. The "Names become rows" pattern runs after them: `mentions` is built from `thinkthen_recognize`, joined to `accounts` by equality, and `thinkthen_usage()` reads identical before and after the join — the stand-in replays, so nothing sends, and the join adds no request. On the real engine the recognize build is the one sending step and the join still adds nothing. The 255-record refusal prints `thinkthen usage: relate takes at most 255 records and 256 came`.

### The conformance slice

All 102 checks pass with zero failures; the divergences print with reasons: the relations half of a recognize case rides `thinkthen_relations` (the scalar is the kinds-only shape the deck draws), the per-subject relate arm is the engine's own (the ruled form is pairs and the stand-in serves it on a text collision), and the pre-existing three (filter-band, usage-and-cache, cancel).

### Vocabulary sweep

`grep -rniE "certainty|likelihood|calibrated|cutoff|gray zone"` over `src`, `tools`, `check.sh`, `README.md`: nothing. `confidence`: nothing — the vendor's field passes under details only, and this surface prints no details for these functions. `accuracy`: nothing. Relations and edges carry `probability`; names carry `strength`.

### The shape decisions this lane made, stated for review

- Ids come back from `thinkthen_relate` as text (any value renders as its text, so an integer id arrives as `1`); a join back needs one cast. Native id types would need the query executed at bind, which this lane did not do.
- `thinkthen_relations` is a scalar list, not a table function, because the C API's literal-only parameters make the correlated row form impossible.
- The kept connection serializes relate's scans through a mutex; parallel scans of one query plan wait, they do not race.

### What was not run (unchecked)

- The wire path for the three functions: the stand-in answers all three from the recordings, so no stub request is sent by them (the wire suite covers the eight verbs).
- A relate query inside a user transaction (the kept connection sees committed state only).
- More than 255 records through a *recorded* set (the guard fires before the recordings are consulted).
- The plugin-style forms, text cutting, and question counts per request: the build team's, per the brief.

## Page-section text for the manual: `relate` over a query (DuckDB)

For the build team to lift:

> ### relate over a query
>
> `relate` reads every record at once, because every pair is one pick-one question. Give it a query and the relation rules:
>
> ```sql
> SELECT * FROM thinkthen_relate('SELECT id, body FROM alerts', ['caused_by']);
> ```
>
> The query selects two columns: the record's id first, its text second. Each row comes back as `(name, source, target, probability)`. `source` and `target` carry the id values as text, so an integer id joins back with one cast: `JOIN alerts a ON a.id::VARCHAR = e.source`. The deck's version of this call drew the query as a subquery, `(SELECT id, body FROM alerts)`; DuckDB binds a subquery only for a table-in-out function, which the stable C API cannot register, so the query crosses as a string. One call takes at most 255 records; past that is a usage error, because the pair count grows with the square of the records. The planner never knows a function costs money: one call is one request for every pair.
>
> For relations as rows from a question file, the beta companion — a list of structs, so `unnest()` makes rows, the same shape `thinkthen_recognize` takes:
>
> ```sql
> SELECT t.id, unnest(thinkthen_relations(t.body, '@names.json')) AS r FROM tickets t;
> ```
>
> The deck's version drew a bare `body`; a scalar takes a column like any other function, so the call sits beside the table in the `FROM`.

## 2026-09-21 — the fix wave: ruling 2 and the nested ci-tools tree

**The defect mapping, ruling 2.** A unit test constructs the contract's defect error at the shim level and asserts this engine's error surface carries it — the `thinkthen defect:` message and the failure text. No public door gains a fault hook.

```
$ cargo test --release --quiet --lib
running 1 test
test result: ok. 1 passed; 0 failed
```

The check runs that test after the build.

**The ci-tools tree, recorded truthfully.** `extension-ci-tools/` is vendored into this branch as ordinary tracked files (committed with the surface landing, `7dd3c31`), not as a submodule and not as an unversioned sweep risk: `git ls-files` lists its files as plain blobs, which is what makes `make release` work with no network. The directory does hold a dangling `.git` pointer file (`gitdir: ../.git/modules/extension-ci-tools`) left from a clone that was never a submodule here; it is untracked, and `git -C extension-ci-tools` commands fail on it. No commit pin is recoverable from this tree, so a future re-pin should clone `https://github.com/duckdb/extension-ci-tools` cleanly, record the commit, and copy the `makefiles/` tree the Makefile includes. No `.gitignore` line was added: the files are tracked and needed by the build, and ignoring tracked files changes nothing while misleading the next reader.

**The working shape for `thinkthen_relations` on this engine:** the scalar list form, the same as `thinkthen_recognize` — `SELECT t.id, unnest(thinkthen_relations(t.body, '@names.json')) AS r FROM tickets t;`. The deck's drawn `SELECT * FROM thinkthen_relations(body, '@names.json')` cannot run here; the finding goes to the deck's owner from another lane.

The check, end to end, with the loopback stub up:

```
$ ./check.sh
== duckdb surface: the error-mapping test
== duckdb surface: wire suite against the stub on 8217
exit 0    (diverge 18-cancel-mid-batch is the stand-in's recorded real-engine requirement)
```

## Lane B items 2, 5, and 6 on the DuckDB surface, 2026-09-21

**Item 2, the new shapes.** `thinkthen_annotate` carries the ruled failed marker where a value would go, and `thinkthen_details` gained two struct members: `requests` (the ordered recording digests, 0053) and `failed_questions` (0054). The decide rows take both from the engine's own `Details`; the score/choose/tag rows compute the one-element list through the engine's own digest rule (`thinkthen_standin::request_digest` with the settings' model — the real engine's details will carry the list for every kind and the call goes away). The two conformance cases now run:

```
$ python3 tools/conformance.py | grep -E "73|74"
ok       73-details-carries-requests
ok       73-details-carries-requests requests
ok       74-annotate-preserves-good-answers
```

```
$ ENGINE_NULL=1 duckdb -unsigned -noheader -list -c "LOAD 'build/release/thinkthen.duckdb_extension';
  SELECT thinkthen_annotate('{"version":1,"questions":{"kind":{"decide":"Is this a complaint?"},"topic":{"decide":"Is this about a refund?"}}}',
    'order 4471: charged twice, please refund');"
{"kind":true,"topic":{"failed":{"cause":"missing_answer","kind":"backend"}}}
```

**Item 5, the fast-backend cancel.** Two tests, with their claims separated honestly:

- `cancel_tests::a_fast_backend_hears_a_cancel_within_a_tick` (the crate's lib tests): 8M records on the null backend at width 1, the token set at 150 ms, the call must return the cancelled kind within 1.5 s. Measured contrast, the same batch un-cancelled: **53.55 s** — so the bound cannot be met by a batch that runs to completion. The busy-arm probe (removing `maybe_tick` from the Ok arm, 2026-09-21, restored after) still returned fast, because the stand-in's workers check the token per record themselves; the caller-side tick's own contribution is the poll the host hands in, which is why the bound stays at one tick.
- `tools/cancel_fast.sh` (end to end through the CLI): SIGINT at t+1 s of a 3M-row null query; the CLI exits within ~a tick and the query never prints its count. Stated limit: DuckDB's own abort ends a native query within a chunk under the same SIGINT, so this cannot isolate the engine's tick — a CLI timing test on a fast backend passes either way. The stub-backed wire suite proves the wire-side shape; the Rust test carries the discrimination.

```
$ ./tools/cancel_fast.sh
the CLI exited 1 0.00s after SIGINT, one tick expected
ok       the interrupt ended the query within about a tick (0.00s)
ok       the query did not run to completion
```

**Item 6, the examples file.** `examples.json` is keyed by function — twelve entries, every public function on this surface — and `tools/examples.py` runs each in a fresh CLI process on the null backend and checks its answer. The site's SQL tab draws from this file.

```
$ python3 tools/examples.py
ok       decide
...       (twelve lines)
12 of 12 examples ok
```

**Fixed on the way: the acceptance script still spoke the refused spelling.** `tools/run_recognize.sh` wrote `names.json` with `from`/`to`, which ruling 1 now refuses, so the working relations replacement inside `working.sql` was failing while `check.sh` printed "the working replacements ran" from a fixed string. The fixture now says `source`/`target`, and the check asserts the real evidence rows — the relations row, the join row, and the join's zero-requests line — instead of a sentence:

```
ok       the working replacements ran with their evidence: the edges, the relations row, the mentions join, and the join's zero requests
ok       the 255-record refusal is in the log
```

**The record row, adopted.** The conformance slice now asserts the ruled `{"input","value"}` row on the bulk forms: the value-printing projections read back as `input|value` pairs in input order — `filter`'s kept rows and `decide_many`'s answers where the case carries them (`05`, `19`; case `06`'s empty list is covered by its count). SQL's own two columns are the row; no new function was added.

```
ok       05-filter-keeps-some-of-five rows
ok       19-decide-many-judgments rows
```
