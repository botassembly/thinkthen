# 0659: Delete Python's handwritten answer and probability projection

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0612

## Outcome

- `Result.value` and `Result.probability` read the answer fields the core places in the packet.
- `_CALL_RESULTS`, `_call_path` and `_call_resolution` stop being generated for Python. The resolution bookkeeping in `Operation` is deleted.
- Pandas and Polars results use the same packet fields (_pandas_calls.py, _polars_calls.py, _frame_calls.py).

### Proof

On the M5 through pm job, the installed wheel's packet dump matches the core oracle, including a recognize case whose entity has no kept proposal.

## Evidence

- libraries/python/thinkthen/_calls.py:93-130 computes each call's `value` and `probability` by hand: per-function unwrapping for find, relate and rank, the recognize span-probability join, picked probabilities and resolution tracking.
- `Operation.step` also tracks resolution itself (_calls.py:178-180).
- This logic already disagrees with TypeScript. Python raises KeyError when a recognized entity has no kept proposal (_calls.py:120). The generated TypeScript reader returns null for the same case (libraries/typescript/results_generated.js:3851-3856).
