---
flow: build
priority: 122
opens: libraries/python sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md sdlc/records sdlc/tickets
---

# 0122: Support pandas columns and frames in the Python surface

Status: ready once design review accepts it. It starts after ticket 0106 lands. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it. The change widens a public surface, so the code reviewer names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A pandas user passes a pandas column or a pandas frame to the Python surface and gets pandas back. `decide`, `decide_many`, `choose`, `score`, and `tag` take a pandas `Series` and return a pandas `Series` with the caller's index and name. `annotate` and `recognize` take a pandas `DataFrame` with `on=` and return a pandas frame that keeps the caller's index. pandas 2 and pandas 3 both work. `import thinkthen` still imports neither pandas nor Polars.

Ian ruled on 2026-09-25 that pandas is supported fully in 0.1 (`sdlc/planning/one-line-plan-2026-09-25.md`, "Ian's ruling on pandas, 2026-09-25"). The ruling overturns the 2026-09-21 line "pandas leaves the surface". ADR 0017's 2026-09-25 amendment records it and names this ticket. The ruling names four things: a column in and a column with the caller's index out, a frame in and the frame with new answer columns out, pandas 2 and 3 both, and no pandas or Polars import. It also says a pandas 2 object column crosses at list speed and the page says so. Claude recommended against a columns-only version. A pandas user's first try is a whole frame, and a bare list back invites the lost-index bug R3-19 found.

This ticket touches only `libraries/python` and the pages that describe it. It builds on ticket 0106's branch. Ian can overturn every decision below except the ruling itself.

## Design and decisions

1. **Naming a pandas object.** The binding names a pandas object by its type's top-level module, as 0105 does. A pandas `Series` is a column. A pandas `DataFrame` is a frame. The check walks the type's method resolution order by class name and module, so a subclass counts, and it imports nothing. Any other pandas object, such as an `Index` or an extension array, raises `UsageError` before any send. That sentence names the fix: pass a pandas `Series`. The order of checks stays 0106's: pandas first, then Polars, then any other Arrow producer. pandas 3 objects expose `__arrow_c_stream__`, so the pandas check must still come first.

2. **A Series goes in through the door, or through the list reader.** `decide`, `decide_many`, `choose`, `score`, and `tag` read a pandas `Series` in one of two ways. Both make exactly one engine call, as 0106's column verbs do.
   - When the Series has `__arrow_c_stream__` and pandas' export returns a stream, the Series crosses 0106's Arrow door. The worker owns the imported batches and reads the text in place. On pandas 3, a `str` column with pyarrow storage and a `string[pyarrow]` column cross zero-copy. A pandas 3 object column crosses the door too, but pandas converts it on every export, so pandas makes one copy before the door reads it.
   - When the method is missing, the Series crosses as a list through 0105's list reader, whole, before the first send. Every pandas 2 Series lacks the method (the 2026-09-21 checks, on 2.2.3 and 2.3.3). So every pandas 2 column crosses at list speed. The throttle and the wall time match the list form.
   - When pandas' own export raises, the Series also crosses as a list. This covers pandas 3 without pyarrow installed and a pandas 3 object column that holds a value pyarrow cannot convert. The fallback covers only an error raised by the export call itself. A door refusal of the exported stream, such as a number format, stands and is never retried as a list.

3. **A Series comes back as the caller's own Series.** The same verbs return `type(series)(values, index=series.index, name=series.name, dtype=dtype)`. The binding never imports pandas. It calls the caller's own class.
   - Rust builds the answers from the same `Cells` the Polars path builds, then hands Python a list of values and a dtype name. So a pandas answer equals the Polars answer for the same texts, value for value, with pandas' missing value in place of Polars' null.
   - The dtypes: `decide` and `decide_many` give `"boolean"`, `score` gives `"Float64"`, `choose` gives `"string"`, and `tag` gives `"object"` with one list of labels per row. "Not sure" is `pd.NA`. A question the backend failed widens its column to `"string"` holding the failed marker text, as Polars widens it.
   - The answer keeps the caller's index object and the caller's name. Row order never changes. The caller's Series is not changed.
   - A Polars `Series` still gets a Polars `Series` named by the verb, and a pyarrow column still gets a list. This ticket changes neither.

4. **A frame goes in through its `on` column.** `annotate(set, df, on=)` and `recognize(df, on=)` take a pandas `DataFrame`. Before any send, the binding checks four things, each with a pinned sentence:
   - `on` names a column of the frame. A missing name gets 0106's sentence "the frame has no column named 'x'".
   - `on` names exactly one column. pandas allows repeated labels, so a repeated `on` label is refused.
   - The frame's column labels have one level. A frame whose columns are a `MultiIndex` is refused.
   - For `annotate`, no question name in the set equals a column label already in the frame. pandas' `assign` would overwrite that column without a word, so a clash is refused.
   The binding then takes `df[on]` and sends it through decision 2. Only the `on` column crosses. The frame's other columns never leave Python, and the whole-frame `__arrow_c_stream__` export is never called. That export converts every column, and the answer needs one. `annotate` makes one `annotate_with` call. `recognize` makes one `recognize_with` call per text under one deadline, as 0106's Polars frame does.

5. **A frame comes back with the caller's index.**
   - `annotate` returns `df.assign(...)` with one new column per question, in set order. Each new column is built as decision 3 builds a Series, with `index=df.index`. `assign` reads an identical index without reordering, so repeated labels and a `MultiIndex` hold. The new frame keeps every original column, its dtype, its column order, and its index. The caller's frame is not changed.
   - `recognize` returns a long frame of the caller's frame class, one row per name, with 0106's columns: `row` (counted from 1), `text`, `kind`, `start`, `end`, and `strength`. Its index holds the source row's label, taken from `df.index` by position. So `df.join(names)` lines up on any index. A text with no names adds no row. `relations` with `on=` stays refused, as for Polars.

6. **Nulls.** A `None`, `NaN`, or `pd.NA` in the text column raises 0106's null sentence, "the column holds nulls; the engine needs text, so drop or fill them first", with zero sends. The Arrow door already refuses a null with that sentence. The list path reads the caller's `hasnans` first, so a pandas 2 column gets the same sentence and never "record 1 is not a str".

7. **Indexes.** Answers are positional, and the caller's index object rides back unchanged.

   | Index | Series verbs | `annotate` | `recognize` |
   |---|---|---|---|
   | Default `RangeIndex` | kept | kept | source labels 0, 1, 2 |
   | Non-default labels, such as `[5, 7, 9]` (R3-19) | kept, and a join on the caller's frame has no missing value | kept | source labels |
   | Row `MultiIndex` | kept | kept | source tuples |
   | Repeated labels, such as `[1, 1, 2]` | kept, row order unchanged | kept, row order unchanged | source labels, repeated |
   | Column `MultiIndex` | not applicable | refused before any send | refused before any send |
   | Empty Series or frame | zero sends, an empty result with the same index, name, and dtype | zero sends, the frame with empty new columns | zero sends, an empty long frame |

8. **What stays refused.** The pandas refusal of 0105 and 0106 leaves. The constant `PANDAS`, its tests, and its pinned sentence go. These stay refused before any send:
   - A pyarrow `Table`, a `RecordBatchReader`, an object that exposes only `__dataframe__`, a Polars `LazyFrame`, and any other frame passed with `on=`. The sentence becomes "annotate with on= takes a Polars or pandas DataFrame; a list of str takes no on=". Only the words "or pandas" change.
   - A pandas or Polars column passed to `filter`, `rank`, `find`, or `relate`, and any frame passed to them or to `annotate` without `on=`. The list-only sentence gains "or pandas" in the same way.
   - A pandas Series passed to `details`. It keeps 0106's sentence "details reads one str, not a column".
   - A pandas frame passed to a column verb. It keeps 0106's sentence "a data frame is not a column; pass df[\"name\"], or annotate with on=".
   - Any other pandas object (decision 1).

9. **No pandas import and no pandas dependency.** The wheel imports neither pandas nor Polars, and `pyproject.toml` gains no pandas extra. A pandas user already has pandas. The binding touches pandas only through the caller's own object: `type`, `.index`, `.name`, `.columns`, `.hasnans`, `.assign`, item access, `index.take`, and `__arrow_c_stream__`. Python never loops a row.

10. **No new test-only hook.** The address proof reuses 0106's `_arrow_probe` under the `probe` feature. The throttle proofs build `tt.Engine(throttle=8, base_url=...)`, as 0106 does. Every other proof runs through the public calls.

11. **The page says what is fast.** `README.md` and `NOTES.md` state: a pandas 2 column crosses at list speed and still makes one engine call at the full throttle. A pandas 3 `str` or `string[pyarrow]` column crosses zero-copy. `astype("string[pyarrow]")` makes a pandas 3 object column fast, and it gives pandas 2 no fast path. The spelling `str[pyarrow]` is not valid past pandas 2.2.3, so no page uses it (the 2026-09-21 issue's first correction).

## What moves from the tag

The tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) holds the 2026-09-21 pandas work. None of its pandas code comes across as written.

- `tests/test_pandas_checks.py`: its five checks become the outside-in tests below. Check 4 inverts. The tag returned a plain list, and this ticket returns a pandas Series.
- `tests/bench_width_pandas.py` and `tests/bench_cost_pandas.py` do not come across. The throttle proof replaces the width bench. The null-backend cost numbers stay in the tag's `NOTES.md` as evidence, and no gate repeats them.
- `_probe_frame_rebuild` in `src/arrow.rs` does not come across. A pandas frame never goes through the Arrow frame path, so nothing needs to probe its rebuild.
- The pandas lines of the tag's `thinkthen/__init__.py` do not come across. Their "one line back" advice, `pd.Series(answers, index=text.index)`, becomes the binding's own return.
- `test_review3_offline.py:132`, R3-19: 0106 retired it under the 2026-09-21 ruling. This ticket revives it as the index proof below.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The record plants each bug and shows its test turn red, then green once the bug is removed.

| Row | Status before | Re-proof here | Planted bug |
|---|---|---|---|
| R1-6 | closed by 0105 as a refusal | In `tests/test_pandas.py`, on both lanes: a pandas Series passed to `filter` raises the list-only sentence with zero counted sends. The same Series passed to `decide_many` answers. | Skip the container check for pandas objects. The Series iterates as a list, `filter` answers, and the count is nonzero. |
| R2-11 | closed by 0105 as a refusal | In `tests/test_pandas.py`, on both lanes: a pandas `DataFrame` passed to `annotate` without `on=` raises the list-only sentence with zero counted sends. | Skip the container check for pandas objects. The frame iterates, pandas yields the column labels, and they are sent. |
| R3-19 | retired by 0106 | A Series with index `[5, 7, 9]` and name `"body"` gets back a Series whose index equals `[5, 7, 9]` and whose name is `"body"`. `df.join(answer.rename("x"))["x"]` holds no missing value. The same holds for `annotate` on that frame and for `recognize`'s source labels. | Rebuild without `index=`. The answer gets a fresh `RangeIndex`, and the join reads all missing. |

## Proof

Every test below is outside-in through the public calls. Each engine call runs in a child on its own loopback backend, as 0105 set up. The new file `tests/test_pandas.py` runs in both pandas lanes. No test skips. Where the two majors differ, each lane asserts its own row.

1. **Answers and shapes.** For each verb and each Series kind the installed pandas offers, the answer is the caller's class, with the caller's index, name, and the dtype of decision 3. Its values equal the Polars answers for the same texts. pandas 3 runs `str`, `string[pyarrow]`, and object columns. pandas 2 runs object and `string[pyarrow]` columns. `annotate` on a frame keeps every original column equal and adds the set's columns. Plant: return 0106's list for a pandas column. The class assertion turns red.
2. **The caller's index survives (R3-19).** The table in decision 7 runs row by row: a non-default index, a row `MultiIndex`, repeated labels, and an empty frame. Each asserts the returned index equals the input index and a join on the caller's frame holds no missing value. Plant: rebuild without `index=`. Every non-default row turns red. A second plant builds the new frame columns as positional lists over a fresh index. The repeated-label row turns red.
3. **Throttle equality at 8 in flight.** In a child, `tt.Engine(throttle=8, base_url=".../arm/delay/100/v1")` runs 200 texts through `decide_many` as a list and as each pandas Series kind of the lane. Each run takes about 2.5 s with 200 counted sends and equal answers. Each pandas wall time falls within 5 percent of the list's. On 0092's held arm, each form reaches exactly 8 counted sends and no more. Plant: answer a pandas column row by row through the single-text call. It holds 1 in flight and runs about eight times longer. Both assertions turn red.
4. **Zero copy on pandas 3, and the fallback on pandas 2.** On pandas 3, `_arrow_probe` over a `str` Series and a `string[pyarrow]` Series reads the same buffer addresses pandas' own export hands out. That is experiment 205's proof form, and 0106's address test already runs it for Polars. The route row of the edge table shows the verbs take the door. On pandas 2, the test asserts the Series lacks `__arrow_c_stream__`, the precondition of the list path. Plant: copy the texts into a `Vec<String>` before the hand-off. The addresses differ.
5. **Every refusal sends nothing.** The edge table below runs in one child per lane. Each row pins its whole sentence. After every row, the backend's `count` line reads 0. A "sends nothing" test counts loopback requests, never a dry run.
6. **Secrecy.** 0105's secrecy test gains every pandas call: each column verb over a pandas Series on the refusing arm, the 401 arm, and the 503 arm, `annotate` and `recognize` on a pandas frame on the same arms, every pandas refusal in the edge table, and the `repr` of each returned pandas value. No message and no `repr` carries the fake key or the address credentials. The test runs in both lanes. Plant: a pandas failure path formats the child's environment into its message. The fake key shows, and the test turns red.
7. **pandas absent.** 0106's import test gains pandas: after `import thinkthen` in a fresh child, neither `pandas` nor `polars` is in `sys.modules`. A second child sets `sys.modules["pandas"] = None`, so any pandas import raises. It then imports `thinkthen` and runs `decide_many` over a list and over a Polars Series. Both answer. Plant: add `import pandas` inside a `try` at the top of `__init__.py`. The first child finds pandas in `sys.modules` and turns red. A second plant imports pandas without the `try`. The second child raises on import and turns red.
8. **Two pinned pandas versions.** pandas 3.0.6 stays the pin in `requirements-dev.txt`, and the main lane runs every test on it. A new `requirements-pandas2.txt`, hashed by `uv pip compile --generate-hashes --offline` as the first file is, pins pandas 2.3.3. It reuses the first file's pins for numpy 2.5.3, pyarrow 25.0.1, polars 1.44.2, polars-runtime-32 1.44.2, and pytest 9.1.1 and its helpers. It adds only what pandas 2 needs beyond them, such as `pytz` and `tzdata`, each named in the record with its version. `check.sh` builds a second venv from it beside the first, installs the probe build of the extension there, and runs `tests/test_pandas.py` and `tests/test_secrecy.py`. A pin missing from uv's cache exits 77, and the rung reports "not run", never "pass". 2.3.3 is the newest pandas 2 the 2026-09-21 checks ran. The oldest they ran is 2.2.3, and the page names both.

### The edge-case table

Each row is one assertion in proof 5 or proof 2. "Door" means 0106's Arrow door. "List" means 0105's list reader. Each plant turns its row red.

| Input | pandas 3.0.6 | pandas 2.3.3 | Planted bug |
|---|---|---|---|
| `str` Series (pyarrow storage) to `decide` | door, zero copy, `"boolean"` Series back | not the pandas 2 default, and not run | Send every pandas column through the list path. The route row below turns red. |
| `string[pyarrow]` Series to `score` | door, zero copy, `"Float64"` Series back | list, same answers | Drop the fallback. pandas 2 raises `UsageError` for a missing capsule. |
| object Series to `choose` | door after pandas' own copy, `"string"` Series back | list, same answers | Drop the fallback. pandas 2 turns red. |
| `int64` Series to `decide` (the route row) | "the column's Arrow format is 'l', not text" | "record 0 is not a str" | Send pandas 3 through the list path. The pandas 3 sentence changes. |
| object Series holding `"a", 1` to `decide` | export raises, list, "record 1 is not a str" | "record 1 is not a str" | Let the export error escape. pyarrow's own error shows. |
| Series holding `None`, `NaN`, or `pd.NA` to `tag` | 0106's null sentence | 0106's null sentence | Skip the `hasnans` read. pandas 2 reads "record 1 is not a str". |
| Series to `filter`, `rank`, `find`, `relate` | the list-only sentence | the list-only sentence | Skip the container check for pandas objects. The Series iterates and sends. |
| Series to `details` | "details reads one str, not a column" | same | Route `details` through the column path. It sends. |
| `DataFrame` to `decide` | "a data frame is not a column; pass df[\"name\"], or annotate with on=" | same | Skip the frame check on the column path. The pinned sentence turns red. |
| `DataFrame` to `annotate` without `on=` | the list-only sentence | same | As R2-11. |
| `DataFrame` with `on="missing"` | "the frame has no column named 'missing'" | same | Look the column up after the send. |
| `DataFrame` with two columns named `body` | the repeated-label sentence | same | Take `df[on]` unchecked. A frame reaches the column path. |
| `DataFrame` with `MultiIndex` columns | the column-level sentence | same | Skip the level check. `assign` writes tuple labels. |
| `DataFrame` with a column named as a question | the clash sentence | same | Skip the clash check. `assign` overwrites the column. |
| `recognize(df, relations=..., on=)` | 0106's relations sentence | same | Check relations after the send. |
| pandas `Index` to `decide_many` | the other-pandas-object sentence | same | Read it as an iterable. It sends. |
| pyarrow `Table` to `annotate(on=)` | the frame sentence with "or pandas" | same | Accept any Arrow-stream frame. It sends. |

The table's rows that answer (the first three) assert answers against the list form in the same child and count the sends each makes. Every refusal row asserts a count of 0. The builder pins the sentence each lane actually gives. A row whose observed route or sentence differs from this table stops the build until the ticket is updated.

### The four questions for each new test

| Test | Behavior it protects | Credible regression | Why no existing test catches it | Test-only hook |
|---|---|---|---|---|
| Answers and shapes | A pandas column gets a pandas Series back | The tag's list return comes back | 0106 tests Polars and pyarrow only | None |
| The caller's index survives | Answers line up with the caller's rows on any index | R3-19: a fresh index turns a join into all missing values | No index exists in Polars, so 0106 has nothing to check | None |
| Throttle equality | A pandas column runs at the list's speed and width | A per-row loop for the list-path fallback | 0106's throttle test runs a Polars Series only | None. `tt.Engine(throttle=8)` is public |
| Zero copy and fallback | pandas 3 text crosses in place, and pandas 2 takes the list path | A copy before the hand-off, or a lost fallback | 0106's address test reads Polars buffers only | `_arrow_probe`, 0106's existing hook under `probe` |
| Refusals send nothing | No pandas input half works | A refusal checked after the send | The pandas rows of 0105 and 0106 pin a refusal this ticket removes | None |
| Secrecy over pandas calls | No new failure path leaks the key | A pandas error message formats settings | 0105's secrecy test has no pandas call | None |
| pandas absent | The package imports and answers without pandas | A convenience `import pandas` in `__init__.py` | 0106's import test checks Polars only | None. `sys.modules` blocking is standard Python |

The rewritten rows of `test_inputs.py` and `test_door.py` replace the pandas refusal rows. They add no new test.

## The check it adds to the gate ladder

No new rung. `libraries/python/check.sh` gains the pandas 2 lane of proof 8. The main lane runs the new file with the rest of `tests/`. Every `cargo` and `maturin` step keeps `--locked` and `--offline`. The `surfaces` rung runs the script as before.

## Pages

- `libraries/python/README.md`: the pandas refusal line becomes the pandas support line, with decision 11's sentence and the two tested versions.
- `libraries/python/NOTES.md`: a short "pandas (ticket 0122)" section with decisions 2, 3, 5, and 6 and the tested versions. The line "The tag's pandas probe and pandas advice are gone" and R3-19's retirement line change to say what replaced them.
- `thinkthen/__init__.py` and `__init__.pyi` docstrings name the pandas forms.
- `sdlc/planning/libraries/python.md`: the refusal line becomes the support line.
- ADR 0047, Python section: the pandas refusal bullet gets a dated 2026-09-25 amendment line naming the ruling and this ticket. The accepted text above it stays.

## Budgets

Nonblank lines, counted as 0106 counts them.

- Rust glue in `src/frame.rs`, `src/input.rs`, `src/engine.rs`, and `src/lib.rs`: at most 150 lines added. `PANDAS` and its refusal leave first, and the record names what they saved.
- `src/arrow/`: at most 40 lines added, for the `Cells` to Python values step. No `ffi.rs` file changes, and no `unsafe` is added.
- Python package, `__init__.py` and `__init__.pyi`: at most 90 lines added.
- Python tests: `tests/test_pandas.py` at most 450 lines. Edits to `test_inputs.py`, `test_door.py`, and `test_secrecy.py` at most 60 net lines.
- Scripts: at most 25 lines added to `check.sh`. The generated `requirements-pandas2.txt` does not count.
- Documentation: at most 80 net lines across the pages above.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` rise to the measured totals in the commit that adds the code. The root `sdlc/ratchet.json` does not change. The commit says what grew and why.

## Stop rules

Stop and re-score with the owner before any of these:

- Crossing a budget.
- Adding a runtime dependency or a pandas extra, or any import of pandas or Polars in the package.
- Touching `crates/thinkthen`, `conformance/`, any `ffi.rs` file, or any `unsafe` block.
- Adding a test-only export, flag, or hook beyond 0106's `probe` hooks.
- A Python loop over rows on any pandas path.
- pandas 2.3.3 failing to install beside the reused pins. The builder records which pin conflicts and does not bump a shared pin without review.
- The pandas 3 address proof failing on the pinned versions. The builder records the addresses and does not weaken the test.
- A pandas behavior the edge table does not cover that would need a new public keyword.

## Exclusions

Changes to the Polars door or the list reader beyond the "or pandas" words in two sentences. A pandas door for `filter`, `rank`, `find`, or `relate`. A whole-frame pandas Arrow export. A pandas extra in `pyproject.toml`. Release wheels and uploads (queue item 4). Any change to `thinkthen` or `conformance/`. Any live or paid call.

## Dependencies

After 0106 lands, and so after 0105, 0117, and their own dependencies. The branch starts from 0106's head at `c2a3cbb6`. The builder merges main once 0106 lands and before the build starts. If 0105 or 0106 changes a sentence, a hook, or a file this ticket names, the builder updates this ticket first.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The code reviewer checks the pandas 2 pins, the rebuild through the caller's class, and that no pandas import entered the package.

## Complexity

Contract 3; state and timing 2; reach 2; proof 3; cost of error 3; total 13. Final level: 3. The door and the worker already exist. The risk sits in the rebuild: a lost index gives silently wrong rows.

## What Ian can overturn

- The dtypes of decision 3. The fallback is the list form's Python values in an object column.
- Refusing nulls. The alternative answers a null row "not sure" and sends nothing for it.
- Refusing a question name that clashes with a column. The alternative overwrites the column, as pandas' `assign` does.
- The shape of `recognize` on a pandas frame. The alternative puts one list of names per row into a new column of the caller's frame.
- The pandas 2 pin at 2.3.3. A second lane at 2.2.3 would pin the oldest version the checks ran.
- The fallback when pandas' export raises. The alternative refuses with a sentence that names pyarrow.
- Keeping Polars' answer named by the verb while pandas keeps the caller's name.

## Evidence

- Starts from: The 2026-09-21 checks, closed in `sdlc/issues/2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md`, with their record on the tag `surfaces-wave7-frozen-2026-09-24b` in `libraries/python/NOTES.md` under "pandas checks" and in `tests/test_pandas_checks.py`. They ran pandas 2.2.3, 2.3.3, and 3.0.6 on the stand-in. They found that every pandas 2 Series lacks the Arrow capsule and crosses as a list, that pandas 3 `str` and `string[pyarrow]` columns export stable buffers the door reads in place, that a pandas 3 object column converts on every export, and that the width bench gave all four containers 32 in flight within 0.18 percent. The issue's two corrections hold: `str[pyarrow]` is not valid past 2.2.3, and the fast path is pandas 3 only. R3-19 (`test_review3_offline.py:132` on the tag) found that a list back over a non-default index turns a join into all missing values. `experiments/205-thinkthen-libs/python/NOTES.md` ran a pandas 3.0.6 Series through `filter` with no pandas import, and `FINDINGS.md` gives the address-equality proof form. Experiments 228 and 255 hold no pandas finding, and `repos/jev-experiments` holds none.
- Keeps: Every Polars and list behavior of 0105 and 0106: one engine call per column, the zero-copy Polars door, Polars frames in and out, the worker's ownership and attached release, the exit gate, the throttle proofs, and every pinned sentence except the two that gain "or pandas". `import thinkthen` imports neither pandas nor Polars.
- Changes: The pandas refusal leaves. A pandas Series crosses the Arrow door when it exports a stream and the list reader when it does not, and it comes back as the caller's Series with its index and name. A pandas frame crosses through its `on` column and comes back with new answer columns and its index. Nulls, repeated `on` labels, `MultiIndex` columns, and name clashes are refused before any send. R3-19 revives as a regression test.
- Proof: Answers and shapes against Polars, the index table with R3-19, throttle equality within 5 percent at 8 in flight on both pandas majors, the pandas 3 address proof, zero counted sends for every refusal, the secrecy test over every pandas call, the pandas-absent import child, and the pandas 2.3.3 and 3.0.6 pins, each with its planted bug.
- Defers: A pandas categorical text column. pandas 3 exports it as a dictionary array the door refuses, and pandas 2 reads it as a list. This ticket pins both and defers a dictionary reader. pandas below 2.2.3 and Python below 3.12 stay untested in the gate, as 0105's Defers records. Column forms of `filter`, `rank`, `find`, and `relate`. A faster return path through a pandas Arrow constructor. Whether Polars refuses a question name that clashes with a column before its sends. The builder files that in `sdlc/issues/` if it sends first. The product side's deck and slide copy about pandas.

## Review

- Design review: pending.
- Code review: pending.
