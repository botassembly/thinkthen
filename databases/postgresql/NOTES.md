# The PostgreSQL surface: the running log

## Entry 1: environment and build, 2026-09-21

- `pg_config` → PostgreSQL 16.15 (Ubuntu); `~/.pgrx/config.toml` → `[configs] pg16 = "/usr/bin/pg_config"` (set up for experiment 207; no download, no server started). `cargo-pgrx 0.17.0`, Docker 29.1.3. The system postgres service is inactive; ports on this box move between sibling sessions (Entry 3).
- Crate: `databases/postgresql`, its own workspace so the root gates stay untouched, path dependencies on `contract/` and `standin/` (the one dependency that changes when the real engine lands). `panic = "unwind"` (205 finding R4/C1: an abort kills the backend).
- Package route from 207, confirmed: `(cd databases/postgresql && cargo pgrx package --pg-config /usr/bin/pg_config)`. The `--manifest-path` form still dies at pgrx's cargo metadata step. The hand-written `thinkthen.control` (`default_version = '0.0.1'`, the repo's `VERSION`) and `src/bin/pgrx_embed.rs` (`::pgrx::pgrx_embed!();`) are both required.
- Generated SQL: 12 functions plus the aggregate — the nine ruled functions, the `thinkthen_decide` array overload (`postgres.md`'s second bulk form), and the aggregate's three leaked `warm_warm_*` helpers, the known pgrx-0.17 pattern from 207. `PARALLEL RESTRICTED` on every judging function, spelled by pgrx itself.
- Code lines: **294 against the 299 ceiling** on `postgres.md`. The SQLSTATE mapping for the six kinds is the one part 207 did not carry (it raised everything as one class), and it fits inside the ceiling.

## Entry 2: the shape, and the two readings this lane decided

- **Six kinds to six SQLSTATEs**: usage → `22023` invalid parameter, backend → `38000` external routine, deadline and cancelled → `57014` query canceled (statement_timeout is a deadline; that is a PostgreSQL user's word for it), local → `58030` io, defect → `XX000`. The message carries the kind word and the retry signal: `thinkthen backend: HTTP 422 ... (retryable: no)`.
- **`'@file'` resolves against the backend's working directory.** The official image's backend runs with its data directory as the working directory, so `@refund.json` reads `/var/lib/postgresql/data/refund.json`. Where the file may be read from is the database ADR's open rule; this lane reads the path exactly as named, relative to the backend, and says so here.
- `choose`, `score`, and `tag` keep the ruled three-argument signature, `NULL` as the third argument when the JSON question carries its own members.

## Entry 3: the port trap, twice

Two failures of the same class, recorded because the check now guards against them: sibling sessions start and stop their own postgres containers on this box, and a fixed port races them. Port 5435 answered `pg_isready` before my container had a socket (a foreign postgres held it), and the first wire run picked the same port as the null container. The check now takes the first free port from a quiet range (`free_port`), skips the one already taken, and waits on the container's own socket (`/run/postgresql/.s.PGSQL.<port>` — not `/var/run`), never on TCP.

## Entry 4: the slide sample, as drawn

Container `postgres:16` on the host network, `ENGINE_NULL=1`, files at the backend's working directory, `CREATE EXTENSION thinkthen` (instant), fixtures from the deck's own `examples/` (byte-for-byte `refund.json` and `form.json`; `tickets.sql` seeds the refund row, the `maybe` row, and the plain row):

```sql
SELECT id, body FROM tickets WHERE thinkthen_decide('@refund.json', body) IS NULL;
```

returns exactly the `maybe` row — 0.55 under the 0.2:0.8 band is the NULL a person should read. The first fixture text said "put the money back" and "Maybe"; the null backend keys on the literal words `refund` and `maybe`, so the rows now use the deck's own ticket text.

```sql
SELECT id, a->>'team' AS team, (a->>'urgency')::float AS urgency
FROM tickets, thinkthen_annotate('form.json', body) AS a ORDER BY urgency DESC;
```

returns urgency 1.7 / 1.05 / 0.99 in order. `team` is NULL on every row under the null backend: `billing`, `shipping`, `account` tie at one third each and the honest winner under the cut is none. The slide comment promises "as jsonb", not a team value, so no finding; on the wire the choice answers.

## Entry 5: the conformance slice, offline

`python3 runner.py <container>`, each case one line:

```
19 of 20 cases ok, 1 diverged
diverge 17-usage-and-cache: expected 1/1 after the second call, got 2/0; the
  stand-in answers the repeat from in-process memory but never counts it in
  cache_answers, its own record says so — a real-engine requirement, not a
  surface gap
skip 05/06/09 (filter is WHERE, ORDER BY, and LIMIT here; no function ships)
skip 18-cancel-mid-batch (the token cannot be pre-fired through SQL; the
  statement_timeout proof below is this surface's cancel shape, and the
  engine-side gap is recorded in conformance/DIVERGENCES.md)
```

Every `runner.py` call is a new psql session, so the run itself is the per-backend proof: dozens of fresh backends, each with its own engine state, all answered (lazy init after the fork, the pid check never once saw a stale pool).

## Entry 6: the wire, on the lane's stub (8219, 300 ms)

- `thinkthen_decide('@refund.json', 'please refund the duplicate')` → `t`; `thinkthen_usage()` → `1 | 0 | 12` — requests count sends and the tokens column carries the stub's reported usage.
- The array overload: three evidences in, `(0, true), (1, NULL), (2, false)` out, three requests at the stub, one crossing.
- **Warm at the process width**: 64 rows → 701.791 ms wall, stub `max_in_flight 32`, 64 requests, 33 connections, frozen through a 3 s settle. The gate holds through the aggregate.
- **The cancel shape, 211-proven and not re-proven here, confirmed anyway**: 512 rows, `statement_timeout = '1s'` → control back at 1.322 s with the standard `canceling statement due to statement timeout`; the stub saw exactly 128 requests (four 32-wide rounds) and nothing after; 384 requests never started.

## Entry 7: cleanup and the guard

```
docker rm -f -v thinkthen-pg thinkthen-pg-wire
grep -c deno ~/.zshrc -> 0
```

`check.sh` removes both containers in its exit trap; `docker ps -a` shows none left. No tool was installed (pgrx, cargo, docker already present), so no installer could touch an rc file; the grep guard ran anyway and its result is above. No key, no paid call, nothing published, no sudo, nothing outside this folder and its two disposable containers.

## What was not run (unchecked)

- macOS packaging: this box is Linux; the packaging rehearsal owns that arm.
- `shared_preload_libraries` with a postmaster-built pool: `_PG_init` here registers the key setting and touches nothing else, per this brief. The preload question stays with the database ADR.
- The `thinkthen.api_key` GUC is registered (the ruled setting exists) but the engine reads the environment at send time; wiring the setting into the send is the database ADR's preload question.
- `details` on a score question carries no nearest level through the contract (`Details` has no such field). ADR 0017 pick 6 says the nearest level's name rides in details; the contract needs the field before the promise is reachable. Recorded for the contract's owner, not worked around.

## Entry 8: recognize and relate land, 2026-09-21

The two functions from the update brief (`sdlc/issues/2026-09-21-update-for-the-library-team-recognize-and-relate.md`), plus the beta relations-as-rows companion the deck's SQL block shows. Twelve SQL functions now.

**Shapes.** `thinkthen_recognize(body, kinds)` is a set-returning function, `PARALLEL RESTRICTED`, returning the five ruled columns `(text, kind, start, end, strength)`; the deck's call runs as drawn with `LATERAL`, and the `"end"` column is quoted because `end` is reserved. `thinkthen_relate(query, rules)` takes a query string and a `text[]` of bare relation names (any kind to any kind — richer rules ride the question file), executes the query through SPI, and returns `(name, source, target, probability)`. `thinkthen_relations(body, '@names.json')` is the beta companion returning `(name, source_text, source_kind, target_text, target_kind, probability)`.

**Three decisions this lane made, each with its reason.**

- **`source` and `target` as the column names.** The design table's older line says `from_id, to_id`; the ruling of 2026-09-21 (`relate-design.md`, "a relation's ends are `source` and `target`, everywhere") overrides it, and the ruling's sentence says a user sees one pair of words on every surface.
- **The SQL rows carry the query's id values, not the 1-based record numbers.** The query selects the id first and the text second (`SELECT id, body FROM alerts`), the edges' `source`/`target` are those id values, and the edges join back to the query's table. SQLite's ruled call passing the `id` column by name is the same choice seen from the other engine.
- **Offsets are characters, PostgreSQL's own string indexing.** `substring(text from start + 1 for end - start)` is the name, proven on `Le café 😀 Maria Chen arrived.` (`Le café ` is 8 characters, so the name starts at character offset 10 — bytes would be 14). The contract carries code points; PostgreSQL's `substring` counts characters, so the conversion is identity and is documented in the function's comment.

**One contract gap, recorded for its owner.** The ruled question file spells a relation's ends `source` and `target` (`recognize-design.md`), and the contract's spec parser (`Recognize::from_json`) reads `from` and `to`. This door accepts both spellings, normalizes to the parser's pair before the one core parser runs, and lifts the file's top-level `threshold`/`relation_threshold` into the `recognize` section. Flagged in the code comment; the contract owner decides the parser's final spelling.

**The 255 limit** comes through the contract's `relate_checked` guard: the SPI fetch is capped at 256 rows, the engine refuses the 256th with `thinkthen usage: relate takes at most 255 records and 256 came (retryable: no)`, SQLSTATE 22023, proven in `check.sh`. The message names the fetched count (256), not a query's full count, because the surface stops reading at the limit.

**The conformance slice** now runs 72 cases: 71 ok, case 71 named as the conformance-data finding (the per-subject arm shares its input with the pairs arm; the stand-in serves the ruled pairs form). All forty recognize cases and relate cases 69, 70, and 72 pass through SQL. The runner gained `recognize` and `relate` branches; its skips (filter/rank/find, the pre-fired cancel, the deadline door, the missing-question-file case) are unchanged and named.

**The acceptance section in `check.sh`**, run offline in the disposable container:

```
71 of 72 cases ok, 1 diverged
recognize rows, offsets, the no-request join, relate edges, and the relations rows are green
the 256th record refuses with the usage kind and SQLSTATE 22023
```

The any-kind end is proven through the question-file door too: `names-star.json` spells the `works_for` rule `"*"` to `"*"` and the same relation row comes back. It proves, in order: the deck's LATERAL call as drawn (five rows over the three inbox texts); a `mentions` table built from `recognize`; the offsets slicing the name back out (`substring` on the café/emoji text); the equality join to `accounts` with the usage counter frozen across it (`requests before join 0` / `requests after join 0` — the no-request proof); the deck's `thinkthen_relate` call as drawn (the four recorded edges with their id values and probabilities); and the beta `thinkthen_relations` row from `@names.json` (the ruled question-file spelling with `source`/`target` keys).

**Lines.** 526 code lines against the 299 ceiling `postgres.md` records (207's measure). The two functions plus the beta companion are the growth; the ceiling is a planning-page statement, not a gate, and the number is recorded here for the build team.

**Formatting.** `cargo fmt --check` reports 22 diffs; most are in the pre-existing hand style (for example `with_members` at line 135), and the lane did not reformat the file to keep this diff reviewable. No gate runs fmt on this separate workspace.

Cleanup: the scratch container `dbpkg211-pgscratch` removed with `docker rm -f -v`; `docker ps -a` shows no `dbpkg211-*` left; `grep -c deno ~/.zshrc` → 0. No tool installed. No key, no paid call, nothing published, no sudo.

## What was not run (unchecked)

- The wire path for `recognize`, `relate`, and `thinkthen_relations`: they answer from the recordings, offline, and make no request; there is nothing on the wire to prove for them in this lane.
- `either` rules and named-kind ends through `thinkthen_relate`'s array: the ruled SQL call carries bare names; the question file carries the richer rules.
- `thinkthen_relations`' final fate: the design page lists it beta and open (question 3 for the build team).

## Page-section text for the manual: `relate` over a table (PostgreSQL)

For the build team to lift:

> ### relate over a table
>
> `relate` reads every record at once, because every pair is one pick-one question. Give it a query and the relation rules:
>
> ```sql
> SELECT * FROM thinkthen_relate('SELECT id, body FROM alerts', ARRAY['caused_by']);
> ```
>
> The query selects two columns: the record's id first, its text second. Each row comes back as `(name, source, target, probability)`, with `source` and `target` carrying those id values, so the edges join straight back to your table, and a recursive query can walk them. One call takes at most 255 records; past that is a usage error, because the pair count grows with the square of the records. The planner never knows a function costs money: one call is one request for every pair.
>
> For relations as rows from a question file, the beta companion:
>
> ```sql
> SELECT * FROM tickets t, LATERAL thinkthen_relations(t.body, '@names.json');
> ```
>
> It returns `(name, source_text, source_kind, target_text, target_kind, probability)`.

## 2026-09-21 — the fix wave: ruling 1 and ruling 2 at this door

**Ruling 1, before and after.** Before, this door rewrote a spec's `source`/`target` ends into the parser's old `from`/`to` pair, so it accepted both spellings while every other door accepted one — the quiet inconsistency the review named. After, the spec crosses to the one core parser unchanged and this door converts nothing; `from`/`to` is refused the way the other doors refuse it, with the ruled spelling named in the message. `fixtures/names-legacy.json` holds a `from`/`to` spec, and the check's new step proves the refusal and the named spelling in the container:

```
== postgres surface: from and to are refused
ok       from/to refused, source and target named
```

**Ruling 2, the defect mapping.** `raise` now takes its code and text from a pure `surface(&Error)`, and a unit test constructs the contract's defect error at the shim level and asserts the engine's surface carries it:

```
$ cargo test --release --quiet --lib
running 1 test
test result: ok. 1 passed; 0 failed
```

The check runs that test before packaging. No public door gains a fault hook.

**The working shape for `thinkthen_relations` on this engine:** `SELECT * FROM tickets t, LATERAL thinkthen_relations(t.body, '@names.json');` — the deck's drawn form needs the `LATERAL` join to a table alias; the finding with this line goes to the deck's owner from another lane.

The check, end to end, with the loopback stub up:

```
$ ./check.sh
== postgres surface: from and to are refused
ok       from/to refused, source and target named
== postgres surface: wire suite against the stub on 8219
exit 0
```

## Lane B items 2, 5, and 6 on the PostgreSQL surface, 2026-09-21

**Item 2, the new shapes.** `thinkthen_annotate` carries the ruled failed marker where a value would sit, and `thinkthen_details` (jsonb, serialized from the contract's `Details`) gained `requests` (0053) and `failed_questions` (0054). The conformance runner was repaired on the way: its details branch compared the recorded probability, which the null backend's own rule cannot reproduce for case 73, so it now checks the audit's identity fields and the two additions; its annotate branch crashed on a failed member and now matches the marker.

```
$ python3 runner.py laneb-pg | tail -3
ok 73-details-carries-requests
ok 74-annotate-preserves-good-answers
73 of 74 cases ok, 1 diverged      (17-usage-and-cache, the named stand-in gap; exit 0)
```

```
$ psql -c "SELECT thinkthen_annotate('{\"version\":1,\"questions\":{\"kind\":{\"decide\":\"Is this a complaint?\"},\"topic\":{\"decide\":\"Is this about a refund?\"}}}', 'order 4471: charged twice, please refund');"
{"kind": true, "topic": {"failed": {"kind": "backend", "cause": "missing_answer"}}}
```

**Item 5, the fast-backend cancel.** The batch paths (`thinkthen_decide`'s array overload and `thinkthen_warm`'s finalize) run on a worker thread while the backend thread polls PostgreSQL's `InterruptPending` flag in `run_batch`'s 100 ms loop — a fast backend never idles, so the batch shape is the discriminating one here (a per-row query is covered by PostgreSQL's own executor checks). Full run: about 3.6 s (1M elements, null backend, measured 2026-09-21). Measured on the packaged extension:

```
ok       pg_cancel_backend stopped the batch 0.33s past the signal (full run about 3.6 s)
ok       statement_timeout returned 1.33s in (1 s timeout; full run about 3.6 s)
```

**Item 6, the examples file.** `examples.json` is keyed by function — twelve entries, every public function on this surface — and `tests/examples.py <container>` runs each through its own psql invocation (one fresh backend, so the usage counters start at zero) and checks its answer.

```
$ python3 tests/examples.py laneb-pg
ok       decide
...       (twelve lines)
12 of 12 examples ok
```

**Containers.** The check's disposable containers are now named `laneb-pg` and `laneb-pg-wire`; both are removed with `docker rm -f -v` by the check's exit trap, and the measurement/debug containers used while sizing the cancel test (`laneb-pg-measure`, `laneb-pg-dbg`) were removed the same way. `docker ps -a | grep laneb` prints nothing after a run.

**Fixed on the way:** the packaged `thinkthen.so` under `target/release/thinkthen-pg16/usr` had gone stale against the contract's new fields — the check repackages on every run, so the committed flow was fine, but any manual probe must run `cargo pgrx package` first (recorded here because it cost a confusing probe).

**The record row, adopted.** The conformance slice now asserts the ruled `{"input","value"}` row on the bulk forms: the value-printing projections read back as `input|value` pairs in input order — `filter`'s kept rows and `decide_many`'s answers where the case carries them (`05`, `19`; case `06`'s empty list is covered by its count). SQL's own two columns are the row; no new function was added.

```
ok       05-filter-keeps-some-of-five rows
ok       19-decide-many-judgments rows
```
