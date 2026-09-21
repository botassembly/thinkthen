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
