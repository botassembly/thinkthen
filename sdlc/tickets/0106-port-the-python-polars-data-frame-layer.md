---
flow: build
priority: 106
opens: libraries/python sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0106: Port the Python Polars data frame layer

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the Polars data frame layer of the Python surface from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto ticket 0105's core binding. A Polars `Series` crosses into the engine zero-copy through the Arrow C stream form, and `annotate` and `recognize` on a Polars `DataFrame` return the frame with new columns. The wheel never imports Polars. Python never loops a row.

Ian ruled on 2026-09-21 that Python's data frame is Polars at 0.1 and that pandas leaves the surface (ADR 0017, "Ruled after acceptance, 2026-09-21: the data frame is Polars"). His clarification the same day makes pure Python and Polars both first-class, with all scaling and vectorization in Rust. The proof he named is equality at the gate: a Polars column holds the same width as the list form. Polars rides as the optional extra `pip install thinkthen[polars]`. The only open Polars question is a Rust Polars door after 0.1 (ADR 0047 item 8), and this ticket does not touch it.

Draft ADR 0047 fixes the crate's place, and ticket 0105 builds the crate. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. Ian can overturn any decision below.

## What moves from the tag

- `src/arrow.rs`, the Arrow C stream door in both directions. It is 2,698 nonblank lines with 121 `unsafe` sites, and its unit tests start at line 2,226. It becomes the directory module `src/arrow/`, split into files under 500 nonblank lines: the C structs and guards, the readable-memory check (`Readable`, the `/proc/self/maps` snapshot, the file-size bound, and the `mincore` fallback off Linux), the column reader, the frame reader, and the frame and series writers. `mod arrow` carries the one `#[allow(unsafe_code, reason = "…")]` under ADR 0047 item 3. It is the binding's only FFI module.
- The python4 lane's fixes, already on the tag: one memory-map snapshot per call, the file mapping bound, the pointer-table checks, the per-batch shares (`BatchShare`), byte-for-byte schema metadata, and the refused 2 GiB answer column.
- The two freeze follow-ups from `sdlc/records/surfaces-freeze-2026-09-24.md`. `d74d95fb`: the `snapshot()` test helper moves out of `refusal`'s doc comment. `d048261e`: the five copies of the `UNREADABLE` bounds check fold into one helper, and the null batch child gets its own refusal sentence.
- The unit tests in `mod malformed_tests`, each with its guard-page or reservation setup.
- In `thinkthen/__init__.py`: `_column_or_value`, the `on=` branches of `annotate` and `recognize`, and the rebuild through `type(records)(frame)`.
- Tests: `test_polars_door.py`, `test_review7_arrow.py`, `tests/sliced_struct_stream.py`, `test_deadline_column.py`, the Polars cases inside the review suites, `bench_width_polars.py`, `bench_cost_polars.py`, and the frame half of `slide_sample.py`. They keep their assertions, move off `ENGINE_NULL` onto the 0092 backend, and regroup by topic.
- The `polars` optional extra in `pyproject.toml`.

## What is rewritten against the public API

- **Column verbs.** `decide` on a Polars `Series` calls `decide_many` once over the borrowed strings. `choose`, `score`, and `tag` on a `Series` call `annotate` over a one-question set, the bulk form 0095 rules. One `CallOptions` covers the whole column. That gives one deadline and one interrupt check per column. The tag's per-row loop and its between-rows token check leave.
- **Frames.** `annotate(set, frame, on=)` reads the `on` column through the frame reader, calls `Engine::annotate_with` once, and writes one new column per question from `AnnotatedRecord::values`. `recognize(frame, on=)` calls `recognize_with` for each text in Rust under one `CallOptions` and writes its columns as at the tag. `relate` takes no frame. It caps input at 255 entities, and a user passes `df.select("name", "kind").rows()`. The tag's `relate_stream` retires.
- **Filter, rank, and find over a column.** The tag left them on lists. They stay on lists. A `Series` passed to them raises `UsageError` naming the list form. No need at 0.1 asks for more.
- **Who is refused.** The door reads any object that exposes `__arrow_c_stream__` or `__arrow_c_array__`, as the tag did, so a `pyarrow` column works too. A pandas object stays refused by its module before any request, per the ruling. The pandas rebuild probe `_probe_frame_rebuild` retires. 0105's refusal tests change with it: the Polars and `pyarrow` cases become door tests, and the pandas cases stay.
- **Nulls and failures.** A null text is refused with the tag's sentence. A failed question widens its answer column to text and carries the failed marker, as `test_a_failed_question_widens_its_column_to_text` pins.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. Ticket 0105 deferred eleven Python rows and two row halves here. Ten rows and both halves are re-proved against the real engine through the 0092 loopback backend. One row retires. The record plants each bug below and shows its test turning red, then green once the bug is removed. A test that can crash the interpreter runs in a child process and asserts its return code. "Counted" means the 0092 backend's `count` line.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-3 | closed | `annotate` on a frame of three chunks built with `pl.concat(..., rechunk=False)` returns each row's own answers under `MALLOC_PERTURB_=165`. | Read chunk 0 for every chunk. |
| R1-4 | closed | `annotate(on=)` returns the caller's `Categorical`, `Enum`, struct, and list columns equal to the input. | Rebuild aliased columns from a copy that drops the dictionary. |
| R1-12 | closed | Every schema and array the door hands out clears its `release` pointer on release. `pyarrow` imports the output and releases it in a child process that exits 0. | Leave `release` set. The child aborts. |
| R2-12 | closed | A frame sliced at a nonzero offset puts each answer on its own row (`sliced_struct_stream.py`). | Ignore the array's `offset`. |
| R2-17 | closed | 200 refused Arrow inputs in a row each release their stream. Peak resident memory grows under 8 MiB. | Skip the release on the refusal path. |
| R3-4 | partial | A string-view column whose buffer count is one short, and one whose sizes overrun the buffers, raise `UsageError`. The child exits 0. | Restore the off-by-one buffer count. The child dies of SIGSEGV. |
| R4-15 | open | A moved output child stays readable after its parent's release under `MALLOC_PERTURB_`. A Polars `Enum` comes back as `Enum` with its metadata. An answer column past 2 GiB of text raises `UsageError`. | Give children no share. The child process dies. |
| R5-6 | partial/waive | A UTF-8 extent past a guard page, a sizes buffer shorter than its count, a file mapping past its file's end, and a pointer table short of its count each raise `UsageError` in a child. | Remove the sizes-buffer check. The child dies of SIGSEGV. |
| R7-2 | open | UTF-8 offsets `60000000, 60000001` over a three-byte buffer raise `UsageError`. The child exits 0. | Force `Readable` to answer yes. The child dies of SIGSEGV. |
| R7-8 | open | The new `NOTES.md` states exactly which sizes-buffer shapes are refused and names the test that proves each. A `check.sh` step fails when `NOTES.md` names a test the suite lacks. | Cite a test name that does not exist. |
| R1-24 Polars half | closed | A token cancelled from a second thread during `score` over a 200-row `Series` at width 8, on the held arm, raises `Cancelled` with at most 8 counted sends. One `deadline=` covers the whole column. | Build `CallOptions` per row. The deadline restarts and the count grows. |
| R4-23 Polars half | closed | A `SIGINT` from a timer thread during the same `score` stops new sends within one tick and raises `Cancelled`. | Run the column without the interrupt check. All 200 send. |

R3-19 retires. It found that the pandas docstring's advice gave all-NaN answers on a non-default index. pandas left the surface under the 2026-09-21 ruling, so no pandas advice or door remains. The record names the ruling as the reason.

Two R5-6 shapes stay waived as at the tag, and this ticket records them in ADR 0047's Python section (R2-29): an extent that runs into another readable allocation, and a guard page off Linux, where `mincore` sees only unmapped pages. Memory a producer unmaps mid-call also stays a recorded risk. Ian can overturn each waiver. The levers are in the tag's `NOTES.md`, and the new `NOTES.md` repeats them.

## Other acceptance

- Width equality, the proof Ian named. On the 0092 held arm at width 8, `decide_many` over a list and over a Polars `Series` of the same 200 texts each reach exactly 8 counted requests while the replies are held. Both then finish with 200 counted requests and equal answers. A planted per-row call through Python makes the `Series` count 1 and turns the test red.
- Zero copy: the values and view buffer addresses Rust reads equal the addresses Polars reports.
- `import thinkthen` leaves `polars` out of `sys.modules` in a fresh subprocess.
- The slide sample's `tt.annotate("form.json", df, on="body")` runs as drawn on a Polars frame against the 0092 generic arm.
- Every shared case that 0105 runs over a list also runs over a `Series` and gives equal values.
- The unit tests in `mod malformed_tests` pass. Each names the one-line mutant that turned it red at the tag, and the record re-runs three of them.
- The ADR 0047 policy check passes, and `unsafe` appears only under `src/arrow/`. A planted `unsafe` block in `src/lib.rs` fails `lint`.

## The check it adds to the gate ladder

No new rung. `libraries/python/check.sh` gains the Polars steps: the door suite, the Arrow safety suite in child processes, the width equality, the address proof, the frame slide sample, and the `NOTES.md` test-name check. The `surfaces` rung runs it as before. `requirements-dev.txt` keeps its hashed pins for Polars, `polars-runtime-32`, and `pyarrow`. The two pandas benches and `test_pandas_checks.py` retire, and pandas stays pinned only for 0105's refusal tests.

## Budgets

- Production Rust: at most seven new files under `src/arrow/` and 2,300 nonblank lines, each file under 500. The tag's production part is lines 1 to 2,225 of `arrow.rs`. The record names what the fold of the five bounds checks and the removed pandas probe saved first.
- Rust unit tests: at most 600 nonblank lines.
- Python package: at most 120 nonblank lines added to `__init__.py` and `__init__.pyi`.
- Python tests: at most eight files and 1,300 nonblank lines.
- Scripts: at most 40 nonblank lines added to `check.sh`.
- Documentation: `NOTES.md`, `README.md`, `sdlc/planning/libraries/python.md`, and the ADR 0047 Python section, at most 200 net nonblank lines.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` rise to the measured totals in the commit that adds the code. The commit says what grew and why. The root `sdlc/ratchet.json` does not change.
- No new dependency. The Arrow layer uses only the standard library and pyo3, as at the tag.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen`, adding a pandas path, or starting a Rust Polars door.

## Exclusions

A pandas door of any kind. A Rust Polars `Series` door (ADR 0047 item 8, after 0.1). Polars plugin expressions. `filter`, `rank`, `find`, and `relate` over a column. Release wheels and uploads (queue item 4). Any change to `thinkthen`. Any live or paid call.

## Dependencies

After 0105 lands, and so after 0086, 0098, 0093, 0094, 0099, and 0092.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The code reviewer reads every `unsafe` block and the readable-memory check against the C data interface.

## Complexity

Contract 2; state and timing 3; reach 2; proof 4; cost of error 4; total 15. Final level: 4. The door reads producer memory through 121 `unsafe` sites inside a host process, and a missed check kills the user's interpreter.

## Review

- Design review: pending.
- Code review: pending.
