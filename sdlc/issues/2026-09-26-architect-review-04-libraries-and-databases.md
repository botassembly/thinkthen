Status: open. Filed 2026-09-26 by the marketing lead from a fresh architect review. Ticket 0168 (`sdlc/tickets/0168-a-stop-reaches-every-caller.md`) carries the SQLite held permit.

# Architect review 04: libraries and databases

A fresh reviewer tested the Rust, Python, C and other libraries and the DuckDB, SQLite and PostgreSQL extensions as an architect who calls ThinkThen from code or SQL. The review ran against main `9d652bed`. It rated the topic fair and found 0 severity 1, 4 severity 2 and 12 severity 3 issues. The full detail sits in the architect review report 273, 04. Work file names such as `04-work/p12.sql` refer to the report's local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. Libraries and SQL drop a question file's calibration `profile`, so the question identity differs from the command's (severity 2)

Evidence. `04-work/py4.py` and a DuckDB query over the same text. With `{"decide": "...", "profile": "jev-bench"}`, the command's `question_sha256` is `6a0e5df25f5e...`, and Python's and DuckDB's are `968a10461272...`. Without `profile`, all three give `968a1046...`. Code: `crates/thinkthen/src/public/results.rs:209-210` and `:226`.

What an integrator hits. Audit rows or dashboards joined on `question_sha256` split one question into two across surfaces. The calibration guard that warns when a threshold runs on another backend never fires in a library or database.

Direction. Carry the calibration name through the public question type, and add a shared conformance case with `profile` set.

## 2. The PostgreSQL README states a throttle range that the engine refuses (severity 2)

Evidence. `databases/postgresql/README.md:40` says "0 to 32". The setting accepts 0 (`databases/postgresql/src/call.rs:399-404`). The builder refuses 0. SQLite shows the builder sentence: `thinkthen_throttle(0)` gives `a throttle is a whole number from 1 through 32`.

What an integrator hits. An administrator sets `thinkthen.throttle = 0`. The setting is accepted and every later call fails.

Direction. Set the setting's minimum to 1 and fix the README row.

Fixed by Quick Fix qf-review-273 in commit `417cbde5`, from branch `ticket/qf-review-273`. A check on the setting refuses any value other than -1 and 1 through 32 where it is set, with the engine's sentence. The minimum did not move to 1, because -1 stays the unset value that leaves the engine's throttle. `SET` fails, and a configuration file's bad value draws a warning and leaves -1. SQLite has no such hole, because `thinkthen_throttle(0)` refuses at the setter call. DuckDB has no check step for an extension setting, so its `SET` succeeds and the next call refuses, as `databases/duckdb/README.md` says. The record is `sdlc/records/qf-review-273.md`.

## 3. DuckDB's `thinkthen_warm` ignores session settings (severity 3, carried here for two reviews)

Reviews 04 (I8) and 08 (issue 13) both found this. `04-work/p12.sql`: warm sent 10 requests under a total of 2. `04-work/p13.sql`: with `SET thinkthen_cache`, warm and then decide sent 20 requests for 10 texts. `databases/duckdb/README.md:40` and `:64` mention the bypass in the fine print. The budget control and the "warm first" pattern do not combine, so a caller pays twice or runs past the budget. `2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md` item 2 already covers part of this. Direction: refuse warm when a session cache or total is set, or give warm its settings as arguments.

## Already filed

- A cancelled Rust or C call can return success (review 04, I2, severity 2). See `2026-09-26-cancelled-c-scalar-call-can-return-success.md`. The reviewer verified the code path at `public/options.rs:241-250` and `engine/mod.rs:156-160`.
- `cache_bytes` does nothing, and three database READMEs call it a cap (reviews 04 I3 and 08 issue 5, severity 2). See `2026-09-26-settings-some-surfaces-cannot-reach.md` item 3 and `2026-09-25-public-library-api-gaps.md` item 4. Review 08 adds one fact: ADR 0017 line 87 says a warm pass "evicts its own answers", and that sentence needs a correction too. In a fresh SQLite process, `SELECT thinkthen_cache_bytes(1);` returns 1 and leaves a 1,913-byte cache untouched.
- Libraries and SQL cannot set the timeout, retries, a backend profile, replay-only or record-only (reviews 04 I11 and 07 I11). See `2026-09-26-settings-some-surfaces-cannot-reach.md` item 1 and tickets 0148 and 0149.

## Severity 3 titles

- In SQL, one failing row fails the whole statement, and there is no per-row error value. `04-work/p3.sql`: one 422 row among 3,000 sent 1,503 requests and failed, and `TRY()` is refused for volatile functions.
- A deadline bounds a call, not a query, and DuckDB and SQLite have no query-wide bound that reaches a running call.
- SQLite and PostgreSQL scalars hold one request in flight per statement. 200 rows took 22.1 s as scalars and 2.9 s after `thinkthen_warm`.
- DuckDB's `thinkthen_warm` ignores session settings. Carried above as item 3.
- DuckDB refuses every call in the process after 16 distinct engine settings (`databases/duckdb/src/engines.rs:26`).
- The throttle belongs to the process: the first value sticks for its whole life, and each loaded copy has its own.
- Libraries and SQL cannot set the timeout, retries, backend profile, replay-only or record-only. Already filed.
- A SQLite cancel returns at once, but the detached worker keeps its request and its throttle permit.
- DuckDB's relate cannot see the caller's open transaction and prints DuckDB's misleading message for an uncommitted table.
- Bulk gaps: the list calls refuse choose, score and tag over many texts, and frame recognize runs one text at a time.
- The missing-key error has a different kind on the command than on every library and extension, and its advice names a Rust-only call.
- Packaging and loading sharp edges, such as DuckDB loading only with `-unsigned` and a `thinkthen.so` in the working folder shadowing the Python package. Release work is tracked in `2026-09-25-release-and-install-for-0-1.md`.
