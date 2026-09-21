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
eight verbs plus `decide_many`, `question`, `details`, `usage`,
`reset_usage`; `deadline` in seconds on every entry point; the six kinds
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
