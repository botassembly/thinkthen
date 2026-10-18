# ThinkThen for Python

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```python
import thinkthen as tt

question = "Does the customer ask for a refund?"
message = "Please refund my order. It arrived broken."
with tt.Engine() as engine:
    refund = engine.decide(question, message)
    assert refund.value is True
    assert refund.probability > 0.5
    assert len(refund.results) == 1
    assert refund.facts is not None
```

`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find` and `annotate` take a question followed by their input. `recognize` and `relate` take their input first and an optional authored question second. Pass `options`, `levels` or `labels` beside question text, or pass an authored question dictionary, a path object or `QuestionSource(name="name")`. Plain strings remain question text for atomic calls. Enum, Literal and the optional Pydantic authoring forms convert to the same native declaration.

`Result.value` presents the ordinary answer. `Result.probability` presents its probability when the function supplies one. Filter returns the selected inputs, rank returns their original indices and find returns the selected input or `None`. `Result.results` retains the typed rows with their probabilities, sources, image facts, identifiers and model provenance. `Result.facts` contains the call's final counts. Printing the result withholds its content. A failed call raises a typed error and never supplies a false answer.

```python
import thinkthen as tt

selection = tt.read_files("documents", unit="line")
question = "Does this line describe a refund?"
with tt.Engine(model="local-1") as engine:
    kept = engine.filter(question, selection)
    for row in kept.results:
        print(
            row.source.file, row.source.first_line,
            row.source.last_line,
        )
```

`Files` also supports `jsonl=True` and `media="image"`. `Item` supplies explicit JSON values, per-record context, candidate descriptions and ordered `Image(media="image/png", data=bytes)` attachments. An omitted context stays absent; explicit JSON null remains null. Native declarations decide which contexts and options are admitted. Files and items use the same ten functions.

`engine.asyncio` mirrors the named calls and keeps the event loop responsive. Task cancellation closes its producer and native session without waiting for a held provider. Engine context exit closes every active session. Errors have a native kind and retry signal. Started failures retain any completed results and actual facts; unsettled facts remain absent.

`engine.iterate(function, question, input, **controls)` returns a lazy typed iterator over the same native session. It starts on the first pull and supports context-manager cleanup, cancellation and `facts` after settlement. `engine.plan(function, question, complete_input, **controls)` returns a typed `Plan` from the native no-send preview for atomic calls and single-question rank. Read `plan.records`, `plan.requests`, `plan.estimated_input_tokens.lower` and `.upper`, or the largest-request estimates. `plan.first_body_utf8` contains the first prepared body when present. The preview needs no key and refuses iterators before consuming them.

Pandas and Python Polars Series namespaces are opt-in:

```python
import thinkthen.pandas
# Equivalent Polars registration: import thinkthen.polars
refund = series.tt.decide("Does this ask for a refund?", engine=engine)
```

All ten namespace methods retain original row positions and null masks. Filter returns selected original rows. Rank, find and relate issue one whole-set call. Polars presents ordinary answers as typed columns, including nested lists and structs. Columns retain Object values when a typed representation would change a value or add an absent member. Detailed results retain the native result objects. `on=` selects DataFrame input columns for the ordinary engine methods; annotation adds named typed columns and failures, while recognition preserves its documented dataframe presentation. Async calls share this conversion. The package imports neither dataframe library by default. Pydantic support is available through `thinkthen[pydantic]`.

Build a wheel with `sh build-wheel.sh`. `check.sh` installs a wheel outside the source package and runs routine behavior and shared cases; `THINKTHEN_ARTIFACT` selects an existing wheel. `THINKTHEN_TEST_PROFILE=full` selects the entire shared inventory at a candidate. Windows and Apple package qualification run at the candidate.
