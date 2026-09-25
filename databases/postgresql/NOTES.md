# Notes on the PostgreSQL port

Ticket 0111 ported this surface from tag `surfaces-wave7-frozen-2026-09-24b` onto the public `thinkthen` API. The tag's `NOTES.md` stays at the tag as history. `sdlc/records/0111-build-postgresql-surface.md` holds the measurements.

## What the port found

- Each `Engine` counts its own sends. The ticket assumed one process total. The binding keeps every engine a backend built and adds their counters in `thinkthen_usage()`, so the totals carry across a rebuild.
- `decide_many` streams. A batch over `max_requests` sends its first records before the engine refuses. The binding holds the records, so it checks the count before any send and refuses with the engine's sentence.
- A request total per backend (Ian's ruling of 2026-09-25) needs the counters of every engine, so the binding keeps one engine per settings plan and records it right after the build. A cancelled call's sends then still count.
- `thinkthen::Error` has no public constructor. The binding's own refusals use a small `Refusal` type with the same kind, message, and retry signal.
- The public API has no parser for the command's inline relate rules. `relate.rs` repeats the `NAME` and `NAME=SOURCE:TARGET` grammar.
- `cargo pgrx package` names the extension after the one `.control` file in the folder. The package `thinkthen-postgresql` still ships `thinkthen.so` and `thinkthen--0.0.1.sql`.
- pgrx cannot read a set's columns through a type alias. A `TableIterator` return type is spelled out in each `#[pg_extern]` signature.
- A `SET` of a `thinkthen.*` name before the library loads makes a placeholder, and PostgreSQL checks no range then. The range test loads the library first.
- The loopback backend answers about 41 ms after each request on a reused connection. `curl` sees the same delay, so it sits in the backend, not the engine. A 20,000-row warm at throttle 32 takes 46 to 52 s, over the ticket's 30 s bound. A second warm over the same 20,000 cached rows takes 2.5 s, so the aggregate's text state is not the cause. A Quick Fix on main sets `TCP_NODELAY` in the backend, and the step has not run with it yet.
- A 20,000-row scalar decide over cached answers takes about 1.05 s, about 52 µs a row, under the ticket's 100 µs stop.

## How the check runs

- `runtime.sh` checks the pinned server package, extracts it once, and compares its version with `/usr/bin/pg_config`. A mismatch or a missing tool reports "not run" with the fetch command.
- Every test gets a fresh server start, a fresh loopback backend, and a fresh cache folder. The start refuses any address whose host is not 127.0.0.1 and gives the server a fake key.
- Each step runs in a subshell under errexit, so its first failing line fails it.
- The conformance runner restarts the server for each case on that case's arm. Filter, rank, and find cases report "not run", because SQL spells them with `WHERE` and `ORDER BY`. Case 23 reports "not run", because SQL has no token. Case 25 reports "not run", because no outside boundary reaches a defect. Case `18-annotate-two-groups` reports "not run", because a SQL call's evidence is one whole text.
