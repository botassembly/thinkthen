# DuckDB surface notes

The tag's `NOTES.md` stays at tag `surfaces-wave7-frozen-2026-09-24b` as history. These notes cover the port (ticket 0110).

## Layout

- Every call into DuckDB's C API sits in a file named `ffi.rs`, because `policy.py` allows `unsafe` only there. `src/ffi.rs` holds the types, chunk reads and writes, setting registration, and LOAD. `src/questions/ffi.rs` holds the caller's file system and settings, `src/scalars/ffi.rs` the scalar callbacks, `src/tables/ffi.rs` the usage table and the warm aggregate, and `src/signal/ffi.rs` the SIGINT handler and its install. The ticket's `src/ffi/` folder became these five files.
- `src/scalars.rs` groups each chunk by question and deadline, dedupes texts in first-seen order, and maps answers back by text.
- `src/engines.rs` keeps the engine map. `src/questions.rs` reads questions and `@file`. `src/worker.rs` runs each engine call on a detachable worker. `src/tables.rs` holds usage and warm. `src/errors.rs` holds the one kind table and the one panic guard.

## Choices the ticket did not fix

- The binding depends on `libduckdb-sys` alone. The `duckdb` crate's scalar trait registers no init callback, and every function already goes through the raw C API, so the wrapper and its `arrow` and `hashlink` trees leave. The deny exceptions for `foldhash` and `tiny-keccak` leave with them. Only `zlib-rs` remains, and the deny plant removes that exception.
- A struct result writes every member. A member the verb lacks is `NULL`.
- `thinkthen_details(...).value` is the judgment as JSON text: `"yes"`, `"no"`, or `"unsure"` for decide, the pick or `null` for choose, the position for score, and the label list for tag.
- A `NULL` member inside a list argument makes that row `NULL`.
- An `@file` that is not UTF-8 reads `thinkthen local: the question file PATH was not read: it is not UTF-8 text`.
- A member verb whose one answer failed reads `thinkthen backend: the backend's answer could not be read: CAUSE`, with the cause in plain words.

## Limits

- DuckDB's own `con.interrupt()` does not reach a running engine call. A SIGINT does.
- A SIGINT that lands between a chunked query's last engine call and its return cancels nothing. The query-hook issue names the lever.
- `thinkthen_warm` cannot read a caller's settings, so it refuses `@file`, uses the environment's engine, and sits outside `thinkthen_max_requests_total`.
- Main has no per-call request limit. Under `thinkthen_max_requests_total`, a call sends only as many texts as the total leaves, then refuses with the total's sentence.
- Main's public API refuses `on` in a library question set, so `thinkthen_annotate` reads each record whole. Conformance case `18-annotate-two-groups` does not run here for that reason.
- SQL finds with `ORDER BY` and `LIMIT` over decide, so the find cases do not run here.
- `thinkthen_recognize` takes kind names with no descriptions. The conformance runner checks recognize cases through `thinkthen_relations`, which reads the case's own question file.

## Tests

Every suite under `tools/` starts its own loopback backend and runs each child with a fake key, a loopback address, and fresh XDG folders. `harness.guard` refuses anything else. `check.sh` runs them after the builds, the source checks, deny, and one call through the stock CLI.
