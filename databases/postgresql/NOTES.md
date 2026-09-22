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

## 2026-09-21, the settlement wiring (contract 1fe8173)

- **`nearest` rides details here like every member.** PostgreSQL serializes the contract's own `Details` struct, so the settled field needed no wiring in this shim; what it needed was a proof and an honest page. The conformance runner's details branch now carries a fifth field — `(details->>'nearest')` — and compares it against the case's `nearest_level` when the case carries one; case 13 (`mid`) passes through it. The README's old line ("carries no nearest level until the contract gains the field") is replaced by the settled statement.
- **One bug on the way, mine:** the first version of the runner change carried one extra closing parenthesis in the concatenated SQL, and case 73 caught it as "raised when no failure was expected". Fixed in one edit; the run below is after the fix.
- **Evidence from the run after the fix:** `ok 13-score-levels`, `ok 73-details-carries-requests`, `73 of 74 cases ok, 1 diverged` (the named stand-in gap, 17-usage-and-cache: the stand-in never credits its in-process memory in `cache_answers`), `12 of 12 examples ok`, wire suite skipped by design without the stub on 8219, `check.sh` exit 0, and zero containers left (`docker ps -a` shows none).

## 2026-09-22 — the credential refusal, the message shape, the authority section (punch-list item 5)

Item 5: a configured credential must reach the engine safely or be
refused; it must not silently do nothing.

**The refusal.** `thinkthen.api_key` is the ruled setting; this engine
build takes no key from the host (the stand-in reads no key, and which
channel delivers the setting to the send is the database ADR's open
question 4). `engine()` now checks the setting on every call and raises a
usage error naming the setting and the substitute channel; blank reads as
absent, the rule the command uses for its key variable. A set value is
never silently ignored, and no value rides a message.

**The check**, in a third disposable container (`laneb-pg-key`, its
engine pointed at a free port nothing listens on, a made-up value, no
real key anywhere):

```
ok       a configured key refuses with 22023 naming the setting; reset, the refused address answers with 38000
```

Arm one: `SET thinkthen.api_key = 'made-up-not-a-key'` then a call gives
`22023` (invalid parameter) with the setting and `THINKTHEN_API_KEY`
named, and the value absent from the psql output and the server log
(`docker logs` checked). Arm two: `RESET` then the same call gives
`38000` (external routine) with `the address refused the connection` —
the control proving the refusal above is the setting's, and the
missing-credential case answering loudly.

**Two fixes found on the way, both in this commit.** (1) The message
shape: `surface()` formatted `{error}`, whose Display already carries
`usage: `, so every error read `thinkthen usage: usage: ...`. The other
two surfaces render `thinkthen {kind}: {message} (retryable: ...)`, and
this surface now does too, pinned by `the_kind_word_appears_once`. (2)
The first attempt put the setting read inside `engine()`, which the batch
closures call on their worker thread — and `GucSetting::get` checks the
active thread and panics off the backend's own thread, so cases 19, 69,
70, and 72 died with `the batch thread stopped` (recorded here because it
was caught by this lane's own run, not by review). The batch closures now
take the engine reference before `run_batch` spawns, so no worker thread
reads a setting; the conformance slice is back to 73 of 74 with the known
case 17 divergence.

**README.** A new "Authority: who may do what" section names the six
items, including the refusal and the open question it stands in for.

**Evidence from the full run** (`./check.sh`, after both fixes): the
error-mapping tests 3 passed; conformance `73 of 74 cases ok, 1 diverged`
(the named stand-in gap, 17-usage-and-cache); `pg_cancel_backend` stopped
the batch 0.33 s past the signal and `statement_timeout` returned 1.24 s
in; the credential arm green; `12 of 12 examples ok`; wire suite skipped
by design without the stub on 8219; `check.sh` exit 0; the exit trap
removed all three containers and `docker ps -a` shows no `laneb-*` left.
No key, no paid call, nothing published.


## The review fixes: the grant, the deadline, batch errors, the connector (2026-09-22)

Tried: the PostgreSQL findings of the branch review (groups 3, 4, 5), each
with a test that would have failed before it.

**PUBLIC loses EXECUTE.** A `finalize` `extension_sql!` block revokes the
default PUBLIC grant on every function the extension installs — a DO loop
over `pg_proc` joined to `pg_depend`/`pg_extension`, so the rule is
ownership, not name: both `thinkthen_decide` overloads, the
`thinkthen_warm` aggregate, and its three support functions (`REVOKE ...
ON FUNCTION` accepts an aggregate's signature; verified in a scratch
container before wiring it). The check measures it as 13 named functions
and 16 extension-owned ones, all without PUBLIC. Before, any role could call and could read
server files through `'@path'`. The check now measures it: `PUBLIC holds
EXECUTE on none of the 13 functions`, an ungranted role is refused with
`permission denied for function thinkthen_decide`, and the documented
one-line grant (`GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO
the_app_role;`) lets a role call — which is the trust for `'@path'` reads
too, since the read runs as the server process. README's "Authority"
section now says all of it.

**Batch errors carry the real message.** The question resolves on the
backend thread before `run_batch` spawns, in both the array overload and
the warm aggregate, so a bad question file raises its own error instead of
dying inside the worker. The worker's panic payload is no longer dropped:
`panic_text` downcasts a pgrx error report (keeping its PostgreSQL
message), a `&str`, or a `String`. The check proves three arms: a missing
file names itself (`no-such-file.json`), a broken file returns the usage
kind (`the question file is not valid JSON`), the warm aggregate names the
missing file — and none of them says `the batch thread stopped`. A unit
test (`a_stopped_worker_keeps_its_message`) proves the extraction.

**The deadline is the single-row tool.** New `thinkthen.deadline_ms` GUC
(`Userset`, `-1` none, `0` spent, positive a budget), read by
`call_options()` through the contract's checked `with_deadline_millis` and
carried by every single-request function. A spent budget refuses before
sending (the check: SQLSTATE `57014`, `thinkthen deadline` in the
message), and against the 300 ms stub a 50 ms budget refused in 0.13 s.
Conformance case 27 now runs instead of skipping: `runner.py` gained the
deadline arm. The physics — a blocking socket cannot hear
`pg_cancel_backend`, so the budget is the enforced tool — is stated in the
module doc, the README, and the setting's own description.

**Phase 1 adopted.** `engine()` is `Arc<dyn Engine>` built through
`StandinConnector.connect(&EngineConfig::from_env())`; `relate_checked`
takes `engine.as_ref()`. The connector line is the merge's one-line swap.

**Case 74.** The stand-in arms its synthesized partial-failure fixture only
under `ENGINE_SYNTHETIC_PARTIAL` now, so the null container is started with
that test-only opt-in (`check.sh`); without it case 74 diverges.

Saw (full `./check.sh`, stub up on 8219, exit 0):

```
ok       PUBLIC holds EXECUTE on none of the 13 functions (16 extension-owned, aggregate helpers included)
ok       an ungranted role is refused with permission denied
ok       the documented one-line grant lets a role call (and read '@path')
ok       a missing file in a batch names the file, not a stopped thread
ok       a broken question file returns the usage kind naming the file
ok       the warm aggregate names the file too
ok       a zero budget returns the deadline kind (57014) with nothing sent
73 of 74 cases ok, 1 diverged        (case 17, the stand-in's recorded gap)
ok       a 50 ms budget refused in 0.13s against the 300 ms stub
check.sh exit 0; the trap removed all three containers
```

Means: the group-3, group-4, and connector findings for this surface are
closed with discriminating tests.

Problem and way round it: none open. No key, no paid call, nothing
published; `docker ps -a` shows no `laneb-*` left.

## 2026-09-22 — review 2, item 4: the @ requirement, the narrowed grant, and the update path

**The finding.** Any argument that was not JSON was read as a *file path*
with no `@` needed, so a role holding EXECUTE could read server files as
the postgres user; the documented grant was `GRANT EXECUTE ON ALL
FUNCTIONS IN SCHEMA public`, which also re-grants other extensions'
functions; and the PUBLIC revoke ran only at install, so a future
`ALTER EXTENSION UPDATE` would hand a new function's default PUBLIC grant
to everyone.

**The fix, three parts.**

1. `arg_form` decides every named argument (question, question set,
   recognize spec): `@name` is a file, `{...}` is JSON, and anything else
   is a usage error naming the required form — never a path. Pure, so the
   rule has unit tests.
2. The documented grant now loops over the extension's own functions
   through `pg_depend` (the same query as the revoke), never
   `ALL FUNCTIONS IN SCHEMA public`. The gate proves both sides: the
   granted role calls `thinkthen_decide`, and an unrelated function
   (`tt_control`, PUBLIC revoked) stays refused with permission denied.
3. The revoke block also creates `thinkthen_guard_public()`, an event
   trigger function (SECURITY DEFINER, `search_path = pg_catalog`), and
   an event trigger on `ddl_command_end` for `CREATE FUNCTION`,
   `CREATE PROCEDURE`, and `CREATE AGGREGATE`; each fire revokes PUBLIC on
   every extension-owned function. The guard function takes its own
   revoke by name (it is created after the install loop ran). A function
   created by a future update script is therefore revoked even though the
   update script says nothing about PUBLIC.

**Evidence, pre-fix versus post-fix, by command.** A synthetic next
version (`thinkthen--0.0.1--0.0.2.sql` holding one `CREATE FUNCTION` and
no revoke statement) is installed and `ALTER EXTENSION thinkthen UPDATE TO
'0.0.2'` runs in a disposable container:

| probe | pre-fix | post-fix |
| --- | --- | --- |
| `has_function_privilege('public', 'thinkthen_rehearsal_probe(integer)', 'EXECUTE')` | true | false |
| extension-owned functions with PUBLIC EXECUTE | 1 | 0 |
| bare `'names.json'` spec | reads the file and answers | usage refusal naming `@names.json` |
| granted role calling an unrelated function | succeeds | permission denied |

**The deck's line.** The deck's PostgreSQL tab draws
`thinkthen_annotate('form.json', body)` with a bare name. The ruled
spelling for a file is `'@form.json'`, so the drawn line is stale for its
owner (README.md records it); `check.sh` runs the ruled spelling, and
`package.sh` substitutes it in the verbatim deck extraction with the
reason in a comment.

## 2026-09-22 — review 1/2 leftovers: the batch deadline and the interrupt gate

**The findings.** The batch paths (the array overload, the warm
aggregate, relate) ignored `thinkthen.deadline_ms`, and the poll treated
*any* interrupt as a cancel, so a benign procsignal interrupt (a
memory-contexts request) failed a paid batch with the cancelled kind.

**The fix.** `run_batch` resolves the deadline budget on the backend
thread (`budget_of`, the contract's checked millis door) and carries it
into the worker's options, so a spent budget sends nothing and a budget
mid-batch stops it at the engine's next tick. The poll now cancels the
token only when `QueryCancelPending` or `ProcDiePending` is set — SIGINT
(`pg_cancel_backend`, statement timeout) and SIGTERM
(`pg_terminate_backend`) — and leaves every other interrupt to
`CHECK_FOR_INTERRUPTS`, which services it without ending the batch.

**Evidence, pre-fix versus post-fix, by command.**

| probe | pre-fix | post-fix |
| --- | --- | --- |
| `thinkthen.deadline_ms = 0` on the array overload | completes, count 2 | SQLSTATE 57014, nothing sent |
| `thinkthen.deadline_ms = 0` on `thinkthen_warm` | completes | 57014 |
| `thinkthen.deadline_ms = 50` on a 300k-record array | completes in about a second | 57014 in 0.35 s |
| `pg_log_backend_memory_contexts` mid-batch | `ERROR: thinkthen cancelled` | the batch completes with its count |
| `pg_cancel_backend` mid-batch | stops (control) | stops in 0.29 s |
| `statement_timeout = 1s` on the same batch | stops (control) | stops in 1.32 s |

**The gate's fixture.** The compile-time fixture door (review 1's item)
needs a feature-passing build: the crate forwards the stand-in's
`synthetic-partial` feature, and `check.sh` packages its test artifact
with it (`package.sh` keeps the production shape), so conformance case 74
replays the synthesized partial failure instead of diverging.

**Shared guard.** The worker body runs behind
`thinkthen_contract::catch_panic("the batch thread", ...)`, and the join
fallback's formatter is the pgrx report first, then the contract's own
`panic_text` — one spelling for every other payload.

**The unmaintained crate under pgrx, recorded (item 4).** `serde_cbor 0.11.2`
rides in through the pinned `pgrx 0.17.0` (`cargo tree --locked -i serde_cbor`
in this folder: serde_cbor ← pgrx ← thinkthen), and `cargo deny --offline
check advisories` fails here with `error[unmaintained]` (RUSTSEC-2021-0127,
"No safe upgrade is available!"). The crate is pgrx's own dependency, so the
swap is not ours to make: the plan is to pin as the lock does now, bump pgrx
when it drops serde_cbor (the author's named alternatives are ciborium and
minicbor), and — when deny coverage extends over the surface workspaces at
merge — record an explicit `ignore` for RUSTSEC-2021-0127 in deny.toml with
this note as the argument, or block on the pgrx bump. The build team owns
that call; the ticket beside the review findings carries it.
