---
flow: build
priority: 122
opens: libraries/python sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md sdlc/records sdlc/tickets
---

# 0122: Support pandas columns and frames in the Python surface

Status: ready once design review accepts it. It starts after ticket 0106 lands. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it. The change widens a public surface, so the code reviewer names what it checked (repo `CLAUDE.md`).

## Outcome and authority

A pandas user passes a pandas column or a pandas frame to the Python surface and gets pandas back. `decide`, `decide_many`, `choose`, `score`, and `tag` take a pandas `Series` and return a pandas `Series` with the caller's index and name. `annotate` takes a pandas `DataFrame` with `on=` and returns the frame with new answer columns and the caller's index. `recognize` takes a pandas `DataFrame` with `on=` and returns a long frame of names, as it does for Polars, indexed by each name's source row. pandas 2 and pandas 3 both work. `import thinkthen` still imports neither pandas nor Polars.

Ian ruled on 2026-09-25 that pandas is supported fully in 0.1 (`sdlc/planning/one-line-plan-2026-09-25.md`, "Ian's ruling on pandas, 2026-09-25"). The ruling overturns the 2026-09-21 line "pandas leaves the surface". ADR 0017's 2026-09-25 amendment records it and names this ticket. The ruling names four things: a column in and a column with the caller's index out, a frame in and the frame with new answer columns out, pandas 2 and 3 both, and no pandas or Polars import. It also says a pandas 2 object column crosses at list speed and the page says so. Claude recommended against a columns-only version. A pandas user's first try is a whole frame, and a bare list back invites the lost-index bug R3-19 found.

This ticket touches only `libraries/python` and the pages that describe it. It builds on ticket 0106's branch. Ian can overturn every decision below. The ruling itself is his, and "What Ian can overturn" lists the choices this ticket made inside it, the `recognize` shape first.

## Design and decisions

1. **Naming a pandas object.** The binding names a pandas object by its type's top-level module, as 0105 does. A pandas `Series` is a column. A pandas `DataFrame` is a frame. The check walks the type's method resolution order by class name and module, so a subclass counts, and it imports nothing. Any other pandas object, such as an `Index` or an extension array, raises `UsageError` before any send. That sentence names the fix: pass a pandas `Series`. The order of checks stays 0106's: pandas first, then Polars, then any other Arrow producer. pandas 3 objects expose `__arrow_c_stream__`, so the pandas check must still come first.

2. **A Series goes in through the door, or through the list reader.** `decide`, `decide_many`, `choose`, `score`, and `tag` read a pandas `Series`. Every path makes exactly one engine call, as 0106's column verbs do, or none. The binding runs these steps in order on every pandas version, before any export:
   - *Empty.* At `len(series) == 0`, the verb returns the empty answer with the caller's index, name, and the verb's dtype, and sends nothing. pandas 3 exports an empty object Series as the Arrow `null` type, and `pd.DataFrame({"body": []})["body"]` is `float64`. Neither reaches the door.
   - *Nulls.* When the caller's `hasnans` reads true, the verb raises 0106's null sentence (decision 6). pandas 3 exports a Series of only `None` as the Arrow `null` type. The check catches it before the export.
   - *Categorical.* A Series whose `dtype.name` is `"category"` crosses the list reader on both versions. pandas 3 exports it as a dictionary array, and the door reads no dictionary. The list reader gets each category's text.
   - *The door.* When the Series has `__arrow_c_stream__` and pandas' export returns a stream, the Series crosses 0106's Arrow door. The worker owns the imported batches and reads the text in place. On pandas 3, a `str` column with pyarrow storage and a `string[pyarrow]` column cross zero-copy. A pandas 3 object column crosses the door too, but pandas converts it on every export, so pandas makes one copy before the door reads it.
   - *The list reader.* When the method is missing, the Series crosses as a list through 0105's list reader, whole, before the first send. Every pandas 2 Series lacks the method (the 2026-09-21 checks, on 2.2.3 and 2.3.3). So every pandas 2 column crosses at list speed. The throttle and the wall time match the list form.
   - *The fallback.* When pandas' own export call raises an `Exception`, the Series also crosses as a list. This covers pandas 3 without pyarrow installed and a pandas 3 object column that holds a value pyarrow cannot convert. Only an `Exception` from the export call itself falls back. A `KeyboardInterrupt` or any other `BaseException` from the export stops the call and propagates. A door refusal of the exported stream, such as a number format, stands and is never retried as a list.

   *Why pandas 3 uses the door at all.* The tag's width bench measured no bulk wall-time difference between the containers, because the wire dominates, and this ticket claims none. The door earns its place for three reasons. It reads pandas' own buffers in place, so the list reader's second copy of every text in Rust never exists for a large column. It is the one reader Polars already uses, with its memory checks and its release rules, so pandas 3 adds no second reader to maintain. And the ruling's page sentence, that a pandas 2 object column crosses at list speed, implies a faster pandas 3 path that the page can name. Ian can overturn this and send every pandas column through the list reader (see "What Ian can overturn").

3. **A Series comes back as the caller's own Series.** The same verbs return `type(series)(values, index=series.index, name=series.name, dtype=dtype)`. The binding never imports pandas. It calls the caller's own class.
   - Rust builds the answers from the same `Cells` the Polars path builds, then hands Python a list of values and a dtype name.
   - The dtypes: `decide` and `decide_many` give `"boolean"`, `score` gives `"Float64"`, `choose` gives `"string"`, and `tag` gives `"object"` with one list of labels per row. "Not sure" is `pd.NA`. A question the backend failed widens its column to `"string"` holding the failed marker text, as Polars widens it.
   - The answer keeps the caller's index object and the caller's name. Row order never changes. The caller's Series is not changed.
   - *Why the caller's name.* pandas' own Series methods keep the name: `series.str.len()`, `series.map(f)`, and `series.isna()` all return a Series named as the input. A pandas user expects the same. A Polars answer stays named by the verb, as 0106 set it. The page example shows the rename a user writes to add the answer as a new column: `df.join(tt.decide(late, df["body"]).rename("late"))`.
   - A Polars `Series` still gets a Polars `Series` named by the verb, and a pyarrow column still gets a list. This ticket changes neither.

4. **A frame goes in through its `on` column.** `annotate(set, df, on=)` and `recognize(df, on=)` take a pandas `DataFrame`. Before any send, the binding checks four things, each with a pinned sentence:
   - `on` names a column of the frame. A missing label gets 0106's sentence "the frame has no column named 'x'", with the label shown by Python's `repr`. That gives 0106's exact text for a `str` label.
   - `on` names exactly one column. pandas allows repeated labels, so a repeated `on` label is refused.
   - The frame's column labels have one level. A frame whose columns are a `MultiIndex` is refused.
   - For `annotate`, no question name in the set equals a column label already in the frame. pandas' `assign` would overwrite that column without a word, so a clash is refused.

   *Labels.* `on` accepts any hashable label that names exactly one column, such as `3` or `"body"`. For pandas the label never reaches Rust. Python looks it up with `df[on]` and hands Rust the Series, so Rust's `on: String` in `_annotate_frame` and `_recognize_frame` stays Polars-only and unchanged. The Polars path still requires a `str`.

   The binding then sends `df[on]` through decision 2's steps. So an empty frame sends nothing, and a null or categorical column follows decision 2. Only the `on` column crosses. The frame's other columns never leave Python, and the whole-frame `__arrow_c_stream__` export is never called. That export converts every column, and the answer needs one. `annotate` makes one `annotate_with` call. `recognize` makes one `recognize_with` call per text under one deadline, as 0106's Polars frame does.

5. **What a frame gets back.**
   - `annotate` returns `df.assign(...)` with one new column per question, in set order. Each new column is built as decision 3 builds a Series, with `index=df.index`. `assign` reads an identical index without reordering, so repeated labels and a `MultiIndex` hold. The new frame keeps every original column, its dtype, its column order, and its index. The caller's frame is not changed.
   - `recognize` returns a long frame of the caller's frame class, one row per name, with 0106's columns: `row` (counted from 1), `text`, `kind`, `start`, `end`, and `strength`. Its index holds each name's source-row label, taken from `df.index` by position. A text with no names adds no row. `relations` with `on=` stays refused, as for Polars.
   - *Matching names to rows.* On unique labels, `df.join(names)` puts each name beside its source row. On repeated labels a join multiplies rows, so the `row` column is the exact link: row `k` is `df.iloc[k - 1]`.
   - *This is the ticket's reading of the ruling.* The ruling says the frame comes back with the new answer columns. `annotate` does exactly that. `recognize` returns the long frame for two reasons. It matches the Polars shape of 0106, so one page describes both. And one row can hold several names, so a flat answer column cannot hold them without a list per cell. Ian can overturn this. It stands first under "What Ian can overturn".

6. **Nulls.** A `None`, `NaN`, or `pd.NA` in the text column raises 0106's null sentence, "the column holds nulls; the engine needs text, so drop or fill them first", with zero sends. Decision 2's `hasnans` step runs before any export on every version. A pandas 2 column therefore gets the same sentence, never "record 1 is not a str". A column of only `None` gets it too, before pandas 3 exports it as the Arrow `null` type.

7. **Indexes.** Answers are positional, and the caller's index object rides back unchanged.

   | Index | Series verbs | `annotate` | `recognize` |
   |---|---|---|---|
   | Default `RangeIndex` | kept | kept | source labels 0, 1, 2 |
   | Non-default labels, such as `[5, 7, 9]` (R3-19) | kept | kept | source labels |
   | Row `MultiIndex` | kept | kept | source tuples |
   | Repeated labels, such as `[1, 1, 2]` | kept, row order unchanged | kept, row order unchanged | source labels, repeated. `row` is the link |
   | Column `MultiIndex` | not applicable | refused before any send | refused before any send |
   | Empty Series or frame, any dtype | zero sends, an empty answer with the same index, name, and the verb's dtype | zero sends, the frame with empty new columns | zero sends, an empty long frame |

8. **What stays refused.** The pandas refusal of 0105 and 0106 leaves. The constant `PANDAS`, its tests, and its pinned sentence go. These stay refused before any send:
   - A pyarrow `Table`, a `RecordBatchReader`, an object that exposes only `__dataframe__`, a Polars `LazyFrame`, and any other frame passed with `on=`. The sentence becomes "annotate with on= takes a Polars or pandas DataFrame; a list of str takes no on=". Only the words "or pandas" change.
   - A pandas or Polars column passed to `filter`, `rank`, `find`, or `relate`. A pandas Series passed to `annotate` or `recognize` without `on=`, as a Polars Series is today. Any frame passed to these verbs, or to `annotate` without `on=`. Each gets the list-only sentence. That sentence gains "or pandas" in the same way.
   - A pandas Series passed to `details`. It keeps 0106's sentence "details reads one str, not a column".
   - A pandas frame passed to a column verb. It keeps 0106's sentence "a data frame is not a column; pass df[\"name\"], or annotate with on=".
   - Any other pandas object (decision 1).

9. **No pandas import and no pandas dependency.** The wheel imports neither pandas nor Polars, and `pyproject.toml` gains no pandas extra. A pandas user already has pandas. The binding touches pandas only through the caller's own object: `type`, `len`, `.index`, `.name`, `.dtype.name`, `.columns`, `.hasnans`, `.assign`, item access, `index.take`, and `__arrow_c_stream__`. Python never loops a row.

10. **No new test-only hook.** The address proof reuses 0106's `_arrow_probe` under the `probe` feature. The throttle proofs build `tt.Engine(throttle=8, base_url=...)`, as 0106 does. Every other proof runs through the public calls.

11. **The page says what is fast.** `README.md` and `NOTES.md` state: a pandas 2 column crosses at list speed and still makes one engine call at the full throttle. A pandas 3 `str` or `string[pyarrow]` column crosses zero-copy. A categorical column crosses at list speed on both versions. `astype("string[pyarrow]")` makes a pandas 3 object column zero-copy, and it gives pandas 2 no fast path. The spelling `str[pyarrow]` is not valid past pandas 2.2.3, so no page uses it (the 2026-09-21 issue's first correction). The page shows the `.rename` example of decision 3.

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
| R3-19 | retired by 0106 | A Series with index `[5, 7, 9]` and name `"body"` gets back a Series whose index `.equals` the input's and whose name is `"body"`. On those unique labels, `df.join(answer.rename("x"))["x"]` holds no missing value. The same holds for `annotate` on that frame. | Rebuild without `index=`. The answer gets a fresh `RangeIndex`, the index assertion fails, and the join reads all missing. |

## Proof

Every test below is outside-in through the public calls. Each engine call runs in a child on its own loopback backend, as 0105 set up. The new file `tests/test_pandas.py` runs in both pandas lanes. No test skips. Where the two majors differ, each lane asserts its own row.

Values are checked against the list form in the same child, or against fixed values. The list form is 0105's tested path, so it stands as the reference. No pandas test takes its expected values from the Polars path.

1. **Answers and shapes.** For each verb and each Series kind the lane offers, the answer is the caller's class, with the caller's name and the dtype of decision 3. Its values, read by position, equal the list form's answers for the same texts. pandas 3 runs `str`, `string[pyarrow]`, object, and categorical columns. pandas 2 runs object, `string[pyarrow]`, and categorical columns. A failed question on 0092's refusing arm widens to `"string"` with the fixed failed marker text. `annotate` on a frame keeps every original column equal and adds the set's columns, and each new column's values, by position, equal the list form's `annotate` answers. Plant: return 0106's list for a pandas column. The class assertion turns red.
2. **The caller's index survives (R3-19).** The table in decision 7 runs row by row: a non-default index, a row `MultiIndex`, repeated labels, and an empty frame. For every row, the test asserts `answer.index.equals(df.index)`, the answer's name, and the values by position against the list form. For `annotate`, it asserts the new frame's index `.equals` the input's and each new column's values by position. For `recognize`, it asserts each name's index label equals `df.index[row - 1]`. On the unique-label rows only, a join on the caller's frame holds no missing value. A join cannot check repeated labels, because it multiplies rows. Plant: rebuild without `index=`. Every non-default row turns red. A second plant reverses the values before the rebuild. The by-position assertion turns red on every row, repeated labels included.
3. **Throttle equality at 8 in flight.** In a child, `tt.Engine(throttle=8, base_url=".../arm/delay/100/v1")` runs 200 texts through `decide_many` as a list and as one Series per route the lane offers. The main lane runs a pandas 3 `str` Series for the door and a categorical Series for the list reader. The pandas 2 lane runs an object Series for the list reader. Each run takes about 2.5 s with 200 counted sends and equal answers. Each pandas wall time falls within 5 percent of the list's. On 0092's held arm, each form reaches exactly 8 counted sends and no more. Plant: answer a pandas column row by row through the single-text call. It holds 1 in flight and runs about eight times longer. Both assertions turn red.
4. **Zero copy on pandas 3.** In the main lane, `_arrow_probe` over a `str` Series and a `string[pyarrow]` Series reads the same buffer addresses pandas' own export hands out. That is experiment 205's proof form, and 0106's address test already runs it for Polars. The route row of the edge table shows the verbs take the door. Plant: copy the texts into a `Vec<String>` before the hand-off. The addresses differ.
5. **Every refusal sends nothing.** The edge table below runs in one child per lane. Each row pins its whole sentence. After every row, the backend's `count` line reads 0. A "sends nothing" test counts loopback requests, never a dry run.
6. **Secrecy on the failure arms.** 0105's secrecy test gains each column verb over a pandas Series, and `annotate` and `recognize` on a pandas frame, each on the refusing arm, the 401 arm, and the 503 arm. No message and no `repr` of the error carries the fake key or the address credentials. The test runs in both lanes. Plant: a pandas failure path formats the child's environment into its message. The fake key shows, and the test turns red.
7. **pandas absent.** 0106's import test gains pandas: after `import thinkthen` in a fresh child, neither `pandas` nor `polars` is in `sys.modules`. A second child sets `sys.modules["pandas"] = None`, so any pandas import raises. It then imports `thinkthen` and runs `decide_many` over a list and over a Polars Series. Both answer. Plant: add `import pandas` inside a `try` at the top of `__init__.py`. The first child finds pandas in `sys.modules` and turns red. A second plant imports pandas without the `try`. The second child raises on import and turns red.
8. **Two pinned pandas versions.** pandas 3.0.6 stays the pin in `requirements-dev.txt`, and the main lane runs every test on it. A new `requirements-pandas2.txt`, hashed by `uv pip compile --generate-hashes --offline` as the first file is, pins pandas 2.3.3. It reuses the first file's pins for numpy 2.5.3, pyarrow 25.0.1, polars 1.44.2, polars-runtime-32 1.44.2, and pytest 9.1.1 and its helpers. It adds only what pandas 2 needs beyond them, such as `pytz` and `tzdata`, each named in the record with its version. 2.3.3 is the newest pandas 2 the 2026-09-21 checks ran. The oldest they ran is 2.2.3, and the page names both.

### The two lanes in `check.sh`

- The pandas 2 lane runs after the main lane passes.
- It uses the same host Python the main lane found. Its venv lives beside the first under `~/.cache/thinkthen-toolchains/python/`, with its own cache key: the main key's input plus the pin file's name.
- `PYO3_PYTHON` stays on the main lane's venv. The main lane builds the probe wheel once with `maturin build --locked --offline --features probe` into its scratch folder, beside its `maturin develop`. Both builds share one cargo target and one set of features, so pyo3 compiles once. The lane installs that wheel into venv 2 with `uv pip install --offline --no-deps`. The extension is `abi3`, so one wheel serves both venvs on one host Python.
- *Lane preconditions.* Before its tests, each lane checks one fact in one line of `check.sh` and stops with a named failure when it is false. The main lane checks that a pandas 3 `str` Series has `__arrow_c_stream__`. The pandas 2 lane checks that a pandas 2 Series lacks it. These are pin checks and not tests.
- The pandas 2 lane then runs `tests/test_pandas.py` and `tests/test_secrecy.py`.
- A pin missing from uv's cache exits 77, and the rung reports "not run". A "not run" pandas 2 lane blocks landing. The landing record shows the pandas 2 lane's pass line with its pandas version, or the ticket does not land.

### The edge-case table

Each row is one assertion in proof 5 or proof 2. "Door" means 0106's Arrow door. "List" means 0105's list reader. Each plant turns its row red.

| Input | pandas 3.0.6 | pandas 2.3.3 | Planted bug |
|---|---|---|---|
| `str` Series (pyarrow storage) to `decide` | door, zero copy, `"boolean"` Series back | not the pandas 2 default, and not run | Send every pandas column through the list path. The route row below turns red. |
| `string[pyarrow]` Series to `score` | door, zero copy, `"Float64"` Series back | list, same answers | Drop the list reader for a missing capsule. pandas 2 raises. |
| object Series to `choose` | door after pandas' own copy, `"string"` Series back | list, same answers | Drop the list reader for a missing capsule. pandas 2 raises. |
| categorical text Series to `tag` | list, same answers | list, same answers | Drop the categorical step. The door refuses pandas 3's dictionary array, and the answer row turns red. |
| `int64` Series to `decide` (the route row) | "the column's Arrow format is 'l', not text" | "record 0 is not a str" | Send pandas 3 through the list path. The pandas 3 sentence changes. |
| object Series holding `"a", 1` to `decide` | export raises, list, "record 1 is not a str" | "record 1 is not a str" | Let the export error escape. pyarrow's own error shows. |
| Series holding `None`, `NaN`, or `pd.NA` among texts to `tag` | 0106's null sentence | 0106's null sentence | Skip the `hasnans` step. pandas 2 reads "record 1 is not a str". |
| object Series of only `None` to `decide` | 0106's null sentence | 0106's null sentence | Skip the `hasnans` step. pandas 3 exports the Arrow `null` type, and the door's format sentence shows. |
| empty object Series, named `"body"`, index `[]`, to `decide` | empty `"boolean"` Series named `"body"`, zero sends | same | Skip the empty step. pandas 3 exports the Arrow `null` type, and the door refuses it. |
| `pd.DataFrame({"body": []})` to `annotate(on="body")`, a `float64` column | the frame with empty new columns, zero sends | same | Skip the empty step. On pandas 3 the door refuses the float format, and the answer row turns red. |
| Series to `filter`, `rank`, `find`, `relate` | the list-only sentence | the list-only sentence | Skip the container check for pandas objects. The Series iterates and sends. |
| Series to `annotate` without `on=`, and to `recognize` without `on=` | the list-only sentence | same | Skip the container check for pandas objects. The Series iterates and sends. |
| Series to `details` | "details reads one str, not a column" | same | Route `details` through the column path. It sends. |
| `DataFrame` to `decide` | "a data frame is not a column; pass df[\"name\"], or annotate with on=" | same | Skip the frame check on the column path. The pinned sentence turns red. |
| `DataFrame` to `annotate` without `on=` | the list-only sentence | same | As R2-11. |
| `DataFrame` with `on="missing"` | "the frame has no column named 'missing'" | same | Look the column up after the send. |
| `DataFrame` with integer column labels, `on=3` | answers, and the new frame keeps label `3` | same | Coerce `on` to `str` before the lookup. The frame has no column `'3'`, and the missing-label sentence shows. |
| `DataFrame` with two columns named `body` | the repeated-label sentence | same | Take `df[on]` unchecked. A frame reaches the column path. |
| `DataFrame` with `MultiIndex` columns | the column-level sentence | same | Skip the level check. `assign` writes tuple labels. |
| `DataFrame` with a column named as a question | the clash sentence | same | Skip the clash check. `assign` overwrites the column. |
| `recognize(df, relations=..., on=)` | 0106's relations sentence | same | Check relations after the send. |
| pandas `Index` to `decide_many` | the other-pandas-object sentence | same | Read it as an iterable. It sends. |
| pyarrow `Table` to `annotate(on=)` | the frame sentence with "or pandas" | same | Accept any Arrow-stream frame. It sends. |

The rows that answer assert answers against the list form in the same child and count the sends each makes. Every refusal row and every empty row asserts a count of 0. The builder pins the sentence each lane actually gives. A row whose observed route or sentence differs from this table stops the build until the ticket is updated.

### The four questions for each new test

| Test | Behavior it protects | Credible regression | Why no existing test catches it | Test-only hook |
|---|---|---|---|---|
| Answers and shapes | A pandas column gets a pandas Series back with the list form's values | The tag's list return comes back | 0106 tests Polars and pyarrow only | None |
| The caller's index survives | Answers sit on the caller's rows, by position, on any index | R3-19: a fresh index, or values paired to the wrong rows | Polars has no index, so 0106 has nothing to check | None |
| Throttle equality | A pandas column runs at the list's speed and width on each route | A per-row loop on a list-path route | 0106's throttle test runs a Polars Series only | None. `tt.Engine(throttle=8)` is public |
| Zero copy on pandas 3 | pandas 3 text crosses in place | A copy before the hand-off | 0106's address test reads Polars buffers only | `_arrow_probe`, 0106's existing hook under `probe` |
| The edge table | No pandas input half works, and empty and null columns answer or refuse before any export | A refusal checked after the send, or an empty column sent to the door | The pandas rows of 0105 and 0106 pin a refusal this ticket removes | None |
| Secrecy on the failure arms | No new failure path leaks the key | A pandas error message formats settings | 0105's secrecy test has no pandas call | None |
| pandas absent | The package imports and answers without pandas | A convenience `import pandas` in `__init__.py` | 0106's import test checks Polars only | None. `sys.modules` blocking is standard Python |

The rewritten rows of `test_inputs.py` and `test_door.py` replace the pandas refusal rows. They add no new test.

## The check it adds to the gate ladder

No new rung. `libraries/python/check.sh` gains the pandas 2 lane and the two lane preconditions above. The main lane runs the new file with the rest of `tests/`. Every `cargo` and `maturin` step keeps `--locked` and `--offline`. The `surfaces` rung runs the script as before.

## Pages

- `libraries/python/README.md`: the pandas refusal line becomes the pandas support line, with decision 11's sentences, the `.rename` example, and the two tested versions.
- `libraries/python/NOTES.md`: a short "pandas (ticket 0122)" section with decisions 2, 3, 5, and 6, the reason pandas 3 uses the door, and the tested versions. The line "The tag's pandas probe and pandas advice are gone" and R3-19's retirement line change to say what replaced them.
- `thinkthen/__init__.py` and `__init__.pyi` docstrings name the pandas forms.
- `sdlc/planning/libraries/python.md`: the refusal line becomes the support line.
- ADR 0047, Python section: the pandas refusal bullet gets a dated 2026-09-25 amendment line naming the ruling and this ticket. The accepted text above it stays.

## Budgets

Nonblank lines, counted as 0106 counts them.

- Rust glue in `src/frame.rs`, `src/input.rs`, `src/engine.rs`, and `src/lib.rs`: at most 150 lines added. `PANDAS` and its refusal leave first, and the record names what they saved.
- `src/arrow/`: at most 40 lines added, for the `Cells` to Python values step. No `ffi.rs` file changes, and no `unsafe` is added.
- Python package, `__init__.py` and `__init__.pyi`: at most 100 lines added.
- Python tests: `tests/test_pandas.py` at most 500 lines. Edits to `test_inputs.py`, `test_door.py`, and `test_secrecy.py` at most 60 net lines.
- Scripts: at most 35 lines added to `check.sh`. The generated `requirements-pandas2.txt` does not count.
- Documentation: at most 90 net lines across the pages above.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` rise to the measured totals in the commit that adds the code. The root `sdlc/ratchet.json` does not change. The commit says what grew and why.

## Stop rules

Stop and re-score with the owner before any of these:

- Crossing a budget.
- Adding a runtime dependency or a pandas extra, or any import of pandas or Polars in the package.
- Touching `crates/thinkthen`, `conformance/`, any `ffi.rs` file, or any `unsafe` block.
- Adding a test-only export, flag, or hook beyond 0106's `probe` hooks.
- A Python loop over rows on any pandas path.
- pandas 2.3.3 failing to install beside the reused pins. The builder records which pin conflicts and does not bump a shared pin without review.
- The one probe wheel failing to import in venv 2. The builder records why and does not add a second pyo3 build without review.
- The pandas 3 address proof failing on the pinned versions. The builder records the addresses and does not weaken the test.
- A pandas behavior the edge table does not cover that would need a new public keyword.

## Exclusions

Changes to the Polars door or the list reader beyond the "or pandas" words in two sentences. A pandas door for `filter`, `rank`, `find`, or `relate`. A whole-frame pandas Arrow export. A dictionary reader in the door. A pandas extra in `pyproject.toml`. Release wheels and uploads (queue item 4). Any change to `thinkthen` or `conformance/`. Any live or paid call.

## Dependencies

After 0106 lands, and so after 0105, 0117, and their own dependencies. The branch starts from 0106's head at `c2a3cbb6`. The builder merges main once 0106 lands and before the build starts. If 0105 or 0106 changes a sentence, a hook, or a file this ticket names, the builder updates this ticket first.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code. The code reviewer checks the pandas 2 pins, the rebuild through the caller's class, the pandas 2 lane's pass line in the record, and that no pandas import entered the package.

## Complexity

Contract 3; state and timing 2; reach 2; proof 3; cost of error 3; total 13. Final level: 3. The door and the worker already exist. The risk sits in the rebuild: a lost index or a misaligned value gives silently wrong rows.

## What Ian can overturn

- **The shape of `recognize` on a pandas frame.** This ticket reads the ruling's "the frame comes back with the new answer columns" as binding `annotate`, and keeps `recognize`'s long frame, as Polars returns it. The reasons: Polars matches, and one row can hold several names. The owner ruled this during review on 2026-09-25. The alternative puts one list of names per row into a new column of the caller's frame.
- The dtypes of decision 3. The fallback is the list form's Python values in an object column.
- Refusing nulls. The alternative answers a null row "not sure" and sends nothing for it.
- Refusing a question name that clashes with a column. The alternative overwrites the column, as pandas' `assign` does.
- Sending pandas 3 through the Arrow door. The alternative sends every pandas column through the list reader. It gives the same bulk wall time and drops the export fallback, but it keeps a second copy of every text in Rust.
- The pandas 2 pin at 2.3.3. A second lane at 2.2.3 would pin the oldest version the checks ran.
- The fallback when pandas' export raises an `Exception`. The alternative refuses with a sentence that names pyarrow.
- Categorical columns on the list reader. The alternative adds a dictionary reader to the door, with its own `unsafe` review.
- Keeping the caller's name on a pandas answer while a Polars answer stays named by the verb.

## Evidence

- Starts from: The 2026-09-21 checks, closed in `sdlc/issues/2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md`, with their record on the tag `surfaces-wave7-frozen-2026-09-24b` in `libraries/python/NOTES.md` under "pandas checks" and in `tests/test_pandas_checks.py`. They ran pandas 2.2.3, 2.3.3, and 3.0.6 on the stand-in. They found that every pandas 2 Series lacks the Arrow capsule and crosses as a list, that pandas 3 `str` and `string[pyarrow]` columns export stable buffers the door reads in place, that a pandas 3 object column converts on every export, and that the width bench gave all four containers 32 in flight within 0.18 percent. The issue's two corrections hold: `str[pyarrow]` is not valid past 2.2.3, and the fast path is pandas 3 only. R3-19 (`test_review3_offline.py:132` on the tag) found that a list back over a non-default index turns a join into all missing values. `experiments/205-thinkthen-libs/python/NOTES.md` ran a pandas 3.0.6 Series through `filter` with no pandas import, and `FINDINGS.md` gives the address-equality proof form. Experiments 228 and 255 hold no pandas finding, and `repos/jev-experiments` holds none. The 2026-09-25 design review of this ticket found that pandas 3 exports empty and all-`None` object columns as the Arrow `null` type, and that a join cannot check repeated labels.
- Keeps: Every Polars and list behavior of 0105 and 0106: one engine call per column, the zero-copy Polars door, Polars frames in and out, the worker's ownership and attached release, the exit gate, the throttle proofs, and every pinned sentence except the two that gain "or pandas". `import thinkthen` imports neither pandas nor Polars.
- Changes: The pandas refusal leaves. A pandas Series is checked for empty, nulls, and categorical before any export, then crosses the Arrow door when it exports a stream and the list reader when it does not. It comes back as the caller's Series with its index and name. A pandas frame crosses through its `on` column. `annotate` returns it with new answer columns and its index, and `recognize` returns the long frame indexed by source row. Repeated `on` labels, `MultiIndex` columns, and name clashes are refused before any send. R3-19 revives as a regression test.
- Proof: Answers and shapes against the list form, the index table with values by position and R3-19, throttle equality within 5 percent at 8 in flight on each route, the pandas 3 address proof, zero counted sends for every refusal and every empty input, the secrecy test on the failure arms, the pandas-absent import child, and the pandas 2.3.3 and 3.0.6 pins, each with its planted bug. The landing record shows the pandas 2 lane passed.
- Defers: A dictionary reader for categorical columns in the door. pandas below 2.2.3 and Python below 3.12 stay untested in the gate, as 0105's Defers records. Column forms of `filter`, `rank`, `find`, and `relate`. A faster return path through a pandas Arrow constructor. Whether Polars refuses a question name that clashes with a column before its sends. The builder files that in `sdlc/issues/` if it sends first. The product side's deck and slide copy about pandas.

## Review

- Design review, 2026-09-25, at `fcf25242`: not accepted, with 2 high, 4 medium, and 4 low findings. It agreed with the dtypes, the refusals, the pandas-first check, and the rebuild without a pandas import. This version answers each finding: empty and all-null columns before any export, values by position on repeated labels, the `recognize` reading and its overturn line, categorical on the list reader, the pandas 2 lane as a landing condition, the `Exception`-only fallback, the one-build second lane, the test-gate trims, the non-`str` `on` and Series-without-`on=` rows, and the reason for the caller's name. It also records why pandas 3 uses the door. Re-review: pending.
- Code review: pending.
