# The Polars plan: one Rust door, two Python containers

Plan, 2026-09-21. The surfaces experiment on branch `surfaces` carries `contract/`, `standin/`, and the nine surfaces. This page plans the Polars work that follows. It authorizes nothing; the experiments carry the numbers and Ian rules the doors.

## The goal

Ian ruled on 2026-09-21 that Python's data frame is Polars, and he ruled then that the Rust foundation is the point: all scalability and vectorization belong in Rust code. He clarified while this page was written that the Python surface keeps both containers first-class. A plain list of strings crosses into the Rust bulk spine in one call, as it does today on `surfaces`. A Polars Series or DataFrame crosses zero-copy on Arrow buffers. No third story exists, and neither container waits on the other.

Both containers ride the same Rust engine and the same process-wide width gate. The Python surface never loops a row, never chunks, and never bridges a call through a Python lambda or `map_elements`. The plan's central measure follows: the width bench reads the same 9.65 s at 32 in flight through a Polars column as through a list, and that equality is the proof the scaling lives in Rust.

## The two shapes

**The Series door.** The Rust surface takes a `StringChunked` or a `Series` for the single-answer verbs and a DataFrame for `annotate`, all over `contract/`'s `Engine` trait and `Question` builder. The Python wheel carries the same door: a Polars column reaches Rust through the Arrow PyCapsule stream form, `__arrow_c_stream__`, with no import of Polars and no copy of the string buffer (`sdlc/planning/libraries/python.md:45`). That page records the mechanism from the record: a `String` array is offsets over one contiguous UTF-8 buffer, so Rust reads every value as `&str`; the stream form arrives one record batch at a time, and Polars exposes the stream form and not the array form. Precedent for the proof shape is 205's hand-rolled PyCapsule door, which passed buffer-address equality at +15.4 KB on the wheel (local experiment 205's `FINDINGS.md:62`). The record already chose the no-coupling road: the Arrow C interface gives Python users most of the gain with no coupling to Polars releases (`sdlc/planning/libraries/rust-crates-worth-knowing.md:29`).

**The plugin expression.** A Polars plugin registers a Rust expression that runs inside the query engine, so a verb rides in `with_columns` on a lazy frame in Python the way it does in Rust (`rust-crates-worth-knowing.md:29` records the mechanism and its cost: cheap after the Arrow door, coupled to Polars releases). Registration details, feature flags, distribution through pyo3-polars, and the streaming story are knowledge to verify, not record. The local record holds no Polars version facts. Mark every plugin claim as needing a probe, and experiment 216 supplies it.

## What the record already supports

- Arrow crossings cost the same as native vectors: DuckDB kept `vscalar` over the Arrow path after measurement (local experiment 207's `FINDINGS.md:47`).
- Zero-copy column crossings are proven across six languages, with the Python buffer-address equality as the strongest form (local experiment 205's `FINDINGS.md:58` and `:62`).
- The bulk spine holds 1,000 records at 300 ms and `jobs` 32 near 9.65 s, 9.658 s through Python, and the null backend near 1.3 µs a record (local experiment 211's `FINDINGS.md`, answers 1 and 3).
- One process-wide gate caps any host's parallelism: 100 calls at once held 32 in flight where a per-batch gate ran 85 wide on 101 connections (same file, answer 3).
- The deadline rides every call, the counters count sends, and the engine holds zero threads between calls (same file, checks 1, 3, and 4).
- The Python page already promises any iterable crosses once, with the library importing neither pandas nor Polars (`sdlc/planning/libraries/python.md:31`); the capsule path keeps that promise because it needs no import.

## The risks, each with its experiment

1. **Polars' thread pool persists in the host while the engine holds zero threads.** The embedding note must name the two pools separately. The fork story needs its own proof with a warm Polars pool, because the process-ID rebuild covers engine state only. Experiment 214.
2. **No signal reaches a plugin expression.** The deadline is the lever, and whether the plugin API offers an interrupt hook is to verify. Experiment 215.
3. **The plugin API pins to a Polars release**, against the one-version rule. The pin's cost is unmeasured locally. Experiment 216.
4. **A plugin ships as a crate users compile against their own Polars**, beside a wheel for py-polars. Weight and the support matrix are unmeasured. Experiment 216.
5. **Python interrupt behavior during `collect()`** is unverified, including whether `KeyboardInterrupt` lands while Rust waits. Experiment 215.

## The experiment list

**212. Do both containers cross once and hold the gate, with no copy on the Polars path?** Measure the width bench through a Polars column against the list form (both near 9.65 s at 32 in flight), the stub's `max_in_flight` never above `jobs`, the null-backend per-record cost through both containers against the slice baseline, and buffer-address equality for the Series door. Stub only.

**213. Does the width gate cap Polars' own parallelism?** Collect a lazy frame that runs the door over many columns; the stub's `max_in_flight` must stay at `jobs` whatever Polars' pool does. Stub only.

**214. Does a fork with a warm Polars pool answer?** Parent collects, forks, child collects. Stub only, with the engine's own pid rebuild measured beside it.

**215. What do cancel and deadline do inside an expression?** A spent deadline must surface; whether any interrupt hook exists is recorded either way. Stub only.

**216. Does the plugin door work from Python, end to end?** Build a local wheel against the pinned Polars, run a verb in `with_columns` on a lazy frame, match answers against the list form, and record the version pin's cost and the import cost. Nothing is published.

## Sequencing and what Ian rules

The Polars work starts after the surfaces experiment closes and the Python surface carries Polars, and it feeds an ADR amendment. Already ruled: both containers ship first-class, and all scaling and vectorization run in Rust. Still Ian's: whether the plugin door ships beside the Series door or only one of the two, and whether Python-Polars support rides the plugin, the wheel's optional extra, or both.

## What Ian can overturn

All of it. The cheap overturn is the plugin door. The Series door and the list crossing answer both containers without it, and the plugin adds only the lazy-frame expression. The measured costs arrive with experiments 212 through 216, and any one of them can stop the shape it tests.

## Status, 2026-09-25

The Series door now stands on the public API, not on `contract/`. Rust's door is `thinkthen-polars` at `libraries/polars` (ticket 0120), on Polars 0.55. Python's door reads the Arrow C stream capsule (ticket 0106). The two share no Polars crate, and both write frames by ADR 0047 item 10.
