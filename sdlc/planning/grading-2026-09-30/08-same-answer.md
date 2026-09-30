# Area 8: Same answer on all surfaces

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

One file of 55 shared cases, one loopback backend, and one Rust-derived result schema hold every surface to the same request bytes, answers and results. Each surface runs the cases through its own runner and reports pass, fail, or not run with a reason.

Paths are under `crates/thinkthen/src/` unless they start with `conformance/`, `specification/`, `libraries/`, `databases/` or `crates/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code | `result_json.rs` (147), `schema_forms.rs` (28), `conformance/backend/src/` (1,157: `listener.rs` 443, `arms.rs` 432, `lifetime.rs` 254), `conformance/children/` helpers (about 120), `specification/fixtures/types/check.py` (189) |
| Data | `conformance/cases.json` (7,906), `specification/result.schema.json` (2,000, generated), `specification/fixtures/types/corpus.json` (534), `conformance/settings.json`, `backend-profiles.json`, `calibration.json`, `record-values.json`, `routine-ids.txt` (32 IDs) |
| Tests | `schema_tests.rs` (170, 1 test); `cli/conformance_tests.rs` with `cli/conformance_tests/` (1,985, 9 tests); `conformance/consumer/consumer/tests/public/` (1,692); `conformance/backend/tests/` (41 tests across the backend and the consumer, about 800 lines); `crates/thinkthen/tests/polars/cases.rs`; ten surface runners named below |
| Contract | `specification/types.md` "Parity"; `specification/result.md`; `conformance/README.md`; ADRs 0082, 0112, 0111 (section 5), 0113; ticket 0314 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 3 | About 1,650 nonblank lines of code that is not a test: result writer 147, schema forms 28, backend 1,157, children helpers about 120, type-corpus checker 189 |
| States and concurrency | 3 | The loopback backend runs one thread per connection and gates replies with `held`, `round`, `release` and `wait N` lines (`conformance/README.md` "The loopback backend"); otherwise sequential |
| Rules and refusals | 4 | About 30. 55 cases in 10 verbs, six error kinds, eleven backend arms, the `count` and `wait` line grammar, five bare-value rules, offset units per surface, and six distinct not-run reasons across the runners |
| Surfaces touched | 5 | All 22. Eleven surfaces read `cases.json` directly (command, Rust consumer, Polars, Python, TypeScript, Ruby, R, C door, SQLite, DuckDB, PostgreSQL) and the eleven C-door languages run the 54-case type corpus |
| Settings | 4 | Nine setting cases in `conformance/settings.json` (base, cache, throttle, size, batch, profile, record, replay and related) |
| Contract weight | 4 | 7 spec pages with sections (types, result, settings, question-file, annotate, recognize, records) plus the conformance README, and 4 ADRs, 12 in all |
| Churn and debt | 5 | 162 commits on the area's paths since 2026-09-23, about 10 of them fixes or review answers (for example `1cb43fb7d`, `777872560`, `cf74cc779`). No open issue names the area. The loopback fixtures carry the open HTTP/1.0 reuse debt (Debt 020) |

Mean 4.0, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | C | The schema part is sound: one test derives `specification/result.schema.json` from the Rust types and fails on any byte difference, and `THINKTHEN_WRITE_SCHEMA=1` rewrites and still fails so a green run always compares (`schema_tests.rs:1-6`, `:165-180`). The parity part has gaps on primary paths. `find` is proved only by its chosen unit on the C door (`libraries/c/tests/door/cases.rs:250-257`), although `cases.json` case `18-find-second` lists every candidate probability and the C door cannot return one (`libraries/c/src/call.rs:48`, `:273-289`). Case `18-annotate-two-groups` is run by no surface: the command, Rust consumer, C door, Python, TypeScript, Ruby, R and PostgreSQL skip it, and the SQL runners skip any record case with several exchanges (`libraries/c/tests/door/cases.rs:34`, `conformance/consumer/consumer/tests/public/cases.rs:31`, `libraries/python/tests/conformance.py:37`, `databases/postgresql/tests/runner.py:31`, `libraries/r/tests/conformance.R:33`). Yet `conformance/README.md:11` still says that case "fixes aggregate request identity". Polars runs 24 of 55 and skips 31 (`crates/thinkthen/tests/polars/cases.rs:57-75`). `specification/types.md` "Parity" says port integration runs the corpus and the shared cases through each binding. I read only the corpus run (`libraries/go/fixtures/type_cases.py`), so the claim for the eleven C-door languages is unconfirmed |
| Reliability | C | The backend has its own 41 tests and each surface starts a backend per test, so counts do not leak. Fixed 5 s and 8 s waits moved out of routine runs to `test-stress` (`76b1c3cbe`). A known flake remains: Python's `http.server` replies over HTTP/1.0 and ureq pools the connection, so one DuckDB case failed 11 of 200 runs at load 16 to 20 (`sdlc/issues/2026-09-30-ureq-reuses-a-connection-after-an-http-1-0-reply.md`). Only one harness sends `Connection: close` (`databases/duckdb/tools/verbs_budget.py:53`), while 15 fixture files use `BaseHTTPRequestHandler`. Routine runs select 32 IDs (`conformance/routine-ids.txt`) and the other 23 run only at checkpoints |
| Maintainability | C | One owner for the schema (Rust) and one case file are strong. The matching logic is not shared: each runner re-implements request digests, name swapping and float comparison (`libraries/python/tests/conformance.py:40-60`, `libraries/typescript/tests/cases.mjs:17`, `libraries/ruby/tests/conformance.rb`, `libraries/r/tests/conformance.R:36`, `databases/sqlite/tests/conformance.py`, `databases/duckdb/tools/conformance.py`, `databases/postgresql/tests/runner.py`, `libraries/c/tests/door/cases.rs:686-760`, `conformance/consumer/.../cases.rs`). `libraries/c/tests/door/cases.rs` is 768 lines, over the cap. `cli/conformance_tests/command.rs` (492) and `consumer/.../settings.rs` (480) sit at the cap. Each runner keeps its own not-run table, so the skip story is spread over ten files |

## Strengths

- Rust owns the schema. The schema is generated, compared byte for byte, and the test refuses to pass after a rewrite (`schema_tests.rs:165-180`).
- One writer prints the detailed row for the command and the library, so their bytes agree (`result_json.rs:1-2`).
- Every runner counts pass, fail and not run, requires the three counts to add up to the selected cases, and gives a reason for each skip (`libraries/typescript/tests/conformance.test.mjs:42-48`, `libraries/python/tests/conformance.py:251-268`, `databases/sqlite/tests/conformance.py:283-302`). The PostgreSQL runner has no skip list beyond three named reasons (`databases/postgresql/tests/runner.py:29-31`).
- The loopback backend fails loud on an unknown body, arm or request with status 500 and a stderr line, so drift cannot pass (`conformance/README.md` "The loopback backend").
- The type corpus pins a door request, an independent expected result and a schema verdict for 54 cases, and runs through the real C door (`specification/fixtures/types/README.md`).

## Cleanup

1. **Prove find's probabilities on every surface.** Where: `conformance/cases.json` (cases 18 and 19, `operation.probabilities`), `specification/fixtures/types/corpus.json` (find entries carry only the unit), `libraries/c/tests/door/cases.rs:250-257`. Why: the shared case lists every probability and the C-door runner cannot check it. Whether the other runners check it was not read (unconfirmed). Add it to the corpus and to each door runner once the door returns it (area 7 item 1). Size: M. Blocks 0.1: yes, together with that item.
2. **Re-record or delete `18-annotate-two-groups`, and fix the README sentence.** Where: `conformance/cases.json`, `conformance/README.md:11`. Why: every runner skips it, ADR 0111 section 5 packs the groups into one request, and the README still describes a retired behavior. Size: M, because 11 skip lists change together. Blocks 0.1: no.
3. **Confirm or correct the "Parity" claim for the eleven C-door languages.** Where: `specification/types.md` "Parity" (line 54), each port's `public_types.py` or `type_cases.py`. Why: the text says each binding runs the shared cases, while the ports I read run the 54-case type corpus. State what they run. Size: S. Blocks 0.1: no.
4. **Name Polars's 31 skipped cases in the conformance README.** Where: `crates/thinkthen/tests/polars/cases.rs:57-75`, `conformance/README.md`. Why: the README says every surface reads the same cases, and a reader would not learn that Polars runs 24. Size: S. Blocks 0.1: no.
5. **Share one case-matching helper per language family.** Where: the runners listed under Maintainability. Why: digest, name-swap and close-enough float logic exist in at least six languages, and a change to the case grammar touches all of them. Size: L. Blocks 0.1: no.
6. **Send `Connection: close` from every loopback fixture.** Where: the 15 `BaseHTTPRequestHandler` files. Why: removes the known flake class until ureq-proto changes. Size: M. Blocks 0.1: no.
7. **Refresh stale README paragraphs.** Where: `conformance/README.md` (the sentence that fault cases are "schema contracts until the private engine runner lands with the one-crate merge", and the pure-core test description). Why: the command runner exists and calls production code. Size: S. Blocks 0.1: no.

## Confidence: medium

What was read: `conformance/README.md` in full, the schema test and the two schema forms, the head of `result_json.rs`, cases 18 and 19, the skip tables of ten runners, the command runner's entry, the type corpus and its README, ticket 0314's outcome and evidence, and the issue list.

Not checked: no test ran. I did not read the backend source, the Rust consumer beyond its skip list, `cli/conformance_tests/` beyond the header, the R, Ruby and TypeScript case logic beyond their not-run tables, or ticket 0320 (its file is absent from `sdlc/tickets/`, and the cleanup plan records it as landed at `c911fbe4b`). Whether the ports run the shared cases through their own matrices is unconfirmed. The ten-runner count comes from `rg` on `cases.json`, not from running them.
