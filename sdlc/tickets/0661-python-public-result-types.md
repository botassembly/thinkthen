# 0661: Export Python's typed result classes publicly

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0662, 0688, 0727

## Outcome

- Changes: `libraries/python/thinkthen/__init__.py`, `libraries/python/thinkthen/__init__.pyi`, `libraries/python/thinkthen/types.py`, `libraries/python/thinkthen/_calls.pyi`, `libraries/python/tests/native_types.py`, `libraries/python/tests/native_frame_types.py`, `libraries/python/tests/options_types.py`, `libraries/python/check.sh`, `conformance/library-surface/python.json`.
- A public `thinkthen.types` module exports the generated result, row, answer and packet classes and the per-function result classes under stable names without a leading underscore.
- The per-function result classes in the stubs type `value` exactly, so `tt.decide(...).value` type-checks as `bool | None` for a scalar input.
- Tests and examples import only public names. The library surface registry lists `thinkthen/types.py` among Python's declarations.
- Public names carry no format version, per the 2026-10-19 ruling.

### Proof

- On the M5 natively: `pm job run --machine m5 0661 SHA --memory 8 -- sh libraries/python/check.sh` runs `mypy --strict` over the type tests, which annotate every result type using only public imports from the installed wheel. Before the change mypy refuses the imports from `thinkthen.types`, and `decide(...).value` types as the broad `_ReadingValue`.
- `python3 conformance/library-surface.py --library python` passes.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/python.txt` and its surface entry in `conformance/library-surface/python.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- The typed rows and answers exist only in private modules. `tests/native_types.py` imports from `thinkthen._native_results`, `thinkthen._polars_calls` and `thinkthen._frame_calls`, `tests/native_frame_types.py` from `thinkthen._native_results`, and `tests/options_types.py` from `thinkthen._named_generated`. `check.sh` runs `mypy --strict` over `tests/native_types.py` and `tests/options_types.py`.
- `__all__` in `libraries/python/thinkthen/__init__.py` exports none of them, so an application has to import private names to annotate its own code.
- `__init__.pyi` declares per-function result classes (`_DecideResult` through `_RelateResult`) as private names, and `_DecideResult` types `value` as `_ReadingValue`, a union of every JSON shape. `Result` in `_calls.pyi` types `results` as `tuple[_NativeResult, ...]`.
- `conformance/library-surface.json` lists Python's declaration files.
- 0662 settles the result shape first, so this ticket exports the names that remain.
