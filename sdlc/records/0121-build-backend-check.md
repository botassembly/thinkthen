# 0121: Build the backend check command

Status: built, code review pending. Owner: Claude.

Branch `ticket/0121-backend-check`. The build merges `origin/ticket/0086-public-rust-api` at `cd95f138`. Ticket 0086 had not landed when this was built. Ian can overturn every choice this record marks as decided.

## Result

`thinkthen check [--url BASE] [--model NAME] [--timeout SECONDS] [--dry-run]` sends four fixed probes to `BASE/systemone` and prints the eight-row report of `specification/check.md`.

- `core/check.rs` holds the four probes and the report. Each probe is question-set text that the production `QuestionSet::parse` reads, over fixed made-up parcel evidence. `Report` grades a decoded reply, a probe failure, or a stop, and it renders the rows and the count line.
- `cli/check.rs` resolves the address, the model, and the timeout. It builds one engine with no profile, no storage, and the default two retries. For each probe it takes the one chunk from `Engine::split`. A dry run prints that chunk's body. A live run sends the same chunk with `Engine::ask_chunks` and maps the engine error to a row. A missing key, a cancel, and every other engine error leave the command as the failure every command reports.
- The command takes each failure sentence from `cli/failure.rs` through its one reporter, and strips the `thinkthen: ` prefix. No sentence is written twice.
- `conformance/backend` gains `/arm/full/v1` and `/arm/status/CODE/v1`. `generic` takes a `full` flag. The existing arms pass `false` and answer as before.

### Choices made here

- The row names `noul`, `choice`, and `score` are wire vocabulary. The seam policy keeps `noul` inside the adapter. So the adapter gains `wire_type`, which names the type a question travels as, and a one-question probe takes its row name from it. The report builds its rows from the probes. The seam table is unchanged. Ian can overturn this for a seam allowance.
- `Answer::confidence` lost its `#[cfg(test)]`. The grader reads it.
- `wire_name` in the adapter became `pub(crate)`. A failed logical question names its wire range with it.
- The engine is built inline. `asking::engine` takes a judging verb's `Common`, which `check` does not carry.
- The no-address test sets no key. A regression that fell back to the built-in address then still sends nothing to the hosted service.

## Lines and the ratchet

The merge of the 0086 branch measures 60290 nonblank lines. The build measures 60855, so it adds 565, under the 570 the ticket and the coordinator allow. The ratchet commit says 60855 and 565. Its per-part split has a slip, corrected here.

| Part | Budget | Nonblank lines |
| --- | --- | --- |
| `core/check.rs`, with its `mod` line and the `cfg(test)` line removed from `answer.rs` | 110 | 173 |
| `cli/check.rs` | 140 | 113 |
| Wiring in `cli/args` and `cli/mod.rs` | 30 | 28 |
| `conformance/backend/src/arms.rs`, net | 30 | 15 |
| The adapter's `wire_type` | none | 10 |
| `tests/backend/check.rs` and its `mod` lines | 260 | 226 |
| Total | 570 | 565 |

Re-score: `core/check.rs` crosses its 110-line budget by 63. The four probe question sets take 22 lines as rustfmt lays them out, and the report state and grader take the rest. The command and the tests came in 27 and 34 under. The total stays inside 570, which is the ceiling the coordinator set. The queue owner accepts the shift between parts. Ian can overturn it.

## Tests

Every test is in `crates/thinkthen/tests/backend/check.rs`. Each starts its own loopback backend and runs the built command through the harness, with `THINKTHEN_API_KEY=sk-check-0121` or no key. The helper sets `XDG_CACHE_HOME` to a fresh folder. After every run it asserts that the key's bytes appear in neither output stream nor any file under that folder, which holds the usage totals the run wrote. Each test pins the whole standard output.

Four questions, answered once for the file. Each test protects the behavior its name states. The credible regression is the planted bug below. No earlier test runs `check`. None needs a test-only hook: the arms are the real boundary, and `THINKTHEN_TEST_RETRY_WAIT_MS` already shortens retry waits. The pure report has no unit tests, since the command tests cover every row.

## Planted bugs

PLANTS

## Ladder

LADDER

## Review

REVIEW
