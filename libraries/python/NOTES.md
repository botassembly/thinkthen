# The Python surface, lane notes

## 2026-09-21 — the lane lands

Tooling already present at the user level from 205: maturin 1.15.0 (`uv
tool`), uv. Folder-local venv `.venv` (CPython 3.13), made with
`uv venv .venv && uv pip install --python .venv/bin/python pytest pandas`;
removal is `rm -rf .venv`. pandas is transient, for the slide sample's
data frame only. Nothing was installed outside this folder; no key, no
paid call, nothing published, no sudo.

The shim binds `thinkthen-contract`, never the stand-in beneath it:
pyo3 0.29.2, `abi3-py310`, module `thinkthen._thinkthen` behind a
`thinkthen` package whose `__init__` owns the data-frame arm of
`annotate`. `panic = "unwind"` in the release profile, per the ADR. All
eight verbs plus `decide_many`, `question`, `details`, `usage`;
`deadline` in seconds on every entry point; the six kinds
map to exception classes that carry `kind` and `retryable`; `Cancelled`
subclasses `KeyboardInterrupt` (pyo3 gives an exception one base, per
the 211 shape); a bulk wait runs the signal poll and raises within a
tick plus one in-flight round.

## The check, by command

```
$ ./check.sh                      (stub on 8211 at 300 ms)
surface tests, null backend:   18 passed
conformance slice, offline:    18 passed, 0 failed, 1 skipped
slide sample:                  runs as drawn, below
cancel on the wire:            raised Cancelled, nothing served after (32)
```

Without the stub the script says `wire tests skipped` and everything
offline still runs. Conformance 17 (usage and cache) is the recorded
divergence: the stand-in holds no disk cache, so it counts 2 sends and 0
cache answers where the case expects 1 and 1. Conformance 18 (cancel)
runs as `tests/test_cancel.py` on the wire: SIGINT at t+1 s into a
40-record batch at width 8 raised `Cancelled` with the stub's count
frozen at 32 through a 2 s settle.

## The slide sample, as drawn

```
decide          -> True   (comment: True)
band decide     -> False   (comment: None)
filter          -> []   (comment: none drawn)
annotate        -> columns ['body', 'team', 'urgency', 'wants_refund']
  three new columns: ['team', 'urgency', 'wants_refund']
```

Two findings, both reported and not hidden:

1. **The band comment does not reproduce.** `tt.decide(refund, "I was
   charged twice. Can you fix this?")` answers `False` on the stand-in:
   the evidence carries none of the three-bucket keywords, so its
   probability falls below the band's low side. The real backend's
   recording put this evidence inside the band (`None`), which is where
   the slide's comment came from. The slide changes: either the comment
   or the evidence. This is the finding Phase A predicted.
2. **The column order follows the frame, not the set.** `wants_refund`
   lands last because `annotate` attaches in the set's name order but
   pandas places new columns at the end in insertion order
   (`team`, `urgency`, `wants_refund`); the set's own order is
   `wants_refund`, `team`, `urgency`. Three new columns either way. If
   the slide means to promise set order in the frame, the wrapper needs
   an explicit reorder; the slide's comment only promises the columns.

## The stub's wire shapes

The loopback stub answers one noul shape a request. The `choose`,
`score`, `tag`, and `annotate` wire shapes (a `choice` question with
criteria, a `score` keyed by level) cannot be served by it, so those
verbs run on the stand-in's offline backend and on recordings, the same
reason job 3 marked five exchanges `shaped-to-contract`. `decide`,
`decide_many`, `filter`, and `rank` are proven on the wire by the
conformance slice and the cancel proof.

## The digest equality

`tt.question(decide=..., threshold=(0.2, 0.8), true_=..., false_=...)`
and `tt.question(file="refund.json")` give the same digest, which the
test asserts. The yes and no arms are grammar keys like the threshold;
parts and files agree once the parts carry them.

## 2026-09-21 — the Polars swap: both containers, one Rust door, the equality

Ian's ruling (sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md, Polars section and its clarification): the Python surface supports both pure Python and Polars DataFrame Python, both first-class; all scalability and vectorization live in Rust; Python never loops a row. This lane is the plan's experiment 212 (sdlc/planning/polars-plan.md).

**What was built.** `src/arrow.rs`: the Arrow C stream door in both directions. In: `__arrow_c_stream__` consumed by hand per the C Data Interface (the 205 precedent, extended for the `vu` string-view layout Polars 1.44 exports — 16-byte views, strings of 12 bytes or less inline, the rest references into data buffers). `decide_many` takes a Polars Series; `annotate(set, frame, on=)` consumes the frame's stream and returns the frame through the same stream form, and the wrapper returns `type(records)(frame)`. The wheel never imports Polars: the door duck-types `__arrow_c_stream__`, and the wrapper's only Polars-side call is the class of the caller's own object. Out: `ArrowFrame` mints a fresh C stream; the schema is deep-copied into a tree the consumer owns and releases, and each batch hands out its own allocation whose `release` frees it — the input hold rides an `Arc` per batch, so aliased original columns stay valid exactly as long as the consumer owns them.

**The equality proof** (the plan's central measure), stub on 8211 at 300 ms, jobs 32, 1,000 records:

```
list   wall 9.660 s  stats {'connections': 33, 'max_in_flight': 32, 'requests': 1000}
series wall 9.662 s  stats {'connections': 1, 'max_in_flight': 32, 'requests': 1000}
spread 0.0162% of the slower run
width equality: both containers hold the same gate
```

The 33-to-1 connection difference is pool reuse: the series run rode the pool the list run had already opened. Wall time, requests, and in-flight are identical, which is the proof the width and the scaling run in Rust.

**Zero copy, proven by address** (the 205 shape): the values and view buffers Rust reads are the buffers Polars handed out.

```
python side: values 0x7c2e1b217000 offsets/views 0x7c2e1b277210 n 3
rust side  : values 0x7c2e1b217000 offsets/views 0x7c2e1b277210 n 3
match: True
```

**Null-backend cost**, 10,000 records, warm: list 2.726 µs/record, series 2.545 µs/record — the same class, and both far under the 50 µs budget. (`tests/bench_cost_polars.py`.)

**The suite.** `tests/test_polars_door.py`, 10 tests: parity of `decide_many` and `annotate` against the list door, multi-row multi-column frames, the address proof, inline/reference/blank layouts, nulls refused with the reason, a non-text column refused with its format, a missing `on` column named, the container rule named for a plain list with `on=`, and a subprocess proving `import thinkthen` leaves `polars` out of `sys.modules`. The existing surface suite stays green (18 tests); the slide sample runs as drawn on a Polars frame (`three new columns: ['team', 'urgency', 'wants_refund']`; the band-decide stand-in finding is the sample's own, unchanged, as the task ruled).

**Two bugs the door caught, recorded.** First, the producer originally handed arrays with no-op releases and freed everything at stream release; Polars keeps zero-copy views, so the data was freed under it — nondeterministic garbage (an urgency reading of 4.3e-315) until per-batch ownership landed. Second, the wiring loop overwrote the aliased original columns' buffers pointer with an empty `Vec`'s dangling pointer; only the new columns are wired now.

**Rules kept.** Loopback stub only, no key, no paid call, nothing published, no sudo, nothing outside this folder edited. Polars installed folder-local: `uv pip install --python .venv/bin/python polars` (polars 1.44.2). No runtime installer touched any rc file:

```
$ grep -c "deno\|polars" ~/.zshrc
0
```

**Not run in this lane (unchecked).** The plugin expression and the version pin (experiment 216); the gate against Polars' own parallelism (213); fork with a warm Polars pool (214); deadline and interrupt inside an expression (215); `filter`, `rank`, and `find` over Polars containers — they still take lists, the door is ready for them; the optional extra `pip install thinkthen[polars]` is declared in `pyproject.toml` but was not exercised from an index.

## pandas checks

The five checks from `repos/thinkthen/sdlc/issues/2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md`, run on the stand-in engine: no pandas door, no pandas import, no paid call. Versions: pandas 3.0.6, 2.3.3, and 2.2.3; pyarrow 25.0.1; Polars 1.44.2; Python 3.13.5. New artifacts: `tests/test_pandas_checks.py` (9 tests, wired into `check.sh` as "== the pandas checks, null backend / 9 passed"), `tests/bench_width_pandas.py`, `tests/bench_cost_pandas.py`.

### Check 5 first: the frame call raises, and the sentence names no fix — the issue's bar is not met

```
$ ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_pandas_checks.py -q
.........                                                                [100%]
9 passed in 0.36s

$ .venv/bin/python - <<'PY'
import pandas as pd, thinkthen as tt
df = pd.DataFrame({"body": ["a", "b"], "id": [1, 2]})
tt.annotate("tests/fixture/form.json", df, on="body")
PY
ValueError : DataFrame constructor not properly called!
```

The wrapper takes the capsule path (every tested pandas frame carries `__arrow_c_stream__`, back to 2.2.3), Rust reads the frame and appends its columns, and the refusal lands at `type(records)(frame)` — pandas' own constructor error. Two facts from the run: it never half works (the frame is untouched, no columns appear, and the column-names-judged hazard does not occur because the frame takes the capsule path on every tested version), and the sentence names no fix, so the issue's "refuses with a sentence that names the fix" bar is not met. pandas 2.3.3 and 2.2.3 refuse the same way, same message, checked in their venvs. Per the task, the library was not bent; the product side rules on the refusal sentence.

### Check 1: object-dtype answers equal the list, on every tested pandas

```
pandas 3.0.6  object capsule: True   answers equal: True   (the Arrow door; every Series exports)
pandas 2.3.3  object capsule: False  answers equal: True   (the list door)
pandas 2.2.3  object capsule: False  answers equal: True   (the list door)
```

On pandas 3 the object column takes the Arrow door and re-converts on every export (check 2's table); on pandas 2 it crosses as an iterable, list speed. One crossing and 32 in flight are proven by the width bench below: 1,000 requests and `max_in_flight` 32 for every container, one `decide_many` call each.

### Check 2: the fast path is a pandas-major story, and the issue's spelling is wrong

```
pandas 2.2.3  str[pyarrow]      -> accepted, dtype string[pyarrow]
pandas 2.3.3  str[pyarrow]      -> TypeError: data type 'str[pyarrow]' not understood
pandas 3.0.6  str[pyarrow]      -> TypeError: data type 'str[pyarrow]' not understood
```

The issue's spelling works only on 2.2.3 (as an alias). The valid spellings are the default `str` (Arrow-backed on pandas 3) and `string[pyarrow]`. The address form, 205's, single process:

```
polars             fmt vu  exports stable: True   walk(data,off) == probe(vals,views): True
pd default str     fmt U   exports stable: True   match: True
pd string[pyarrow] fmt U   exports stable: True   match: True
pd object          fmt u   exports stable: False  match: False (fresh buffers every export)
```

So the 205 form holds for the Arrow-backed pandas columns on pandas 3.0.6 and the door reads exactly those buffers. An object column's exporter converts on every call, so no address form can hold for it; its answers are check 1's. On pandas 2.2.3 and 2.3.3 a `string[pyarrow]` Series carries no capsule at all — the fast path exists only on pandas 3 in this matrix, and pandas 2 columns all cross at list speed and still work.

### Check 3: equal on the wire, an export constant on tiny batches

Width bench, 1,000 records at a 300 ms stub, jobs 32 (`ENGINE_BASE_URL=http://127.0.0.1:8211/v1 ENGINE_WIDTH=32 .venv/bin/python tests/bench_width_pandas.py`):

```
list             wall 9.651 s  stats {'connections': 33, 'max_in_flight': 32, 'requests': 1000}
pd object        wall 9.669 s  stats {'connections': 1, 'max_in_flight': 32, 'requests': 1000}
pd str (arrow)   wall 9.659 s  stats {'connections': 1, 'max_in_flight': 32, 'requests': 1000}
polars           wall 9.669 s  stats {'connections': 1, 'max_in_flight': 32, 'requests': 1000}
slowest pd object 9.669 s | fastest list 9.651 s | spread 0.18%
```

Null backend cost, 10,000 records, three passes in two orders: all four containers land at 2.3–2.6 µs a record and the differences move with run order, not with the container. Where the containers do separate is the one-record-at-a-time loop, 2,000 calls each:

```
single-record call: list             51.07 us/call
single-record call: polars           55.42 us/call
single-record call: pd str (arrow)   98.82 us/call
single-record call: pd object       115.76 us/call
```

That ~45–65 µs is the Arrow export setup per call, worst on the object column, invisible in a bulk batch. The manual's slow-one/fast-one statement, from these numbers: bulk batches are indistinguishable across the four containers; in a one-row-at-a-time loop the list and Polars are cheapest, and a pandas column pays an export per call.

### Check 4: what comes back is a plain list of booleans, for every container

`decide_many` returns judgments, not a re-created column: `type(out) is list` for the list, both pandas forms, and Polars; the one conversion line is `pd.Series(out)`, which the test runs and which comes back bool. The test states each returned type.

### The pin, and the absent-pandas run

Oldest pandas the checks ran on: 2.2.3 (the oldest release with Python 3.13 wheels; pandas 2.0 and 2.1 cannot install on this interpreter and are unchecked). On 2.2.3 everything crosses as a list and the frame refuses the same way. The zero-copy path exists only on pandas 3.0.6 in this matrix — so the product sentence "one astype line makes it fast" is true on pandas 3 (`astype("string[pyarrow]")`, or the default) and has no fast path to point at on pandas 2.

pandas absent, from the built wheel in a clean venv:

```
$ maturin build --release    # thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl
$ uv venv /tmp/tt-nopd && uv pip install --python /tmp/tt-nopd/bin/python target/wheels/thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl
$ cd /tmp && /tmp/tt-nopd/bin/python -c "..."
wheel install from /tmp: [True, False]
thinkthen from: /tmp/tt-nopd/lib/python3.13/site-packages/thinkthen/__init__.py
pandas findable: False
```

`pandas`, `polars`, and `pyarrow` were absent from `sys.modules` after `import thinkthen`, and a `decide_many` ran on the null backend. The in-tree test pins the same claim for pandas and Polars (subprocess import check).

Unchecked: pandas below 2.2.3 (no wheels for this interpreter; the older-major frame hazard could not be tested, and all tested majors refuse the frame); categorical, nullable, and other dtype spellings beyond the five checks; the reproduction venvs (`.venv-pd2`, `.venv-pd22`) were removed after the runs; the commands above recreate them.

### The refusal now names the fixes, 2026-09-21

Before: `tt.annotate("form.json", pandas_df, on="body")` raised pandas' own `ValueError: DataFrame constructor not properly called!` from inside the return path — the issue's bar ("refuses with a sentence that names the fix") was not met, and the call never half worked.

After: the wrapper catches the failed reconstruction of the input's own type and raises the library's own `UsageError`, chained from the original, naming the two remedies in order:

```
annotate with on= cannot rebuild a pandas frame from the Arrow stream it
returns. Pass the column instead — tt.annotate(set, df[column]) — which
returns a list of dictionaries, one per row, or convert once and back —
tt.annotate(set, pl.from_pandas(df), on=column).to_pandas()
```

Both remedy lines were run before being named:

- Remedy 1, recorded returned type: `tt.annotate("tests/fixture/form.json", df["body"])` returns a `list` of dicts, one per row, keys the set's question names (`{'team', 'urgency', 'wants_refund'}`).
- Remedy 2: `tt.annotate("tests/fixture/form.json", pl.from_pandas(df), on="body").to_pandas()` returns a pandas DataFrame, shape (3, 5), columns `['body', 'id', 'team', 'urgency', 'wants_refund']`.

The task's guessed line `pd.DataFrame(arrow_frame.to_pydict())` does not work and is not named: the stream door's `ArrowFrame` has no `to_pydict`, and the pyarrow route is unsafe (next finding). The catch is generic — any capsule frame whose reconstruction fails gets the same refusal; a pyarrow Table input returned a clean `TypeError` before and now gets the named refusal too, with the pandas wording. No pandas import entered the wheel; no per-value path; the existing final `UsageError` for a frame without the capsule is unchanged.

**Finding, outside this fix: `pa.table(ArrowFrame)` aborts the process.** Minimal reproducer:

```
ENGINE_NULL=1 .venv/bin/python - <<'PY'
import pandas as pd, pyarrow as pa
from thinkthen._thinkthen import annotate_stream
df = pd.DataFrame({"body": ["a", "b", "c"]})
af = annotate_stream("tests/fixture/form.json", df, "body")
pa.table(af)
PY
# /arrow/cpp/src/arrow/c/helpers.h:64:: ArrowSchemaRelease did not
# cleanup release callback — exit code 134 (SIGABRT), core dumped
```

pyarrow consuming the `ArrowFrame` through the C stream aborts the process: the exported schema capsule's release callback does not satisfy the C++ Arrow contract. Not reachable through the public pandas path (the refusal fires first) and not fixed here — recorded for a follow-up, because the likely blast radius is any C++ Arrow consumer of the stream (pyarrow; engines binding it). Polars consumes the same frame fine, which is the slide path.

Tests: the two check-5 tests now pin the refusal and its message, and `test_check_5_both_remedies_run` runs both named lines. Full run: `ENGINE_NULL=1 .venv/bin/python -m pytest tests/ -q` → 38 passed; the pandas checks file alone → 10 passed.

## 2026-09-21 — recognize and relate

The two new functions, through the python surface, against the stand-in
and the recordings only: no paid call, no key, no wire.

What landed:

- `tt.recognize(text, kinds=..., relations=..., threshold=...,
  relation_threshold=..., deadline=...)` returns records with `.entities`
  and `.relations`; a relation value is a `(from, to)` pair whose ends are
  kinds or the one-character string `"*"`; `kinds` is a list or a path to
  a question file whose `recognize` section carries the spec. Offsets
  count Python code points, so `text[start:end]` is the name.
- `tt.recognize(frame, on="column")` returns the ruled long frame: columns
  `row` (the source row's number, counted from 1), `text`, `kind`, `start`,
  `end`, `strength`; one row per name; relation rules refuse with a
  sentence naming the text form.
- `tt.relate(records, relations=..., either=..., threshold=...)` returns
  edge records with `name`, `source`, `target`, `probability` (and the
  kind fields when a rule names kinds); more than 255 records refuses
  with a usage error before anything else; `tt.relate(frame, on=...)`
  returns the edges as a four-column frame.
- `tests/conformance.py` runs the 45 recognize and relate cases now:
  `65 passed, 0 failed, 6 skipped` (five pre-existing skips and the
  per-subject finding below).
- `tests/test_recognize_relate.py`: 14 tests, the deck's calls as written.
- `tests/bench_recognize_scale.py`: the frame door against one-call-per-row
  at a thousand rows.

The scale proofs, as printed:

```
frame:   1000 rows, one call,  wall 0.003 s, 3000 name rows
per-row: 1000 calls, one each, wall 0.002 s, 3000 name rows
identical answers: True
relate list door:  [('caused_by', 1, 4, 0.94), ('caused_by', 2, 4, 0.94)]
relate frame door: [('caused_by', 1, 4, 0.94), ('caused_by', 2, 4, 0.94)]
relate doors agree: True
```

The two forms of each function answer identically; the frame path adds no
Python-side row work, and the wall times sit beside each other. The 300 ms
width bench does not apply to these two functions: they read the
recordings and never touch the wire.

A fixed defect found on the way, in this surface's own Arrow door:
the new-string-column offsets were one byte off (a leading zero byte with
`len - 1` offsets), so every real string value built into a frame came
back shifted — `annotate`'s choose/tag columns were never exercised with
a non-null value offline, and the wire stub cannot answer those verbs, so
nothing had caught it. Proven with a tag question through the frame door
(`'\\x00['` before, `'[]'` after), fixed to the standard cumulative
offsets, and pinned by
`test_a_real_string_result_column_rides_the_frame_door_whole`.

Findings reported, not bent around:

1. The deck's Python recognize sample asks the rule `located_in` on the
   Maria Chen sentence; the recording covers `works_for` and `based_in`,
   and the stand-in never invents an answer. The refusal names the
   covered rules, and the gap is pinned in its own test for the deck's
   owner.
2. Case `71-relate-R03-persubject-10` shares its identical ten records
   with `72-relate-R04-pairs-10`; the stand-in serves the ruled pairs
   form, so the per-subject expectation cannot be reached by a replay
   keyed on input. Recorded as a conformance-data finding for the build
   team, same as the TypeScript lane.
3. The contract's `Relation` doc says the question file spells
   `source`/`target`, but the landed parser reads `from`/`to`, and the
   conformance cases use them. The file door here converts nothing on its
   own; the test uses the parser's spelling and names the gap.

Vocabulary sweep against the product vocabulary page
"The words for numbers" (restricted: the vendor's summary word, accuracy,
calibrated; banned: certainty, likelihood, score-as-a-probability, cutoff,
gray zone), over this folder's code, tests, notes, and scripts:

```
$ grep -rniE "confidence|accuracy|calibrated|certainty|likelihood|cutoff|gray zone" libraries/python
--include="*.rs" --include="*.py" --include="*.md" --include="*.sh" --include="*.toml" \
| grep -v /target/ | grep -v /.venv/ | grep -v __pycache__
(no matches)
```

The name number is bound as `strength` (Ian's settled name, carried into
the contract by `78204cb`); the relation and edge number is `probability`.

Unchecked: nothing in these two functions against the wire — there is no
wire path for them in the stand-in; the real engine's pair-questioning
and text cutting are the build team's.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 4: `reset_usage` is removed. The grep proof:

```
$ grep -rn reset_usage src tests thinkthen
(no matches)
```

`functions.toml` lost the row and `src/generated.rs` was regenerated (14 functions). The counter tests take differences instead of resetting: `test_usage_counts_sends` and the conformance runner's usage case.

Ruling 1 aftermath: the question-file fixture in `tests/test_recognize_relate.py` and the two runner sites in `tests/conformance.py` were re-keyed to `source`/`target`; the wrapper docstring says `(source, target)`.

Ruling 2: `the_defect_kind_maps_to_defect_error` — a `#[cfg(test)]` unit test in `src/lib.rs` — constructs a contract `Error` with kind `defect` and asserts `python_error` raises `DefectError` with `kind="defect"` and `retryable=False`. It runs in `check.sh`:

```
$ cargo test --no-default-features --lib
test result: ok. 1 passed
```

The crate's `extension-module` feature moved to the default feature list so the test binary links libpython; `check.sh` computes `LIBDIR`/`BASEP` from the venv's own interpreter. The dead `options` binding the review named is not present in this tree, and a fresh test build prints zero warnings.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this surface and proven offline. Commands and output as they
happened.

**`Details.requests` and `failed_questions` (0053, 0054).** The `details`
dictionary now carries the ordered recording digests and the failure
count beside the trail it already had:

```
$ ENGINE_NULL=1 .venv/bin/python -c "import thinkthen as tt; print(tt.details(tt.question(decide='Is this a complaint?', threshold=0.5), 'i want a refund'))"
{'probability': 0.97, 'answer': True, 'model': 'jev-latest', 'digest': '0b3e8345...', 'sends': 1, 'requests': ['d3476c41e74ad2e0321e6510e6aa5c9b28e9d67f8dc35790683a9bdf5de71bc6'], 'failed_questions': 0}
```

**The failed marker, this host's spelling: a dict.** `annotate` rows carry
the ruled `{"failed":{"kind":...,"cause":...}}` mapping for a field whose
logical question failed while its neighbours answered; the words come from
the contract's own serialization, so the two spellings cannot drift:

```
$ ENGINE_NULL=1 .venv/bin/python -c "import thinkthen as tt; print(tt.annotate('tests/fixture/form.json', ['order 4471: charged twice, please refund']))"
[{'team': None, 'urgency': 1.7, 'wants_refund': {'failed': {'kind': 'backend', 'cause': 'missing_answer'}}}]
```

The Polars door widens a question's whole column to text when any member
failed, because a marker is neither a bool nor a number: the good answers
ride as `true`/`false` text and the failed cell carries the marker JSON.
No failed cell can read as `false` or a bare `null`.

```
$ ENGINE_NULL=1 .venv/bin/python -c "...pl.DataFrame({'body': [row, 'I demand a refund today']})..."
Schema({'body': String, 'team': String, 'urgency': Float64, 'wants_refund': String})
```

**The record row (go-ahead item 4).** The conformance slice builds the
ruled `{"input","value"}` dicts from the surface's own outputs and checks
them where the cases carry rows (`05`, `06`, `19`), the same shape the
database slices assert with their two columns; no Python function was
added.

**Conformance, offline: 73 and 74 run green.**

```
67 passed, 0 failed, 6 skipped
known divergence: 17-usage-and-cache waits on the disk cache
```

The skip list is the pre-existing six; case 73 checks the audit's identity
fields (the null backend's own rule cannot reproduce its recorded
probability) and case 74 checks the marker and the count.

**The fast-backend cancel, still green.**

```
$ ENGINE_NULL=1 .venv/bin/python tests/test_cancel_fast.py
elapsed 0.819
OK  the interrupt landed at 0.819 s, within a tick of the signal
```

**The examples file.** `examples.json` holds all ten functions keyed by
name, each a runnable call and the answer the null backend gives;
`tests/examples.py` runs each in a fresh process (a private directory for
the question files) and `check.sh` calls it:

```
10 of 10 examples ok
```

**The StringView path, re-proven.** Polars 1.44 crosses the `vu` layout
(checked with the ctypes schema walk), the door's `Text::View` reader
handles inline and referenced strings, and buffer-address equality through
the view path still holds:

```
$ ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_polars_door.py tests/test_pandas_checks.py -q
20 passed in 2.64s
```

The pandas checks run through the same door and pass unchanged; the full
suite (`tests/ -q`) is 56 passed. The slide sample still reports the
pre-existing band finding; `check.sh` exits 0.

## 2026-09-22 — the settle wave on the Python surface

The contract lane's settlements (`1fe8173`) wired into this surface, per Ian's "all details settled" of 2026-09-21. One unit, committed whole.

**The column forms (item 1).** `decide` and `score` take a Polars column and return a column, closing the deck's two as-drawn lines at `surfaces.md`: the decide line runs the whole column through the batch spine (32 wide, one crossing) and returns a boolean column whose nulls are "not sure"; the score line takes `levels` beside the text (the wrapper's public signature is `tt.score(question, text, levels=None, *, deadline=None)` — the deck passes `levels` positionally) and returns a number column. The way out is one Arrow array capsule pair (`__arrow_c_array__`, format `b` or `g`) from the new `ArrowSeries` in `arrow.rs`; the wrapper rebuilds the host's own column with `type(text)(answer)` and never imports Polars. The stand-in carries no bulk score in the contract, so the score column runs one call a record through the shim (recorded, not hidden); the decide column takes the engine's batch spine. Tests: `test_the_decode_columns_run_as_the_deck_draws`, `test_a_column_answer_carries_nulls_for_not_sure`, `test_the_column_form_matches_the_list_form`.

**The pair shapes (settlement 2).** `rank` returns `{"index", "record", "probability"}` records and `find` returns `{"index", "unit", "probability"}` or `None` (the wrapper fills the record/unit; the Rust side returns the pair). `top` moved to the wrapper. The runner's rank and find arms now run cases 21 and 24 (case 25 is the recorded divergence: the stand-in's find always returns a best — the real engine's relative form owns the none arm, the same divergence TypeScript records). `examples.json`'s rank expectation carries the pair shape.

**The settled rules.** `details` carries `nearest` (contract settlement 1). A built question plus `options`/`labels`/`levels` refuses as ambiguous naming the key (`settle_verb_question`). A zero or negative deadline is spent, never refused (`test_a_past_deadline_is_spent_not_refused`). The keyword relation path now runs the core's one `check_kinds` rule (`build_recognize`), so an end outside the asked kinds refuses before the engine sees it. The `relate` and `recognize` docstrings say `source`/`target`, not the refused `from`/`to`.

**Green by command.** `./check.sh` exit 0: the shim unit test, 24 surface tests, 15 Polars-door tests (three new), 10 pandas checks, 15 recognize/relate tests, the fast-cancel proof, the scale bench, 10 of 10 examples, and 69 passed / 0 failed / 4 skipped on the conformance slice (17 known divergence; 18 runs as the cancel test; 25 and 71 recorded divergences; 26 the file door). The slide sample's trailing band mismatch is the sample's own recorded finding.

## 2026-09-21 — punch-list item 2: the loops to hand the engine

Two per-record scheduling loops live in this surface's native layer, each
conversion-ready for the engine's bulk entry point, none converted here to
avoid inventing a second bulk implementation (the architect's boundary;
the full inventory with the R loops is `sdlc/records/2026-09-21-punch-list-report.md`):

- `recognize_stream` — `src/lib.rs` (the `for (place, text) in
  references` loop calling `engine().recognize_opts` per record).
  Conversion-ready -> the engine's future `recognize_many(ask, texts)`.
- `score` over an Arrow column — `src/lib.rs` (the `for text in &texts`
  loop calling `engine().score_opts` per record; it also runs without the
  poll, one more reason the bulk entry is where it belongs).
  Conversion-ready -> the engine's future `score_many`.

## 2026-09-22 — the review wave on the Python surface

The branch review (`sdlc/issues/2026-09-22-surfaces-branch-review-the-full-findings.md`)
found six things here: the multi-piece annotate (finding 3), the damaged
caller columns (finding 4), the Arrow release pointers (group 2), the
pandas Series path (finding 6), the score column's per-row deadline
(group 4), and the contract door to adopt (connector, checked deadline,
panic guard). Every fix carries a test that fails on the code before it;
the before/after evidence is from a scratch build of `HEAD`'s `src/` and
`thinkthen/__init__.py`, run in this crate's venv, no key and no network.

**The multi-piece annotate.** One root template was read out for every
batch, so the second piece described the first piece's children — and the
first piece's box is freed as soon as the consumer releases it. Each
`BatchKeep` now carries its own root and `frame_get_next` reads that one.

```
$ (before) pl.DataFrame(stream) over a 2 + 3 piece table
rows: (4, 4) ['please refund order 1', 'short note', 'please refund order 1', 'short note']
$ (after) the same stream, both consumers
pyarrow: rows 5, batches 2; polars: (5, 4), five distinct rows
```

**The caller's other columns.** The aliased child dropped its dictionary
pointer and the output schema carried no nested children, so a categorical
came back as codes, a struct came back empty, and a list panicked Polars
(`assertion failed: index < self.n_children as usize`). The alias now
carries `dictionary` and `children` whole, and the schema is a deep copy
of each original column's own tree (`SchemaTree::copy`), including
dictionaries and nested children. `test_annotate_keeps_categorical_struct_and_list_columns`
pins categorical, struct, list, and a null column beside the answers.

**The release pointers.** No release callback cleared its pointer, which
the C data interface requires; pyarrow's helpers abort the process on it,
and the stream's own release double-freed on the capsule destructor path.

```
$ (before) pa.RecordBatchReader.from_stream(frame)
/arrow/cpp/src/arrow/c/helpers.h:64:: ArrowSchemaRelease did not
cleanup release callback — SIGABRT, exit 134
$ (after) the same call, and pa.array(wrapper) for the array capsules
rows: 5, batches: 2; [True, False]
```

**The pandas Series (finding 6).** `decide`/`score` handed the Rust
wrapper to `type(text)(answer)`, and pandas' Series constructor has no
Arrow capsule door: the whole batch ran, then came back as one object row.

```
$ (before) tt.decide(ask, pd.Series([...]))
type: Series len: 1 value: [<builtins.ArrowSeries object at 0x...>]
$ (after)
type: list len: 3 [True, False, True]
```

`ArrowSeries::to_list` reads the answers without taking the capsule, and
the wrapper rebuilds the host's own column only for the Polars family;
every other host gets the plain list the list door returns, so the settled
`pd.Series(answers)` line stays the way back. `test_the_review_series_finding_answers_every_row`
and `..._holds_for_score` compare against the list door.

**One deadline for the score column.** The column loop rebuilt the
options row by row, so every row got a fresh budget and the caller's
deadline could never run out. The options are built once before the loop.
`tests/test_deadline_column.py` serves the score shape from its own
loopback server, one request a row, each sleeping 0.15 s, under a 0.25 s
budget:

```
$ (before) .venv/bin/python -m pytest tests/test_deadline_column.py -q
Failed: DID NOT RAISE DeadlineError — 1 failed in 1.16s
$ (after)
1 passed in 0.75s (the column stopped inside the budget)
```

**The contract door.** `engine()` builds through the contract's
`Connector` (`StandinConnector` + `EngineConfig::from_env`), the one line
the merge changes; `call_options` is the contract's checked
`with_deadline_seconds`, so NaN, an infinity, or a negative is a usage
error instead of `Duration::from_secs_f64`'s panic; every engine step runs
under `guarded`, which is the contract's shared `catch_panic` — the local
copy is gone, and a panic comes back as the defect kind (unit test
`a_panicking_step_comes_back_as_the_defect_kind`).

**The deadline rule, second review.** The C door's `-1` sentinel was
copied into Python, so a budget computed as `end - now` that landed on
`-1` silently disabled the deadline. Only the explicit `None` means no
deadline now; every negative — `-1` included — is refused as a usage
error, and zero stays the contract's spent deadline that sends nothing.
`test_the_c_doors_sentinel_is_not_a_deadline`,
`test_a_computed_budget_that_landed_below_zero_is_refused`,
`test_zero_stays_a_spent_deadline`, and `test_no_deadline_is_the_explicit_none`
pin the four arms; the unit test `the_deadline_conversion_refuses_what_cannot_be_a_budget`
carries `-1.0` in its refused list. Against the pre-fix wheel the first
of those fails and the last two pass, so the tests discriminate.

**The cancel token.** `tt.CancelToken()` is a handle any thread can set;
every verb takes `token=`. A bulk call is armed with the shim's own token
and the poll bridges the caller's token to it at the engine's ticks, so a
shared token's `cancel` stops exactly the calls it was handed and an
interrupt never fires the caller's token; the score column reads the
token between rows, so a controller thread stops it before the next row
is sent. `test_review2_wire.py` proves both against its own delayed
loopback server (4 requests served of 30 before the cancel; 3 of 12 rows
for the column).

**Error classes, second review.** `Cancelled` was a `KeyboardInterrupt`
only; it is now a subclass of both, built at module init with two bases
(pyo3's `create_exception!` gives one), so `except ThinkThenError` and
`except KeyboardInterrupt` both catch a cancel. A signal handler that
raises anything else — a `SystemExit`, its own error — has that error
carried out of the poll and raised when the call stops, never turned into
`Cancelled`; `test_review2_signals.py` proves both in child processes
(handler `SystemExit(3)` raised as itself at 1.0 s into a 5.6 s batch;
the cancel caught as `ThinkThenError` at 1.0 s).

**Refuse before any request, second review.** A pandas frame or pyarrow
table handed to `annotate(..., on=)` ran the whole paid batch and only
then hit the constructor that cannot rebuild the frame. The wrapper now
probes the host's own constructor with an empty frame of the same shape
(`_probe_frame_rebuild`) before the input's stream is taken; the refusal
is a usage error naming the two remedies, with the constructor's own
error as its cause. The offline witness reads the engine's request
counter (`test_review2_findings.py`: 0 requests where the pre-fix wheel
spent 9), and the wire witness counts POSTs through a loopback server
(`test_review2_wire.py`: 0, with a positive control proving the counter
moves).

**Refused inputs are released.** A batch read before a refusal was never
released: `series_column` and `frame_column` now own each batch before
reading it (so `Drop` releases it on the error path), the producer's
schema rides a `SchemaGuard` that releases on every error path, and
`borrow_strings` takes the caller's `(skip, count)` explicitly.
`tests/sliced_struct_stream.py` records both release pointers; the two
release tests fail against the pre-fix wheel. The RSS measurement (a
0.6 MB input, 100 refusals per path) reads 0.0–0.2 kB a call on every
refusal path after the fix.

**The sliced struct stream.** When a struct array is sliced the Arrow
format lets the root carry the offset and length with whole children;
pyarrow and polars propagate slices into the children instead, so this
layout is built by hand in `tests/sliced_struct_stream.py`. The shim read
the children from row zero and answered about the wrong rows (the
caller's rows 2–5 were answered with rows 0–2); `frame_column` now reads
`(root offset + child offset, root length)` and `build_frame` normalizes
the aliased originals to the same cut, so the output carries no root
offset and every child names exactly the caller's rows.
`test_a_sliced_struct_stream_answers_the_callers_rows` fails against the
pre-fix wheel.

**Cross-side note for the merge.** The stand-in's partial-failure fixture
(finding 7) fires only under `ENGINE_SYNTHETIC_PARTIAL=1`. `check.sh`
arms it for the three runs that pin the marker (`test_surface.py`,
`test_polars_door.py`, the conformance slice), and
`test_the_fixture_failure_is_off_without_its_opt_in` proves in a fresh
process, with the opt-in unset, that the fixture text answers like any
other input.

**Discriminating evidence.** The pre-fix wheel was built from `HEAD` in a
scratch worktree (`maturin build` + `uv pip install --target`) and the new
test files were run against it from outside the package: 13 of the new
tests fail there (the sentinel deadline, the two `Cancelled` base tests,
the sliced stream, both release tests, both refusal counters — offline
and wire — and the two signal children), and all pass against the fixed
wheel. The two token tests fail there too, on the missing `token=`
argument.

**Stale expectations the fixture build exposed.** Two Python tests and
the annotate example still assumed the old sorted name order (the core's
parser keeps file order since `49c3b78`): the widened column is `urgency`
(the last file-order question), the frame's new columns follow the file,
and the example's expected dict is in file order. The Python gate had not
been run with a fixture-armed build since that change; the fixes are
test-and-example only, and `test_a_failed_question_widens_its_column_to_text`
now derives the failed question from the list door's own marker instead
of naming it.

**Green by command.** `./check.sh` exit 0 with the stub up on 8211
(`STUB_DELAY_MS=300`): 3 shim unit tests, 27 surface tests, 18
Polars-door tests, 12 pandas checks, the deadline-column test, 12
second-review findings, 2 signal children, 6 wire findings, 15
recognize/relate tests, 4 ownership tests, the fast-cancel proof (the
interrupt landed at 0.819 s), the scale bench, 10 of 10 examples, 78
passed / 0 failed / 6 skipped on the conformance slice, the slide sample
with its recorded band mismatch, and the wire cancel (32 requests served
at return, none after).

**Memory checks.** The debug build (`maturin develop`, no `--release`)
runs the suites green, and a 200-iteration stress of the multi-piece,
rich-column, capsule, and pandas paths under
`MALLOC_CHECK_=3 MALLOC_PERTURB_=165` is clean. Miri cannot run a pyo3
extension, valgrind is not installed, and an ASan build was attempted with
the nightly toolchain and refused at the dependency build
(`error[E0463]: can't find crate for thiserror_impl` under
`-Zsanitizer=address`); recorded rather than worked around. One build
trap for the next lane: a scratch copy of this crate that shares its
`target/` dir can leave a stale artifact under the same package name;
`cargo clean --release -p thinkthen-python` clears it.

## 2026-09-23: the third review's Python findings (refs surfaces-review-3)

**Item 4, malformed Arrow views and offsets (HIGH).** The guards: one cap,
`MAX_ONE_STRING` (64 MiB — far past any text the engine classifies), applied to
a view's size, its data-buffer offset, their sum, and a regular string array's
offset span; reversed or negative offsets refuse by name; the dead `index < 0`
sign check became the correct `u32` compare. The probes are four FFI-level
tests in `src/arrow.rs::malformed_tests`, hand-built C structs over real small
buffers — the reviewer's exact shapes.

- Probe `reversed_offsets_are_refused_not_wrapped`, pre-fix:
  `thread panicked ... null pointer dereference occurred ... SIGABRT` (debug
  catches the underflow; release wraps into the reviewer's exit 139).
- Post-fix: `test result: ok. 4 passed` (`cargo test --no-default-features
  --lib malformed`).
- Real columns unchanged: `pytest tests/test_polars_door.py
  tests/test_pandas_checks.py tests/test_surface.py` → `57 passed` with the
  synthetic build.

**Item 14, a pre-fired token spent a full batch.** The bulk door watched the
caller's token only in its poll, so the first batch flew; the single verbs
handed the token to the engine, which refuses at entry. Both doors now read
the token before any request is built (`bulk` pre-check; `call_options`
pre-check). Probe `tests/test_review3_wire.py`, the server counts:

- Pre-fix (lib.rs stashed): `AssertionError: a cancelled token must not spend:
  1 requests` — `2 failed` (bulk and column forms).
- Post-fix: `4 passed`, including the guard-against-overfire test: a live
  token still sends and a controller's cancel lands between rows
  (`ENGINE_WIDTH=1`, spent < all).

**Item 21, one deadline spelling.** The surface's own blanket negative
refusal is gone; the contract's `deadline_from_seconds` owns the conversion:
-1 crosses as the sentinel, every other negative refuses, zero stays spent,
NaN and the oversized refuse. `pytest tests/test_review3_offline.py` → 9
passed; the two review-era tests that asserted the superseded "-1 refused"
rule were rewritten to the decided one (`test_surface.py`,
`test_review2_findings.py`, and the crate's own
`the_deadline_conversion_refuses_what_cannot_be_a_budget`).

**Item 22, the builder's silent drops.** `question` now refuses by name:
options beside decide, levels beside choose, labels beside score, true/false
beside choose/score/tag, and anything beside `file`. Fail side: the new test
raised `DID NOT RAISE` on `question(decide=..., options=[...])`; fix side: 9
passed.

**Item 22, the pandas advice.** Every pandas line the shipped docstrings
advise now runs as a test (`test_the_decide_docstring_advice_round_trips`,
`test_the_annotate_docstring_advice_round_trips`). The reviewer's all-NaN
column reproduced and is diagnosed, not papered over: the null backend
answers the `form.json` tag question with None, and pandas spells a null
answer NaN — the round trip is honest, and the test pins `team.isna().all()`
so a real drop cannot hide behind it. A separate standin defect was found on
the way and is out of this folder's scope: under `ENGINE_NULL=1`,
`tt.score` with two levels raises `BackendError` because the null backend's
Score arm always invents three odds regardless of the question's level count
(`standin/src/lib.rs` null_reply, Score arm) — recorded here for the standin
owner.

`./check.sh` at the committed tree: exit 0, `78 passed, 0 failed, 6 skipped`
(offline stages), the crate's 7 Rust tests green.

## The fourth review's python items, closed with probes (2026-09-23)

Item 3, the Arrow crash family: the per-string cap came down from 64 MiB to
4 MiB, because the C data interface carries no buffer lengths and the cap
is the only bound the door can enforce; the reviewer's own probe — a view
at offset 60,000,000 over a 100-byte buffer — names sixty million, past
the cap, and refuses before any read. Null tables (buffers, offsets,
children in a frame root) refuse; a 2^40 length refuses instead of
aborting `Vec::with_capacity`; negative lengths refuse; the data-buffer
count arithmetic is pinned at its boundary (index == count refused), and
the conformant two-data-buffer round trip — the layout real producers
export, with an inline view — passes where the old unit tests built
layouts producers do not make.

Fail-then-pass, grafted onto the old code (the test module lifted onto the
pre-fix validation):

- `the_reviewers_sixty_million_offset_is_refused` — old: SIGSEGV; new: ok.
- `a_two_to_the_forty_length_is_refused_not_allocated` — old: SIGABRT; new: ok.
- `a_null_buffer_table_is_refused_not_dereferenced` — old: SIGSEGV; new: ok.
- `a_null_offsets_pointer_is_refused` — old: panic/abort; new: ok.
- `view_index_equal_to_the_data_count_is_refused` — refused on both: the
  boundary pin, answering the review's off-by-one claim with a test.

Item 14, signals: `step` now runs every call on a worker holding the
call's own cancel token while the calling thread polls
`py.check_signals`, so a signal lands on the token before the next
request and the raise returns without waiting for the in-flight send.
Fail-then-pass through `tests/test_review4_signals.py` against the
counting stub: old code — both signal tests FAIL (the raise held behind
the send, and the score column ran on); new code — 3 passed. The stub
counts the requests: one signal, one served request, and a column stops
before its later rows.

Item 15: the builder refuses two verbs ("question() takes one verb") —
fail-then-pass: old extension FAILED the new test (the old
'options-does-not-belong' message), new passes. The pandas advice is
`pd.Series(answers, index=text.index)` on every docstring that names it,
because the zero-indexed form turns every aligned join into NaN on a
non-default index — pinned by a test that builds the NaN shape first.

Leftovers: the review-3 suites and the review-4 signal suite run from
`check.sh` in their own stages; `thinkthen/py.typed` and
`thinkthen/__init__.pyi` carry the typed face (every name in the stub
exists at runtime, verified); the wheel is tagged `manylinux_2_34` and
matches the glibc floor the extension needs (verified by building it);
cross-kind calls refuse as usage naming the question's own kind, not as
backend answer-shape errors (old: `kind: backend`; new: UsageError), and
the one test that expected the ambiguity wording for a cross-kind call
was corrected to expect the kind wording.

One operational note: a `git stash` in this shared worktree popped the
build team's preserved ticket-0074 stash; the tree was restored by
command (conflicts resolved to the pre-pop side, the stash's untracked
files removed, the stash entry itself left intact in the list for its
owner). File copies under /tmp are the only safe swap here.

## Surfaces review 5, item 3: the Arrow door reads inside declared extents

The C data interface carries no byte length beside a buffer pointer; it
declares extents instead. `borrow_strings` now reads only inside them. A
string view's data count is the table minus three (validity, views, and
the trailing buffer-sizes buffer), so no view can name the sizes buffer,
and every long view is checked against the length the sizes buffer
declares for its data buffer. A Utf8 or LargeUtf8 array's offsets are
checked nonnegative and monotone from the first row read to the array's
last offset, which declares the data extent, before any data byte is
read. A row claim past four billion (length or offset) is refused before
it is allocated or walked. A single row longer than 4 MiB is refused; the
old cap compared absolute buffer positions and refused every ordinary
`pa.string` column whose text passed 4 MiB (200,000 rows, 10 MB: old
UsageError, new 200,000 answers).

Fail-then-pass: the module's `malformed_tests` against the old reader
kill the test binary with SIGSEGV; against the new one, 9 passed. The
reviewer's guard-page probes (buffers ending at an unreadable page): old,
nine malformed shapes dump core; new, the view straddling or passing its
declared end, a view index naming the sizes buffer, a Utf8 row past the
last offset, and a 2^40 offset each refuse as usage.

Still open, and not provable by any reader: a producer whose declarations
agree with each other but whose allocation is shorter — offsets `0, 200`
over a 3-byte buffer, or `60000000, 60000001` over one. The last offset
is the only data length the interface gives, so such a column cannot be
told from a real one of that size. That lie stays the producer's; the
per-row ceiling keeps any one such read under 4 MiB.

## Surfaces review 6, verifier P1: the extent cap bounds where a read starts

The verifier found the review-5 cap incomplete. The 4 MiB per-row ceiling
bounds how far a single row reaches past its start, not where the start
sits. An offset (Utf8) or view data-offset that is itself enormous starts
the read far past the buffer, and the small row length then clears the
per-row check. Four shapes that review 5 refused now crashed: a
string-view data-offset near 2^32, LargeUtf8 offsets near i64 max, Utf8
offsets near i32 max, and offsets `2^40 .. 2^40 + 13` over a three-byte
buffer.

The fix caps the declared data extent absolutely, not just the per-row
reach. `MAX_DATA` is 1 GiB: a view's declared data-buffer size, and a
Utf8/LargeUtf8 final offset (the data length the interface gives), each
must fall in `0 ..= MAX_DATA`, checked before any data byte is read. Every
offset and view is then bounded by that extent, so the start is bounded
too. `MAX_BUFFERS` (64) bounds the buffer-table length before the reader
reads the last (sizes) entry, so a view array claiming 2^40 buffers, or a
sizes buffer shorter than the count, is refused instead of walked. (Review 7
corrected this sentence: the bound covered the 2^40 table only. A sizes
buffer shorter than its count was still read past until the review 7
readability probe below.)

The trade-off. 1 GiB sits below 2 GiB, so an i32-max offset is refused; it
is 256x the 4 MiB per-row cap, so a legitimate producer that keeps any one
row under the per-row ceiling and any one data buffer under 1 GiB passes.
Real string columns chunk, so a large column is many buffers, none near
the cap. Confirmed on the new reader through the wire stub: a 6 MB
`pa.string` column of 6000 rows (`pa_big_5mib_total`) sends all 6000, five
1 MiB rows (`pa_5x1mib`) send all five, and utf8/large/view slices with a
nonzero first offset borrow their bytes. A single row past 4 MiB
(`pa_one_row_5mib`) is still refused by the per-row cap — the documented
cost, and the reason the cap is per-buffer, not per-column.

Fail-then-pass, end to end: the four crash shapes run against a wheel
built from the committed reader dump core (SIGSEGV, exit 139); against the
new reader each refuses as usage with a sentence naming the extent. Two of
the review-5 unit tests stayed green when their fix was broken because they
read garbage that happened to be refused for another reason; each now pins
the exact refusal sentence, and mutating the view-index check (`>=` to `>`)
or removing the offsets-order check turns its test red for the changed
sentence.

## Surfaces review 7: the reader probes each extent before it reads (refs R3-4, R7-2, R5-6, R7-8)

R3-4 and R7-2 found a regression. Utf8 offsets `60000000, 60000001` over a three-byte buffer crashed the host (exit 139) at the wave 7 tip. The wave 5 build refused the shape because it capped every absolute offset at 4 MiB. Review 5 dropped that cap because it refused every ordinary column past 4 MiB. Review 6 then capped the declared extent at 1 GiB, and 60,000,001 sits under it. No size cap can refuse this shape and keep a 60 MB column. The two carry the same declarations.

Decision: the reader asks the kernel before it reads. `readable` in `src/arrow.rs` checks each extent the reader relies on: the offsets or views the rows read, the whole sizes buffer the table's count declares, and each data buffer's declared extent (a Utf8 array's last offset, a view buffer's declared size). (The second pass narrowed the data check to the bytes the selected rows read.) On Linux it copies one byte a page through `process_vm_readv` against this process. The kernel answers an unreadable page (unmapped, guard, or reservation) with an error instead of a signal. Elsewhere, or where that call is refused, `mincore` finds unmapped pages. (The second pass below replaced both on Linux with a `/proc/self/maps` snapshot, because `mincore` misses protected pages on Linux too.) An extent that fails is refused as usage: "the column's buffers declare bytes this process cannot read". The per-row 4 MiB cap and the 1 GiB extent cap stay. The per-row cap now runs before the probe, so a giant row keeps its own sentence.

Observed (t7b probes against this build): R3-4.sh 0 crashes (was 1). R5-6.sh 0 crashes and 0 over-reads served (was 3 crashes: `u_end_past_guard`, `U_end_past_guard`, `sizes_short_idx1`). `sizes_short_idx0` is now refused too, because its table declares two data buffers and its sizes buffer holds one. A 60,000-row, 40 MB column borrows whole as `pa.string`, `pa.large_string`, and a Polars view column, and a pyarrow slice of its tail borrows. (The second pass corrected this sentence. The probe made one system call per 1,024 pages, but it covered byte 0 to the last offset, so a 10-row tail slice of a 1 GiB column cost 158 ms and faulted in a whole mapped file.) The unit tests `an_extent_past_readable_memory_is_refused` and `a_sizes_buffer_shorter_than_its_count_is_refused` build each shape against a 64 MiB unreadable reservation. With `readable` forced to answer yes, both test binaries die of SIGSEGV.

R5-6 waiver, recorded. Two shapes stay open and are waived. First, an extent that runs into another readable allocation reads that allocation's bytes. No reader can see an allocation's end, so this remains the producer's lie. Second, off Linux, `mincore` sees unmapped pages only, so a guard page (mapped, unreadable) still faults there. (The second pass corrected this: a Linux host whose sandbox refused `process_vm_readv` fell back to `mincore` and faulted too. Linux no longer uses `mincore`, so macOS is now the only such platform, and check.sh already names it experimental.) Ian can overturn this waiver. The lever is a stronger probe on macOS (`mach_vm_read_overwrite`), or refusing input off Linux.

Ian can overturn the probe decision too. The lever is to restore the absolute 4 MiB cap, which refuses this shape and every column past 4 MiB.

## Surfaces review 7: each output child owns its buffers, and metadata rides through (refs R4-15)

R4-15 found three faults in the frame that `annotate(on=)` returns.

A moved child read freed memory. The C data interface lets a consumer move a child array out of a batch and release the parent, and the moved child must stay valid until its own release. Every child carried a release with no owner, and the root's release freed the whole batch. Decision: each emitted batch sits behind a reference count (`BatchShare`). The root and every child carry one share each. The root's release releases each child the consumer did not move out, then drops its own share. A moved child keeps its share, so the batch's buffers and the caller's aliased frame live until the moved child's release. An aliased original column still leaves its producer's own children to the producer, released when the last share drops.

The output schema dropped every metadata pointer. A Polars `Enum` came back `Categorical`, and a field's own keys were lost. Decision: the schema copy keeps each field's metadata byte for byte, and the frame's root keeps the caller's schema metadata. A blob that names a negative length is dropped instead of walked. (The second pass found this walk trusted every other length and crashed on crafted blobs. It now refuses them; see below.)

The answer columns' offsets were `values.len() as i32`, which wraps negative past 2 GiB of text. Decision: a text answer column past 2 GiB is refused as usage. The frame builder and the table builder share one offsets writer. Ian can overturn this. The lever is to widen such a column to the `U` layout (i64 offsets) instead of refusing it.

Observed: t7b `R4-15a.py` under `MALLOC_PERTURB_=165` read the moved child's offsets pointer as `0xa5a5a5a5a5a5a5a5` and dumped core at the wave 7 tip, and passes on this build. `R4-15c.py` failed twice (Enum, field metadata) and passes. `tests/test_review7_arrow.py` runs from check.sh. It moves every child out in a child process under `MALLOC_PERTURB_`, releases the parent, and rereads each buffer table and the integer column's values. It also pins the Enum round trip and both metadata levels. Against a wheel whose children get no share and whose metadata copy returns null, all three tests fail. The unit tests `an_answer_column_past_i32_offsets_is_refused` and `schema_metadata_is_copied_byte_for_byte` fail against the matching one-line mutants.

## Surfaces review 7, second pass: the verifier's three findings (refs R3-4, R5-6, R4-15)

### The Linux check reads the memory map (R5-6)

The verifier forced `process_vm_readv` to fail with EPERM through seccomp. The reader fell back to `mincore`, which reports a protected page as mapped, and R5-6.sh crashed three times (`u_end_past_guard`, `U_end_past_guard`, `sizes_short_idx1`).

Decision: on Linux the reader takes one snapshot of `/proc/self/maps` per column read (`Readable` in `src/arrow.rs`) and checks each extent against its readable ranges. A protected page and an unmapped page both fail. `process_vm_readv` and the Linux use of `mincore` are gone. A process that cannot read its own map gets a usage refusal naming `/proc/self/maps` for every Arrow column, so the reader never reads unchecked. Off Linux, `mincore` stays, under the waiver above.

A seccomp filter that kills a process on `process_vm_readv` (the verifier's low risk) no longer matters, because the reader never makes that call. The check opens and reads one file. A filter that kills on `openat` or `read` kills the host on any file access, well before this reader runs. A filter that answers those calls with an error gets the usage refusal.

Observed: R5-6.sh under the verifier's EPERM wrapper (`noreadv`) went from 3 crashes to 0, and R3-4.sh stays at 0. Both pass without the wrapper. `tests/test_review7_arrow.py::test_a_guard_page_is_refused_without_process_vm_readv` installs the same seccomp filter in a child process, confirms the call fails with EPERM, and hands the reader a pyarrow column that runs into a PROT_NONE page. The child died of SIGSEGV on the previous build and now gets the refusal sentence. The unit test `the_memory_map_keeps_readable_ranges_and_merges_touching_ones` pins the parser. Ian can overturn this decision. The lever is to restore a kernel-call probe with the map as its fallback.

### The check covers only the bytes the selected rows read (R3-4)

The verifier measured the first pass on a 1 GiB Utf8 column of 1,048,576 rows. A 10-row slice at the end took 157.6 ms a call against 0.31 ms before. Walking the column as 1,024 slices took 103 s against 6.2 s. A tail slice of a memory-mapped 1 GiB file faulted in the whole file (RssFile 40 to 1,065 MiB). The data check covered byte 0 to the last offset, and a view buffer's whole declared size.

Decision: the data check covers only the bytes the selected rows read: a Utf8 column's first to last offset of those rows, and each view buffer's lowest to highest byte those rows name. The offsets, views, and sizes checks were already bounded by the rows. Observed with the verifier's `cost.py` on this build: 0.47 ms for a 10-row tail slice, 3.49 s for the 1,024-slice walk, 3.02 s for the whole column in one call, and 3.8 ms for the mapped tail slice with RssFile 40 to 41 MiB. The 0.47 ms includes one read of `/proc/self/maps`. The unit test `only_the_bytes_the_selected_rows_read_are_checked` makes every byte outside the rows read unreadable, for a Utf8 column, a Utf8 slice, and a view. Checking from byte 0 again turns it red with the refusal sentence.

### The metadata walk is bounded and checked (R4-15)

The first pass's metadata copy trusted every length a producer's blob declared. The verifier's `meta.py` crashed the host (exit 139) on 5 of 5 crafted shapes: a key length past a guard page, a value length past it, a pair count of 2^31 - 1 against a guard page, a 2 GiB key length in a readable heap blob, and a bad blob on the frame's root schema.

Decision: the walk runs against the same memory-map snapshot as the column check. Each i32 length word is checked readable before it is read. A blob is capped at 16 MiB (`MAX_METADATA`), which bounds the pair count and every length. The whole blob is checked readable before it is copied. A blob that fails refuses the call as usage: "a column's Arrow metadata names lengths past its readable bytes or past 16 MiB". A negative length now refuses too, where the first pass dropped the blob. Real metadata runs to kilobytes, so the cap costs no real producer. Ian can overturn the refusal. The lever is to drop a bad blob and answer without it.

Observed: `meta.py` went from 5 crashes to 0. The two sound shapes still carry their metadata, and the five crafted ones get the refusal. R4-15a.py, R4-15c.py, and the verifier's `threads.py` (30 rounds of cross-thread releases) pass. The unit test `crafted_metadata_is_refused_before_it_is_read` places four of the shapes against guard pages or a readable heap blob. Ported onto the first pass's walk, and against a mutant that skips the readable checks, it dies of SIGSEGV.

The comment at `emit_batch` said the batch pointers were single-threaded by contract. The C data interface lets a consumer release the root and each moved child from any thread, and the shares count atomically. The comment now says that.

