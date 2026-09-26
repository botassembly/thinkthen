# 0136: Build Polars frames refuse before sending and write cells as the Rust door does

Status: built; design review accepted after two rounds; code review accepted after one round of fixes; ladder green after the merge of `origin/main`. Owner: Claude.

Branch `ticket/0136-polars-checks-before-send`, built in lane `thinkthen-lane-4`. The ticket is `sdlc/tickets/0136-polars-checks-before-send.md`. Ian can overturn every decision the ticket lists.

## Result

- A Python Polars frame now refuses a question named as one of its columns before any request. The sentence matches pandas: `the frame already has a column named 'late'; rename it first`.
- A Python Polars frame whose column nests past 64 levels is now refused before any request. The door copies the caller's schemas and the frame's metadata in `write::kept`, before `answered` sends. Polars 1.44.2 exports a column imploded 70 times. The design review asked for that measurement, and a script in the builder's scratch folder took it.
- Widened cells, on Polars and pandas frames, take their text from `value_json`, as the Rust door does. A widened score of 1.0 reads `1.0`, where it read `1`. A Polars frame's tag column holds the JSON array text, as ADR 0047 item 10 says. The binding's own widened-cell builder, `widened`, and its JSON quoting, `quoted`, are gone.
- `serde_json` `=1.0.151` with `raw_value` joins the Python binding. Its lock gained one line, `"serde_json"` in the `thinkthen-python` entry. No package entry was added, and no version changed.
- The Rust Polars door needed no change. It already refuses a missing `on` column and a clash before sending, and its test counts 0 sends.

## Reviews

- Design review 1 (fresh read-only Claude session): two blocking findings. The lock would gain a line. A deep Polars column could reach the schema copy after the sends, and the builder measured that it can. Also one should-fix on refusal order, and five nits. The ticket was rewritten whole.
- Design review 2: two nits (a plant's send count and prose), fixed. Accepted.
- Code review 1 (fresh read-only Claude session): three findings. The README scoped tag text to every frame. The refusal order differs from pandas for a null or non-text `on` column. The docstring ran 1 line over budget. All three were fixed, and the order is recorded in the ticket's edge table. The reviewer named what it checked for the dependency: same version and feature set as `crates/thinkthen` and `libraries/c`, and a one-line lock change.
- Code review 2: accepted. One nit was kept: the docstring says "is refused first" to stay within its 2-line budget.

## Plants

Each plant was applied to the source, built, run against `tests/test_door.py`, and reverted from a saved copy. Each restored file was touched, and the extension was rebuilt before the next green run. The script and logs sit in the session scratchpad under `t0136/`, outside the repository. A grep of the diff for plant text found none.

| Plant | Test | Result |
| --- | --- | --- |
| P1: the clash check never matches | `test_what_the_door_refuses_sends_nothing` | Red: Polars raises `DuplicateError` after the sends, and the child exits nonzero |
| P2: the clash check runs on the calling thread before `arrow::frame` | same | Red: the missing-column row prints the clash sentence |
| P3: `kept` runs after `answered` | same | Red: the backend counts 4 sends, 3 for the clash row and 1 for the deep row |
| P4: a widened score takes `f64` display | `test_a_frame_writes_cells_as_value_json_holds` | Red: `urgent` reads `1`, wanted `1.0` |
| P5: a Polars frame's tag column stays a list | same | Red: `kinds` is `List(String)`, wanted `String` |

Before the fix, both tests were red for the reasons the ticket names. The clash row raised `DuplicateError` after the sends. The cell test printed `['1', ...]` and a `List(String)` tag column.

## Ladder

`origin/main` was merged in `3b0f14ee`, then again in `10eb195d`. The second merge brought only issue files. Each rung ran directly, with no outer `flock`, and `THINKTHEN_API_KEY` unset.

| Rung | Result | Wall time |
| --- | --- | --- |
| `install` | pass | 11 s |
| `lint` | pass | 219 s |
| `test` | pass | 151 s |
| `spec` | pass | 929 s |
| `surfaces` | fail, `libraries/python` | 848 s |
| `lint`, after the fix | pass | 140 s |
| `surfaces`, after the fix | pass, all ten | 515 s |

The first `surfaces` run failed on the Python check's clippy: a `type_complexity` lint, and a probe-feature unit test in `src/arrow/ffi.rs` that still called the old `frame` signature. The builder's dev loop had run the Python tests and not clippy. The fix added a `Kept` type alias and moved the unit test to the new signature. Both clippy passes were then clean. `lint` and `surfaces` ran again. `install`, `test`, and `spec` did not run again, since the fix touched only `libraries/python` and its pages, and those rungs build nothing there.

The `spec` rung ran while the one-minute load stood near 10, from other builders. Its time is not a clean measure.

## Lane trial

- The lane started cold: 29 MB, with no build folders. So this ladder is a cold ladder in a lane, not a warm one. The next ticket in lane 4 gives the warm measure.
- After the first ladder, `du -sh` of the lane read 9.0 G. After the rerun it read 9.3 G.
- Ladder wall time, first run: 2,158 s across the five rungs. The rerun of `lint` and `surfaces` took 655 s.

## Ratchets

| Ceiling | Before | After | Why |
| --- | --- | --- | --- |
| `sdlc/ratchet.json` (crates and conformance) | 66557 | 66557 | unchanged; `crates/thinkthen` untouched |
| `libraries/python/ratchet.json` (Rust) | 4604 | 4629 | the `value_json` parse, the clash check, and the split schema copy, net of the removed builder and quoting |
| `libraries/python/ratchet.py.json` (Python) | 2253 | 2301 | three refusal rows, the cell test with its loopback listener, and a docstring line |

There is no separate Polars surface ratchet. `libraries/polars` holds only `check.sh` and `README.md`, and the root ceiling covers the feature's code.

## Budgets

Nonblank lines against `origin/main`:

- `libraries/python/src`: +25 net (budget 30).
- `libraries/python/tests`: +48 (budget 50).
- `libraries/python/Cargo.toml` and `Cargo.lock`: 1 line each (budget 1 each).
- `thinkthen/__init__.py` docstring: 2 lines changed (budget 2).
- `README.md` and `NOTES.md`: 2 lines added (budget 4).
- `crates/thinkthen`: 0.

## Stop rules

None crossed. The listener's score landed at exactly 1.0, and the engine took its replies. The lock gained only the one line. No file another in-flight ticket owns was touched.

## Deferred gaps

As the ticket lists them. The list form's failed marker keeps the binding's cause table. No public per-member JSON accessor exists. No Rust door test widens a score. No shared case widens a score. Also, a Python Polars frame with a null or non-text `on` column and a clashing name gets the null or type sentence, where pandas and Rust give the clash sentence. The ticket's edge table records it, and all three send nothing.

## Closes, at landing

- `sdlc/issues/closed/2026-09-25-a-polars-frame-sends-before-a-question-name-clash-fails.md`: the lander moves it to `closed/`.
- Item 8 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`: the lander marks it settled. Ticket 0134 has that file open.
