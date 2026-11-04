# Java API

The `dev.thinkthen` module requires Java 22 or later. Java applications use `ThinkThen` and close it with try-with-resources. Module-path applications enable native access with `--enable-native-access=dev.thinkthen`; classpath applications use `--enable-native-access=ALL-UNNAMED`.

`ThinkThen.builder()` accepts `backend(Backend)`, `model(String)`, `url(URI)`, `apiKey(String)`, `batch(int)`, `retry(RetryPolicy)` and `recordCodec(RecordCodec)`. The engine admits settings and resolves omitted defaults. When the builder has no explicit backend, `THINKTHEN_BACKEND` selects it; an explicit `backend(...)` takes precedence. `Backend` has `SYSTEMONE`, `OPENAI` and `STUB`. `RetryPolicy.builder()` accepts `maxRetries(int)`, `initialBackoff(Duration)` and `attemptTimeout(Duration)`. Credentials have no getters and never appear in representations or diagnostic logs.

A record may be a text `String`, an immutable `JsonValue`, a string-keyed map, a boxed JSON scalar or null. Single-record calls refuse collections, iterators, streams, arrays and unknown host objects without a codec with `IllegalArgumentException`; pass collections to a stream call. Other objects use the optional `RecordCodec`. A string remains text even when it contains JSON. Null and `JsonNull` produce failed missing-record rows. A caller's iterator or codec exception propagates unchanged. Malformed JSON from a codec produces a failed record.

| Method | Result |
| --- | --- |
| `decide(String, Object)` | `Row<Boolean, NoDetail>` |
| `choose(String, Object, List<String> choices)` | `Row<String, LabelDetail>` |
| `tag(String, Object, List<String> labels)` | `Row<List<String>, LabelDetail>` |
| `score(String, Object, List<String> levels)` | `Row<Double, LevelDetail>` |
| `find(String, Object)` | `Row<Optional<Span>, NoDetail>` |
| `recognize(Object, List<String> kinds)` | `Row<List<Entity>, RecognizeDetail>` |
| `relate(Object, List<RelationRule> rules)` | `Row<List<Relation>, RelationDetail>` |
| `annotate(AnnotationQuestions, Object)` | `Row<Annotations, NoDetail>` |
| `filter(String, Iterable<T>)` | `Stream<T>` of kept original items |
| `filterRows(String, Iterable<T>)` | `Stream<Filtered<T>>` |
| `rank(String, Iterable<?>, List<String> levels)` | `Stream<Ranked<JsonValue>>` |
| `annotateStream(AnnotationQuestions, Iterable<T>)` | `Stream<Annotated<T>>` |

Each method has a final options overload with its function's immutable options type. Each record method also has a `…Stream` counterpart over `Iterable` or `Stream`. Annotate streams retain each original record as `Annotated<T>.item()` alongside its annotation `row()`. Options use `builder()` and `defaults()`. Every options builder accepts `field(Pointer)` and an optional execution `name(String)`. Decide, choose and filter accept `threshold(Threshold)`. Tag, find, recognize and relate accept only `Threshold.Cut` or a double convenience. Score and rank have no threshold. Rank accepts `top(int)`. Decide and filter also take `yesMeans(String)` and `noMeans(String)`. Choose, tag, score and rank take `descriptions(Map<String,String>)`. The engine validates semantic values when a call opens.

`Threshold.cut(double)` and `Threshold.band(double, double)` construct typed thresholds. `Pointer.of(String)` constructs a field pointer. `RelationRule.of(source, target, relation)` creates a basic rule. `RelationRule.builder(source, target, relation)` requires the three rule fields and accepts optional `reads(String)` and `either(boolean)`.

`Row<A,D extends Detail>` is sealed, with `Answered`, `Unsure` and `Failed` cases. Every row exposes `answer()`, `OptionalDouble probability()`, `Optional<D> detail()`, `isAnswered()`, `isUnsure()`, `isFailed()` and `throwIfFailed()`.

`answer()` returns the typed plain answer. It throws `UnsureException` on an unsure outcome and `ThinkThenException` on a failed outcome. `UnsureException` extends `NoSuchElementException`, carries the probability, and is separate from engine failures. `detail()` returns the optional detail on answered or unsure rows and throws the original engine error on failed rows. A failed row's probability is empty. Decide and filter probabilities mean probability of yes, including when the answer is no.

All rows expose typed `function()`, `status()`, signed 64-bit input `position()`, optional exact JSON `id()` and optional typed `source()`. Source retains backend, model, question identity and optional execution name. `Filtered<T>` and `Annotated<T>` retain the exact original `item()` and its `row()`. `Ranked<JsonValue>` carries an owned snapshot: text is a `JsonString`, JSON keeps its JSON type, and missing or unrepresentable input is `JsonNull`. These types expose direct answer, probability, status and position access.

`filter` returns only kept original items. It drops unsure records, delivers all kept items, then throws one `ThinkThenException` with the first failed row's code and all failed rows in `failures()`. `filterRows` returns every outcome. Rank's answer is its rank, separate from input position. Rank retains owned JsonValue snapshots, including when top-ranking changes output order.

Returned values and collections are immutable snapshots that survive client close. Label maps and annotation entries preserve caller order. `LabelDetail` exposes label probabilities; `LevelDetail` exposes level probabilities in level order; `RecognizeDetail` contains typed piece and kind probabilities; `RelationDetail` contains names and scored pairs. No public answer, detail or error exposes a backend reply or a Jackson type. `Span` and `KindDetail` use signed 64-bit, zero-based UTF-16 offsets with an exclusive end and retain matched text.

`Toolbox.builder().put(name, question).build()` and `Toolbox.fromDefinitions(json)` create typed toolboxes. Function factories such as `ChooseQuestion.of(text, choices, options)` capture their required lists and options. `Toolbox.load(Path)` and `Toolbox.fromJson(String)` read saved entries. A toolbox selector such as `choose(name)` returns an optional typed question. Pass that question to the matching client function. Its `named(String)` method returns a copy with an explicit execution name. Selecting an entry never assigns an execution name.

Build `AnnotationQuestions` from eligible typed questions. Its builder excludes rank at compile time. A general toolbox may contain rank and cannot be passed to annotate. `Annotations` provides typed selectors and an immutable map of sealed `AnnotationEntry` values. Each `AnnotationOutcome` is answered, unsure or failed and retains its function's answer and detail types. It exposes the same answer, probability, detail and status methods as a row. Named selectors such as `annotations.decide(name)` return the outcome directly and throw `ThinkThenException` with code `usage.argument` when the name is absent or belongs to another function; its message lists the available names.

Call-level failures throw `ThinkThenException`. Reading a failed `answer()` or `detail()`, calling `throwIfFailed()`, or finishing a kept-only filter with failed rows also throws it. A single-record backend failure returns `Failed`. `Row.throwIfFailed()` explicitly throws for a failed row. Exceptions carry the generated `ErrorCode`, optional HTTP status and cause. Error names, values and families derive from the shared engine registry.

Streams open their call on first traversal and cannot split. Collections for functions other than rank snapshot at that first traversal and use the engine’s finite input entry point so host scheduling cannot split their batches. Rank pulls both collections and lazy inputs as the engine requests records. Other iterables supply records as the engine requests input. Exhaustion, traversal failure and close release the call. Use try-with-resources for short-circuit traversal. Closing a stream cancels an active call. Filter retains original objects until their rows arrive. Rank retains input snapshots in the engine, bounded by top when selected, and never keeps a host map of consumed inputs. Closing clears retained references. A cleaner supplies fallback reclamation.

Interrupting a blocking caller requests cancellation, preserves its interrupt flag and throws `CancellationException`. Native blocking work runs on library-owned platform threads so virtual-thread callers can park. Client close cancels active operations and refuses later calls. Native resources remain owned until active operations finish.

`client.async()` supplies `CompletableFuture` counterparts for single-record and typed-question calls. Both future cancellation forms cancel underlying work. Cancelling before execution starts prevents a native call. The async view shares client ownership. Java streams provide the streaming API.

Diagnostic `System.Logger` messages contain lifecycle events and stable error codes. Applications configure logging. The internal JVM driver is shared with Kotlin and is exported only to the Kotlin module.
