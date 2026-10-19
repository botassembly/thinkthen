# 0662: Run Python calls through the Rust run function

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0660, 0688, 0689, 0727

## Outcome

- Changes: `libraries/python/src/`, `libraries/python/thinkthen/_calls.py`, `libraries/python/thinkthen/_calls.pyi`, `libraries/python/thinkthen/_pandas_calls.py`, `libraries/python/thinkthen/_polars_calls.py`, `libraries/python/thinkthen/_frame_calls.py`, `libraries/python/thinkthen/_native_results.py`, `libraries/python/thinkthen/_native_results.pyi`, `libraries/python/tests/`, `build-tools/results/templates/python.py`, `build-tools/results/registry.py`, `build-tools/results/legacy/python.txt`.
- Python passes host values to `Engine::call`, the Rust run function from 0629, and builds results by mapping the 0689 value tree. `Result.value` and `Result.probability` read the packet's answer fields.
- Rust decides which controls apply, their defaults, and how `on` maps. The control lists in `_question`, the `kinds` default and the hand-built descriptor dictionaries in `_item` and `_input` are deleted.
- `_CALL_RESULTS`, `_call_path` and `_call_resolution` stop being generated for Python, and the per-type conversion in `src/results_generated.rs` gives way to the value-tree mapping. `Operation.step`, the resolution bookkeeping and the async polling loop are deleted, with their private closed, reader and producer error text. Async calls run the run function on a worker thread and cancel through its handle.
- Pandas and Polars results read the same packet fields (`_pandas_calls.py`, `_polars_calls.py`, `_frame_calls.py`).
- Label normalization for Enum, Literal and Pydantic in `_labels.py` stays as host-type conversion only.
- `_calls.py`, `_pandas_calls.py` and `templates/python.py` leave this language's exception file `build-tools/results/legacy/python.txt`.
- This changes no public Rust API in `crates/thinkthen`, so `specification/rust-api-0.2.md` is untouched. If the work needs a new public Rust declaration, it updates that file's Authority entry and Added block in the same change.

### Proof

- On the M5 natively: `pm job run --machine m5 0662 SHA --memory 8 -- sh libraries/python/check.sh` passes, and the installed wheel's dump from 0660 matches the 0642 oracle, including a recognize case whose entity has no kept proposal. Before the change that case raises `KeyError` in `Result.probability`.
- `python3 build-tools/results/answer_rules.py` passes with the three Python entries removed from `LEGACY_FILES`; before the change it reports the host answer rules in `_calls.py`.
- Each removed control rule has a case where Rust returns the same typed usage error that Python used to produce.
- An async call cancelled while the loopback provider is held returns promptly.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/python.txt` and its surface entry in `conformance/library-surface/python.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
Python binds to Rust directly, yet it computes answers, restates the grammar and runs its own session loop in `libraries/python/thinkthen/_calls.py`:
- `Result.value` and `Result.probability` compute each call's answer by hand from `_CALL_RESULTS` (`value_projection`, `probability_mode`) and `_call_path`: per-function unwrapping for find, relate and rank, the recognize span-probability join, picked probabilities and resolution tracking. `Operation.step` tracks resolution itself through `_call_resolution`. `_pandas_calls.py` reads `_CALL_RESULTS` in `_probability_rows`.
- `_CALL_RESULTS`, `_call_path` and `_call_resolution` are generated into `_native_results.py` by `build-tools/results/templates/python.py`; `build-tools/results/registry.py` also generates `libraries/python/src/results_generated.rs`.
- That logic already disagrees with TypeScript. Python raises KeyError when a recognized entity has no kept proposal (the `proposals[...]` lookup in `Result.probability`). The generated TypeScript reader `callResult` in `libraries/typescript/results_generated.js` returns null for the same case.
- The contract says a binding "never restates the grammar" (`libraries/BINDING-AUTHOR.md`, "Shared ownership and contracts"). `_question` keeps a per-function list of controls to drop when None for recognize and relate and another for annotate, defaults `kinds` to an empty list for recognize, and `_item` and `_input` build request descriptors as dictionaries.
- `Operation`, `AsyncCalls` and `Session` in `_calls.py` are a session loop with async polling. The native side is `_Engine._request_session` in `src/engine.rs` and `src/worker.rs`.
- `build-tools/results/answer_rules.py` lists `_calls.py`, `_pandas_calls.py` and `templates/python.py` in `LEGACY_FILES`, and says each language's phase 3 call ticket removes its entries.

This ticket merges the former 0659.
