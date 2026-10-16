# 0601 — performance-and-memory-tradeoffs

Status: OPEN.

Milestone: 0.3

Draft: filed 2026-10-16 by the 0.2 PM at Ian's request, for the 0.3 team to order.

## Outcome

For each item below, the 0.3 team does one of two things. It either documents the right way to use ThinkThen clearly enough that a person or a coding agent finds it at the point of use, or it ships a better design that removes the cost. It records which one it chose and why.

## Evidence

- Ian, 2026-10-16, after the 0.2 performance and memory review: "Here's the issue, here's the workaround, or how you do it the right way, or how you document it. Then, what are some potential options that an investigation could try to solve, and what would the ideal solution be?" The 0.3 team owns the investigation; this ticket does not prescribe the fix.
- None of these blocks 0.2. Each is shipped behavior, known and accepted. Read every path on current main before acting; the references below were read on `ticket/0543-final` on 2026-10-15 and 2026-10-16.

## Items

### 1. Plain SQL calls in SQLite and PostgreSQL send one request per row

- Issue: `thinkthen_decide(question, body)` over a SQLite or PostgreSQL table judges each row in its own call. A 10,000-row table makes 10,000 requests and pays for the question 10,000 times. DuckDB groups the rows of each chunk that share a question and settings into one engine call and asks a repeated input once (`databases/duckdb/cpp/src/portable.cpp`, `databases/duckdb/bridge/src/ffi/scalar/ffi.rs`), so the same SQL is cheap there and costly here.
- Right way today: gather the column once and join back. `thinkthen_decide_many(question, (SELECT json_group_object(id, body) FROM t))` in SQLite, or `jsonb_object_agg` in PostgreSQL, then join on `key`. Ticket 0597 teaches this on every database page and on a "Batching and cost" page.
- Options to investigate: a SQLite or PostgreSQL aggregate or window form that packs a column the way DuckDB's scalar does; a planner hint; a warning in the call's facts when a statement makes many single-row calls with one question; batching across rows inside one statement through the engine's queue.
- Ideal: the natural `SELECT thinkthen_decide(q, body) FROM t` packs rows on every SQL host, so the cheap form is also the obvious one.

### 2. A session holds unread answers in memory

- Issue: ticket 0598 makes a push never wait on unread answers. The session keeps finished answers in order until the caller reads them, so memory grows with what is unread. Progress messages share that queue and are held too. Release QA for 0.2 measures 100,000 records pushed before any read.
- Right way today: read answers while feeding, or use the library's iterator or stream call, which feeds and reads for you.
- Options to investigate: a configurable cap that turns into a clear "read answers first" error instead of growth; spilling held answers to the cache folder; dropping held progress messages; making the iterator call the only documented front door (0599 item 12).
- Ideal: unread answers cost a small, bounded amount, and a caller who exceeds it gets a plain message that names the fix.

### 3. SQLite keeps eight batched results per connection

- Issue: each SQLite connection keeps its eight most recently used `_many` and `thinkthen_rank` results, each with its question, keyed input and settings bytes, key map and answer rows (`databases/sqlite/README.md`, `databases/sqlite/src/many.rs`). Eight slots bound the count, not the bytes, so a very large batch stays in memory until the connection closes or eight newer results push it out.
- Right way today: the README states it. Close the connection after a large batch, or split very large inputs.
- Options to investigate: a byte budget across slots; keep only the answers and key map, not the input bytes; let a slot fall back to the on-disk answer cache; a setting to turn reuse off.
- Ideal: reuse stays fast for repeated scans, and its memory has a stated byte bound.

### 4. Rank, find, recognize and relate answer only after all input arrives

- Issue: these functions need the whole set before answering. Library sessions, SQL, and the command line without `--top` hold every judged record before rank sorts. The command line's `--top N` keeps only the N best (`crates/thinkthen/src/public/complete/rank.rs`). Find compares all units at once. A caller who waits for an answer before finishing its input hangs (0599 item 1). An endless stream cannot be ranked.
- Right way today: rank a bounded window, or use `--top N` on the command line. `--top N` bounds kept rows but not the records judged. Finish input before reading.
- Options to investigate: rank by windows with a merge, or carry the command line's top-N that keeps only N records to library sessions and SQL; a usage error when a caller reads before finishing on these functions; documentation on each function's page that says "answers come after input ends."
- Ideal: no hang on any reading order, and memory for a top-N rank grows with N, not with the input.

### 5. SQL details rows repeat each call's facts

- Issue: in SQL, each details row carries the facts of the call that produced it, so a 10,000-row result repeats one call's facts 10,000 times (`databases/duckdb/README.md`: "`facts` belongs to the native call and repeats across its rows. Read it once"; `databases/sqlite/README.md`; `databases/postgresql/README.md`). 0.2 already moved rank and relate observations to the row with ordinal zero after one DuckDB case used 15 GB.
- Right way today: read facts from one row.
- Options to investigate: facts only on the row with ordinal zero, like observations; a separate facts function keyed by call; a compact facts reference.
- Ideal: per-row results carry only per-row data, and call facts appear once.

### 6. Leaving a Python stream early keeps its session alive

- Issue: breaking out of `Engine.iterate` without `close` keeps the session open until `engine.close` (0599 item 6). The engine holds each open session in a set, and neither the stream nor the session has a finalizer, so garbage collection does not free it (`libraries/python/thinkthen/__init__.py`, `libraries/python/thinkthen/_calls.py`).
- Right way today: use `with` or call `close` on the stream.
- Options to investigate: close the session when the iterator is garbage collected or exits a loop early, which needs the engine to hold sessions weakly; warn when an engine closes with open sessions.
- Ideal: an abandoned stream frees its session promptly with no special code.

### 7. An occasional slow cache save on Windows

- Issue: hosted Windows runs once measured a 1.412-second answer-cache storage transaction against a usual of about 15 ms. Two investigations found no ThinkThen defect. One run in ten failed a cancellation check from it during 0.2 qualification.
- Right way today: none needed by users; the 0.2 release record names it.
- Options to investigate: measure on a real Windows machine rather than a hosted runner; check antivirus scanning and file-system flush behavior; batch cache writes; move the write off the answer path.
- Ideal: cache saves never delay an answer, on any platform.

## Scope

- Changes: decided per item by the 0.3 team. Documentation goes with 0597's pages where it fits.
- Keeps: every request body and answer, unless an item's chosen design says otherwise and its ADR is accepted.

## Proof

For each item, either a documentation page a fresh reader uses correctly on the first try, or a measured before-and-after under the release stress suite at 10,000 and 100,000 records.

## Defers

The 0.2.1 documentation work in ticket 0597 covers today's right ways, with no release bump. The other session footguns are in ticket 0599.
