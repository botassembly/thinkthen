---
flow: build
priority: 136
opens: libraries/python/src/frame.rs libraries/python/src/arrow/write.rs libraries/python/src/arrow/mod.rs libraries/python/Cargo.toml libraries/python/Cargo.lock libraries/python/thinkthen/__init__.py libraries/python/tests/test_door.py libraries/python/README.md libraries/python/NOTES.md libraries/python/ratchet.json libraries/python/ratchet.py.json sdlc/records sdlc/tickets sdlc/issues
---

# 0136: Polars frames refuse before sending and write cells as the Rust door does

Status: in progress. Design accepted after two reviews (the second found only nits, fixed here). Owner: Claude.

Lane: thinkthen-lane-4

Review route: a fresh read-only Claude session reviews this design and the final diff. The change adds a dependency to the Python binding, so the code reviewer names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A Python Polars user who calls `tt.annotate(set, frame, on="body")` on a frame that already has a column named as a question gets the pandas refusal before any request goes out. Today every row is sent first. Then Polars raises its own `DuplicateError`. A frame with a column nested more than 64 levels deep is refused before any request too. Today it is refused after every row is sent. The same user gets widened cells and a frame's tag column with the text the Rust Polars door writes, taken from the engine's `value_json`. Today the Python door builds that text itself, so a widened score of 1.0 reads `1`.

The authority is the coordinator's brief for 0136 and the backlog `sdlc/planning/issue-backlog-2026-09-25.md`, section A row 1 ("A1": `2026-09-25-public-library-api-gaps.md`, item 8 only). The ticket settles `sdlc/issues/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md` and item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. ADR 0047 item 10 (the Polars column table) is the contract for item 8. The coordinator's rule for this ticket: a failure the code can find before sending refuses before any request goes out, on the Python Polars and Rust Polars surfaces where it matters.

## What is wrong today

1. `_annotate_frame` in `libraries/python/src/frame.rs` reads the frame, sends every row, and builds the answer frame. It never compares question names with the frame's columns. The issue counted 2 sends on a two-row frame before Polars refused the answer frame. The pandas path already refuses first in `_on` (`libraries/python/thinkthen/__init__.py`), with the sentence `the frame already has a column named 'late'; rename it first`.
2. `write::frame` copies the caller's column schemas and the frame's metadata into the answer frame after `answered` has sent every row. The copy refuses a schema deeper than 64 levels and metadata past its bytes. Polars 1.44.2 builds and exports a column imploded 70 times (measured on 2026-09-26 with a script kept in the builder's scratch folder). Polars did not refuse to export it. Only pyarrow refused to import it. So a plain Polars frame reaches the depth refusal after every send.
3. `widened` in `libraries/python/src/arrow/write.rs` builds each widened cell by hand. A score uses Rust's `f64` display, so 1.0 becomes `1` where `value_json` holds `1.0`. The failed marker comes from a hand-built string and the binding's cause table in `libraries/python/src/engine.rs`. A tag cell uses the binding's own JSON quoting. It writes a newline as `\u000a`, and the engine writes `\n`.
4. A Python Polars frame's unwidened tag column is a `List(String)`. ADR 0047 item 10 and the Rust door (`crates/thinkthen/src/public/frame/column.rs`) write a frame's tag column as the JSON array text.

## What already holds

A survey of both Polars surfaces for other failures found before sending:

- The Rust Polars door's `annotate_frame` already refuses a missing `on` column and a name clash before its first send. `crates/thinkthen/tests/polars/door.rs` `every_refusal_is_pinned_and_sends_nothing` pins both sentences and counts 0 sends. Its Series methods check the question kind and the column's type and nulls before sending. It copies no schema. This ticket changes nothing in `crates/thinkthen`.
- The Python Polars door already reads the `on` column, its type, and its nulls before sending, in `arrow::frame` and `arrow::series`. `test_what_the_door_refuses_sends_nothing` pins those sentences with 0 sends.
- A question name holds only lowercase ASCII letters, digits, and underscores (`check_name` in `crates/thinkthen/src/core/question_set.rs`). So an answer column's name never fails its C string, and `'{name}'` equals Python's `repr` of it.
- The one refusal left after the send is an answer column past 2 GiB of text. Only the answers can show it.

## Design

### The caller's schemas are copied before the send

`write::frame` splits in two. A new `write::kept(hold, memory, read)` copies the caller's column schemas and the frame metadata and returns them. `write::frame` takes that copy, adds the answer columns' schemas, and builds the batches as today. `_annotate_frame` calls `kept` on the worker right after `arrow::frame` and before `answered`. A deep schema or bad metadata then refuses with today's sentence and sends nothing.

### The clash check

Also on the worker, after `kept` and before `answered`, `_annotate_frame` compares each question name, in set order, with the copied column names. A match refuses with `UsageError` and the pandas sentence: `the frame already has a column named '{name}'; rename it first`. `arrow::frame` runs first, so a frame with no `on` column gets that refusal whatever else it holds, as pandas and the Rust door do. The check reads the Arrow names the door already copies, so it needs no Polars API.

### Cells from `value_json`

`frame::answered` builds every column of a multi-question `annotate`, for a Polars frame and for a pandas frame alike. It gains one argument, `tag_text`, true for a Polars frame and false for pandas. It parses each record's `value_json` once into its members' raw JSON texts, keyed by name. The C binding already uses this pattern (`libraries/c/src/call.rs`, `Members`). For each question:

- When any row of the question failed, the column widens to text. Each cell takes the member's raw text unchanged. A choice holds its plain label instead, and a JSON `null` stays null. The Rust door's `widened` follows the same rule.
- Otherwise, a tag question with `tag_text` set becomes a text column of each member's raw JSON array text.
- Otherwise, `arrow::annotated` builds the typed column as today.

`arrow::annotated` loses its widening. `widened` and `quoted` leave `write.rs`. A choose column keeps its plain labels. A one-question call, as a Series verb makes, never holds a failed answer. The engine ends it with a `Backend` error (ADR 0047 item 10), and `test_pandas.py` pins that error. The list form's marker `dict` keeps the cause table in `engine.rs`. It builds Python values and widens nothing (deferred gap 1).

The Python binding gains the direct dependency `serde_json = { version = "=1.0.151", features = ["raw_value"] }`. Its lock already holds that version through `thinkthen`. The lock gains one line: `"serde_json"` in the `thinkthen-python` entry's dependency list. No package entry is added, and no version changes.

The `annotate` docstring in `__init__.py` says a pandas frame refuses a question named as one of its columns. It changes to say any frame does.

## Decisions

1. **Refuse the clash, as pandas does.** The issue offers the other choice: overwrite the column, as `assign` would, for both libraries. Refusing keeps the two Python frame kinds and the Rust door alike. A user's own column is never lost to an answer.
2. **One sentence per library.** Python Polars takes the pandas sentence. The Rust door keeps its own sentence, `the frame already holds a column named body`. Its test pins it. A Rust caller reads Rust's error, and a Python caller reads Python's.
3. **The clash check reads the copied Arrow names on the worker.** It follows the `on` check, as on the other two frame kinds. It uses no Polars attribute, and a pyarrow table never reaches it, since `polars_frame` refuses one first.
4. **Copy the caller's schemas before the send.** The copy already exists. Moving it earlier costs no second copy and turns a refusal after the sends into one before them.
5. **Parse `value_json` in the binding.** The public API gains nothing, since ticket 0134 owns the public API gaps. The binding parses with `serde_json`'s `RawValue`, as the C binding does. A public per-member JSON accessor would let both doors drop the parse (deferred gap 2).
6. **A Polars frame's tag column becomes JSON array text.** This follows ADR 0047 item 10. It changes what a Python Polars user reads: a `String` cell such as `["bill"]` in place of a list. A Polars Series from `tag` stays `List(String)`, as in Rust. A pandas tag column stays an `object` column of lists, as ticket 0122 decided.
7. **pandas widens from `value_json` too.** A pandas widened cell shares `answered`, so its score of 1.0 also reads `1.0`. Ticket 0122 said pandas widens as Polars does.
8. **The proof serves exact replies from a loopback listener in the test child.** Every arm of the shared backend answers every record alike, so no arm gives one frame a failed row and an answered row of the same question. No arm answers a score at a whole number either. The generic arm's first level takes 0.9. A new shared case would run on every surface's runner. The listener is a Python `http.server` on 127.0.0.1 inside the child, as in `test_secrecy.py` `test_the_fake_key_arrives_at_a_loopback_listener`. It picks a reply by the request's `state` and counts requests.

## Edge cases

| Input | Python Polars | pandas | Rust Polars |
| --- | --- | --- | --- |
| Frame has a column `late`, set asks `late` | `UsageError the frame already has a column named 'late'; rename it first`, 0 sends | unchanged, same sentence, 0 sends | unchanged: `the frame already holds a column named late`, 0 sends |
| Set asks a question named as the `on` column | the clash sentence for that name, 0 sends | unchanged | unchanged |
| Two clashing names | the first in set order | unchanged | unchanged |
| No `on` column, and a clashing name | `the frame has no column named 'missing'`, 0 sends | unchanged, the missing-column sentence | unchanged, the missing-column sentence |
| A null or non-text `on` column, and a clashing name | the null or type sentence from `arrow::frame`, 0 sends | the clash sentence, 0 sends | the clash sentence, 0 sends |
| A column imploded 70 times | `UsageError a column's schema nests deeper than 64 levels`, 0 sends (was: every row sent first) | n/a | answers; no schema copy |
| A pyarrow table with `on=` | unchanged: the Polars-or-pandas sentence, 0 sends | n/a | n/a |
| Widened score 1.0 | `1.0` (was `1`) | `1.0` (was `1`) | `1.0` |
| Widened score 1.2 | `1.2` | `1.2` | `1.2` |
| Failed cell | the marker from `value_json`, `{"failed":{"kind":"backend","cause":"missing_probability"}}` | same | same |
| Widened decide, not sure | null | null | null |
| Widened choice | the plain label, or null for nothing fits | same | same |
| Tag, no failure, frame | `String` `["bill"]`, and `[]` for no label | `object` lists, unchanged | `String` `["bill"]` |
| Tag label holding a newline, widened or frame | the engine's `"a\nb"` escape, by construction and not tested | widened: same | same |
| Tag, Series verb | `List(String)`, unchanged | `object` lists, unchanged | `List(String)` |

## Proof

Each test counts requests at a loopback listener, never through `--dry-run`. Each runs in a child through the existing `run` helper.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `test_door.py` `test_what_the_door_refuses_sends_nothing`, three new rows, as the issue names | `engine.annotate(form, pl.DataFrame({"body": texts, "late": texts}), on="body")` prints `UsageError the frame already has a column named 'late'; rename it first`. The same frame with `on="missing"` prints the missing-column sentence. A one-row frame of `body` and a column imploded 70 times prints `UsageError a column's schema nests deeper than 64 levels`. The test's existing `backend.count() == 0` covers all three | P1: delete the clash check. The frame sends 3 rows. Polars raises `DuplicateError`, and `said` does not catch it. The child exits nonzero, and the test fails. P2: run the clash check before `arrow::frame`. The missing-column row prints the clash sentence. P3: call `kept` after `answered` again. The clash check moves with it, so the clash row sends 3 and the deep row sends 1, and the count reads 4 |
| `test_door.py` `test_a_frame_writes_cells_as_value_json_holds`, new | A two-row Polars frame, `a` and `b`, meets a set of `late` (decide), `urgent` (score over three levels), and `kinds` (tag over `bill` and `ship`). The listener answers `a` with `urgent` at probabilities 0.25, 0.5, 0.25, a position of exactly 1.0. It answers `b` with `urgent` lacking level 0. The test pins `urgent` as `["1.0", '{"failed":{"kind":"backend","cause":"missing_probability"}}']`, `kinds` as a `String` column of `["bill"]` twice, `late` as `Boolean`, and 2 listener requests | P4: widen a score with `f64` display. `urgent` reads `1`. P5: build a frame's tag column as a list again. The dtype and cells differ |

The four questions for each new test:

- **What behavior does it protect?** The refusal rows protect the rule that a failure found before sending refuses before any request, in the same order as the other frame kinds. The cell test protects ADR 0047 item 10: a Python Polars frame's widened cells and tag cells hold the text of `value_json`, byte for byte, as the Rust door writes them.
- **What credible regression fails it?** P1 to P5. P1 and P3 are today's code, and P4 and P5 are today's cells. P2 is the easy slip of checking names before the frame is read. The marker cell pins byte identity with the Rust door's `MARKER`. Today's hand-built marker gives the same bytes, so no credible plant turns that one cell red.
- **Why does no existing test catch it?** No Python test gives a frame a clashing column or a deep one. `conformance.py` compares widened cells after `json.loads`, so `1` and `1.0` read alike, and no shared case widens a score. `test_pandas.py` pins only an all-failed marker column.
- **Does it need a test-only hook?** No. The listener is a real HTTP listener on the loopback address, reached through the engine's own `base_url`, as `test_secrecy.py` already does. It replaces the shared backend only because no arm can mix a failed and an answered row in one call.

## Budgets

Nonblank lines, measured with `grep -c .` against `origin/main`.

- `libraries/python/src`: at most 30 added, net of lines removed.
- `libraries/python/Cargo.toml`: 1 line added. `Cargo.lock`: 1 line added, as the design says.
- `libraries/python/thinkthen/__init__.py`: the docstring only, at most 2 lines changed.
- `libraries/python/tests`: at most 50 added.
- Pages: `libraries/python/README.md` and `NOTES.md`, at most 4 lines changed in total.
- `crates/thinkthen`: 0. The public API: no change.
- Each Python ratchet moves to the measured total in the commit that changes the code.

## Stop rules

1. Stop if the fix needs any change in `crates/thinkthen`, the public API, or a file another in-flight ticket owns (0132, 0133, 0134, 0135, and the flaky-test Quick Fix).
2. Stop before crossing a budget.
3. Stop if any plant stays green.
4. Stop if the engine refuses the listener's reply shape or the score does not land at exactly 1.0. The listener is then wrong about the wire, and the design needs another route.
5. Stop if adding `serde_json` adds a package entry to the Python lock or changes any version.

## Scope and exclusions

Excluded: the Rust Polars door, the public API, the list form's marker `dict`, `recognize` on a frame, the conformance runner, and every other surface. No live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Routing

Builder: Claude (Opus subagent) in lane 4. Reviewer: a fresh read-only Claude session for design and for code. The code review names what it checked for the new dependency.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 2; total 9. Final level: 2. The risk is a pandas or Polars column whose type or text changes by accident. The typed columns keep their builder, and the existing door, pandas, and conformance tests pin them.

## Deferred gaps

1. The list form's failed marker is a `dict` built from the binding's cause table. A public `FailureCause` name would retire the table.
2. A public per-member JSON accessor on `AnnotatedRecord` would let the Rust door, the Python door, and the C binding drop their `value_json` parses. It belongs to the public API owner.
3. No Rust door test widens a score. The Rust door takes the text from `value_json` by construction.
4. No shared case widens a score, so `conformance.py`'s `json.loads` comparison stays blind to number text on every surface.

## What Ian can overturn

- Decision 1: refuse the clash. The other choice overwrites the column, as `assign` would, on both libraries.
- Decision 6: a Python Polars frame's tag column is JSON array text, per ADR 0047 item 10. The other choice amends the table to a list column in both doors.
- Decision 5: `serde_json` in the Python binding. The other choice is a public accessor, in the public API owner's ticket.
- Decision 8: a listener in the test child. The other choice adds a shared case that every surface runs.

## Closes

- `sdlc/issues/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md`. The lander moves it to `closed/` with a status line naming this ticket.
- Item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`. The issue stays open for its other items. Ticket 0134 has that file open, so the lander marks item 8 settled by 0136 in whichever landing comes second.

## Evidence

- Starts from: The name-clash issue. It counted 2 sends on the loopback backend before Polars raised `DuplicateError`. Item 8 of the public API gaps issue, with its line references in `write.rs`, `engine.rs`, and `frame.rs`. ADR 0047 item 10. The Rust door's `widened` and `member` in `crates/thinkthen/src/public/frame/column.rs` (ticket 0130), and its refusal test in `crates/thinkthen/tests/polars/door.rs`. Ticket 0122 decision 4 and `_on` give the pandas sentence. The C binding's `RawValue` parse of `value_json` in `libraries/c/src/call.rs`. The shared backend's arms in `conformance/backend/src/arms.rs` answer every record alike. The design review's measurement request, answered on 2026-09-26: Polars 1.44.2 exports a column imploded 70 times.
- Keeps: Every Series verb and its column types. The pandas frame checks, sentences, and dtypes, tag lists included. Every refusal sentence the door pins today. The Rust Polars door. The list form's `dict` marker. The caller's frame columns, aliased and never copied.
- Changes: A Python Polars frame refuses a question named as a column, and a column nested past 64 levels, before any send. A widened cell on either Python frame kind takes its text from `value_json`. A Python Polars frame's tag column holds the JSON array text. The binding's own widened-cell builder and its JSON quoting leave.
- Proof: Three rows in `test_what_the_door_refuses_sends_nothing` and the new cell test in `test_door.py`, each counting requests at a loopback listener, with plants P1 to P5 each turning a test red.
- Defers: The list form's cause table, a public per-member JSON accessor, a Rust test that widens a score, and a shared case that widens a score.
