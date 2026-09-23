# Notes: the SQLite surface. Write as you go, newest entry last, each entry Tried / Saw / Means / Problem and way round it.

## Entry 1: the build, and the traps in it (2026-09-21)

Tried: `cargo build --release` in `databases/sqlite/`, a workspace of its own like the C surface's. The crate is `thinkthen-sqlite` with lib name `thinkthen0`, on rusqlite 0.40.2 `loadable_extension` + `functions`, binding `thinkthen-contract` and `thinkthen-standin` by path, `panic = "unwind"`, entry written by hand: `sqlite3_thinkthen_init` storing the db pointer and calling `Connection::extension_init2`.

Saw: four traps, all compiler-caught. (1) Edition 2024 wants `unsafe extern "C"` for the `sqlite3_is_interrupted` declaration and `#[unsafe(no_mangle)]` on the entry. (2) rusqlite's `Aggregate` trait is `Aggregate<A, T>` with `init/step/finalize` on `&self`, not an `Init`-carrying trait. (3) `ffi::Error::new` takes a raw code, so the mapping uses `ffi::SQLITE_INTERRUPT` and friends, not ErrorCode variants. (4) A plain-text question is `Question::decide(text)?.cut(0.5)`, the C surface's own spelling. The build is green and `ldd` shows `libthinkthen0.so` carries `DT_NEEDED libsqlite3.so.0`, the same direct link the 207 experiment recorded for the 3.41 interrupt call the routed headers lack.

Means: the 207 recipe carried over whole; the entry-point rule (basename `thinkthen0`, trailing digit stripped, entry `sqlite3_thinkthen_init`) holds on this toolchain too.

Problem and way round it: none open on the build.

## Entry 2: the stock CLI and the slide (2026-09-21)

Tried: the slide's install line demands the stock CLI, and this machine has none. Downloaded the official `sqlite-tools-linux-x64-3530400` into `.runtimes/` (folder-local, `chmod +x`, the zip removed), then `.load ./thinkthen` over `tests/slide.sql`.

Saw: `.load ./thinkthen` resolves `./thinkthen.so` when the bare name is not a file, so the extension ships beside the CLI as `thinkthen.so` — the entry still derives from `thinkthen`. The slide ran exactly as drawn: warm 5, the WHERE kept rows 1, 3, 4, the count said 3. Runtime-installer guard: `grep -c deno ~/.zshrc` → `0`; no rc file was touched by anything this lane ran.

Means: the sample is the stock-CLI proof, not a harness of mine.

## Entry 3: the surface and the session map (2026-09-21)

Tried: eight functions in the ruled shape — `thinkthen_decide`, `_choose`, `_score`, `_tag`, `_annotate`, `_details`, `_usage`, `_warm` — with the question argument accepting plain text, JSON, or `'@refund.json'` (pick 9), resolved relative to the process working directory, and `thinkthen_usage('reset')` as the reset arm. A per-process answer map keyed by question digest and evidence holds the session memory: warm fills it, the scalar functions read it, `cache_answers` counts the serves. In SQL, `filter` is the `WHERE thinkthen_decide` pattern the slide draws; `rank` and `find` are the database ADR's to rule as aggregate forms, and this lane did not invent names for them.

Saw: null suite 19 of 19 — the band reads NULL, warm plus WHERE plus count send nothing after the warm, the map's serves are counted, a missing `'@file'` is a local failure naming the file, a blank question is a usage refusal, score is a REAL, tag is a JSON array, annotate is an ordered object.

Means: the ruled empty value, the six kinds in SQLite's message text with `retryable` carried, and the `'@'` spelling all behave.

Problem and way round it: `details` on a non-decide question answers the stand-in's "the answer carries no probability" backend error — the same stand-in gap the R surface recorded; the null suite asserts details on a decide question and the gap is on record here.

## Entry 4: the wire proofs, stub on 8218 (2026-09-21)

Tried, with the stub at 300 ms and `ENGINE_WIDTH=32`:

```
STUB_PORT=8218 python3 tests/wire_suite.py
```

Saw: 12 of 12. Warm over 128 rows: every row judged, `max_in_flight` exactly 32, one request a row, wall inside the width bound (1.2 s shape). The query after the warm sent nothing and read the saved answers. `sqlite3_interrupt` at 700 ms into a 200-row warm: the statement failed with `thinkthen cancelled` inside one round of the signal, and the stub's request count never moved after the return — the poll on the calling thread hears the host's interrupt and the token lands between records, one in-flight round deep, with no watchdog thread. A refused address (a closed port, its own child process because the engine builds once a process) failed as `thinkthen backend: ... the address refused the connection`, final, no retryable. `os.fork` after a wire call: the child opened its own connection, loaded the extension, and answered its own call under a 10 s alarm — the engine's process check repaired the fork through the surface. Conformance slice offline: 15 ok, 2 diverge (`09` the verb-level band rule and `18` the pre-fired token, both recorded in `conformance/DIVERGENCES.md` as real-engine requirements), 3 skip (the two backend-refusal cases need the wire, which the suite proves, and nothing else).

Means: the interrupt shape is the poll callback the ADR rules, proven here with no extra thread; the fork story holds through SQL; the gate holds through the aggregate.

## Entry 5: what this lane did not run (unchecked)

- No second connection's interrupt: the poll checks the db handle of the connection that loaded the extension, and this surface ships for the one-connection shape; a multi-connection host is the database ADR's.
- No parallel writers on one database file; the session map is per process, as the cache page rules for a database with no folder named.
- No packaging: no `sqlite3` package, no clean-container install; the packaging rehearsal owns that.
- No `rank` or `find` aggregate; the ADR rules their SQL shapes and this lane left the names unclaimed.

## Entry 6: `recognize` and `relate` land as the table-valued functions (2026-09-21)

Tried: the two new functions in the ruled SQLite shape — `thinkthen_recognize(text, 'person,organization')` as an eponymous table-valued function over rusqlite's vtab API (the `vtab` feature joined the manifest), answering the five columns `(text, kind, start, end, strength)` from the replay recordings, and `thinkthen_relate('alerts', 'id', 'body', 'caused_by')`, which reads the whole named table through a nested read-only SELECT on the same connection, stops one row past the ruled record limit, and answers `(name, source, target, probability)` with the id column's values riding as `source`/`target` (INTEGER or TEXT ids).

Saw:

```
$ ENGINE_NULL=1 python3 tests/tvf_suite.py
15 of 15 ok — the deck's calls as drawn, the five columns, the default kinds,
the emoji offsets, the no-names text, the repeated name, both refusals, the
255 limit, and the join pattern with usage requests 0 -> 0

$ ./check.sh
the slide in the stock CLI, null suite 19/19, tvf suite 15/15,
conformance slice: 94 ok, 2 diverge (the two recorded real-engine
requirements: the band-on-filter verb rule and the pre-fired token),
1 skip (the per-subject relate arm; the ruled pairs form is case 72),
wire suite skipped (no stub on 8218), exit 0
```

The conformance slice now runs all forty-one `recognize` cases through the TVF (names; the seven cases that ask relation rules carry the "relations ride the beta question-file function" note) and three of the four `relate` cases (`R01-demo` 4 edges, `R02-pickone` 11, `R04-pairs-10` 10); the per-subject arm is skipped with its reason. Update 2026-09-23: the relate cases are now `69-relate-alerts`, `70-relate-founders`, and `71-relate-staff` under method H, the per-subject case is retired, and a refused case prints a counted FAIL line (`tests/test_conformance_driver_fail.py`).

**One real defect found and fixed on the way.** The first build segfaulted at the first TVF call: a rusqlite cursor struct must carry `ffi::sqlite3_vtab_cursor` as its first field, or SQLite's write of `pVtab` lands in the next field — in this module's case, the rows vector. The instrumented run showed `cap=343406448`, the vtab pointer, where a capacity belonged; both cursors now carry the required base field, and the gdb trace is reproducible from this entry's story.

**Offsets in SQLite's own indexing.** The recorded offsets are code points; SQLite's `substr` and `length` count characters; no conversion exists and none is needed. The proof, on the synthesized accent-and-emoji row:

```
substr('Le café 😀 Maria Chen arrived.', start + 1, end - start) -> 'Maria Chen'   (start 10, end 20)
```

Note for the build team: this lane's brief said "offsets are byte counts"; the contract's `Entity` docs ("count code points"), the design page ("in that surface's own string indexing"), the recordings, and SQLite's own character counting all agree on characters. The emoji test pins the character path; byte-offset columns would break SQL-side slicing and the deck.

**The vocabulary sweep, pasted from this folder:**

```
$ grep -rniE "certainty|likelihood|cutoff|gray zone|confidence|calibrated|accuracy" src/ tests/ NOTES.md README.md
(no output)
```

Relations and edges carry `probability`; names carry `strength`; the vendor's own field name appears nowhere in this folder. The vendor's `confidence` passes under details only — the recordings carry it, and the details door for these two functions arrives with the real engine.

**The page section for the manual, relate's whole-table form** (for the build team to lift):

> `relate` needs every record at once, so a database call takes a table or a query, never a single row. In SQLite, name the table, the id column, the body column, and one or more relation names: `SELECT * FROM thinkthen_relate('alerts', 'id', 'body', 'caused_by');`. Each row of the table is one record, counted from 1 in the order the SELECT reads it; the edges come back as rows `(name, source, target, probability)`, and `source`/`target` carry the values of the id column, so the edges join straight back to the table. More than 255 records is refused with a usage error naming the limit, because pair counts grow with the square of the records. Read a table once and join by equality, or walk the edges with a recursive query.

**Not run, honestly:** the beta `thinkthen_relations` line in the deck's SQL block — the build team's page leaves open whether relations ride `thinkthen_recognize` or a separate function, nothing was invented for it, and the literal `'*'` any-kind spelling lives in that question-file form, so it is not exercised through this call shape (bare relation names are the any-to-any rule by construction); the other error kinds through these two functions offline (the replay produces usage errors only; backend and cancelled arrive with the real engine's wire); SQL forms beyond the deck's (a JOIN expression as the source, extra arguments).

**A stale line on the design page:** `relate-design.md`'s database table still says the rows are `(name, from_id, to_id, probability)`. The deck's own note and the update brief rule `source`/`target` on every surface, and this surface emits `source`/`target`. Flagged here so the page can be corrected in one edit.

## 2026-09-21 — the fix wave: ruling 2 and the relate guard at this door

**The defect mapping, ruling 2.** A unit test constructs the contract's defect error at the shim level and asserts SQLite's own error surface carries it — the `thinkthen defect:` message and the generic error code. No public door gains a fault hook.

```
$ cargo test --release --quiet --lib
running 1 test
test result: ok. 1 passed; 0 failed
```

The check runs that test after the build.

**The relate guard's home is now one place.** Before, this door called the engine's `relate_opts` directly and relied on the stand-in repeating the 255 guard; after, it enters through `thinkthen_contract::relate_checked`, so the guard lives only in the contract and this door inherits it the way every other door does. Behavior is unchanged and the door's 255 test still proves the refusal.

**The working shape for relations on this engine:** this surface has no `thinkthen_relations` call shape yet — the beta companion is unbuilt here, and nothing was invented for it; the working relate shape is the table call `SELECT * FROM thinkthen_relate('alerts', 'id', 'body', 'caused_by');`. The deck's drawn `thinkthen_relations` form cannot run on SQLite as written; the finding goes to the deck's owner from another lane.

The check, end to end, with the loopback stub up:

```
$ ./check.sh
== sqlite surface: the error-mapping test
== sqlite surface: wire suite against the stub on 8218
wire suite done
exit 0
```

## Lane B items 2, 5, and 6 on the SQLite surface, 2026-09-21

**Item 2, the new shapes.** `thinkthen_annotate` carries the ruled failed marker where a value would sit — `{"failed":{"kind":"backend","cause":"missing_answer"}}` — and `thinkthen_details` gained `requests` (the ordered recording digests, 0053) and `failed_questions` (0054). The conformance driver was repaired on the way: its details branch compared the recorded probability, which the null backend's own rule cannot reproduce for case 73 (evidence "I want my money back" → 0.03, recorded 0.97), and its annotate branch crashed on a failed member. It now checks the audit's identity fields — model, digest, and the two new fields — and matches failed members by their marker, and case 72's relate skip reads the ruled `source`/`target` spelling (the fix wave re-keyed the cases, and the old `from`/`to` probe had started skipping every relate case).

```
$ ENGINE_NULL=1 python3 tests/conformance_driver.py | tail -5
skip     71-relate-R03-persubject-10: the per-subject form is an engine-internal arm; the surface serves the ruled pairs form, which case 72 covers
ok       72-relate-R04-pairs-10: 10 edges
ok       73-details-carries-requests: audit carried, digest equal
ok       74-annotate-preserves-good-answers: fields assembled
conformance slice done
```

```
$ python3 -c "...annotate with the partial-failure record..."
{"kind":{"answer":true},"topic":{"failed":{"cause":"missing_answer","kind":"backend"}}}
```

**Item 5, the fast-backend interrupt.** Two tests with their claims separated, as on DuckDB:

- `cancel_tests::a_fast_backend_runs_the_poll_within_a_tick` (crate lib tests): the poll closure is wired exactly as `hear_interrupts` wires it — a flag set from another thread, the poll turning it into a token cancel — over 8M records at width 1 on the null backend. Measured discrimination: with the busy-arm tick removed from the stand-in the test fails in **53.74 s** (the batch ran to completion); with the tick it passes in 0.33 s.
- `tests/cancel_fast.py` (end to end): `conn.interrupt()` at 0.5 s into a 1M-row null warm (5.5 s un-interrupted); the statement ends at 0.50 s. Stated limit: SQLite's own step loop aborts between warm flushes on a fast backend, so the message is SQLite's `interrupted`, not `thinkthen cancelled`, and this cannot isolate our tick; the stub-backed wire suite proves the slow-backend shape where our poll carries the stop.

```
$ python3 tests/cancel_fast.py
ok  the stop is an error, not a count
ok  the stop lands within about a tick, past 0.5s (0.50s)
the fast-backend interrupt holds
```

**Item 6, the examples file.** `examples.json` is keyed by function — ten entries, every public function on this surface (decide, choose, score, tag, annotate, details, usage, warm, and the two table-valued functions) — and `tests/examples.py` runs each in its own fresh process on the null backend (the usage counters are process-wide) and checks its answer.

```
$ python3 tests/examples.py
ok       decide
...       (ten lines)
10 of 10 examples ok
```

The check, end to end:

```
$ ./check.sh
== sqlite surface: fast-backend interrupt
== sqlite surface: the function examples
== sqlite surface: wire suite skipped, no stub on 8218
exit 0
```

**The record row, adopted.** The conformance slice now asserts the ruled `{"input","value"}` row on the bulk forms: the value-printing projections read back as `input|value` pairs in input order — `filter`'s kept rows and `decide_many`'s answers where the case carries them (`05`, `19`; case `06`'s empty list is covered by its count). SQL's own two columns are the row; no new function was added.

```
ok       05-filter-keeps-some-of-five rows
ok       19-decide-many-judgments rows
```

## 2026-09-21 — lane B item 8: the SQLite roads, and the load-time floor check

The issue asked for the road around the stale routed headers. Three roads evaluated with the numbers; the chosen one is implemented.

**The roads, evaluated.**
1. **Vendor `sqlite3.c` at our floor** — rejected. A loadable extension is loaded into a process that already holds SQLite (the CLI, Python's module, an app); a vendored amalgamation puts a second SQLite in that process, invites symbol interposition between the two copies, and adds roughly 1.5–2 MB to a 4.4 MB artifact for zero behavior gain. The entry-point rule is unaffected either way; the point of a loadable extension is to ride the host's own routines.
2. **System link with a version check at load** — chosen and implemented (below). The direct link is already the build's shape; the check turns a cryptic loader failure into a named refusal.
3. **The direct link alone (status quo)** — kept as the link shape, superseded by road 2 for the failure mode.

**The check, implemented.** `init` now calls the routed `sqlite3_libversion_number()` (present in the routed bindings — it predates 3.34) and refuses a host below the floor by name through the load error:

```
thinkthen needs SQLite 3.41.0 or newer (the interrupt check uses sqlite3_is_interrupted); this host is 3.40.0 (3040000)
```

The message is built by a pure `version_refusal(host)` so the failure path is unit-tested without an old SQLite; the live call sits in `init` where the loadable API is initialized and is exercised by the stock-CLI load in `check.sh` (host 3.53.4 passes). An old-host live test is not possible on this box — no 3.40 binary exists here; recorded, not pretended.

**Numbers.** `libthinkthen0.so`: 4,654,056 bytes, `DT_NEEDED libsqlite3.so.0` resolving from `/lib/x86_64-linux-gnu/libsqlite3.so.0`; check host: SQLite 3.53.4 (`.runtimes/sqlite3`); floor: 3.41.0 (superseded 2026-09-22: the direct-only floor is 3.50.0, the README and the load check carry it). Tests: 3 passed (the mapping test, the new refusal test, and the existing suite). The full check runs green through the stock-CLI slide load.

## 2026-09-21, the settlement wiring (contract 1fe8173)

- **`SQLITE_DETERMINISTIC` dropped.** The ruled page holds that the functions are volatile; all four registrations that carried the flag — `choose`, `score`, `tag`, `warm` — now use the volatile flags, so no paid call is legal in an index expression or a CHECK constraint. One comment in the registration says why.
- **`nearest` wired into details.** `thinkthen_details` carries `"nearest"` — the nearest level's name for a score question, null otherwise — the contract's settled field. The null suite pins both: `details nearest is null off score` and `details carries the nearest level for a score` (`mid` on the recorded text).
- **The relate rule grammar is grown to the ruled spellings.** The four-bare-names shape is replaced: a single `'@file'` or `'{...}'` argument is the question file's `relate` section through the contract's `from_json`, and each rule argument otherwise takes `NAME`, `NAME=SOURCE:TARGET` with `*` for any end, or the `either:` prefix for the both-ways rule — every spelling translated into the file grammar and parsed by the contract's one reader, so no grammar lives in this shim. A one-end rule refuses with the ruled spelling named. The TVF suite proves all of it: named star ends, the inline spec, the file form, `either:caused_by` reaching the recorded edges, `either:same_as` answering empty below the bar, and the one-end refusal.
- **The named divergences and the gap are on the page.** The shim's session answer map is recorded as the stand-in's divergence that the real engine's cache replaces (map and hit counter deleted together at the swap); the volatile-flag rule is stated; and the missing per-call deadline option is named with case 27's skip, per the settled rule that a gap is named rather than implied.

The full check is green with no stub: build, null suite, the table-valued functions suite, the conformance slice (74 cases), the wire suite skipped by design without the stub.

## 2026-09-21 — the hidden reset removed (punch-list item 4)

The architect's punch list, item 4: SQLite still accepted
`thinkthen_usage('reset')`, which cleared the process counters and the
shim cache — the removed reset capability under another spelling. The
spelling now refuses, the counters are cumulative only, and callers
subtract snapshots.

Code. `usage()` in `src/lib.rs`: the reset branch is gone; any argument
refuses with the usage kind. `'reset'` gets the message that names the
substitution — "the reset spelling is removed; the counters are
cumulative, so take two snapshots and subtract them" — and any other
argument gets "thinkthen_usage takes no arguments". The shim cache stays,
now marked in its doc comments as temporary and deleted together with
its hit counter at the engine swap (the README's named divergence already
said so; the source comments now say it too).

Tests. `tests/null_suite.py`: the pre-warm reset is gone, so the warm
check tells the cumulative truth — two of the five review rows were
already answered by the decide checks above, so warm judges 3 and serves
2 from the session map (both proven as deltas). The reset check now
proves the refusal (message contains "cumulative" and "subtract") and a
new check proves the counters are cumulative: a fresh sentence grows
`requests` by one and never lowers a counter. 23 of 23 pass.
`tests/conformance_driver.py` case 17 is read as deltas between two
snapshots, and its `after_reset` arm — data from the old capability — is
superseded in place by the refusal check.

```
$ ENGINE_NULL=1 python3 tests/conformance_driver.py | grep 17
ok       17-usage-and-cache: sends and served answers counted by delta; the reset spelling refuses (after_reset superseded)
$ ./check.sh
... 73 and 74 green, conformance slice done, wire suite skipped (no stub)
```

The shared conformance file is untouched: case 17 keeps its recorded
`after_reset` arm as history, and the driver names the supersession in
its line. Nothing published, no key, no paid call.

## 2026-09-22 — the authority section and the non-determinism proof (punch-list item 5)

The architect's punch list, item 5, this surface's half: name the
extension's authority, and keep SQLite non-deterministic.

**README.** A new "Authority: who may do what" section names the six
items: question-file access (the one `'@name'` file, against the process
working directory), backend selection (the environment, read at first
call; SQL cannot name a backend), credential source
(`THINKTHEN_API_KEY` at send time; the stand-in reads no key and sends
none), query execution (SQLite's own step loop, the warm pass, the
whole-table relate read, no `SQLITE_DETERMINISTIC` flag), connection
lifetime (process-long, fork repaired by the engine's pid check, idle
pool pruned), and the cancellation channel
(`sqlite3_is_interrupted` through the wait's poll).

**The proof.** `check.sh` gained the volatile-flag section, run in the
stock CLI:

```
== sqlite surface: volatile stays the flag
   flagged|0
   known|8
   Parse error near line 5: unsafe use of thinkthen_decide()
ok       no deterministic flag on any of the eight; an index expression refuses
```

`pragma_function_list` shows the eight functions and zero carrying the
deterministic bit (0x800); an index expression over `thinkthen_decide`
refuses with `unsafe use of thinkthen_decide()`. The check asserts all
three lines, so the flag cannot come back unnoticed.

Cleanup: this surface needs no containers; `docker ps -a` shows no
`laneb-*` left. No key, no paid call, nothing published.


## Entry 21: the review fixes — direct-only, per-connection handles, guards (2026-09-22)

Tried: the SQLite findings of the branch review (group 2 and group 3), each
with a test that would have failed before it.

**Direct-only.** Every function and both table-valued modules are now
`SQLITE_DIRECTONLY` (the scalar and aggregate registrations carry the flag;
the two `connect` bodies call `VTabConfig::DirectOnly`). A view or trigger
inside an attached database can no longer make a paid call or read a file,
whatever the host's `trusted_schema` says — Python's default is on, and the
new `tests/schema_refusal.py` runs there. Before, against the previous
build: `FAIL 2 a view in an attached file refuses`, `FAIL 3 a trigger in an
attached file refuses`, `FAIL 4 a view over the recognize module refuses`.
After: all five checks green, and the control (`top-level SQL still
answers`) proves the rule did not close the ordinary door.

**Per-connection handle.** The single process-wide `sqlite3*` is gone. The
poll reads the calling connection's own handle from SQLite's context
(`Context::get_connection`, the host's `context_db_handle`), so a closed
connection can never be read and two connections cannot confuse each other.
The host's own `is_interrupted` is resolved at load time from its
`sqlite3_api_routines` table: the crate declares the table's tail past the
3.34.1 bindings (`ApiRoutines`: the 13 function-pointer fields between
`txn_state` and `is_interrupted`, in the host header's order, then the
field itself), reads it only after the 3.41 floor check passes, and holds
the function pointer in a static. The direct `#[link(name = "sqlite3")]`
declaration is deleted, so a host that statically links SQLite — the stock
CLI does — is read by its own copy, never a second one. A layout unit test
pins the offsets.

`tests/two_connections.py` proves the behaviour in two modes. The null mode
(two connections, the second closed, the first interrupted) passes on both
builds because SQLite's own step loop also hears a fast-backend interrupt;
the wire mode (`STUB_PORT=8218 ... wire`) is the discriminator — a 300 ms
backend, so only the poll can carry the stop. Measured against the previous
build: `FAIL the poll carries the stop, not SQLite's step loop` and
`FAIL the stop lands within about a tick, past 0.5s (2.42s)`; the new build
passes in 0.91 s with the `thinkthen cancelled` kind.

**Guards.** Every SQL-function body and the vtab callbacks run behind
`guarded`, which turns a panic into the surface's defect error (with the
panic's text) instead of unwinding across SQLite's C frames. A unit test
proves the shape: `guarded("thinkthen_probe", || panic!("the probe blew
up"))` returns `thinkthen defect: thinkthen_probe panicked: the probe blew
up`.

**Phase 1 adopted.** `engine()` is now `Arc<dyn Engine>` built through
`StandinConnector.connect(&EngineConfig::from_env())` — the one line the
merge repoints at the real engine's connector. No deadline door exists on
this surface, so the named gap ("No per-call deadline option yet") stands;
the checked conversion is not reachable from here.

**Case 74.** The stand-in's synthesized partial-failure fixture is armed
only under the test-only `ENGINE_SYNTHETIC_PARTIAL` opt-in now, so the
conformance driver sets it (`tests/conformance_driver.py`); without it case
74 diverges, as it did on every build after phase 1 landed.

Saw (full `./check.sh`, stub up on 8218, exit 0):

```
== sqlite surface: the untrusted-schema refusals
ok  1 this host trusts schema by default
ok  2 a view in an attached file refuses
ok  3 a trigger in an attached file refuses
ok  4 a view over the recognize module refuses
ok  5 top-level SQL still answers
== sqlite surface: two connections, one closed
the per-connection interrupt holds (null)
== sqlite surface: per-connection interrupt on the wire
the per-connection interrupt holds (wire)
== sqlite surface: conformance slice, offline
ok       74-annotate-preserves-good-answers: fields assembled
conformance slice done
```

Means: the group-2 and group-3 SQLite findings are closed with
discriminating tests, and the engine seam now runs through the contract's
connector.

Problem and way round it: none open. `docker ps -a` shows no container from
this surface (it uses none); no key, no paid call, nothing published.

## 2026-09-22 — review 2, the CHECK-constraint hole, and the 3.50.0 floor

**The finding.** Below SQLite 3.50.0 a CHECK constraint in an untrusted
attached database reached the functions, so a file from somewhere else
could spend money or read a file through its own schema (review 2, item
1; `sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md`).
DEFAULT clauses, views, and triggers were refused; CHECK was not.

**The mechanism, from SQLite's own source.** `sqlite3ExprFunctionUsable`
enforces `SQLITE_DIRECTONLY` only on call nodes carrying `EP_FromDDL`. In
`resolveExprStep` (3.49.0 and earlier) the resolver sets that mark only in
the deterministic branch:

```c
if( (pDef->funcFlags & SQLITE_FUNC_CONSTANT)==0 ){
  sqlite3ResolveNotValid(pParse, pNC, "non-deterministic functions",
                         NC_IdxExpr|NC_PartIdx|NC_GenCol, 0, pExpr);
}else{
  pExpr->op2 = pNC->ncFlags & NC_SelfRef;
  if( pNC->ncFlags & NC_FromDDL ) ExprSetProperty(pExpr, EP_FromDDL);   /* skipped for volatile functions */
}
...
if( (pDef->funcFlags & (SQLITE_FUNC_DIRECT|SQLITE_FUNC_UNSAFE))!=0 ... ){
  sqlite3ExprFunctionUsable(pParse, pExpr, pDef);                        /* sees no mark, allows the call */
}
```

3.50.0 moved the marking into the DIRECT/UNSAFE branch (and 3.53.x added
`pParse->prepFlags & SQLITE_PREPARE_FROM_DDL` to the condition):

```c
if( (pDef->funcFlags & (SQLITE_FUNC_DIRECT|SQLITE_FUNC_UNSAFE))!=0 ... ){
  if( pNC->ncFlags & NC_FromDDL ) ExprSetProperty(pExpr, EP_FromDDL);
  sqlite3ExprFunctionUsable(pParse, pExpr, pDef);
}
```

Every function here is volatile (the ruled flag), so every function here
was in the skipping branch.

**The version matrix, by command.** A C harness links each amalgamation,
loads this artifact, attaches a hostile file, and uses each object:

```
cc -O1 -o harness-3.50.0 harness.c sqlite-amalgamation-3500000/sqlite3.c -lpthread -ldl
./harness-3.50.0 target/release/libthinkthen0.so <hostile.db> 1
```

| object | 3.45.1 | 3.49.0 | 3.50.0 | 3.53.2 |
| --- | --- | --- | --- | --- |
| CHECK | **runs** | **runs** | refused at attach | refused at attach |
| DEFAULT | refused | refused | refused | refused |
| view | refused | refused | refused | refused |
| trigger | refused | refused | refused | refused |
| generated column (crafted) | refused | refused | refused | refused |
| index expression (crafted) | refused | refused | refused | refused |
| partial index (crafted) | refused | refused | refused | refused |

The crafted shapes were refused on the old versions too, by SQLite's own
"non-deterministic functions prohibited in generated columns/index
expressions" rule (created with `writable_schema` text, the shape a
crafted file holds).

**The fix.** `FLOOR` moves from 3.41.0 to 3.50.0: the load-time check
refuses an older host by name, so the vulnerable path is unreachable —
the extension cannot load where the hole is live. The check message names
the floor, the host, and the reason. The interrupt call the old floor
guarded (`is_interrupted`, added in 3.41) is unaffected: 3.50.0 carries
it. `An old host` passing at 3.41 can no longer be true, and the unit
test carries 3.45.1 and 3.49.0 as refused, 3.50.0 as the floor.

**The suite.** `tests/schema_refusal.py` now authors every hostile file
through `writable_schema` (a 3.50.0 host refuses to author them with
CREATE, which is the fix working), and runs both `trusted_schema`
settings: for each, CHECK, DEFAULT, view, trigger, crafted generated
column, crafted index expression, crafted partial index, and a view over
the recognize module all refuse; top-level SQL still answers. It also
proves the floor on both sides: the stock host (3.45.1 here) refuses the
load with the floor's message, and the test host is 3.50.0 or newer.

Because the stock Python library here is 3.45.1, the Python tests run
under a 3.50.0 host built once from the amalgamation:
`tests/host_sqlite.sh` compiles `.runtimes/sqlite-amalgamation-*/
sqlite3.c` into `.runtimes/host/libsqlite3.so.0`, and `check.sh` puts it
on `LD_LIBRARY_PATH` when the stock host is below the floor (macOS 26
carries 3.51.0 and needs nothing). The byte-for-byte evidence: the
pre-fix artifact loads on the 3.45.1 host and the suite's four floor
checks fail; the fixed artifact makes them pass.

## 2026-09-22 — review 1/2 leftovers: the single-row deadline and the watcher

The scalar judgment verbs now carry the settled three-argument spelling
beside the drawn two-argument one: the last argument is the per-call
budget in milliseconds, through the contract's one checked door (`-1`
none, `0` spent, positive a budget, any other negative or a
non-number a usage refusal, above 136 years a usage refusal). SQLite
overloads by arity, so no drawn call changes and
`pragma_function_list` shows fourteen registrations for the eight names.
A spent budget refuses before the session answer map, so it cannot be
answered from the shim's cache.

The interruptible wait is a watcher thread: a scalar call's calling
thread sits inside the engine, so nothing on it can read the host's
`is_interrupted` while a send or a backoff sleeps. The watcher reads the
calling connection's own flag (atomic, safe from any thread) every 5 ms,
arms the call's token, and stops with the call through a condvar, so a
call that ends early waits no tick out and no watcher exists between
calls. The engine's own contract is unchanged: no new request starts,
sent requests finish.

**Proof.** `tests/single_row_cancel.py`: the offline arms (spent budget
returns the deadline kind and sends nothing — the usage counter is read
around it; a negative below the sentinel and a non-number refuse; the
sentinel answers; an oversized budget refuses), and the loopback arms
against a local 503 server whose refusal costs a one-second backoff: a
150 ms budget returns the deadline kind in 0.15 s, the same call with no
stop fails later (>= 2 s) with the backend kind, and
`Connection.interrupt` from another thread lands as the cancelled kind in
0.30 s. Against the pre-fix artifact the arity is wrong (the three
arguments do not exist) and the interrupt call fails with the backend
kind after 3.00 s, so every arm is discriminating.

**Shared guard.** The surface's own `guarded`/`panic_text` pair is gone;
`thinkthen_contract::catch_panic` owns the boundary and the message
("a panic crossed thinkthen_probe: ..."), the same spelling every other
surface's door uses.

## The third review's SQLite fixes: the probes, both outputs (2026-09-23)

Probes ran against the pre-fix build (the pushed tip, rebuilt with the
lane's changes stashed) and the fixed build, through the 3.50.0 host
`.runtimes` supplies. Commands and outputs as observed.

**Item 2 — `named_file` read anything, unbounded.** Pre-fix probes
(`/tmp/sqlite_prefix_probe.py`, `ulimit -v` bounded):
`@/dev/zero` → `thinkthen local: cannot read question file "/dev/zero":
out of memory` (the unbounded read ate the address space limit); a fifo
blocked past its 8 s timeout; a 1,048,577-byte file was read whole and
failed at JSON parse — no cap existed. Post-fix (`tests/null_suite.py`,
28/28): `ok  26 a fifo refuses with the uniform message`, `ok  27 an
endless device refuses with the uniform message`, `ok  28 an over-cap
file names the cap`. The refusal names no cause — a missing file and a
permission failure print the same line (the suite's check 14 asserts the
message carries no filesystem cause).

**Item 17 — warm judged every text under the first question.** Pre-fix
probe: `thinkthen_warm` over `(q1, text), (q2, text)` sent 2 requests,
then `thinkthen_decide(q2, text)` sent a third — the `(q2, text)` pair
never landed in the cache (`PRE-FIX warm: requests after warm=2, after
decide(q2,t)=3 -> CACHE MISS (bug)`). Post-fix: the same warm sends its
rounds per question, and the decide answers from the cache
(`ok  warm sent one round per question`, `ok  the second question's pair
is cached` — the counter does not move). A question change now flushes
the group it held (`step`, the `digest` field).

**Item 25 — the interrupt watcher.** Pre-fix probe
(`/tmp/sqlite_watch_probe.py`): 300 uncached calls took 672 ms, and 81 of
200 single calls paid ≥ 4.5 ms (median 0.13 ms, p90 5.22 ms — the 5 ms
stop penalty the review measured at 40%, here 40.5%). Post-fix: the same
300 calls took 3 ms, median 0.01 ms, 0 of 200 over 4.5 ms, and the
process grew exactly one thread (`1→2`, the shared watcher) where the
old shape spawned a thread per uncached call. The one watcher thread
ticks the registry entirely under its lock, so a call's removal
serializes with every read of its connection; the unit test
(`the_watcher_arms_the_token_and_stops_with_the_call`) proves arming and
that a dropped handle leaves the registry at once.

**Packaging text.** The staged artifact's `readelf -d` shows four
`DT_NEEDED` entries and no SQLite among them — the extension reads the
host's API table through the connection at load. The package README now
states the 3.50.0 floor (naming why: below it a CHECK constraint in an
untrusted database reaches the functions) and the no-link story; the old
text claimed a libsqlite3 link and a 3.41 floor.

**Skip-table adoption.** `tests/conformance_driver.py` imports
`conformance/skiptable.py` (`lookup_reason`); the sqlite skip/diverge set
is byte-identical before and after (`diff` empty, 13 lines).

**Full check.** `databases/sqlite/check.sh`: exit 0, 174 ok lines, null
suite 28 of 28.

## Review 4, items 7, 15 — the fourth-review fix wave (2026-09-23)

**Item 7, the check-then-open race** (`named_file`): opened once with
`O_NONBLOCK|O_NOFOLLOW`, checks on the descriptor. Evidence,
fail-then-pass with the reviewer's shapes (tools/file_door.py, against
the .runtimes 3.50.0 host):

- PRE build: `FAIL symlink read through to: thinkthen usage: a question
  file holds one of decide, choose, tag, or score` — the parser ran, so
  the link's target was read; exit 1.
- POST build: `file door: regular reads, symlink refused at the door,
  fifo refused at the door`; exit 0.
- The race hammer (tools/swap_hammer.py, 20,000 reads with symlink and
  fifo flashes): PRE leaked=0 hang=0 (the straddle window is
  nanoseconds; the reviewer caught it at iteration 59 with wire
  timings); POST leaked=0 hang=0. The straddle is closed by
  construction — the open cannot block — and the stable shapes are
  pinned deterministically by the door probe, which runs in check.sh.

**Item 15, the question cache**: named files cache with their
`(mtime, size)` and re-read on change; both caches hold 4,096 entries
with oldest-first eviction. Evidence (tools/question_cache_probe.py):
PRE fails all three — rewrite served the stale parse (rc=100), delete
served the stale parse (rc=100), 50,000 distinct questions grew RSS
222 MB; POST passes — rewrite re-reads (the new bytes' parse error),
delete refuses, growth 36.9 MB (the bound plus statement churn).

**Item 15, interleaved warm**: the aggregate keeps one pending group per
question, so alternating questions accumulate instead of flushing on
every change. The check's warm suite stays green.

**Item 15, empty options**: recognize and relate carry a cancel token
armed by the shared interrupt watcher (the recognize table now passes
the connection handle from connect through the cursor for it).

**Packaging floor**: the rehearsal's `apt-get install sqlite3` gave
3.45.1, which the extension's own floor refuses — the container half
could never pass. The CLI in the container is now built from the pinned
source (sqlite-autoconf-3500200, sha256 in package.sh, verified before
the copy), and the README names both honest install paths (pinned
source, or a host already at the floor — no stock Linux distribution
checked ships one today).

**Test-name discovery**: the nine Python tests glob the built library
(`.so` or `.dylib`) instead of hard-coding `libthinkthen0.so`.

## The seventh review's SQLite row (2026-09-23)

**R4-17, the saved answers had no bound.** The per-process answer map kept every decision, choice, score, tag set, and annotate object. Probe `R4-17b.py` (300,000 distinct 420-byte texts for one question, fresh process): the build at 14f12f5 grew from 13 MB to 218 MB. The map now holds at most 16 MiB, counted as real memory: each key's two strings held twice, each allocation rounded as glibc rounds it, the answer's own text, and the map's and the order's slots counted twice for the room a table keeps after it doubles. With the fix the same probe grew from 13 MB to 30 MB. The oldest answers leave first. `tools/question_cache_probe.py` now drives 200,000 distinct texts and fails past 40 MB of growth; the 14f12f5 build grew 154 MB there and failed. `saved_answer_tests::the_saved_answers_keep_their_byte_budget` pins the cost and the eviction.

Decided: the budget is a constant here, with no setting. The map is the stand-in's and is deleted at the engine swap, and SQLite has no per-connection setting channel this surface already uses. PostgreSQL's table has the same 16 MiB default and a setting, `thinkthen.saved_answer_kb`. Ian can overturn this; the lever is a setting function if a host needs a larger session memory before the swap.
