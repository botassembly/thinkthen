# 0661: Export Python's typed result classes publicly

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0662, 0688

## Outcome

- A public `thinkthen.types` module, or `thinkthen` itself, exports the generated result, row, answer and packet classes under stable names.
- `Result` is typed per function in the generated stubs, so `tt.decide(...).value` type-checks as `bool | None`.
- Tests and examples import only public names.

### Proof

On the M5 through pm job, `mypy --strict` passes over a caller that annotates every result type using only public imports from the installed wheel.

## Evidence

- The typed rows and answers exist only in private modules. Tests import them from `thinkthen._native_results` and `thinkthen._polars_calls` (libraries/python/tests/native_types.py:3-5, native_frame_types.py:3, options_types.py:5).
- `thinkthen.__all__` exports none of them (libraries/python/thinkthen/__init__.py:12), so an application has to import private names to annotate its own code.
- `Result` is `Generic[T]` with an untyped `results: tuple` (_calls.py:87-93).
