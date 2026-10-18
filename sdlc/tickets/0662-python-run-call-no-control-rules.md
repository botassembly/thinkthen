# 0662: Run Python calls through the Rust run function

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689

## Outcome

- Python passes host values to the Rust run function from 0629 and builds results by mapping the 0689 value tree. `Result.value` and `Result.probability` read the packet's answer fields.
- Rust decides which controls apply, their defaults, and how `on` maps. The control lists, the `kinds` default and the hand-built descriptor dictionaries are deleted.
- `_CALL_RESULTS`, `_call_path` and `_call_resolution` stop being generated for Python. `Operation.step`, the resolution bookkeeping and the async polling loop are deleted, with their private closed, reader and producer error text. Async calls run the run function on a worker thread and cancel through its handle.
- Pandas and Polars results read the same packet fields (`_pandas_calls.py`, `_polars_calls.py`, `_frame_calls.py`).
- Label normalization for Enum, Literal and Pydantic in `_labels.py` stays as host-type conversion only.
- Python's files leave the 0637 answer-rule check's list of exceptions.

### Proof

- On the M5 through pm job, the installed wheel's packet dump matches the 0642 oracle, including a recognize case whose entity has no kept proposal.
- Each removed control rule has a case where Rust returns the same typed usage error that Python used to produce.
- An async call cancelled while the provider is held returns promptly.

## Evidence

Python binds to Rust directly, yet it computes answers, restates the grammar and runs its own session loop in `libraries/python/thinkthen/_calls.py`:
- It computes each call's `value` and `probability` by hand (`_calls.py:93-130`): per-function unwrapping for find, relate and rank, the recognize span-probability join, picked probabilities and resolution tracking. `Operation.step` also tracks resolution itself (`_calls.py:209-211`).
- That logic already disagrees with TypeScript. Python raises KeyError when a recognized entity has no kept proposal (`_calls.py:120`). The generated TypeScript reader returns null for the same case (`libraries/typescript/results_generated.js:3851-3856`).
- The contract says a binding "never restates the grammar" (`BINDING-AUTHOR.md`, "Shared ownership and contracts"). Python keeps a per-function list of controls to drop when None (`_calls.py:63-68`) and for annotate (`:69-71`), defaults `kinds` to an empty list for recognize, builds request descriptors as dictionaries by hand (`_item` and `_input`, `:29-53`), and converts `on=` into `field` (`:174`).
- `Operation`, `AsyncCalls` and `Session` (about `_calls.py:137-300`) are a session loop with async polling.

This ticket merges the former 0659.
