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

The conformance slice now runs all forty-one `recognize` cases through the TVF (names; the seven cases that ask relation rules carry the "relations ride the beta question-file function" note) and three of the four `relate` cases (`R01-demo` 4 edges, `R02-pickone` 11, `R04-pairs-10` 10); the per-subject arm is skipped with its reason.

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
