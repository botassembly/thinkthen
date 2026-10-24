# Java public API proposal

This is an API outline, not source declarations. Package names are `io.thinkthen` for Java and `io.thinkthen.kotlin` for the Kotlin facade. Inline methods offer a default-options overload. Tagged question overloads take `CallOptions` explicitly. A public client builder admits settings once; calls carry no backend settings.

```java
Client.builder()
    .mode(String).adapter(String).url(String).keySource(String)
    .model(String).batch(Number).build();
Client implements AutoCloseable;

Question.inline(String text);
Question.toolbox(JsonNode value, String name); // name may be null for one question
Question.reference(String name);

CallOptions.defaults();
CallOptions.withOn(String pointer);
CallOptions.withThreshold(String threshold);
CallOptions.withTop(int top);
CallOptions.withLevels(List<String> levels);
CallOptions.withCancel(Cancel cancel);
Cancel.cancel();
```

`CallOptions` is immutable; its `with` methods return a new value. They translate values without admitting them. The builder sends only explicitly set client fields and delegates defaults to the engine. Question factories construct tagged JSON without validating its semantics. There is no public generic function-name dispatcher.

The ten named methods below belong to `Client`. `record` is an `Object` serialized as described in the README. The normal string forms below have overloads taking `Question` plus `CallOptions`. Those overloads also support named toolbox questions for every function, with no repeated choices, labels, levels, kinds or rules. String forms that need choices, labels or levels also offer a final `CallOptions` argument. Every single-record method has a corresponding `…Stream` method over `Iterable<?>`, with the same question arguments and row answer type.

| Method | Result |
| --- | --- |
| `decide(String question, Object record)` | `Row<Boolean>` |
| `choose(String question, Object record, List<String> choices)` | `Row<String>` |
| `tag(String question, Object record, List<String> labels)` | `Row<List<String>>` |
| `score(String question, Object record, List<String> levels)` | `Row<Double>` |
| `find(String question, Object record)` | `Row<Span>` |
| `recognize(Object record, List<String> kinds)` | `Row<List<Name>>` |
| `relate(Object record, List<Rule> rules)` | `Row<List<Relation>>` |
| `annotate(JsonNode toolbox, Object record)` | `Row<Map<String, JsonNode>>` |
| `<T> filter(String question, Iterable<T> records)` | `Stream<T>` |
| `rank(String question, Iterable<?> records)` | `Stream<Row<Long>>` |

Filter preserves each original Java object. Rank's `Question` overload accepts a selected toolbox question. Call options `top` and `levels` pass through unchanged. `withLevels` supports inline score ranking. Java consumes native kept/position metadata, failed rows and terminal errors; it does not derive them from answers or thresholds.

```java
final class Row<A> {
    JsonNode id();
    Status status(); // ANSWERED, UNSURE, FAILED
    A answer();     // nullable
    Double p();     // nullable
    ErrorInfo error(); // nullable
    JsonNode detail(); // nullable, lazy
}
record ErrorInfo(String code, String message) {}
record Span(String text, int start, int end) {}
record Name(String text, int start, int end, String kind, double p) {}
record RelationName(String text, int start, int end, String kind) {}
record Relation(RelationName source, RelationName target,
                String relation, double p, Boolean either) {}
record Rule(String name, String source, String target, String reads, Boolean either) {}
ThinkThenException.code(); // preserves the library's dotted code
```

Collection answers are unmodifiable snapshots. JSON leaves remain ordinary Jackson trees, but no accessor exposes a privately owned tree. The constructor snapshots JSON inputs; `id()` and `detail()` return defensive deep copies, and annotation `answer()` returns an unmodifiable map of deep-copied JSON values. Lazy detail initialization is thread-safe and keeps its decoded cache private. `Row` copies raw detail bytes and releases native buffers promptly; it decodes detail on demand. Nullable return values carry JSpecify annotations so Kotlin sees their nullability without a second row hierarchy. No surface type validates an engine rule.

A single private native driver owns client/call/buffer lifetimes and the input-request/pull loop. It reads the shipped [C header](../../crates/thinkthen-ffi/include/thinkthen.h), not an alternate interface. It sends strings as counted UTF-16 and structured records as JSON bytes, selects the native UTF-16 offset unit, copies row bytes, and releases each native owner once. No foreign callback runs on an engine thread. Only cancel may run concurrently with the call driver. A lifetime guard covers handle attachment, cancellation and detachment, including cancellation before call creation. It prevents every native operation from using a freed handle. A separate client guard serializes client close against native call-open; closing rejects later opens with Java `IllegalStateException` without inventing an engine error code. Calls already opened retain the engine independently. Operation-local FFM arenas release temporary arguments and output slots promptly. Call pointers accessed by cancellation or coroutine workers use a scope accessible from those threads; a confined arena never crosses workers. Only the symbol lookup has process lifetime.

The reviewable behavior proof will install the built Java and Kotlin JARs into an isolated consumer. One small driver runs the shared stub cases through each language's public calls. Host cases cover nullable answers, mutation isolation of JSON accessors, retained rows with lazy detail, Java stream closure and original filter objects, cancellation before handle attachment and concurrent client close/open, Kotlin suspension/Flow cancellation, native coded admission and backend errors, and a held loopback cancellation whose request must close. Tests use no credentials or external service. Core semantics are not duplicated in language tests.
