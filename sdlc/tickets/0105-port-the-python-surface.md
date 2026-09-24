---
flow: build
priority: 105
opens: libraries/python sdlc/scripts sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0105: Port the Python surface

Status: design draft; review pending. Owner: Claude.

## Outcome and authority

Port the Python surface from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-python` at `libraries/python`, in its own Cargo workspace. `import thinkthen as tt` keeps its ten verbs, `decide_many`, `details`, `question`, `usage`, the six exception classes, and `CancelToken`. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` names Python as the first surface after C.

Draft ADR 0047 fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern, and this ticket copies it. Ticket 0095 fixes the members this binding calls. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the Python port map. Ian can overturn any decision below.

## Scope: the core binding, then the data frame layer

Ian ruled on 2026-09-21 that Python's data frame is Polars at 0.1 and that pandas leaves the surface (ADR 0017, "Ruled after acceptance, 2026-09-21: the data frame is Polars"). His clarification the same day makes pure Python and Polars both first-class, with all scaling in Rust. Both ship at 0.1. The only open Polars question is a Rust Polars door after 0.1 (ADR 0047 item 8).

The port splits in two tickets for review size. This ticket ports the core binding over plain Python containers: a `str`, and a `list`, `tuple`, or other iterable of `str`. Ticket 0106 ports the Polars data frame layer on top of it: the Arrow C stream door (`src/arrow.rs`, 2,698 nonblank lines and 121 `unsafe` sites at the tag), the Polars column and frame forms, and the eleven deferred rows. 0.1 needs both.

pandas leaves under the ruling, so the core binding refuses a pandas object with `UsageError` before any request, and that refusal stays after 0106. Until 0106 lands, the core binding also refuses a Polars or `pyarrow` object and any object that exposes `__arrow_c_stream__`, `__arrow_c_array__`, or `__dataframe__`. 0106 turns those refusals into the door. A user never sees a returned value change shape.

Cost of the split: the deck's `tt.annotate("form.json", df, on="body")` line and the Polars equality proof wait for 0106. The slide sample's list forms run here.

## What moves from the tag

These files carry over with their intent, then change only as the rewrite section says.

- `thinkthen/__init__.py`: the dispatch layer and its docstrings. The Arrow and frame branches (`_column_or_value`, `_refuse_if_not_rebuildable`, `annotate_stream`, `recognize_stream`, `relate_stream`) leave. The container refusal above takes their place.
- `thinkthen/__init__.pyi` and `thinkthen/py.typed`. The stub follows the new result shapes.
- The six exception classes, each carrying `kind` and `retryable`. `Cancelled` keeps both parents, `KeyboardInterrupt` and `ThinkThenError`.
- `CancelToken` as a frozen `pyclass` over `thinkthen::CancelToken`.
- The deadline rule: `None` and `-1` are none, `0` is spent, and a bool (Python's or NumPy's) is refused with `usage` under ADR 0041's Python amendment.
- `pyproject.toml` (maturin, `abi3-py310`, module `thinkthen._thinkthen`). The `polars` optional extra leaves.
- `build-wheel.sh` and the home-path remap. `check.sh` keeps its shape: offline venv from uv's cache, `maturin develop --locked --release`, the home-path check, the shim unit tests, pytest, the conformance runner, and the examples.
- `examples.json` and `tests/examples.py`. Each expected value is re-derived against 0092's generic arm.
- Tests that call the binding keep their assertions: `test_surface.py`, `test_cancel.py`, `test_cancel_fast.py`, `test_review2_signals.py`, `test_review5_deadline.py`, `test_review5_verbs.py`, and the list-form tests inside `test_review2_findings.py`, `test_review2_wire.py`, `test_review3_offline.py`, `test_review3_wire.py`, `test_review4_signals.py`, `test_ownership.py`, and `test_recognize_relate.py`. The builder regroups them by topic into at most twelve files. Branch review numbers leave the file names.
- The follow-ups in `sdlc/records/surfaces-freeze-2026-09-24.md` for Python (`d74d95fb`, `d048261e`) both touch `arrow.rs` and move to 0106.

These stay behind at the tag: `src/arrow.rs`, `src/generated.rs` and its branch generator, `tests/sliced_struct_stream.py`, `test_polars_door.py`, `test_pandas_checks.py`, `test_review7_arrow.py`, `test_deadline_column.py`, the four width and cost benches, and the frame half of `slide_sample.py`. 0106 ports the Polars pieces of these. The pandas pieces retire under the ruling. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines: the check by command, the rulings, and a pointer to the tag. The tag has no Python `DESIGN.md`. `sdlc/planning/libraries/python.md` is the design page, and this ticket updates it where the port changes behavior.

## What is rewritten against the public API

The Rust shim is rewritten whole over `thinkthen`. It keeps the tag's rule that no rule, retry, or send lives in the binding.

- **Engine.** The stand-in connector, `EngineConfig`, and the `Arc<dyn Engine>` static leave. Every call uses `thinkthen::default_engine()`. The shim adds no engine setting. The environment variables `Engine::from_env` reads configure it.
- **Calls and options.** Each verb calls the matching `Engine` `_with` form with one `CallOptions`. `token=` maps to `CallOptions::cancel` directly. `TokenBridge` and its polling thread leave. `deadline=` goes through `CallOptions::deadline_seconds` after the binding's bool check. `Seconds` keeps only the bool and non-number refusals, and the shim's own conversion leaves.
- **Interrupts.** `step()`, its worker thread, and `bulk()`'s `poll` hook leave. Every call releases the interpreter with `py.detach` and passes one `CallOptions::interrupt` check. The check re-attaches, runs `check_signals`, and stores a raised error in a `Mutex` slot. It returns `true` on any raise. After the call, a stored `KeyboardInterrupt` raises `Cancelled`, and any other stored error (a `SystemExit`, a handler's own error) raises unchanged. The engine runs the check on the calling thread within one 50 ms tick and never during one blocking send (0095). Decision: a Ctrl-C during one in-flight single send waits for that send to end. The tag answered at once and left the send running on a detached worker. Every other host follows the engine's rule, and the send is bounded by the engine's request timeout. Ian can overturn this. The fallback keeps a worker for single calls only.
- **Inputs.** The binding reads a whole iterable into Rust before the first send and refuses a non-`str` item or a lone surrogate with `UsageError` naming the item's index (G11). A generator is read whole first. The slice goes to the engine. Cost: one copy of the texts in memory. Streaming a generator waits for a need.
- **Questions.** `tt.question(...)` composes the question-file object and calls `Question::from_json`, so parts and files give one digest, as at the tag. `file=` calls `Question::load`. A broken rule from arguments raises `UsageError`, and from a file raises `LocalError` (0095, Q16). Choose and tag with runtime labels go through `Question::choose_labels` and `tag_labels`. A scalar `choose` or `tag` calls `details` and reads `value()` with no added send (G5).
- **Results.** `decide` returns `True`, `False`, or `None` from `Answer`. `decide_many` returns a list from `Batch<Row<_, Answer>>`. `score` returns the position. The nearest level moves to `details`. `tag` returns a list of labels. `filter` returns the passing texts. `rank` and `find` wrap each text in one binding `Evidence` type holding its index and keep the tag's `{"index", "record" or "unit", "probability"}` dictionaries. `find` returns `None` when nothing is selected. `annotate` over a list returns one dictionary per record from `AnnotatedRecord::values`. A failed member keeps the tag's failed marker, read from `Failed::kind` and `cause`.
- **Details and usage.** `details` returns `json.loads` of `Details::to_json`. The Python dictionary equals the command's `--details` document. `usage` returns `Engine::usage()` as `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`. Case 17 asserts differences around a call.
- **Recognize and relate.** The Python spec readers (`spec_from_file`, `end_kind`, `ends_pair`, `build_recognize`, `build_relate`) leave. Keyword forms go through `Recognize::builder` and `Relate::builder`. A path goes through `load`. A dictionary goes through `json.dumps` and `from_json`. `Entity` becomes `{name, kind, start, end, strength}`, and offsets count Unicode scalar values. Python indexes a string the same way. `relate` takes `(name, kind)` pairs, dictionaries with `name` and `kind`, or `Entity` values. Each `Edge` carries `relation`, `source`, `target`, and `probability`, with full endpoints.
- **Errors.** One table maps `ErrorKind::name` to the six classes as an exhaustive `match`. One panic guard at the module edge turns a binding panic into `DefectError` (R2-31). `thinkthen` already stops engine panics at its public methods (0086).
- **Registration.** The module registers its functions by hand in one list. A pytest test compares `__all__`, the stub's names, and the module's attributes.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 21 Python rows. This ticket re-proves ten of them, two shared packaging rows, and the host halves of two engine rows. Each re-proof runs against the real engine through the 0092 loopback backend. The record plants each bug below and shows its test turning red, then green once the bug is removed. "Counted" means the 0092 backend's `count` line.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-6 | closed | A pandas `Series` passed to `decide_many` and to `filter` raises `UsageError` with zero counted requests. No one-row wrapper returns. | Drop the `pandas` module check. The Series runs and the count is nonzero. |
| R1-24 | closed | A token cancelled before the call sends zero. A token cancelled from a second thread during a 200-text `decide_many` at width 8, on the held arm, raises `Cancelled` with at most 8 counted sends. A token stops a single `decide` held at the width gate. | Omit `CallOptions::cancel`. The batch sends all 200. |
| R2-11 | closed | A pandas `DataFrame` passed to `annotate` and a `pyarrow` array passed to `decide_many` raise `UsageError` before any send, and the test counts requests. 0106 keeps the pandas case and turns the `pyarrow` case into a door test. | Refuse after the engine call returns. The count is nonzero. |
| R3-18 | closed | `tt.question(decide=..., choose=...)` raises `TypeError` naming both verbs. An unknown keyword raises `TypeError`. Nothing is dropped. | Take the first verb and ignore the rest. The question builds. |
| R4-14 | closed | `check.sh` runs `pytest tests/` whole. The built wheel holds `__init__.pyi` and `py.typed`. The manylinux tag moves to the release ticket (queue item 4). | Two plants. A new failing `tests/test_planted.py` turns `check.sh` red. A wheel built without `py.typed` fails the wheel content check. |
| R4-23 | closed | A `SIGINT` raised from a timer thread during a 200-text `decide_many` at width 8 stops new sends within one tick and raises `Cancelled`. During a held single `decide`, it ends with exactly one counted send and no retry. A handler that raises `SystemExit` surfaces `SystemExit`. | An interrupt check that never calls `check_signals`. The batch sends all 200. |
| R5-7 | closed | The module docstring and `README.md` carry the pinned deadline sentence. `deadline=-1` runs with no deadline. `deadline=-2` raises `UsageError`. | Refuse `-1`. The run test turns red. |
| R5-8 | closed | `deadline=True`, `False`, and `numpy.bool_(True)` raise `UsageError` with zero counted requests. | Drop the bool check. `True` runs as one second. |
| R6-11 | closed | `tt.decide(42, "x")` and `tt.decide({"bad": 1}, "x")` raise `UsageError` with `kind == "usage"` and `retryable is False`. | Raise a bare `TypeError`. The kind check turns red. |
| R6-13 | closed | `tt.choose(a_score_question, "x")` raises a message naming `choose`. The test pins the whole sentence. | Name a fixed verb in the message. |
| R1-31 | waive | Python keeps one kind table. A Rust unit test maps each `ErrorKind` to its class and its `kind` word. | Map `Deadline` to `BackendError`. |
| R2-31 | partial | Python keeps one panic guard. A `check.sh` step counts one `catch_unwind` site in `src`. | Add a second guard. The count check fails. |
| R1-10 host half | engine | A panic inside shim code raises `DefectError` and the interpreter continues. | Remove the guard. `PanicException` escapes. |
| R1-11 host half | engine | `deadline=1e300`, `inf`, and `nan` raise `UsageError` with zero counted requests. | Convert with `Duration::from_secs_f64` in the shim. The call panics. |

Rows that move to 0106: R1-3, R1-4, R1-12, R2-12, R2-17, R3-4, R4-15, R5-6, R7-2, and R7-8, with the Arrow layer. R3-19 (pandas docstring advice) goes to 0106 to retire under the ruling. The Polars-column halves of R1-24 and R4-23 move too. Five of these rows are still open at the tag (R3-4, R4-15, R5-6, R7-2, R7-8), and 0106 owns them.

Rows this ticket does not close: R2-27 waits for the TypeScript ticket, since it compares two surfaces. G9 waits on ADR 0047 item 5. The Python page states whichever answer Ian gives. Under the recommended answer, the page says the width cap holds per loaded copy.

R2-29 asks for rulings on record. The rulings that live only in the tag's `NOTES.md` (`Cancelled`'s two parents, the order of a stored signal error over `Cancelled`, and the pandas refusal under the 2026-09-21 ruling) land in a short Python section of ADR 0047 in this ticket.

## Other acceptance

- Red first: the ported tests fail against an empty `libraries/python` workspace for the stated reason, then pass.
- A bare `cargo test --no-default-features --lib --locked` in `libraries/python` passes with libpython linked, as at the tag.
- The conformance runner reads main's `conformance/cases.json` and runs every applicable case through the 0092 case arm with recomputed digests. It reports a skipped case as not run (R5-32). Case 18 (cancel mid-batch) runs through a Python arm on the held reply.
- A fork test warms the default engine, calls `os.fork`, and gets an answer in the child from the loopback backend. The parent's counters do not move (0096, Q15).
- Offsets: case 68 (an accent and an emoji) returns the same `start` and `end` in Python as in Rust.
- The shim holds no hand-written `unsafe`. The ADR 0047 policy check passes on the binding's manifest, lock, lint table, and release profile. If pyo3's generated code trips `unsafe_code = "deny"`, the builder stops and records the case for an ADR 0047 amendment.
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset in every step, and a secrecy test reads every raised message and `repr` for the key and the base URL's credentials.

## The check it adds to the gate ladder

- `libraries/python/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047, the fifth rung after `spec`) runs it with the 0092 loopback port. A missing Python 3.10 or later, `uv`, `maturin`, or a uv cache without a pinned package reports "not run" and never "pass" (R6-2).
- `check.sh` drops `ENGINE_NULL`, `THINKTHEN_NULL`, the `synthetic-partial` feature, and the wire stub on port 8211. Every step runs against the port it receives.
- `lint` runs the ADR 0047 manifest, lock, lint-table, and profile checks on `libraries/python`. It runs `ratchet.mjs` on `libraries/python/ratchet.json` for Rust and on `libraries/python/ratchet.py.json` for Python. It runs the registry check, which refuses Python without its `check.sh`.
- `requirements-dev.txt` keeps its pins and hashes. It drops `polars-runtime-32` only if no refusal test needs it. pandas, Polars, pyarrow, and NumPy stay as test-only pins for the refusal and bool tests. The wheel depends on none of them.

## Budgets

- Production Rust: at most five files and 1,200 nonblank lines, each file under 500.
- Python package: `__init__.py` and `__init__.pyi` together at most 400 nonblank lines.
- Tests: at most twelve Python test files and 1,600 nonblank lines, plus at most 250 nonblank Rust unit-test lines and the conformance runner at most 250.
- Scripts: `check.sh` and `build-wheel.sh` together at most 180 nonblank lines. Gate changes under `sdlc/scripts` at most 40 nonblank lines, since 0093 builds the pattern.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/libraries/python.md`, and the ADR 0047 section, at most 260 net nonblank lines.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change, since no file under `crates` or `conformance` changes. The record names what each added block earns and where the tag's duplicate code went first.
- No dependency is added to `thinkthen`. The binding's own dependencies are `thinkthen` by path and `pyo3` 0.29. `serde_json` stays only if the shim still needs it after `to_json`, and the record says which.

Stop and re-score before crossing a budget, adding a dependency, touching `crates/thinkthen`, or porting any part of 0106.

## Exclusions

The Polars door and the Arrow layer (0106). A pandas door, which the ruling removed. Wheels for release, manylinux tags, and uploads (queue item 4). Any change to `thinkthen` or its public API. Async forms. An `Engine` class in Python. Any live or paid call.

## Dependencies

After 0086 lands. Also after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`), 0099 (ADR 0041 on main), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 has landed. Ticket 0106 follows this one.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. Signals cross the interpreter lock, and a wrong interrupt rule loses a user's Ctrl-C or spends a batch.

## Review

- Design review: pending.
- Code review: pending.
