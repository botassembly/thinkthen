---
flow: build
priority: 136
opens: libraries/python/src/frame.rs libraries/python/src/arrow/write.rs libraries/python/Cargo.toml libraries/python/Cargo.lock libraries/python/tests/test_door.py libraries/python/README.md libraries/python/NOTES.md libraries/python/ratchet.json libraries/python/ratchet.py.json sdlc/records sdlc/tickets sdlc/issues
---

# 0136: Polars frames refuse a name clash before sending and write cells as the Rust door does

Status: ready. Owner: Claude.

Lane: thinkthen-lane-4

Review route: a fresh read-only Claude session reviews this design and the final diff. The change adds a dependency to the Python binding, so the code reviewer names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A Python Polars user who calls `tt.annotate(set, frame, on="body")` on a frame that already has a column named as a question gets the pandas refusal before any request goes out. Today every row is sent first, and then Polars raises its own `DuplicateError`. The same user gets widened cells and a frame's tag column with the text the Rust Polars door writes, taken from the engine's `value_json`. Today the Python door builds that text itself, so a widened score of 1.0 reads `1`.

The authority is the coordinator's brief for 0136 and the backlog `sdlc/planning/issue-backlog-2026-09-25.md`, section A row 1 ("A1": `2026-09-25-public-library-api-gaps.md`, item 8 only). The ticket settles `sdlc/issues/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md` and item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. ADR 0047 item 10 (the Polars column table) is the contract for item 8. The coordinator's rule for this ticket: a failure the code can find before sending refuses before any request goes out, on the Python Polars and Rust Polars surfaces where it matters.

## What is wrong today

1. `_annotate_frame` in `libraries/python/src/frame.rs` reads the frame, sends every row, and builds the answer frame. It never compares question names with the frame's columns. The issue counted 2 sends on a two-row frame before Polars refused the answer frame. The pandas path already refuses first in `_on` (`libraries/python/thinkthen/__init__.py`), with the sentence `the frame already has a column named 'late'; rename it first`.
2. `widened` in `libraries/python/src/arrow/write.rs` builds each widened cell by hand. A score uses Rust's `f64` display, so 1.0 becomes `1` where `value_json` holds `1.0`. The failed marker comes from a hand-built string and the binding's cause table in `libraries/python/src/engine.rs`. A tag cell uses the binding's own JSON quoting, which writes a newline as `\u000a` where the engine writes `\n`.
3. A Python Polars frame's unwidened tag column is a `List(String)`. ADR 0047 item 10 and the Rust door (`crates/thinkthen/src/public/frame/column.rs`) write a frame's tag column as the JSON array text.

## What already holds

A survey of both Polars surfaces for other failures found before sending:

- The Rust Polars door's `annotate_frame` already refuses a missing `on` column and a name clash before its first send. `crates/thinkthen/tests/polars/door.rs` `every_refusal_is_pinned_and_sends_nothing` pins both sentences and counts 0 sends. Its Series methods check the question kind and the column's type and nulls before sending. This ticket changes nothing in `crates/thinkthen`.
- The Python Polars door already reads the `on` column, its type, and its nulls before sending, in `arrow::frame` and `arrow::series`. `test_what_the_door_refuses_sends_nothing` pins those sentences with 0 sends.
- The Python door copies the caller's column schemas into the answer frame after sending (`write::frame`). That copy can refuse a malformed schema, such as a cycle or metadata past its bytes. `polars_frame` admits only a Polars object, and Polars never exports such a schema, so no user reaches it. It stays where it is (deferred gap 1).

## Design

### The clash check

`_annotate_frame` checks names on the calling thread, after `polars_frame` and before the worker starts. It reads the frame's `columns` attribute. For each question name in set order, a name found there raises `UsageError` with the pandas sentence: `the frame already has a column named {name!r}; rename it first`. Rust builds `{name!r}` with Python's own `repr`, so the two sentences match for every name. A value with no readable `columns` attribute, such as a Polars Series, skips the check, and `arrow::frame` refuses it as today.

The check lives in Rust, not the package, because the package cannot tell a Polars frame from a pyarrow table before `polars_frame` runs. A pyarrow table's `columns` holds arrays, and `name in columns` would compare a `str` with each array.

### Cells from `value_json`

`frame::answered` builds every column of a multi-question `annotate`, for a Polars frame and for a pandas frame alike. It gains one argument, `tag_text`, true for a Polars frame and false for pandas. It parses each record's `value_json` once into its members' raw JSON texts, keyed by name. This is the pattern the C binding already uses (`libraries/c/src/call.rs`, `Members`). For each question:

- When any row of the question failed, the column widens to text. Each cell takes the member's raw text unchanged, except that a choice holds its plain label and a JSON `null` stays null. This is the Rust door's `widened` rule.
- Otherwise, a tag question with `tag_text` set becomes a text column of each member's raw JSON array text.
- Otherwise, `arrow::annotated` builds the typed column as today.

`arrow::annotated` loses its widening. `widened` and `quoted` leave `write.rs`. A choose column keeps its plain labels. A one-question call, as a Series verb makes, never holds a failed answer: the engine ends it with a `Backend` error (ADR 0047 item 10), and `test_pandas.py` pins that error. The list form's marker `dict` keeps the cause table in `engine.rs`, since it builds Python values and widens nothing (deferred gap 2).

The Python binding gains the direct dependency `serde_json = { version = "=1.0.151", features = ["raw_value"] }`. Its lock already holds that version through `thinkthen`, so the lock does not change, and `cargo deny` sees no new crate.

## Decisions

1. **Refuse the clash, as pandas does.** The issue offers the other choice: overwrite the column, as `assign` would, for both libraries. Refusing keeps the two Python frame kinds and the Rust door alike, and a user's own column is never lost to an answer.
2. **One sentence per library.** Python Polars takes the pandas sentence. The Rust door keeps its own, `the frame already holds a column named body`, which its test pins. A Rust caller reads Rust's error, and a Python caller reads Python's.
3. **The check reads the Polars frame's `columns`.** It runs before the worker and before the Arrow export, so a refused frame is never exported. The Arrow schema's names would serve any producer, but the door admits only Polars frames, and Python's `repr` is not reachable on the worker.
4. **Parse `value_json` in the binding.** The public API gains nothing, since ticket 0134 owns the public API gaps. The binding parses with `serde_json`'s `RawValue`, as the C binding does. A public per-member JSON accessor would let both doors drop the parse. It is deferred gap 3.
5. **A Polars frame's tag column becomes JSON array text.** This follows ADR 0047 item 10 and changes what a Python Polars user reads: a `String` cell such as `["bill"]` in place of a list. A Polars Series from `tag` stays `List(String)`, as in Rust. A pandas tag column stays an `object` column of lists, as ticket 0122 decided.
6. **pandas widens from `value_json` too.** A pandas widened cell shares `answered`, so its score of 1.0 also reads `1.0`. Ticket 0122 said pandas widens as Polars does.
7. **The proof serves exact replies from a loopback stub in the test child.** Every arm of the shared backend answers every record alike, so no arm gives one frame a failed row and an answered row of the same question. No arm answers a score at a whole number either. The generic arm's first level takes 0.9. A new shared case would run on every surface's runner. The stub is a Python `http.server` on 127.0.0.1 inside the child. It picks a reply by the request's `state` and counts requests.

## Edge cases

| Input | Python Polars | pandas | Rust Polars |
| --- | --- | --- | --- |
| Frame has a column `late`, set asks `late` | `UsageError the frame already has a column named 'late'; rename it first`, 0 sends | unchanged, same sentence, 0 sends | unchanged: `the frame already holds a column named late`, 0 sends |
| Set asks a question named as the `on` column | the clash sentence for that name, 0 sends | unchanged | unchanged |
| Two clashing names | the first in set order | unchanged | unchanged |
| A pyarrow table with `on=` | unchanged: the Polars-or-pandas sentence, 0 sends | n/a | n/a |
| A Polars Series with `on=` | unchanged: `arrow::frame`'s refusal | n/a | n/a |
| Widened score 1.0 | `1.0` (was `1`) | `1.0` (was `1`) | `1.0` |
| Widened score 1.2 | `1.2` | `1.2` | `1.2` |
| Failed cell | the marker from `value_json`, `{"failed":{"kind":"backend","cause":"missing_probability"}}` | same | same |
| Widened decide, not sure | null | null | null |
| Widened choice | the plain label, or null for nothing fits | same | same |
| Tag, no failure, frame | `String` `["bill"]`, and `[]` for no label | `object` lists, unchanged | `String` `["bill"]` |
| Tag label holding a newline, widened or frame | the engine's `"a\nb"` escape | widened: same | same |
| Tag, Series verb | `List(String)`, unchanged | `object` lists, unchanged | `List(String)` |

## Proof

Each test counts requests at a loopback listener, never through `--dry-run`. Each runs in a child through the existing `run` helper and `child_env`.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `test_door.py` `test_what_the_door_refuses_sends_nothing`, one new row, as the issue names | `engine.annotate(form, pl.DataFrame({"body": texts, "late": texts}), on="body")` prints `UsageError the frame already has a column named 'late'; rename it first`. The test's existing `backend.count() == 0` covers it | P1: delete the clash check. The frame sends 3 rows, and Polars raises `DuplicateError`, which `said` does not catch. The child exits nonzero, and the test fails |
| `test_door.py` `test_a_frame_writes_cells_as_value_json_holds`, new | A two-row Polars frame, `a` and `b`, meets a set of `late` (decide), `urgent` (score over three levels), and `kinds` (tag over `bill` and `ship`). The child's stub answers `a` with `urgent` at probabilities 0.25, 0.5, 0.25, a position of exactly 1.0, and answers `b` with `urgent` lacking level 0. The test pins `urgent` as `["1.0", '{"failed":{"kind":"backend","cause":"missing_probability"}}']`, `kinds` as a `String` column of `["bill"]` twice, `late` as `Boolean`, and 2 stub requests | P2: widen a score with `f64` display: `urgent` reads `1`. P3: build a frame's tag column as a list again: the dtype and cells differ. P4: write the marker from the cause table with `"cause": ` spacing: the marker cell differs |

The four questions for each new test:

- **What behavior does it protect?** The clash row protects the rule that a failure found before sending refuses before any request. The cell test protects ADR 0047 item 10: a Python Polars frame's widened cells and tag cells hold the text of `value_json`, byte for byte, as the Rust door writes them.
- **What credible regression fails it?** P1 to P4. P1 is the bug the issue filed. P2 is today's code. P3 is today's frame tag column.
- **Why does no existing test catch it?** No Python test gives a frame a clashing column. `conformance.py` compares widened cells after `json.loads`, so `1` and `1.0` read alike, and no shared case widens a score. `test_pandas.py` pins only an all-failed marker column.
- **Does it need a test-only hook?** No. The stub is a real HTTP listener on the loopback address, reached through the engine's own `base_url`. It replaces the shared backend only because no arm can mix a failed and an answered row in one call.

## Budgets

Nonblank lines, measured with `grep -c .` against `origin/main`.

- `libraries/python/src`: at most 20 added, net of lines removed.
- `libraries/python/Cargo.toml`: 1 line added. `Cargo.lock` unchanged.
- `libraries/python/tests`: at most 45 added.
- Pages: `libraries/python/README.md` and `NOTES.md`, at most 4 lines changed in total.
- `crates/thinkthen`: 0. The public API: no change.
- Each Python ratchet rises to the measured total in the commit that adds the code, or falls to it.

## Stop rules

1. Stop if the fix needs any change in `crates/thinkthen`, the public API, or a file another in-flight ticket owns (0132, 0133, 0134, 0135, and the flaky-test Quick Fix).
2. Stop before crossing a budget.
3. Stop if any plant stays green.
4. Stop if the engine refuses the stub's reply shape or the score does not land at exactly 1.0. The stub is then wrong about the wire, and the design needs another route.
5. Stop if the Python lock changes when `serde_json` is added.

## Scope and exclusions

Excluded: the Rust Polars door, the public API, the list form's marker `dict`, the schema copy after sending, `recognize` on a frame, the conformance runner, and every other surface. No live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Routing

Builder: Claude (Opus subagent) in lane 4. Reviewer: a fresh read-only Claude session for design and for code. The code review names what it checked for the new dependency.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 2; total 9. Final level: 2. The risk is a pandas or Polars column whose type or text changes by accident. The typed columns keep their builder, and the existing door, pandas, and conformance tests pin them.

## Deferred gaps

1. The Python door copies the caller's column schemas after sending, so a malformed schema refuses after the sends. Only a producer that is not Polars could send one, and `polars_frame` refuses those. Moving the copy before the send needs a spoofed-module producer to prove.
2. The list form's failed marker is a `dict` built from the binding's cause table. A public `FailureCause` name would retire the table.
3. A public per-member JSON accessor on `AnnotatedRecord` would let the Rust door, the Python door, and the C binding drop their `value_json` parses. It belongs to the public API owner.
4. No Rust door test widens a score. The Rust door takes the text from `value_json` by construction.
5. No shared case widens a score, so `conformance.py`'s `json.loads` comparison stays blind to number text on every surface.

## What Ian can overturn

- Decision 1: refuse the clash. The other choice overwrites the column, as `assign` would, on both libraries.
- Decision 5: a Python Polars frame's tag column is JSON array text, per ADR 0047 item 10. The other choice amends the table to a list column in both doors.
- Decision 4: `serde_json` in the Python binding. The other choice is a public accessor, in the public API owner's ticket.
- Decision 7: a stub in the test child. The other choice adds a shared case that every surface runs.
- Deferred gap 1: the schema copy stays after the send.

## Closes

- `sdlc/issues/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md`. The lander moves it to `closed/` with a status line naming this ticket.
- Item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. The issue stays open for its other items. Ticket 0134 has that file open, so the lander marks item 8 settled by 0136 in whichever landing comes second.

## Evidence

- Starts from: The name-clash issue, which counted 2 sends on the loopback backend before Polars raised `DuplicateError`. Item 8 of the public API gaps issue, with its line references in `write.rs`, `engine.rs`, and `frame.rs`. ADR 0047 item 10. The Rust door's `widened` and `member` in `crates/thinkthen/src/public/frame/column.rs` (ticket 0130), and its refusal test in `crates/thinkthen/tests/polars/door.rs`. Ticket 0122 decision 4 and `_on`, which give the pandas sentence. The C binding's `RawValue` parse of `value_json` in `libraries/c/src/call.rs`. The shared backend's arms in `conformance/backend/src/arms.rs`, which answer every record alike.
- Keeps: Every Series verb and its column types. The pandas frame checks, sentences, and dtypes, tag lists included. Every refusal sentence the door pins today. The Rust Polars door. The list form's `dict` marker. The caller's frame columns, aliased and never copied.
- Changes: A Python Polars frame refuses a question named as a column before any send. A widened cell on either Python frame kind takes its text from `value_json`. A Python Polars frame's tag column holds the JSON array text. The binding's own widened-cell builder and its JSON quoting leave.
- Proof: The clash row in `test_what_the_door_refuses_sends_nothing` and the new cell test in `test_door.py`, each counting requests at a loopback listener, with plants P1 to P4 each turning a test red.
- Defers: The schema copy after the send, the list form's cause table, a public per-member JSON accessor, a Rust test that widens a score, and a shared case that widens a score.
