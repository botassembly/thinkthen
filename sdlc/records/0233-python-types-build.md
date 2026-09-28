# 0233 Python label types and annotate models: build record

Status: Source candidate prepared for fresh independent code/API review. The design review accepted `3d44d4dd`; main `f261fd11` granted these exact runtime files. The branch merged that main text update before source edits. No shared Rust, site, or other lane file changed.

## What changed

- `_labels.py` converts Enum classes, direct Literal forms and lazy Pydantic label forms to the existing ordered list/map JSON. It keeps old lists/maps untouched, rejects observable Enum aliases and unknown overrides before a worker, and lets the native parser refuse a collapsed one-label choose Literal. `descriptions=` reaches `question`, scalar/many engine methods and module delegates.
- Recognition keyword forms now construct the existing version-one recognize JSON and call `_Recognize._from_json`. This preserves structured descriptions without widening the text-only Rust builder. The same JSON parser supplies relation readings, thresholds, default `ENTITY`, and canonical request identity; `on=` remains the Python frame-column selector.
- `thinkthen.pydantic` is imported only for Pydantic forms or explicit use. It lowers label/question-set models and `Annotated[Literal, Field]` through the existing parser. `row_model` validates the supplied set natively before extracting its already-validated field kinds and labels for strict Pydantic row validation. File input is read once and maps file errors to `LocalError`; dict/model misuse remains `UsageError`. It never changes `Call.value`.
- The stub names finite question/error kinds, a distinct typed failed marker and typed list annotation rows. `annotate` overloads preserve the frame result route. A direct `Literal[...]` object has no portable static parameter type, so `LabelSet` remains `Any` at that one input boundary; the runtime and native parser validate it. The mypy fixture checks concrete output types and Enum/Literal call spelling.
- `pyproject.toml` adds optional `pydantic>=2.11,<3`. The development proof pins Pydantic 2.12.5, mypy 1.18.2 and transitive packages with hashes. The Python check rung runs mypy and verifies the installed modules and optional-extra metadata. README, notes and the checked Enum example describe the new input and result contract.

## Focused proof

- A single offline listener table in `test_label_types.py` compared **actual captured bodies and returned request digests** for list/Enum/Literal, described map/Enum, one-label override, existing recognition list, ordered relations/either/cuts, no-kinds default, and structured kinds against equivalent version-one `ask` objects. The same listener observed no extra send for unknown override, Enum alias or collapsed one-label choose refusal. Its example case exercised the checked library Enum example.
- `test_pydantic.py` compared captured optional-model and annotated-label requests with maps, validated answered/unresolved/failed rows strictly, refused wrong values and extra keys, preserved file-local versus dict-usage errors, and blocked Pydantic imports while a plain core import and question succeeded.
- Focused selectors: `tests/test_label_types.py`, `tests/test_pydantic.py`, three `test_surface.py` question/export/module cases, two `test_door.py` recognition/annotation frame cases, and `test_call.py::test_batches_labels_and_owned_details` (historical exact batch-one bytes/digest) passed: 11 passed in the combined run. After tightening model Field label agreement, the affected five new cases and strict mypy fixture passed again. `python -m compileall`, `sh -n check.sh`, `git diff --check`, and both ratchets passed.
- With 22 GiB available and load near 2 on 16 CPUs, one offline Python 3.13.5 maturin wheel built in this lane. A fresh isolated wheel install imported without Pydantic, contained `_labels.py` and `pydantic.py`, advertised the optional extra, and then validated a row after hash-pinned optional packages were installed. Installed mypy read the wheel's stub and passed the same strict fixture.
- One additionally selected legacy `tests/test_door.py::test_a_frame_keeps_types_and_nested_failures` fails on current main: its private listener indexes the score reply by `asked["state"]`, which is a dict for the frame request, raising `TypeError: unhashable type: 'dict'` and closing the connection. Ticket 0233 did not change that listener or frame request path; the other selected frame cases pass. `tests/test_door.py` is outside this lane's claim, so the coordinator needs to route that separate test correction if required before landing.

The focused commands ran from `libraries/python` with `THINKTHEN_API_KEY` unset and the pinned Python 3.13.5 venv:

```sh
/tmp/thinkthen-codex-1-py313/bin/python -m pytest -q -p no:cacheprovider --tb=short tests/test_label_types.py tests/test_pydantic.py tests/test_surface.py::test_the_module_the_stub_and_all_name_the_same_things tests/test_surface.py::test_parts_and_a_file_make_the_same_question tests/test_surface.py::test_the_module_functions_equal_an_explicit_engine tests/test_door.py::test_recognize_on_a_frame_equals_each_text_alone tests/test_door.py::test_annotate_on_a_frame_keeps_every_row_and_column tests/test_call.py::test_batches_labels_and_owned_details
/tmp/thinkthen-codex-1-py313/bin/python -m pytest -q -p no:cacheprovider --tb=short tests/test_pydantic.py tests/test_label_types.py
/tmp/thinkthen-codex-1-py313/bin/python -m mypy --strict tests/type_contract.py
PYO3_PYTHON=/home/ian/.local/share/uv/python/cpython-3.13.5-linux-x86_64-gnu/bin/python3.13 maturin build --locked --offline -o target/codex-builds/0233/wheels
```

The built wheel was installed with `uv pip install --no-deps` in fresh Python 3.13.5 venvs with and without the hash-pinned optional packages. Standalone import/metadata/row-validation checks passed in both. `node sdlc/scripts/ratchet.mjs libraries/python/ratchet.py.json` and its Rust counterpart passed from the repository root.

## Measured size and test economy

`libraries/python/ratchet.py.json` is now exactly **3,377** nonblank Python lines, up **483** from the accepted 2,894 baseline. Rust remains **6,353/6,353**; no Rust source was edited. `__init__.py` is 480 nonblank lines under its 500-line ceiling; `_labels.py` is 62 and optional `pydantic.py` is 134. The growth buys the optional adapter, new label normalization, typed proof and bounded listener cases. I sought duplication in the existing question/recognize parsers, `_spec`, `Call`, the old listener helper, and 0214 batch-one test; all are reused. No test was deleted or replaced. The new tests cover new public forms and strict optional validation; existing cancellation/facts and frame suites were not copied or rerun broadly. A second agent must review the raised Python ceiling, public signatures and dependency before landing.

## Owner routes and next preparation lesson

Marketing's [site Python migration issue](../issues/2026-09-28-site-python-examples-need-call-values.md) remains open; this build updates library examples and does not claim the thirteen site copies work. The old diagnostic child-environment Quick Fix and the unrelated `test_door.py` listener failure belong to their owners. Future type-port tickets should check a binding's keyword constructor argument type before promising structured metadata, and distinguish a typing object's source syntax from the arguments Python still exposes to runtime.
