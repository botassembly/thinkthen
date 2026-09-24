---
flow: build
priority: 105
opens: libraries/python sdlc/scripts sdlc/planning/libraries/python.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md
---

# 0105: Port the Python surface

Status: revised after confirmation; final check. Owner: Claude.

## Outcome and authority

Port the core Python binding from tag `surfaces-wave7-frozen-2026-09-24b` (`9df8bae9`) onto the public Rust API as the unpublished crate `thinkthen-python` at `libraries/python`, in its own Cargo workspace. `import thinkthen as tt` keeps its ten verbs, `decide_many`, `details`, `question`, `usage`, the six exception classes, and `CancelToken`. Every call reaches the real engine through `thinkthen`. Queue item 3 of `sdlc/planning/one-line-plan-2026-09-24.md` puts Python first after C.

Draft ADR 0047 fixes the crate's place and its checklist. Ticket 0093 sets the workspace, lint, ratchet, and surface-rung pattern, and this ticket copies it. Ticket 0095 fixes the members this binding calls. Section 3.2 of `sdlc/planning/surfaces-port-guide.md` gives the port map. The 2026-09-24 design review (`sdlc/records/2026-09-24-design-review-0105-0106.md`) found eight items, and its confirmation found two more. This version answers all ten. Ian can overturn every decision below.

## Design and decisions

1. **Two tickets.** Ian ruled on 2026-09-21 that Python's data frame is Polars at 0.1 and that pandas leaves the surface (ADR 0017, "Ruled after acceptance, 2026-09-21: the data frame is Polars"). His clarification the same day makes pure Python and Polars both first-class, with all scaling in Rust. The only open Polars question is a Rust Polars door after 0.1 (ADR 0047 item 8). The port splits for review size. This ticket ports the core binding over a `str` and a `list`, `tuple`, or other iterable of `str`. Ticket 0106 ports the Polars layer on top. 0.1 needs both.
2. **Refusals, in order.** Before any request, the binding checks a container in two steps. First, an object whose type's top-level module is `pandas` raises `UsageError` with the pandas sentence. This check stays after 0106. Second, until 0106 lands, a Polars or `pyarrow` object, or any object that exposes `__arrow_c_stream__`, `__arrow_c_array__`, or `__dataframe__`, raises `UsageError` with the Arrow sentence. The pandas check runs first because pandas 3.0.6 objects expose `__arrow_c_stream__`. The two sentences differ, and tests pin each whole. The pandas sentence says the Python data frame is Polars, from the 2026-09-21 ruling. 0106 turns the Arrow refusal into the door. A user never sees a returned value change shape.
3. **Engine.** Every call uses `thinkthen::default_engine()`. The shim adds no engine setting. The environment variables `Engine::from_env` reads configure it. Python has no `Engine` class.
4. **Calls and options.** Each verb calls the matching `Engine` `_with` form with one `CallOptions`. `token=` maps to `CallOptions::cancel`. `deadline=` goes through `CallOptions::deadline_seconds` after the binding's bool and non-number checks. The tag's `TokenBridge` and its own deadline conversion leave.
5. **Interrupts in batches.** The tag's `bulk()` `poll` hook and `TokenBridge` leave. `decide_many`, `filter`, `rank`, `find`, `annotate`, and `relate` release the interpreter with `py.detach` and pass one `CallOptions::interrupt` check. The check re-attaches, runs `check_signals`, stores a raised error in a `Mutex` slot, and returns `true`. The engine runs it on the calling thread within one 50 ms tick (0095). After the call, a stored `KeyboardInterrupt` raises `Cancelled`. Any other stored error, such as a `SystemExit`, raises unchanged.
6. **Single sends stay promptly interruptible.** The coordinator decided on 2026-09-24 to keep the tag's prompt interrupt for single calls, citing R4-23. That row closed at one request and 0.10 s, and the engine's interrupt check does not run during one blocking send. A wait of up to the 30-second request timeout would reopen it in a REPL or under Jupyter's interrupt button. Design: `decide`, `choose`, `score`, `tag`, `details`, and `recognize` each run on a spawned worker that never touches Python. The worker owns a clone of the question, the text as a `String`, a clone of an internal `CancelToken`, and a clone of the caller's `token=` if given. It builds its own `CallOptions` with the internal token as `cancel` and an interrupt check that reads the caller's token. An interrupt therefore never fires the caller's token, as at the tag. The calling thread releases the interpreter and waits on a channel in 50 ms ticks. On each tick it re-attaches and runs `check_signals`. On a raise it cancels the internal token, detaches the worker, and raises `Cancelled` at once, or the stored error unchanged. The detached send finishes under 0073, and no retry starts, since the token is cancelled. Batches and singles now both raise `Cancelled` on Ctrl-C. Cost: about 50 lines and one thread spawn per single call. Until its send ends, a detached send holds one width permit. No change to `thinkthen` is needed: 0095 and ADR 0047 allow the worker pattern. Ian can overturn the coordinator's decision.
7. **Inputs.** The binding reads a whole iterable into Rust before the first send. A non-`str` item or a lone surrogate raises `UsageError` naming the item's index (G11). A generator is read whole first. Cost: one copy of the texts in memory.
8. **Questions.** `tt.question(...)` composes the question-file object and calls `Question::from_json`, so parts and files give one digest. `file=` calls `Question::load`. A broken rule from arguments raises `UsageError`, and from a file raises `LocalError` (0095, Q16). Choose and tag with runtime labels use `Question::choose_labels` and `tag_labels`. A scalar `choose` or `tag` calls `details` and reads `value()` with no added send (G5).
9. **Results.** `decide` returns `True`, `False`, or `None`. `decide_many` returns a list. `score` returns the position, and the nearest level moves to `details`. `tag` returns a list of labels. `filter` returns the passing texts. `rank` and `find` wrap each text in one binding `Evidence` type holding its index and keep the tag's `{"index", "record" or "unit", "probability"}` dictionaries. `find` returns `None` when nothing is selected. `annotate` over a list returns one dictionary per record from `AnnotatedRecord::values`, with the tag's failed marker from `Failed::kind` and `cause`. `details` returns `json.loads` of `Details::to_json`, equal to the command's `--details` document. `usage` returns `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`.
10. **Recognize and relate.** The tag's spec readers (`spec_from_file`, `end_kind`, `ends_pair`, `build_recognize`, `build_relate`) leave. Keyword forms use `Recognize::builder` and `Relate::builder`. A path uses `load`, and a dictionary uses `json.dumps` and `from_json`. `Entity` becomes `{name, kind, start, end, strength}`. Offsets count Unicode scalar values. Python indexes a string the same way. `relate` takes `(name, kind)` pairs, dictionaries with `name` and `kind`, or `Entity` values. Each `Edge` carries `relation`, `source`, `target`, and `probability`.
11. **Errors and panics.** One exhaustive `match` maps `ErrorKind::name` to the six classes. `Cancelled` subclasses both `KeyboardInterrupt` and `ThinkThenError`. One panic guard at the module edge turns a binding panic into `DefectError`. `thinkthen` already stops engine panics at its public methods (0086).
12. **Registration.** The module registers its functions by hand in one list. The tag's generator retires. A pytest test compares `__all__`, the stub's names, and the module's public attributes. Names that start with `_` are test hooks and fall outside the comparison.

## What moves from the tag

- `thinkthen/__init__.py` and its docstrings. The frame branches (`_column_or_value`, `_refuse_if_not_rebuildable`, `annotate_stream`, `recognize_stream`, `relate_stream`) leave, and 0106 ports the Polars ones.
- `thinkthen/__init__.pyi` and `thinkthen/py.typed`. The stub follows the new result shapes.
- `CancelToken`, the six classes with `kind` and `retryable`, and the deadline rule of ADR 0041 with its Python amendment.
- `pyproject.toml`: maturin `>=1.5,<2`, `abi3-py310`, module `thinkthen._thinkthen`, and `license = { text = "MIT" }`. The `polars` extra moves to 0106.
- `build-wheel.sh` and the home-path remap. `check.sh` keeps its shape, with the changes in the gate section.
- `examples.json` and `tests/examples.py`. Each expected value is re-derived against 0092's generic arm.
- Tests that call the binding keep their assertions: `test_surface.py`, `test_cancel.py`, `test_cancel_fast.py`, `test_review2_signals.py`, `test_review5_deadline.py`, `test_review5_verbs.py`, and the list-form tests inside `test_review2_findings.py`, `test_review2_wire.py`, `test_review3_offline.py`, `test_review3_wire.py`, `test_review4_signals.py`, `test_ownership.py`, and `test_recognize_relate.py`. The frame cases of the last two move to 0106. The builder regroups by topic into at most twelve files.
- `tests/conformance.py`, rewritten onto main's `conformance/cases.json`.

These stay at the tag for 0106 or retire: `src/arrow.rs`, `src/generated.rs` and its generator, `tests/sliced_struct_stream.py`, `test_polars_door.py`, `test_pandas_checks.py`, `test_review7_arrow.py`, `test_deadline_column.py`, `bench_recognize_scale.py`, the four width and cost benches, and the frame half of `slide_sample.py`. 0106 ports the Polars pieces. The pandas pieces retire under the ruling. `NOTES.md` stays at the tag as history. The port writes a new `NOTES.md` of at most 120 lines. The tag has no Python `DESIGN.md`. `sdlc/planning/libraries/python.md` is the design page, and this ticket updates it.

## Error-index rows

Source: `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. The index lists 21 Python rows. This ticket re-proves ten of them. It also re-proves the Python halves of six cross-surface rows and of two engine rows. Each re-proof runs against the real engine through the 0092 loopback backend unless marked as a unit test. The record plants each bug below and shows its test turning red, then green once the bug is removed. "Counted" means the 0092 backend's `count` line.

| Row | Status at tag | Re-proof here | Planted bug |
|---|---|---|---|
| R1-6 | closed | A pandas `Series` passed to `decide_many` and to `filter` raises `UsageError` with the pinned pandas sentence and zero counted requests. | Drop the pandas module check. The Arrow sentence answers, and the pinned-sentence assertion turns red. |
| R1-24 | closed | A token cancelled before the call sends zero. A token cancelled from a second thread during a 200-text `decide_many` at width 8, on the held arm, raises `Cancelled` with at most 8 counted sends. A token stops a single `decide` held at the width gate. | Omit `CallOptions::cancel`. The batch sends all 200. |
| R2-11 | closed | A pandas `DataFrame` passed to `annotate` raises the pinned pandas sentence, and a `pyarrow` array passed to `decide_many` raises the pinned Arrow sentence. Both count zero requests. 0106 keeps the pandas case and turns the `pyarrow` case into a door test. | Refuse after the engine call returns. The count is nonzero. |
| R3-18 | closed | `tt.question(decide=..., choose=...)` raises `TypeError` naming both verbs. An unknown keyword raises `TypeError`. | Take the first verb and ignore the rest. The question builds. |
| R4-14 | closed | `check.sh` runs `pytest tests/` whole. The built wheel holds `__init__.pyi` and `py.typed`. The manylinux tag moves to the release ticket (queue item 4). | A new failing `tests/test_planted.py` turns `check.sh` red. A wheel built without `py.typed` fails the wheel content check. |
| R4-23 (batch) | closed | A `SIGINT` from a timer thread during a 200-text `decide_many` at width 8 stops new sends within one tick and raises `Cancelled`. | An interrupt check that never calls `check_signals`. The batch sends all 200. |
| R4-23 (single) | closed | The test holds one `decide` on the held arm and sends `SIGINT` at 0.2 s. `Cancelled` arrives by 0.3 s. The test then releases the reply, and the count stays at exactly one. | Run single calls on the calling thread. The 0.3 s assertion turns red. |
| R5-7 | closed | The module docstring and `README.md` carry the pinned deadline sentence. `deadline=-1` runs with no deadline. `deadline=-2` raises `UsageError`. | Refuse `-1`. The run test turns red. |
| R5-8 | closed | `deadline=True`, `False`, and `numpy.bool_(True)` raise `UsageError` with zero counted requests. | Drop the bool check. `True` runs as one second. |
| R6-11 | closed | `tt.decide(42, "x")` and `tt.decide({"bad": 1}, "x")` raise `UsageError` with `kind == "usage"` and `retryable is False`. | Raise a bare `TypeError`. |
| R6-13 | closed | `tt.choose(a_score_question, "x")` raises a message naming `choose`. The test pins the whole sentence. | Fix the message on `decide`. |
| R2-25 Python half | closed | `Cancelled` is a subclass of `KeyboardInterrupt` and of `ThinkThenError`. A `SIGINT` handler that raises `SystemExit` during a batch surfaces `SystemExit`. | Raise `Cancelled` for every stored error. |
| R2-10 Python half | partial | `deadline=-1` and `None` mean none. `deadline=0` raises `DeadlineError` with zero counted requests. `-2` and `4294967296` raise `UsageError`. | Treat any negative as none. `-2` runs. |
| R1-34 Python half | closed | The built wheel's `METADATA` carries `License: MIT`. The stand-in's recordings no longer exist to ship. | Remove the license field. The wheel content check fails. |
| R4-19 and R5-34 Python half | closed and not re-probed | Every `cargo` and `maturin` call in `check.sh` and `build-wheel.sh` passes `--locked` and `--offline`. A `check.sh` step reads both scripts and fails on a call without them. | Drop `--locked` from `build-wheel.sh`. The step fails. |
| R3-30 Python half | not re-probed | The runner has no local skip list. It reports every case in `cases.json` as pass, fail, or not run with the reason, and the three counts sum to the file's count. | Skip one case silently. The sum check fails. |
| R1-31 | waive | One kind table. A Rust unit test maps each `ErrorKind` to its class and `kind` word. | Map `Deadline` to `BackendError`. |
| R2-31 | partial | One panic guard. A `check.sh` step counts one `catch_unwind` site in `src`. | Add a second guard. |
| R1-10 host half | engine | Rust unit test with libpython linked: the guard runs a closure that panics and returns `DefectError` with `kind == "defect"`. The same test then runs a second attached call, which proves the interpreter continues. | Remove the guard. The panic fails the test. |
| R1-11 host half | engine | `deadline=1e300`, `inf`, and `nan` raise `UsageError` with zero counted requests. | Convert with `Duration::from_secs_f64` in the shim. The call panics. |

Rows that move to 0106: R1-3, R1-4, R1-12, R2-12, R2-17, R3-4, R4-15, R5-6, R7-2, and R7-8, with the Arrow layer. R3-19 goes to 0106 to retire under the ruling. The Polars-column halves of R1-24 and R4-23 move too.

Not closed here: R2-27 waits for the TypeScript ticket, since it compares two surfaces. G9 waits on ADR 0047 item 5. The Python page states whichever answer Ian gives.

R2-29 asks for rulings on record. The rulings that live only in the tag's `NOTES.md` (`Cancelled`'s two parents, a stored signal error over `Cancelled`, and the pandas refusal) land in a short Python section of ADR 0047.

## Other acceptance

- Red first: the ported tests fail against an empty `libraries/python` workspace for the stated reason, then pass.
- `cargo test --no-default-features --lib --locked --offline` in `libraries/python` passes with libpython linked.
- The conformance runner runs every applicable case through the 0092 case arm with recomputed digests. Case 18 (cancel mid-batch) uses 0092's existing held arm. No backend arm is added.
- A fork test warms the default engine, calls `os.fork`, and gets an answer in the child. The parent's counters do not move (0096, Q15).
- Case 68 (an accent and an emoji) returns the same `start` and `end` in Python as in Rust.
- The shim holds no hand-written `unsafe`.
- Nothing reaches a non-loopback address. `THINKTHEN_API_KEY` stays unset. A secrecy test reads every raised message and `repr` for the key and for the base URL's credentials.

## The check it adds to the gate ladder

- `libraries/python/check.sh` joins the surface registry as landed. The `surfaces` rung (ADR 0047, the fifth rung after `spec`) runs it with the 0092 loopback port. A missing Python 3.10 or later, `uv`, or `maturin` reports "not run" and never "pass" (R6-2). So does a uv cache or cargo cache that lacks a pinned package, and the line names the one fetch to run on a networked machine.
- Every `cargo` and `maturin` step passes `--locked` and `--offline`. `check.sh` drops `ENGINE_NULL`, `THINKTHEN_NULL`, the `synthetic-partial` feature, and the wire stub on port 8211.
- `lint` runs on `libraries/python`: the ADR 0047 manifest, lock, lint-table, and profile checks; deny as `cargo deny --offline --manifest-path libraries/python/Cargo.toml check --config deny.toml advisories bans licenses sources` against the root `deny.toml`, planted with a git-sourced dependency that `[sources] unknown-git = "deny"` refuses; `ratchet.mjs` on `libraries/python/ratchet.json` for Rust and `ratchet.py.json` for Python; and the registry check.
- The binding's deny call adds `sources` to the root's set. It guards more than a license plant would: it refuses a git or unknown-registry crate, and the license check still runs beside it.
- Lints. The binding's table equals the root table except `unsafe_code = "deny"`. The root forbids `missing_debug_implementations`, `unreachable_pub`, and `unsafe_code`, and denies `expect_used`, `unwrap_used`, `indexing_slicing`, and `panic`. A local `allow` cannot lift a forbid. If pyo3's generated code trips any forbid-level lint, the builder stops and records the case for an ADR 0047 amendment. The tag's manifest already records one such case: `unreachable_pub` against a generated `pub fn`. Hand registration removes that source.

## Dependencies and second review

- Rust: `thinkthen` by path with default features off, and `pyo3` `0.29.2` with `abi3-py310`, the tag's lock and the cached crate here. `serde_json` stays only if the shim still needs it after `to_json`, and the record says which.
- Build: maturin `>=1.5,<2`. 1.15.0 is installed here.
- Test pins in `requirements-dev.txt`, unchanged from the tag with hashes: pytest 9.1.1, numpy 2.5.3, pandas 3.0.6, polars 1.44.2, polars-runtime-32 1.44.2, and pyarrow 25.0.1. pandas, Polars, and pyarrow serve the refusal tests here and 0106's door tests. The wheel depends on none of them.
- These enter main for the first time. The code reviewer checks each entry, the binding lock, and deny's result, and the review record says so (repo `CLAUDE.md`).

## Budgets

- Production Rust: at most five files and 1,200 nonblank lines, each file under 500. 0106 adds its own files under its own budget.
- Python package: `__init__.py` and `__init__.pyi` together at most 400 nonblank lines. The tag measures 369.
- Tests: at most twelve Python test files and 1,800 nonblank lines, at most 250 nonblank Rust unit-test lines, and the conformance runner at most 250.
- Scripts: `check.sh` and `build-wheel.sh` together at most 220 nonblank lines. The tag measures 170 before the wheel content, guard count, flag, and not-run steps. Gate changes under `sdlc/scripts` at most 40 nonblank lines.
- Documentation: `README.md`, the new `NOTES.md`, `sdlc/planning/libraries/python.md`, and the ADR 0047 section, at most 260 net nonblank lines.
- Ratchet: `libraries/python/ratchet.json` and `ratchet.py.json` each set `max` to the measured total. The root `sdlc/ratchet.json` does not change. The record names what each block earns and where the tag's duplicate code went first.

Stop and re-score before crossing a budget, adding a dependency beyond this list, touching `crates/thinkthen`, or porting any part of 0106.

## Exclusions

The Polars door and the Arrow layer (0106). A pandas door. The ruling removed it. Release wheels, manylinux tags, and uploads (queue item 4). Any change to `thinkthen`. Async forms. An `Engine` class in Python. Any live or paid call.

## Dependencies

After 0086 lands. Also after 0098 (labels, spec readers, JSON methods, `ErrorKind::name`), 0093 (the registry, the `surfaces` rung, the ratchet argument, and the binding policy checks), and 0094, since the plan puts C before every other surface. 0092 and 0099 have landed. Ticket 0106 follows this one.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 3; reach 2; proof 3; cost of error 3; total 13. Final level: 3. Signals cross the interpreter lock, and a wrong interrupt rule loses a user's Ctrl-C or spends a batch.

## Review

- Design review: `sdlc/records/2026-09-24-design-review-0105-0106.md` found eight items, all answered: the R1-6 plant, six unowned Python halves, the R1-10 trigger, the Ctrl-C test's end, deny and the forbid-level lints, the dependency versions and second review, `--offline`, and the small fixes. The confirmation (same file) found that the deny plant could not turn red and asked for a prompt single-call interrupt. Both are applied. The final check is pending.
- Code review: pending.
