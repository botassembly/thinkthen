---
flow: build
priority: 134
opens: crates/thinkthen/src/public/relate.rs crates/thinkthen/tests/public_members.rs crates/thinkthen/tests/polars/door.rs libraries/python/Cargo.toml libraries/python/Cargo.lock libraries/python/src/arrow/write.rs libraries/python/src/frame.rs libraries/python/tests/test_door.py libraries/python/NOTES.md libraries/python/ratchet.json libraries/python/ratchet.py.json sdlc/ratchet.json sdlc/issues sdlc/records sdlc/tickets
---

# 0134: Withhold a relate entity's kind, and make Python Polars write the column table

Status: ready. Owner: Claude.

Lane: thinkthen-lane-2

Review route: a fresh read-only Claude session reviews this design and the final diff. The Python binding gains a direct dependency on `serde_json`, and a public type's `Debug` line changes, so the code review names what it checked (repo `CLAUDE.md`). The coordinator's code review is that second review.

## Outcome and authority

Two fixes that a 0.1 user would hit:

1. A library user who formats a public `thinkthen::Entity` with `{:?}` sees neither its name nor its kind. Today the kind prints in clear (`sdlc/issues/2026-09-25-the-public-entity-debug-prints-its-kind.md`).
2. `annotate` over a Polars frame writes the same column text through the Python door as through the Rust door, by ADR 0047 item 10. Today Python prints a widened score of 1.0 as `1`, builds the failed marker from its own cause table, and writes a frame's tag column as a list column. This is item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md` and backlog item A1 in `sdlc/planning/issue-backlog-2026-09-25.md`.

Authority: the backlog puts item 8 first in section A, "before 0.1 ships". The entity issue is a secrecy defect, and the repo `CLAUDE.md` requires a secrecy test over every `Debug` line. The coordinator assigned both issues to this ticket and asked it to take what 0.1 needs and defer the rest with reasons.

## Scope

In: the entity issue whole, and item 8 of the API gaps issue.

Out, each named under "Deferred gaps" with its reason: items 1 to 7 and 9 of the API gaps issue, and Ian's 2026-09-25 settings sweep that goes with item 4.

Item 4 stops at a file another builder owns. Removing `cache_bytes` changes `databases/postgresql/check.sh` lines 488 and 489, which pin the `thinkthen.cache_bytes = 0` refusal. The flaky-test Quick Fix owns that script. This ticket does not touch it and hands item 4 back to the coordinator.

## Prior evidence

- ADR 0047 item 10, amended 2026-09-25: the Polars column table. A widened cell "takes its text unchanged from `AnnotatedRecord::value_json`, the engine's one serializer. No door keeps its own table of failure causes." Tag in a frame is "`String` holding the JSON array text".
- Ticket 0130 moved the Rust door into `thinkthen`. Its `public/frame/column.rs` already follows the table. It parses the record's `value_json` into raw members and takes a widened cell or a frame tag cell from the member's raw text. Its lane test `a_failed_question_widens_to_the_markers_text` pins the marker and a frame tag cell on shared case `17-annotate-partial`. That case widens only a choose column, so no test pins a widened score, decide, or tag cell through either door.
- Ticket 0095, "Result JSON": bare scalar values get no public method. Each door writes them, checked against the command. This ticket keeps that ruling and adds no public method.
- Ticket 0106 built the Python door. `libraries/python/src/arrow/write.rs` `widened` formats each widened cell itself: `position.to_string()` for a score, a hand-built marker from `crate::engine::cause`. `annotated` writes a tag as `Cells::Lists` for every caller.
- Ticket 0122 shares `frame.rs`'s `answered` between a Polars frame and a pandas frame. Its README and tests fix a pandas tag column as `object` with one list per row. The ADR table is the Polars table, so pandas keeps its lists.
- `libraries/python/tests/conformance.py:120` reads a widened cell with `json.loads`, so it cannot see number text. `libraries/python/tests/test_secrecy.py:31` already starts its own `http.server` listener inside a test child, which is the precedent for the Python proof below.
- Four other bindings already depend on `serde_json` directly: C (with `raw_value`), TypeScript, SQLite, and DuckDB. Python's lock already holds `serde_json` 1.0.151 with `raw_value` through `thinkthen`.
- The core `RelationEntity` `Debug` (`crates/thinkthen/src/core/relation.rs:19`) withholds name and kind through `Withheld`.

## Decisions

1. **The entity's kind is withheld.** `Entity`'s `Debug` prints `kind` as `Withheld(self.kind.len())`, as it already prints `name`. The line becomes `Entity { name: <N bytes withheld>, kind: <M bytes withheld> }`. The accessors `name()` and `kind()` do not change.
2. **Python takes member text from `value_json`, as the Rust door does.** `frame.rs` parses each record's `value_json()` once into its raw members, with `serde_json` and `RawValue`. It hands each member's raw text beside its `Annotated` value to `arrow::annotated`. The Python crate adds `serde_json = { version = "1.0.151", features = ["raw_value"] }`, the C binding's exact line. The lock gains one dependency edge and no package.
   - The other choice was a public `NamedAnnotation::value_json()`. It would retire the Rust door's parse too, but it reverses ticket 0095's ruling, widens the frozen inventory, and adds a `Debug` field. Ian can overturn this choice.
3. **One cell rule for both doors.** A widened cell, and a tag cell in a Polars frame, is the member's raw text, except that a choice holds its plain label and a raw `null` (not sure, nothing fits) is a null cell. This is the Rust door's `widened` rule. Python's own `widened`, `quoted`, and its use of `engine::cause` on the column path go.
4. **The tag column follows the caller.** A Polars frame's tag column is `String` holding the JSON array text. A Polars `Series` from `tag`, a pandas `Series`, and a pandas frame keep one list per row (tickets 0106 and 0122). `arrow::annotated` takes a two-way `Tags` switch, `Lists` or `Text`. `_annotate_frame` passes `Text`, and every other caller passes `Lists`.
5. **The list form keeps its dict.** `engine::annotated` builds a failed member as a Python `dict` from `engine::cause` for the list return. That is not the column table, and the shared cases already compare it with the engine's `bare` value (`conformance.py:113`). It stays. See "Deferred gaps".
6. **Both doors pin one table.** A new Rust lane test and a new Python test each serve the same three records from a loopback listener that answers per record, and each pins the same cell literals. Equal literals in both tests are the byte-identical proof.

## Edge cases

The table both tests pin. The set is `late` (decide, band `0.1:0.9`), `urgency` (score over three levels), `kinds` (tag over `bill` and `ship`, cut 0.5), and `team` (choose over `billing` and `shipping`). The listener answers each record by its text. It omits every question whose text the record names, and answers the rest in full: yes 0.95 or 0.5, the score's whole mass on the middle level, `bill` 0.9 and `ship` 0.1, `billing` 0.9.

| Record | `late` | `urgency` | `kinds` | `team` |
| --- | --- | --- | --- | --- |
| all answered | `true` | `1.0` | `["bill"]` | `billing` |
| omits `late` and `urgency` | marker | marker | `["bill"]` | `billing` |
| not sure, omits `kinds` and `team` | null | `1.0` | marker | marker |

Every column widens, because each has a failed row. The marker is the engine's `value_json` text for a missing answer, `{"failed":{"kind":"backend","cause":"missing_answer"}}`, and the build pins the exact text the engine gives. Over the first record alone nothing widens: `late` is `Boolean` true, `urgency` is `Float64` 1.0, `kinds` is `String` `["bill"]` in a Polars frame, and `team` is `String` `billing`.

Other inputs:

| Input | Result |
| --- | --- |
| Tag with no label past the cut, in a Polars frame | `[]`, as the Rust door writes it |
| A one-question Series call whose answer failed | `BackendError`, unchanged (0106, 0122) |
| pandas frame with a failed question | `string` column of the same member text; tag stays lists when nothing failed |
| `Entity::new` with a blank name or kind | `Usage`, unchanged |

## Proof

| # | Test and where it runs | What it proves | Planted fault that turns it red |
| --- | --- | --- | --- |
| 1 | `public_members.rs` `a_builders_debug_line_withholds_its_question_labels_and_descriptions` gains one `Entity` row and one pinned line, `test` rung | An entity built with a marked name and kind prints no marker, and its plain and pretty lines equal the pinned text exactly | Print `kind` in clear again |
| 2 | `test_door.py` `test_a_frame_writes_the_column_table`, `surfaces` rung (Python lane) | The edge-case table through the Python door: widened decide, score 1.0, tag, choose, marker, and the unwidened frame dtypes with tag as text. Cells compared as exact strings, not after `json.loads` | (a) Restore Python's own `widened`: the score cell reads `1`. (b) Pass `Tags::Lists` from `_annotate_frame`: the unwidened `kinds` dtype reads `List(String)` |
| 3 | `polars/door.rs` `a_widened_frame_holds_the_engines_text`, `surfaces` rung (Rust Polars lane) | The same table through the Rust door, with the same literals as row 2 | Write a widened score with `f64` display in `column.rs`: the cell reads `1` |
| 4 | The existing Python tests: `test_a_failed_question_widens_its_column_to_text` (pandas), `test_annotate_on_a_frame_keeps_every_row_and_column`, `test_a_column_answers_as_its_list_does`, and `conformance.py` | pandas keeps list tags and its marker, a Series `tag` keeps a list, and the shared cases still pass | None new. They must stay green |

The four questions for each new test:

- **Row 1.** It protects the promise that a public `Debug` line prints no caller text. A credible regression is the pre-fix line, or a new field printed in clear. No test formats a public `Entity` today: the command's secrecy sweep never meets one. No test-only hook: it builds an `Entity` through `Entity::new`.
- **Row 2.** It protects ADR 0047 item 10 on the Python door. A credible regression is the pre-fix code: its own score text, its own marker, a list tag column. `conformance.py` parses widened cells with `json.loads` and the pandas test widens only a choose column, so neither sees number text or a frame tag. No test-only hook: it runs the public `tt.annotate` against a loopback `http.server` in the test child, as `test_secrecy.py` does.
- **Row 3.** It protects the same table on the Rust door. A credible regression is a door that formats a cell itself. The existing lane test widens only a choose column. No test-only hook: it serves `common::engine` from `conformance_backend::Listener::answering`.
- **Row 4.** No new test.

The build runs each plant, records it red, and restores it.

## Budgets, in nonblank lines

Measured today: `write.rs` 546, `frame.rs` 349, `test_door.py` 159, `public_members.rs` 257, `relate.rs` 272, `polars/door.rs` 277, `NOTES.md` 66.

- `relate.rs`: 1 changed line, 0 added.
- `write.rs`: at least 15 fewer. It loses `widened`, `quoted`, and the cause lookup, and gains the `Tags` switch and the shared cell rule.
- `frame.rs`: at most 25 added.
- `libraries/python/Cargo.toml`: 1 added. `Cargo.lock`: 1 added, and no package added or moved.
- `test_door.py`: at most 50 added. `public_members.rs`: at most 12 added. `polars/door.rs`: at most 60 added.
- `NOTES.md`: at most 3 added. The issue edits and the record are outside the budget.
- Ratchets equal the measured counts. The root ceiling (66,404) rises by at most 72. Python's Rust ceiling (4,604) moves by the `write.rs` and `frame.rs` net. Python's Python ceiling (2,253) rises by at most 50.

## Stop rules

Stop and bring it back when any of these happens.

1. A fix needs a file another running builder owns: ticket 0132's `engine/http.rs`, backend tests, `specification/backends.md`, `specification/check.md`, or `public_controls.rs`; the flaky-test Quick Fix's `libraries/c/tests/door`, `databases/postgresql/check.sh`, or `libraries/polars/tests/throttle_equality.rs`; or ticket 0133's rung scripts, children check, or interrupt handler test switch.
2. The fix needs a public API change, a new crate in any lock, or a change to any surface but Python.
3. The engine's `value_json` text differs from the table above in a way the ADR does not describe.
4. A test needs a test-only hook.
5. A budget would be crossed.
6. An existing Python or Rust lane test changes its expected text.

## Overlaps

| Owner | Shared | Note |
| --- | --- | --- |
| 0132 | `sdlc/ratchet.json` | Re-measure the ceiling at merge |
| 0133 | `sdlc/ratchet.json` | Same |
| Flaky-test Quick Fix | `sdlc/ratchet.json`, and `crates/thinkthen/tests/polars/` as a folder | This ticket edits `door.rs` only. The Quick Fix edits `throttle_equality.rs` |

## Deferred gaps

Items of `sdlc/issues/2026-09-25-public-library-api-gaps.md`, each left open in that issue:

- **Item 4, `cache_bytes`, and the settings sweep.** Ian ruled it out of 0.1. It is 0.1 work, but it needs `databases/postgresql/check.sh`, which the flaky-test Quick Fix owns (stop rule 1). It also spans the public API, 0084, and every binding with the setter: Python, Ruby, R, TypeScript, SQLite, DuckDB, and PostgreSQL. It gets its own ticket once the Quick Fix lands.
- **Item 1, counters per engine or per process.** The backlog pairs it with the status issue's option 1, which builds on the place item 1 picks. Both go in one ticket.
- **Item 2, a public inline rule parser; item 3, public error constructors; item 9, `LoadedQuestion` as a decision question.** Each adds to the public API without breaking it. Each can ship after 0.1, and the backlog puts them in section C.
- **Item 5, find's none option and annotate parts.** Each needs a new spelling in 0084 and adds to the API. Section C.
- **Item 6, the engine reuse test and a library `warm`.** The product reading of 2026-09-22 set it low. A library `warm` is a new method, and the counting test needs every surface's harness.
- **Item 7, typed descriptions and annotate forms outside Rust.** A cross-surface design over Python and TypeScript with a digest test. Section C.
- **The list form's failed `dict`.** Python still builds it from `engine::cause`. The shared cases compare it with the engine's `bare` value, so it cannot drift unseen. Building it from `value_json` is a later cleanup.
- **The Rust door's own parse.** `column.rs` keeps its `member` parse, and Python gains a twin. A public per-member JSON method would retire both, against ticket 0095's ruling (decision 2).

## Closes

- `sdlc/issues/2026-09-25-the-public-entity-debug-prints-its-kind.md`, moved to `closed/` at landing.
- Item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`: the landing commit moves it under "Already fixed". The issue stays open.

## What Ian can overturn

- Item 4 deferred to its own ticket because of the file ownership, although his ruling puts it in 0.1.
- Python parses `value_json` with its own `serde_json` dependency, in place of a public per-member JSON method (decision 2).
- pandas keeps list tag columns while a Polars frame writes JSON text (decision 4).
- The list form keeps its cause table (decision 5).

## Evidence

- Starts from: ADR 0047 item 10 and its amendment; ticket 0130's Rust door `column.rs` and its lane test on `17-annotate-partial`; ticket 0095's "Result JSON" ruling; tickets 0106 and 0122 for Python's frame and pandas shapes; `test_secrecy.py`'s in-child `http.server`; the core `RelationEntity` `Debug`.
- Keeps: the public API and its frozen inventory; every other surface; the Rust door's output; pandas and Series shapes; the list form's values; each refusal and error sentence; `Entity`'s accessors.
- Changes: `Entity`'s `Debug` withholds `kind`. The Python Polars frame writes widened cells and tag cells from `value_json`, and its tag column is JSON text. Python adds a `serde_json` dependency.
- Proof: the four rows above with their plants, and the `install`, `lint`, `test`, `spec`, and `surfaces` rungs once each after a merge of `origin/main`.
- Defers: API gaps items 1 to 7 and 9, item 4 for file ownership, the list form's cause table, and one shared per-member JSON method.

## Complexity

Contract 1; state and timing 0; reach 2; proof 2; cost of error 1; total 6. Two local fixes, a new binding dependency with no new package, and three tests.
