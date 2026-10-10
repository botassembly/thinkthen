# ThinkThen for Python

Python uses one `Engine` with ten named functions. Rust owns admission, reading, execution, cache identity and result fields. Python converts ordinary values, schedules calls and cleans up sessions.

```python
import thinkthen as tt

with tt.Engine(cache=False) as engine:
    refund = engine.decide("Does this ask for a refund?", "Please refund the charge.")
    print(refund.value, refund.probability)
    print(refund.results[0].answer_id, refund.facts.requests_sent)
```

`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find` and `annotate` take a question followed by their input. `recognize` and `relate` take their input first and an optional authored question second. Pass `options`, `levels` or `labels` beside question text, or pass an authored question dictionary, a path object or `QuestionSource(name="name")`. Plain strings remain question text for atomic calls. Enum, Literal and the optional Pydantic authoring forms convert to the same native declaration.

`Result.value` presents the ordinary answer. `Result.results` retains every Rust-owned generated result, including probabilities, sources, image facts, reading choices, identifiers and model provenance. `details=True` presents those generated rows as the value. `Result.facts` contains actual final settlement. Printing withholds content; equality, `to_dict()` and pickling preserve absence, null, false, wide integers and permitted extension fields. Embedded failures remain typed failures and cannot become false answers.

```python
selection = tt.read_files(["notes.txt"], unit="line")
with tt.Engine(cache=False) as engine:
    kept = engine.filter("Does this ask for a refund?", selection, details=True)
    for row in kept.results:
        print(row.source.file, row.source.first_line, row.source.last_line)
```

`Files` also supports `jsonl=True` and `media="image"`. `Item` supplies explicit JSON values, per-record context, candidate descriptions and ordered `Image(media="image/png", data=bytes)` attachments. An omitted context stays absent; explicit JSON null remains null. Native declarations decide which contexts and options are admitted. Files and items use the same ten functions.

`engine.asyncio` mirrors the named calls and keeps the event loop responsive. Task cancellation closes its producer and native session without waiting for a held provider. Engine context exit closes every active session. Errors have a native kind and retry signal. Started failures retain any completed results and actual facts; unsettled facts remain absent.

`engine.iterate(function, question, input, **controls)` returns a lazy typed iterator over the same native session. It starts on the first pull and supports context-manager cleanup, cancellation and `facts` after settlement. `engine.plan(function, question, complete_input, **controls)` calls the native no-send preview for supported functions. It refuses iterators instead of consuming them as a preview.

Pandas and Python Polars Series namespaces are opt-in:

```python
import thinkthen.pandas
# Equivalent Polars registration: import thinkthen.polars
refund = series.tt.decide("Does this ask for a refund?", engine=engine)
```

All ten namespace methods retain original row positions and null masks. Filter returns selected original rows. Rank, find and relate issue one whole-set call. `on=` selects DataFrame input columns for the ordinary engine methods; annotation adds named typed columns and failures, while recognition preserves its documented dataframe presentation. Async calls share this conversion. The package imports neither dataframe library by default. Pydantic support is available through `thinkthen[pydantic]`.

## Replace old calls

| Previous call | Current call |
| --- | --- |
| `tt.decide(q)(rows)` and `Judge` | `tt.decide(q, rows)` |
| `engine.complete.decide(q, rows)` | `engine.decide(q, rows)` |
| `tt.details(q, row)` | `tt.decide(q, row, details=True)` |
| `thinkthen.complete.Item` and `Image` | `tt.Item` and `tt.Image` |
| `complete.Files` | `tt.Files` or `tt.read_files` |
| Complete dataframe engine/accessor | Ordinary engine or `series.tt` |
| Legacy batch methods | `engine.iterate(function, question, rows)` |
| Judge preview | `engine.plan(function, question, rows)` |

Build a wheel with `sh build-wheel.sh`. `check.sh` installs a wheel outside the source package and runs routine behavior and shared cases; `THINKTHEN_ARTIFACT` selects an existing wheel. `THINKTHEN_TEST_PROFILE=full` selects the entire shared inventory at a candidate. Windows and Apple package qualification run at the candidate.
