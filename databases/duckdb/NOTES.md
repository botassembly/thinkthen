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

**Wire, stub on 8217** (the in-repo `tools/wire-stub` binary, `STUB_PORT=8217`; at the time of this run the stub lived in `experiments/205-thinkthen-libs/shared`):

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

## 2026-09-21 — lane B item 8: the scalar-bind road, found and proven

The issue asked whether a road exists around the broken bind surface while staying on the stable C API. The answer, with a runtime probe behind it: **the road exists in the unstable C API this extension already compiles against, and it works at runtime on v1.5.5. Our pinned Rust binding cannot reach it today; three unlock paths are named below. The first-row check stays the working road meanwhile.**

### What the headers actually expose

Read from the vendored source under the extension's build inputs (`libduckdb-sys 1.10505.0/duckdb.tar.gz`, the DuckDB v1.5.5 tree):

- `duckdb_extension.h:723-732`, the unstable block "New functions around scalar function binding": `duckdb_scalar_function_set_bind` (723), `duckdb_scalar_function_bind_set_error` (724), `duckdb_scalar_function_get_client_context` (725), `duckdb_scalar_function_bind_get_extra_info` (729), **`duckdb_scalar_function_bind_get_argument_count` (730)**, **`duckdb_scalar_function_bind_get_argument` (731)**.
- `duckdb.h:5534-5545` (stable header): `duckdb_expression_is_foldable` and `duckdb_expression_fold(context, expr, out_value)`.
- `duckdb/src/main/capi/scalar_function-c.cpp:62-80` — `CScalarFunctionInternalBindInfo` holds the bind `arguments`; `:128-133` the cast helpers; `:160-172` `CScalarFunctionBind` runs the callback at plan time and a set error becomes a `BinderException`.
- `duckdb/src/main/capi/table_function-c.cpp:348-357` — where the generic `duckdb_bind_get_parameter_count`/`_get_parameter` live; they cast the info to the **table-function** bind info, which is the different layout 207 hit as `-7` and segfaults. The 207 verdict was about that family; the scalar-specific accessors were not tried.

### The runtime probe, verbatim

A temporary raw-ffi scalar, `thinkthen_bindprobe(question)`, registered from `src/bindprobe.rs` (removed after the run): its bind callback read the argument through the scalar accessors, folded it through the client context, and refused a question without the word "refund" via `duckdb_scalar_function_bind_set_error`; its invoke wrote `ok` and read nothing.

```
$ ./duckdb-bin/duckdb -unsigned :memory: -c "LOAD '...'; SELECT thinkthen_bindprobe('Does the customer ask for a refund?');"
 ok
$ ./duckdb-bin/duckdb -unsigned :memory: -c "LOAD '...'; SELECT thinkthen_bindprobe('Is this a complaint?') FROM range(0);"
Binder Error: probe: the question must mention a refund; got "Is this a complaint?"
$ ... CREATE TABLE q AS SELECT 'Is this a complaint?' AS t; SELECT thinkthen_bindprobe(t) FROM q;
 ok
```

Zero rows proves plan time: no row executed, and the error is ours, raised through the scalar error setter. The column case proves the non-foldable path: the check skips and the invoke runs.

### Why our extension cannot attach it today

`duckdb-rs 1.10505.0` (the pinned crate): the `VScalar` trait has no bind hook and `Connection::register_scalar_function` builds its `ScalarFunctionSet` internally (`src/vscalar/mod.rs:195-233`); `ScalarFunction` keeps its pointer private with no `set_bind` and `ScalarFunctionSet::register_with_connection` is `pub(crate)` (`src/vscalar/function.rs:30,43-45`); and `DataChunkHandle::new_unowned` is `pub(crate)` (`src/core/data_chunk.rs:43`), so a self-written trampoline cannot wrap the callback's chunk to reuse the existing invoke bodies. The loadable bindings themselves carry every accessor (`libduckdb-sys .../bindgen_bundled_version_loadable.rs:11280-11416, 10614-10670`), so the gap is the crate's surface, not the C API.

### The unlock paths, with costs

1. **Patch or vendor the crate (smallest):** add `ScalarFunction::set_bind` and one registration variant that takes the bind callback (~20 lines), then the extension's bind callback calls the ffi accessors directly. Cost: a forked dependency to carry per release. The probe above is the acceptance test for the patch.
2. **Upstream it to duckdb-rs:** the same patch, once. Nobody opens anything upstream without Ian's word.
3. **The C++ API fork** already before the build team (the issue's sharpened section).

Until one lands, the first-row check remains: zero requests wasted, reported one row late, proven on the stub.

## 2026-09-21, the settlement wiring (contract 1fe8173)

- **`nearest` everywhere it is one name.** The details struct's score member is `nearest` (was `level`), the annotate JSON's score key is `nearest` (was `level`), and the README's `thinkthen_details.nearest` matches — the contract's settled field, no private spelling left in this door. The null suite pins it: `details score nearest` reads `mid`.
- **The warm poison clears when raised.** `POISON` now `.remove`s the entry at the raise site, so one failed warm costs one raised read and the pair is retried on the next call instead of failing forever; the doc comment says so. The null suite proves the whole arc in one piped process: the failed warm returns 0, the first read raises `thinkthen backend`, the second read answers `true` — `poison raises once` and `poison clears and the pair answers`, both green.
- **The beta relations columns carry the ruled end names.** `thinkthen_relations` returns `(name, source, source_kind, target, target_kind, probability)`; `source` and `target` hold the entity's *text* — documented on the rows and in the README — and the kind columns hold the kind words. The example and the acceptance's working replacements were updated with it.
- **The deadline gap is named on the page.** The README's ruled-shape section now says there is no per-call deadline option on this surface yet, the host's own statement timeout is the stop, and conformance case 27 is skipped for that reason — the settled shape is a per-call budget beside the cancel token, recorded as a gap rather than implied.

The full check is green with no stub: build, the error-mapping test, the null suite (including the two poison proofs), the fast-backend cancel, the function examples, the conformance slice (74 cases), the slide, and the recognize acceptance with its pinned divergences.

## 2026-09-22 — the host-SIGINT proof and the authority section (punch-list item 5)

Item 5: prove the LOAD-time signal handler coexists with its host, and
name the extension's authority.

**The proof.** `tools/host_signal.py`, run by `check.sh` through the
build's own venv Python (3.12.3, duckdb 1.5.5 — the CLI's exact
version), two child processes so one arm's signal cannot color the
other's evidence:

```
host: Python 3.12.3 with duckdb 1.5.5; the extension is built for v1.5.5
-- host SIGINT, the after arm
ok       the host's handler took SIGINT after LOAD and the extension still answered true
-- host SIGINT, the chain arm
         the query ended 1.00s after the signal with: Invalid Input Error: thinkthen cancelled: the wait was cancelled
ok       a signal stopped the running query and reached the host's chained handler
note     after the cancelled statement the next call answered: Invalid Input Error: thinkthen cancelled: the wait was cancelled
```

The `after` arm is the job-2 shape the punch list names: the host
installs its own SIGINT handler after LOAD, the handler fires, the
process survives, and the extension keeps answering. The `chain` arm
(the default Python shape, and the CLI's) proves the extension's handler
stops a 3-million-record query within one tick and reaches the host's
handler through the chain.

**Found, not fixed: the token is one shot per process.** The `note` line
is a real defect, measured and deliberately not asserted: once the
extension's own handler cancels the process-wide token, every later call
in that process answers `cancelled`. The CLI exits before it matters; a
long-lived host (a kernel that catches the interrupt, a service embedding
the extension) is poisoned after one Ctrl-C. A fix means a re-armable
current token — surface-local, but with a real choice about when a fresh
statement begins, and a scalar function gets no statement hook from
DuckDB, so the fix shape belongs to the architect. The reproduction is
this `note` line; the finding is in the punch-list report.

**README.** A new "Authority: who may do what" section names the six
items, including the one-shot token.

Cleanup: no containers; `docker ps -a` shows no `laneb-*` left. No key,
no paid call, nothing published.

## 2026-09-22 — the review fix wave on the DuckDB surface

The review's findings 1, 2, 3, 4, and 5 for this surface, each with the
test that failed before it. Commands and their outputs, not
recollections.

**Finding 1, the row mapping (wrong answers with no error).** `decide`,
`probability`, and `score` wrote the k-th distinct text's answer to the
k-th row; with repeated texts or a NULL, most rows read a neighbor's
answer or a zero. The fix indexes every row through its own slot
(`distinct.slots`), the way `choose`, `tag`, `details`, and `annotate`
already did.

```
$ ENGINE_NULL=1 duckdb-bin/duckdb -unsigned -noheader -list -c "LOAD …;
    SELECT count(*) FILTER (WHERE thinkthen_decide('Is this a complaint?', t))
        || '|' || count(*) FILTER (WHERE NOT thinkthen_decide('Is this a complaint?', t))
    FROM (SELECT CASE WHEN i % 2 = 0 THEN 'I demand a refund today'
                      ELSE 'thanks for the help' END AS t FROM range(10000) r(i));"
1|9        # the two distinct texts answered rows 0 and 1; 10,000 rows
           # returned 5 true and 9,995 false

$ tools/mapping_suite.sh      # after the fix
ok  decide alternating 10,000 rows split exactly (5000|5000)
ok  decide one text repeated answers every row (4|0)
ok  decide NULL rows stay NULL and consume no answer (2|1|2)
ok  decide three-text cycle keeps every row's answer (6|3|0)
ok  probability one text repeated answers every row (4|0)
ok  probability NULL row stays NULL beside a repeat (2|1)
ok  score one text repeated answers every row (4|0)
ok  score two texts alternating answer per row (3|0)
ok  choose repeated text answers every row (4)         # regressions
ok  tag repeated text answers every row (4)
ok  details repeated text answers every row (3|1)
ok  annotate repeated text answers every row (3|1)

$ git stash && make release && tools/mapping_suite.sh   # old code
FAILED   decide alternating 10,000 rows split exactly: want '5000|5000', got '1|9'
```

**Finding 2, the driver could not fail.** `tools/conformance.py` printed
`FAILED` and returned 0, compared with `want in got` (so a long rendered
box containing the word `true` passed), ignored a path argument, and ran
the SQL through `-init`, which renders the interactive boxes. Now: exact
comparison, the argument is the cases file, and any FAILED line returns
1. Error cases go through `check_error`, which requires the output to be
an error whose text starts with the expected kind.

```
$ python3 tools/conformance.py          # 107 checks, exact
ok       74-annotate-preserves-good-answers
$ echo $?
0

$ tools/conformance_selftest.sh
corrupted 01-decide-yes-cut
ok       the corrupted expectation fails the driver (exit 1)
ok       the real conformance file passes from the repository root

$ git stash && python3 tools/conformance.py /dev/null; echo $?
0        # the old driver ignored the argument and read the good file
```

Case 74's failed marker rides the stand-in's `ENGINE_SYNTHETIC_PARTIAL`
opt-in; the driver arms it from the case's own expectation (a `failed`
member), not from a case id.

**Finding 3, `@file` reads ignored `enable_external_access`.** Every
`@`-file door now refuses before touching the disk when the database's
own setting is off; the switch is read through
`duckdb_client_context_get_config_option` (unstable C API, which the
extension declares via `USE_UNSTABLE_C_API=1` / `C_STRUCT_UNSTABLE`).
Scalars read every loaded database's setting and refuse if any forbids.
Relate reads the caller's own context at bind.

```
$ ENGINE_NULL=1 duckdb -c "LOAD …; SET enable_external_access=false;
    SELECT thinkthen_decide('@tools/null-cut.json', 'I demand a refund today');"
Invalid Input Error: thinkthen local: the question file …/null-cut.json
was not read: enable_external_access is off for this database
# before the fix the same call read the file and answered true

$ tools/security_suite.sh
ok  decide @file refuses with access off
ok  annotate @set refuses with access off
ok  relate @rules file refuses with access off
ok  decide @file answers with access on
ok  plain text answers with access off
ok  relate names the temporary-table boundary
ok  a missing non-temporary table keeps the raw error
```

**Finding 3, relate and the caller's connection — ruled option A.** The
stable C API cannot reach the caller's connection (`duckdb_query` needs a
`duckdb_connection`; a client context yields catalogs, config, and the
file system only, and temp tables live in per-connection `ClientData` —
verified in `client_data.hpp` and live: a temp table returns
`Catalog Error: Table with name tt does not exist`). What is fixed: the
scan runs on a connection belonging to the CALLER'S DATABASE, resolved at
bind through `duckdb_client_context_get_catalog` against a per-database
registry built at init, so one process-global connection no longer serves
whichever database loaded last. When the query misses the caller's
temporary table, the error names that boundary in words.

```
$ configure/venv/bin/python tools/two_databases.py
a.db: 4 edges (want 4), b.db: 11 edges (want 11)
ok       each database's relate ran on its own connection
# before the fix: "no recorded answer for the rule caused_by on these
# records; the recording covers founded, works_for" - a.db's query ran
# against b.db's table

$ ENGINE_NULL=1 duckdb -c "…CREATE TEMP TABLE tt…; SELECT * FROM
    thinkthen_relate('SELECT id, body FROM tt', ['caused_by']);"
Invalid Input Error: thinkthen local: the relate query names the
temporary table tt, and the stable C API cannot run a query on the
calling connection, so relate cannot see temporary tables; materialize
it (CREATE TABLE ... AS SELECT) or run the query directly
```

MERGE-NOTE material for the build team: the boundary this leaves is
exactly the one the C++ door would lift. Temp-table and open-transaction
visibility for relate is impossible through the stable C API; the C++
fork under their consideration (the duckdb-rs bind-callback question)
is what removes it. Same-name databases in one process cannot be told
apart through the client context and are refused as a defect rather than
guessed.

**Finding 5, panics across the C boundary.** The entrypoint and every
hand-written callback (relate's bind, init, scan, its destructors;
warm's update, combine, finalize, state lifecycle; usage's bind, init,
scan, destructor) now run inside `guard` containment, where a panic
becomes the callback's error (or a logged quiet containment where no
error channel exists). The scalar functions were already contained by
duckdb-rs's registration helpers. The `ENGINE_TEST_PANIC` door arms one
boundary for the check alone.

```
$ tools/panic_suite.sh
ok  the extension load: the panic became the callback's error (exit 1)
ok  relate bind: the panic became the callback's error (exit 1)
ok  usage scan: the panic became the callback's error (exit 1)
ok  the arm is dormant when unset

# with the containment disabled the same arm dies at the boundary:
thread caused non-unwinding panic. aborting.
```

**Group 4, the one-shot interrupt token — fixed at last.** The finding
recorded above ("the token is one shot per process") is closed. The
surface now keeps a live token the handler cancels only when a query is
plausibly running (an engine call in flight, or one started within
250 ms), and a call that starts on a cancelled token re-arms when the
interrupt has been served (a call returned the cancelled kind) or when
the last call is stale. An idle Ctrl-C is the host's gesture and no
longer poisons anything.

```
$ configure/venv/bin/python tools/rearm_suite.py
the interrupted call ended with RuntimeError: Query interrupted
ok       the interrupt stopped one query and the next query answered
# before the fix: "FAILED   the query after the interrupt errored, so the
# interrupt poisoned the process: … thinkthen cancelled: the wait was
# cancelled"

$ ENGINE_NULL=1 configure/venv/bin/python tools/host_signal.py chain …
the query ended 1.00s after the signal with: … thinkthen cancelled: the wait was cancelled
ok       a signal stopped the running query and reached the host's chained handler
note     after the cancelled statement the next call answered: True
# the note was "…answered: … thinkthen cancelled…" before the fix
```

Residual, named: a signal landing while the engine is idle but within
250 ms of the last call still counts as aimed at that query, so a query
started inside that window is interrupted once. The window is four
orders of magnitude above the chunk cadence and the alternative (no
window) leaves the chained-host query unstoppable, which the `chain`
arm proved.

**Phase 1 adoption.** Construction now goes through the contract's
connector (`StandinConnector` behind `EngineConfig`), so pointing this
surface at the real engine is the connector dependency line;
`engine_call`/`with_engine` hold the in-flight count for the interrupt
window. No deadline door exists on this surface, so the checked
conversion has no call site here.

**The deck dependency is gone from the check.** `tools/run_recognize.sh`
read the drawn calls out of the private deck repo; it now reads
`tools/drawn-calls/recognize.sql`, a vendored copy of the three drawn
lines with the deck named as the source of truth. `check.sh` passes with
no private deck on disk. `README.md` still names the deck path; that
line belongs to the hygiene lane's sweep.

`check.sh` runs the whole set: build, unit tests, null suite, mapping
suite, security suite, conformance driver and its selftest, cancel, host
signal, examples, re-arm, two databases, panic guards, the slide, the
recognize acceptance, and the wire suite when a stub is up (skipped
offline, and said so).

## 2026-09-22 — the second review wave on the DuckDB surface

The second review's DuckDB items, fixed on this surface only, with the
commands that prove each one. Every new test was run against the built
extension (null backend, offline, no key); the gate is `check.sh`, and
its last run on this wave ends `207 ok`, no `FAILED`, exit 0.

**Relate reads records and nothing else (review items 1 and 2).** The
query is refused when it holds more than one statement
(`duckdb_extract_statements` counts them), and it runs inside
`BEGIN TRANSACTION READ ONLY`, rolled back either way, so no statement
it holds can write and nothing it does commits beside the caller's own
transaction. The review's exact scenario — a DELETE inside the relate
query surviving the caller's ROLLBACK — is the first test in
`tools/relate_guard_suite.sh`:

```
ok       relate refuses a DELETE inside its query
ok       the DELETE never happened, the caller's ROLLBACK included
```

The refusal is DuckDB's own: `Cannot write to database "memory" -
transaction is launched in read-only mode`. A second statement gets
`the relate query is one SQL statement and this one holds 2`.

**Routing by database identity (review item 13).** Names cannot tell two
databases apart: two in-memory databases both report `memory`, and the
first fix's name probing resolved exactly one match and failed both. Each
load now registers an extension setting on its own database,
`thinkthen_instance_token`, whose default value is unique to the load
(process, serial, clock); a bind reads the caller's own token through
`duckdb_client_context_get_config_option`, so the caller's database
answers with its own identity and no name is consulted. Names remain the
fallback when the setting could not register. `tools/two_databases.py`
now proves four shapes: two file databases, two in-memory databases, the
`ATTACH ... USE other` search path (the relate query runs under the
caller's own `search_path` setting, read at scan time), and the release
below:

```
ok       two in-memory databases each relate on their own connection
ok       USE other reached the relate query: it read the attached table
ok       leaving USE behind reads the main table again
```

**The kept connection closes with its database (review item 13, third
part).** A kept connection holds the database instance alive, and with it
the file's lock: after the caller closed every connection, the file
stayed locked and an in-process reopen hung. A reaper thread now watches
each kept connection's own `duckdb_connection_count()` and, when the kept
connection is the only one left, disconnects it and forgets the entry;
the query gate serializes its count and its disconnect against every
other use. The reaper's first cut wrapped the function in `count(*)` and
read the function's one row as a count of one, so every entry was reaped
on the first tick; the count now reads the row's value, and the gate
caught the mistake immediately (`two_databases.py` failed every
in-memory case). Both halves are pinned:

```
ok       the closed database's file reopened in-process
ok       another process opened the closed database's file
```

**The file-access door everywhere a file opens (review item 3).**
`thinkthen_relations` now takes the same door as the other scalars; the
door honors `allowed_directories` and `allowed_paths` the way
`DBConfig::CanAccessFile` does (external access off, then the carve-outs),
and the read itself goes through DuckDB's own file system
(`duckdb_client_context_get_file_system` + `duckdb_file_system_open`), so
`disabled_filesystems` refuses with DuckDB's own words — its state lives
in the file system, not in a readable setting, which is why the setting
read alone could not see it. A prepared relate re-checks at execution:

```
ok       allowed_directories admits the file it names
ok       allowed_directories refuses the file it does not name
ok       disabled_filesystems refuses a local read
ok       relations @file refuses with access off
ok       a prepared relate re-checks file access at execution
```

The refusal text for the plain switched-off case keeps its old wording
plus the carve-out clause; `security_suite.sh`'s relate expectation was
updated to the unified message.

**Nested relate refuses instead of hanging (review item 6).** The inner
call used to block forever on the gate its own outer query holds. DuckDB
runs the inner query's callbacks on its own thread with the kept
connection's own client context, so the discriminator is the connection
id: the caller's id equals the kept connection's id exactly when the
callback runs inside a query that connection is executing. The
thread-local mark I tried first did not fire (the inner call arrived on
another thread, which the debug run showed before it was removed), and
the connection-id comparison does:

```
ok       a relate inside a relate refuses instead of hanging
```

**The interrupt boundary (review item 14, first half).** The 250 ms
window cancelled the next query when a signal landed near a finished
call (four runs in five). The rules now: the handler cancels while an
engine call is in flight or while calls arrive in a burst (two call
starts within `BURST_MS`, 10 ms, and the latest within it); a call takes
a cancelled token while another call runs with it or the burst is
active; a call that returned the cancelled kind marks the token spent,
and the last call out clears it once the burst is over. Ten rounds of
signal-after-a-lone-query all leave the next query answering, and the
host-SIGINT chain arm still stops a running chunked query (1.00 s after
the signal):

```
ok       ten interrupts near calls left the next query answering
ok       a signal stopped the running query and reached the host's chained handler
```

Residual, named and unfixable with the calls the surface can see: a
signal landing in the gap between a chunked query's last call and its
return, when the next call already belongs to the next query, still
cancels that next query. The C API exposes no per-query hook for scalar
functions — `VScalar` carries only `invoke`, and the function-info
accessors are `extra_info`, `bind_data`, `init_data`, and
`local_init_data` — so the surface cannot tell "the next chunk of this
query" from "the first chunk of the next one". The burst window is as
close as call timings come; `tools/rearm_suite.py`'s docstring pins it.

**DuckDB's own cancel (review item 14, second half).** `con.interrupt()`
ends a relate or scalar query at DuckDB's next boundary, and the surface
stays clean for the next query (`ok DuckDB's own interrupt ended the
query (InterruptException) and the next answered`). It cannot reach
*inside* an engine call: the C API has no progress or interrupt hook a
table or scalar function can observe (`duckdb_table_function_set_progress`
does not exist; the API's interrupt surface is `duckdb_interrupt`, which
sets a connection's own flag). The engine call itself stops on the SIGINT
token and on deadlines, which is the whole of what the door allows.

**Warm judges under the right question, and errors are SQL errors
(review item 6 of the leftovers).** The aggregate binds the question the
first row carries instead of following the last one seen, and a group
carrying more than one question refuses. Failures now raise through
`duckdb_aggregate_function_set_error` — the channel the old poison map
existed to work around — so the warm query itself hears them; the poison
map, its scalar-side consumer, and the `counts as zero` path are gone.
The null suite's poison block became a `warm failure raises as the warm's
own error` proof, plus three new cases:

```
ok       warm judges the question the rows carry
ok       warm grouped by question judges each group
ok       warm refuses a group carrying two questions
```

**One question-file read per query (review item 7).** `@file` arguments
were resolved per row, so a 20,000-row query opened the file 20,000
times. Resolved questions are now cached by canonical path and
modification time, so a query reads the file once and an edited file is
read again. `tools/atfile_suite.sh` counts the opens with strace:

```
ok       one question file open for twenty thousand rows
ok       an edited question file is read again
```

The first cut of that test measured nothing: `count(*)` over the
projection let DuckDB drop the unused scalar, so the file was never read.
The query must use the answer (`sum(...)`), which the suite's comment
now says.

**The stand-in is named only at the connector line (review item 8).**
`thinkthen_standin::request_digest` is gone: a non-decide details row
takes model, digest, nearest, requests, and failed_questions from the
engine's own `details_opts`, the contract door the decide branch already
used. A choose or tag reply has no engine details yet, so its details
call now reports the engine's own refusal (`the answer carries no
probability`, the kind `backend`) instead of a fabricated audit row; the
struct's doc comment says so. `check.sh` proves the surface keeps exactly
one stand-in reference:

```
ok       the only stand-in reference is the connector import
```

**Conformance-driver updates the other lanes' changes required.**
Annotate fields come back in the set's own file order now, so the driver
builds its expectation in that order, not sorted; the failed marker
serializes `{"kind":...,"cause":...}`; and the stand-in's partial-failure
opt-in is compile-time, so `check.sh` builds a second, fixture-armed
extension beside the default one (the C surface's pattern) and the driver
loads it for the one case that replays the marker. `null_suite.sh`,
`mapping_suite.sh`, and `examples.json` carried the same stale annotate
order and now carry the file order.

## The third review's DuckDB findings, fixed with fail-then-pass probes

The wave's source is `sdlc/issues/2026-09-22-surfaces-branch-third-review-the-unheld-fixes.md`
on main. The probes live in `tools/review3_duckdb.py` (run by `check.sh`),
`tools/atfile_suite.sh`, and the updated suites below. The fail-first
round built the review's own tip `37240fd` as the baseline extension and
ran the same probes against it; the outputs are pasted as they ran.

### The baseline: eleven of fourteen probes fail on 37240fd

```
$ cp /tmp/duckdb-prefix/thinkthen.duckdb_extension build/release/   # the 37240fd build
$ ./configure/venv/bin/python tools/review3_duckdb.py
FAILED   the identity token is beyond SQL: SET thinkthen_instance_token: no error raised
FAILED   the identity token is beyond SQL: SET GLOBAL: no error raised
FAILED   relate refuses COPY TO: ... the relate query must return the id and the text as its first two columns
FAILED   relate refuses EXPORT DATABASE: ... the relate query must return the id and the text as its first two columns
FAILED   relate refuses ATTACH: ... the relate query must return the id and the text as its first two columns
FAILED   relate refuses SET GLOBAL: ... the relate query failed
FAILED   relate refuses SET VARIABLE: ... the relate query must return the id and the text as its first two columns
FAILED   the 8-million-row cap names the count (refused in 4.4s): relate takes at most 255 records and 8000000 came
FAILED   a score's details read NULL probability and sends: (0.0, 0, 'mid')
FAILED   fifty idle databases burn under 5% of a core (measured 8.8%)
FAILED   a host that ignored SIGINT keeps its ignore (the kernel's mask): REPLACED ignored-before
== review3 duckdb suite: 11 failed
```

Reading the baseline: COPY TO, EXPORT DATABASE, ATTACH, SET GLOBAL, and
SET VARIABLE all RAN (the "must return the id and text" refusal is the
post-execution shape — the statements executed and returned no columns,
and the COPY wrote its file); the 8-million-row refusal came from the
engine's own cap after every row was read (4.4 s); a score's details
carried the buffer's zeros; fifty idle databases burned 8.8% of a core
(the review's 500 measured 64%).

### The fix: fourteen green on the wave's commit

```
$ make release && ./configure/venv/bin/python tools/review3_duckdb.py
ok       the identity token is beyond SQL: SET thinkthen_instance_token
ok       the identity token is beyond SQL: SET GLOBAL
ok       relate refuses COPY TO
ok       relate refuses EXPORT DATABASE
ok       relate refuses ATTACH
ok       relate refuses SET GLOBAL
ok       relate refuses SET VARIABLE
ok       the 8-million-row cap names the count (refused in 0.0s)
ok       a score's details read NULL probability and sends, and its nearest level
ok       a closed database's file is released after the reaper's pass
ok       a fresh LOAD after the guarded release answers relate
ok       fifty idle databases burn under 5% of a core (measured 0.8%)
ok       a host that ignored SIGINT keeps its ignore (the kernel's mask)
== review3 duckdb suite: green
```

### What each fix is

- **1 (the use-after-free):** every handle the registry gives out is a
  counted `KeptGuard` (`connections.rs`); the reaper retires a database
  before counting, refuses new guards on a retired one, disconnects only
  at zero guards under the gate, and a guard dropping on a retired
  database pokes the reaper so the release lands promptly. The second
  window (`kept_context`) takes a guard too. The reviewer's C-API
  prepare→free→execute segfault is structurally closed; the bounded
  probes here prove release-with-prepared-statements and the fresh LOAD
  after a guarded release, not the segfault itself (that needs the
  reviewer's exact C-API harness).
- **6 (the token):** the token setting is gone. Identity is a uniquely
  named in-memory database ATTACHed to the caller's instance at LOAD;
  the caller's context resolves it only inside its own instance. The
  SET and SET GLOBAL attacks error as unknown settings; the `''`
  fallback no longer exists because there is no setting to blank.
- **7 (SELECT-only):** the prepared statement's own kind is checked
  (`duckdb_prepared_statement_type`) before anything runs, so the
  single-statement writes are refused as the writes they are; the
  read-only transaction stays as the belt.
- **11 (details):** the probability and sends children set NULL before
  the values write (one wrapper at a time — the two-wrapper rule the
  crate documents); a choose or tag question answers a NULL row instead
  of raising the engine's refusal.
- **12 (the cap):** `SELECT count(*) FROM (<query>)` runs first; the
  refusal names the query's own count before any row is read, and a
  belt wall stops collection at 256. The locks are per database, so one
  database's long relate cannot block another's.
- **13 (signals):** `sigaction` with `SA_SIGINFO`; the host action is
  kept and chained with the signature its flags name, `SIG_DFL` is
  restored and re-raised, `SIG_IGN` never installs (proven against the
  kernel's ignore mask). A call starting within the interrupt's window
  serves the cancelled token, so one Ctrl-C stops every query the
  signal found running. `con.interrupt()` still cannot reach a
  synchronous table-function scan — that boundary is the deadline's to
  bound (the scalars' new third argument), named in the README.
- **26 (the reaper):** budgeted passes (at most eight entries) with a
  wrapping cursor, try-lock gates, and a sleep that backs off to two
  seconds while nothing is released. Fifty idle databases measure 0.8%
  of a core (the baseline's 8.8%).
- **27 (per-row file opens):** one mtime-keyed text cache
  (`question_text_cached`) serves annotate and relations; the at-file
  suite counts one open for two thousand rows on each reader beside
  decide's twenty thousand.
- **Adoptions:** the contract's `panic_text` and `catch_panic` own the
  panic-to-text (both local copies deleted); `conformance.py` reads the
  skip table through `conformance/skiptable.py` (the private parser is
  gone); the DuckDB pin is one `tools/version.env` sourced by every
  script; `package.sh` follows the host's arch; the at-file suite
  generates its own fixture (no gitignored `null-cut.json` dependency).

### The suites after the wave

```
$ tools/atfile_suite.sh
ok       one question file open for twenty thousand decide rows
ok       one question file open for two thousand annotate rows
ok       one question file open for two thousand relations rows
ok       an edited question file is read again
$ ENGINE_NULL=1 tools/null_suite.sh            # 23 ok
$ ./configure/venv/bin/python tools/two_databases.py   # 6 ok
$ ENGINE_FIXTURE_EXTENSION=... tools/conformance.py    # 117 ok, 0 failed
$ tools/panic_suite.sh                         # 4 ok, exit 0
$ tools/security_suite.sh                      # 11 ok, exit 0
$ tools/relate_guard_suite.sh                  # 5 ok, exit 0
```

The relation spec's own validation caught a bug this wave introduced on
the way: the empty budget slice starved the relations zip and the
function answered `[]` for everything; the at-file suite caught it
(zero file opens, a `not-a-spec` string answering instead of erroring)
before it could land.

## The fourth review's DuckDB wave, closed with its probes (2026-09-23)

Items 4, 5, 6, 8, 9 and the first half of 13 are fixed, each with a
probe that failed on the reviewed tip (`37240fd`) and passes on this
tree — `tools/review4_fastpath.py` (the reaped database's caller read
the next database's table; now the reload refusal), `tools/review4_forge.py`
(both forgeries routed the attacker onto the victim; now refused, and a
read-only database is never routed to while others load),
`tools/review4_retired_read.py` (the read succeeded past the retired
refusal; now refused), `tools/review4_rowcap.py` (the reviewed code
crashed the probe outright; twelve trials now finish under 0.01 s with
the cap visible), `tools/review4_details.py` (probability and sends
read NULL, model read ''; now 0.97 and 1 and NULL), and
`tools/review4_cancel.py` (one Ctrl-C left a queued relate running six
seconds past the signal; the running query now dies at the interrupt
and the queued one refuses before starting, both at one second).

Item 13's remaining half is a boundary, recorded here instead of
papered over: `con.interrupt()` interrupts the CALLER's task, and the
stable C API offers no door from a client context into the query the
kept connection is running. What the cap already bounds (rows) is
bounded; a genuinely slow per-row query finishes its current run and
the outer query cancels at the next chunk boundary. The SIGINT path
bridges fully: the handler interrupts every busy kept connection
through `duckdb_interrupt`.

The leftovers this wave: `ENGINE_TEST_PANIC` is compile-time now
(`--features test-panic`; the default build answers `true` under
`ENGINE_TEST_PANIC=*` where it used to fire); the signal handler reads
its host action through a OnceLock, never a mutex; warnings are zero;
the Makefile reads the one pin from `tools/version.env`; the check
exports `DUCKDB_TEST_VERSION` so the venv installs v1.5.5 instead of
"latest stable", recorded in `scripts/gate-hermeticity.md` with the
git-pinned sqllogictest fetch; the canonical-path cache keys on mtime
so twenty thousand @file calls pay one stat each (the atfile suite's
open-count proofs still green, and an edited file still re-reads).

Open, with state: the logical-type leaks in `warm.rs` and `usage.rs`
(handles not destroyed — the reviewer's lines moved under the parallel
clippy pass; unfixed); the nine deprecated flat-API result sites
(reading values through `duckdb_value_*` — working, unfixed);
`tools/review4_rowcap.py` crashes this ctypes harness roughly one run
in six AFTER all verdicts print — a layout-sensitive teardown
corruption in the probe harness itself, not the extension (the same
twelve trials run clean inline and in the other probes); and the
review-3 suite's cap expectation was updated to the during-scan
wording, which is the point of finding 8's fix.

The full check at this tree: exit 0, 223 ok, wire skipped without the
stub on 8217.

## The fifth review's DuckDB findings (2026-09-23)

Each finding was reproduced on 398d7bb first, then fixed, then rerun.

- **The SIGINT handler took a lock.** It walked the kept-connection registry under its mutex and built a `Vec`, and neither is async-signal-safe. Old: the unit test `signal_tests::a_sigint_landing_while_the_registry_is_held_returns` failed at 10.00 s with "deadlocked the handler". New: it passes in about 1 s. The handler now sets the token and writes one byte to a pipe, and the `thinkthen-interrupt` thread does the registry walk. The chained-host arm of `tools/host_signal.py` missed its 2.5 s bound intermittently while the load average on this machine sat near 300 from parallel builds. It missed on the old build too. At load 113, 10 of 10 trials passed on each build. The miss is the chunk-gap boundary the third review recorded, and heavy load widens it.
- **The row cap bounded the count, not the holding.** A window over eight million rows peaked at 453 MB, and a grouping at 641 MB, before the 256-row refusal. The plan is now read first (`EXPLAIN (FORMAT JSON)`). Any sort, grouping, window, ungrouped aggregate, materialized WITH, delim join input, or join build side estimated at more than 1,000,000 input rows refuses before anything runs. A LIMIT without its own estimate stops the walk: DuckDB rewrites a small LIMIT over a table into a rowid join whose build side scans the whole table, and the first version of this guard refused that plain query (caught by `tools/review3_duckdb.py`). The guard reads estimates, so it is a best effort. With the fix, both shapes refuse at 0.00 s and peak at 62 MB. A grouped, joined, or windowed query over a 2,000-row table still passes. A per-relate `memory_limit` was rejected because that setting is database-wide in DuckDB and would squeeze the host's own queries while a relate runs. `tools/relate_guard_suite.sh` pins both refusal sentences and the small grouped pass.
- **Logical types leaked.** `warm.rs` created three types and `usage.rs` created two, and none was destroyed. The `usage.rs` pair leaked on every bind. Each type is now destroyed after use. `check.sh` counts creates against destroys in each file. On 398d7bb it names `usage.rs: 2 created, 0 destroyed` and `warm.rs: 3 created, 0 destroyed`.
- **The identity's randomness.** `random_hex` read SipHash output from `std::hash::RandomState`, and every name in the process shared one seed. It now reads 16 bytes from `/dev/urandom`, which Linux and macOS both provide. No dependency was added. A read failure leaves the database without a probe, so it cannot be routed to.
- **The drawn calls had drifted.** `tools/drawn-calls/recognize.sql` matched the deck page as it stood on 2026-09-21 (page sha256 `218c8e28…`). Five later deck revisions changed its relations and relate lines. The file is now a frozen fixture that records that hash, and `check.sh` pins the sha256 of its call lines. Re-vendoring the current deck lines is a decision for the steering session, because it changes the acceptance's pinned divergences.
- **A connection's own interrupt** is a known limit, documented in the README with the header evidence. The item stays open.
- **The check failed on a fresh tree.** `check.sh` never created `build/fixture`, so the metadata append failed with "No such file or directory". The fixture build also ran twice. The directory is now made, and the second copy is gone.

### The first run of the rewritten conformance runner (2026-09-23)

The gate lane rewrote `tools/conformance.py` without a DuckDB 1.5.5 CLI. Its first run came on the merged wave-6 tree (ff177aa) through `check.sh`. It printed 127 ok lines, 12 skips, and no FAILED line. The recognize cases compare their relations, and each skip prints its reason from the shared table. No code change was needed.
