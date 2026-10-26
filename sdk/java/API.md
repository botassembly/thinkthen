# Java API

The `io.thinkthen` module requires Java 22 or later. Java applications use `ThinkThen` and close it with try-with-resources. Module-path applications enable native access with `--enable-native-access=io.thinkthen`; classpath applications use `--enable-native-access=ALL-UNNAMED`.

`ThinkThen.builder()` accepts `backend(Backend)`, `model(String)`, `url(URI)`, `apiKey(String)`, `batch(int)`, `retry(RetryPolicy)` and `recordCodec(RecordCodec)`. The engine admits settings and resolves omitted defaults. `Backend` has `SYSTEMONE`, `OPENAI` and `STUB`. `RetryPolicy.builder()` accepts `maxRetries(int)`, `initialBackoff(Duration)` and `attemptTimeout(Duration)`. Credentials have no getters and never appear in representations or diagnostic logs.

A record may be a text `String`, an immutable `JsonValue`, a string-keyed map, a list, a boxed JSON scalar or null. Other objects use the optional `RecordCodec`. A string remains text even when it contains JSON. Null and `JsonNull` produce failed missing-record rows. A caller's iterator or codec exception propagates unchanged. Malformed JSON from a codec produces a failed record.

| Method | Result |
| --- | --- |
| `decide(String, Object)` | `Row<Boolean, NoDetail>` |
| `choose(String, Object, List<String> choices)` | `Row<String, ChooseDetail>` |
| `tag(String, Object, List<String> labels)` | `Row<List<String>, TagDetail>` |
| `score(String, Object, List<String> levels)` | `Row<Double, ScoreDetail>` |
| `find(String, Object)` | `Row<Optional<Span>, NoDetail>` |
| `recognize(Object, List<String> kinds)` | `Row<List<Entity>, RecognizeDetail>` |
| `relate(Object, List<RelationRule> rules)` | `Row<List<Relation>, RelateDetail>` |
| `annotate(AnnotationQuestions, Object)` | `Row<Annotations, NoDetail>` |
| `filter(String, Iterable<T>)` | `Stream<ItemRow<T, Boolean, NoDetail>>` |
| `rank(String, Iterable<T>, List<String> levels)` | `Stream<ItemRow<T, Long, RankDetail>>` |

Each method has a final options overload with its function's immutable options type. Each record method also has a `…Stream` counterpart over `Iterable<?>`. Options use `builder()` and `defaults()`. Every options builder accepts `evidence(Pointer)` and an optional execution `name(String)`. Decide, choose and filter accept `threshold(Threshold)`. Tag, find, recognize and relate accept only `Threshold.Cut` or a double convenience. Score and rank have no threshold. Rank accepts `top(int)`. The engine validates semantic values when a call opens.

`Threshold.cut(double)` and `Threshold.band(double, double)` construct typed thresholds. `Pointer.of(String)` constructs an evidence pointer. `RelationRule.builder(source, target, relation)` requires the three rule fields and accepts optional `reads(String)` and `either(boolean)`.

`Row<A,D extends Detail>` is sealed. Pattern matching exposes the payload appropriate to each status:

- `Answered<A,D>` has `answer()`, `OptionalDouble probability()` and `Optional<D> detail()`.
- `Unsure<A,D>` has a probability and optional detail.
- `Failed<A,D>` has a `RowError` with `ErrorCode`, engine message and optional HTTP status.

All rows expose typed `function()`, `status()`, input `position()`, optional exact JSON `id()` and optional typed `source()`. Source retains backend, model, question identity and optional execution name. A filter or rank `ItemRow` retains its exact original input object and its row. Filter returns every outcome, including false, unsure and failed. Rank's answer is separate from the original input position. Returned values and collections are immutable snapshots that survive client close. Text spans use UTF-16 offsets.

`ChooseDetail` and `TagDetail` expose label probabilities. `ScoreDetail` and `RankDetail` expose ordered level probabilities. `RecognizeDetail` contains typed piece and kind probabilities. `RelateDetail` contains typed relations. No public answer, detail or error exposes a backend reply or a Jackson type.

`Toolbox.builder().put(name, question).build()` and `Toolbox.parse(json)` create typed toolboxes. Function factories such as `ChooseQuestion.of(text, choices, options)` capture their required lists and options. A toolbox selector such as `choose(name)` returns an optional typed question. Pass that question to the matching client function. Its `named(String)` method returns a copy with an explicit execution name. Selecting an entry never assigns an execution name.

Build `AnnotationQuestions` from eligible typed questions. Its builder excludes rank at compile time. A general toolbox may contain rank and cannot be passed to annotate. `Annotations` provides typed selectors and an immutable map of sealed `AnnotationEntry` values. Each `AnnotationOutcome` is answered, unsure or failed and retains its function's answer and detail types.

Only call-level failures throw `ThinkThenException`. A single-record backend failure returns `Failed`. `Row.raiseForStatus()` explicitly throws for a failed row. Exceptions carry the generated `ErrorCode`, optional HTTP status and cause. Error names, values and families derive from the shared engine registry.

Streams open their call on first traversal and cannot split. Collections snapshot at that first traversal and use the engine’s finite input entry point so host scheduling cannot split their batches. Other iterables supply records as the engine requests input. Exhaustion, traversal failure and close release the call. Use try-with-resources for short-circuit traversal. Closing a stream cancels an active call. Filter and rank retain original objects until their rows arrive; top-ranking may retain all consumed originals until ranked output begins. Closing clears those references. A cleaner supplies fallback reclamation.

Interrupting a blocking caller requests cancellation, preserves its interrupt flag and throws `CancellationException`. Native blocking work runs on library-owned platform threads so virtual-thread callers can park. Client close cancels active operations and refuses later calls. Native resources remain owned until active operations finish.

`client.async()` supplies `CompletableFuture` counterparts for single-record and typed-question calls. Both future cancellation forms cancel underlying work. Cancelling before execution starts prevents a native call. The async view shares client ownership. Java streams provide the streaming API.

Diagnostic `System.Logger` messages contain lifecycle events and stable error codes. Applications configure logging. The internal JVM driver is shared with Kotlin and is exported only to the Kotlin module.
