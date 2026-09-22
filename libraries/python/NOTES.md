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

Vocabulary sweep against `repos/mktg/products/thinkthen/vocabulary.md`
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
