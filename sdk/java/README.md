# ThinkThen for Java

ThinkThen gives Java programs ten functions that judge evidence by meaning. Java 22 or later is required. Add `io.github.botassembly:thinkthen-java` to a Maven or Gradle application. The JAR includes its native engine. Run a classpath application with `--enable-native-access=ALL-UNNAMED`.

```java
import io.thinkthen.Client;
import io.thinkthen.Row;

try (var client = Client.builder().adapter("stub").build()) {
    Row<Boolean> row = client.decide(
        "Does the customer ask for money back?",
        "Please refund my order.");
    if (row.status() == Row.Status.UNSURE) {
        System.out.println("Please review this message.");
    } else if (Boolean.TRUE.equals(row.answer())) {
        System.out.println("Send this message to the refunds queue.");
    }
}
```

The stub needs no key or service. For a real backend, omit `adapter("stub")` and set its key in the environment. The engine resolves settings and credentials once when the client opens. The builder accepts the fields in the [client settings contract](../../schemas/client.schema.json), with Java names such as `keySource`. Backend names remain strings. The same client and function API serves both modes; proxy transport is supplied by the engine when available.

## Records and streams

A `String` is raw text, including a string that looks like JSON. A Jackson `JsonNode`, Java map, list, number, boolean, null or serializable object supplies structured JSON. Jackson performs ordinary host serialization; the engine admits and validates the result. Single-record methods treat a list as one JSON record. Methods ending in `Stream`, plus `filter` and `rank`, consume an `Iterable` lazily.

```java
import java.util.List;
import io.thinkthen.CallOptions;

var messages = List.of(
    "Please refund my order.",
    "Where is my delivery?");
try (var client = Client.builder().build();
     var rows = client.decideStream("Does this need review?", messages)) {
    rows.forEach(row -> System.out.println(row.id() + " " + row.answer()));
}

try (var client = Client.builder().build();
     var kept = client.filter("Is a refund requested?", messages)) {
    kept.forEach(System.out::println);
}
```

Streams are sequential `java.util.stream.Stream` values with unsplittable sources; requesting parallel traversal cannot run a call driver concurrently. Close them with try-with-resources, including after short-circuit operations such as `findFirst`. Closing cancels unfinished native work and releases the call. Completing a stream also releases it. A client may open independent calls concurrently. Closing the client prevents new calls; existing calls keep their engine alive. An input iterator's exception propagates unchanged and cancels its call. The library does not own the input `Iterable` or close external files behind it.

The driver sends one record only when the native call requests input. It retains original inputs only while needed to return selected filter records. Native row position and selection metadata select those original objects; Java never repeats a threshold or filter rule. All row order, ranking, failed-row and final-failure behavior comes from the shared call. The library never collects an entire input stream to drive it.

## Questions and results

Use `choose(question, record, choices)`, `tag(question, record, labels)`, `score(question, record, levels)`, `find(question, record)`, `recognize(record, kinds)`, `relate(record, rules)` and `annotate(toolbox, record)`. `rank(question, records)` and `filter(question, records)` accept sequences. The [API guide](API.md) lists answer types and overloads.

```java
import io.thinkthen.Question;
import io.thinkthen.Rule;
import com.fasterxml.jackson.databind.ObjectMapper;

var json = new ObjectMapper();
var toolbox = json.readTree("""
    {"questions":{"refund":{"decide":"Is a refund requested?"}}}
    """);
var refund = Question.toolbox(toolbox, "refund");
try (var client = Client.builder().build()) {
    var selected = client.decide(refund, "Please refund my order.", CallOptions.defaults());
    var names = client.recognize("Ada works for Acme.", List.of("person", "company"));
    var edges = client.relate("Ada works for Acme.",
        List.of(new Rule("works_for", "person", "company", null, null)));
}
```

Strings are literal questions; the library reads no question files. `Question.toolbox` and `Question.reference` construct the tagged forms in the [call contract](../../schemas/request.schema.json). The engine owns their admission. A toolbox-selected call gets its function options from that question. Per-call options carry evidence selection, a threshold string, rank's `top`, and an optional cancellation handle. A threshold string can express a cut or a band using the engine grammar.

`Row<A>` is a hand-written immutable value with `id()`, `status()`, `answer()`, `p()`, `error()` and `detail()`. Nullable fields represent absent values or the contract's null answer. Answers use Java booleans, strings, numbers, immutable lists and small records for spans, names and relations. Annotate holds a map of named JSON outcomes. `id` and `detail` are ordinary Jackson JSON trees returned as defensive deep copies. Annotation JSON values receive the same treatment. Mutating a returned tree never changes a row or a later accessor result. The row owns its snapshots; `detail()` decodes its copied raw JSON only on first access and keeps the cached tree private. Rows remain usable after their call or client closes.

Text crosses as counted UTF-16; the shared engine rejects unpaired surrogates with `usage.record` before any backend request. Text spans use UTF-16 indices, so `text.substring(start, end)` extracts them. The engine converts offsets in answers and detail; Java adds no offset calculation. The [shared row schema](../../schemas/row.schema.json) owns each answer's meaning and probability fields.

## Errors and cancellation

`ThinkThenException` extends `RuntimeException` and exposes `code()` and the engine message through `getMessage()`. Single calls throw their native final error. Streams preserve failed rows; native completion can throw a final error after delivered values. An unsure row is an ordinary result. Serialization failures retain Jackson's cause; an iterator's own exception remains its original exception. Loader failures remain Java linkage or I/O failures and do not invent engine codes.

```java
import io.thinkthen.Cancel;
import io.thinkthen.ThinkThenException;

var cancel = new Cancel();
var options = CallOptions.defaults().withCancel(cancel);
try (var client = Client.builder().build()) {
    // Another thread may call cancel.cancel() while this method is blocked.
    var row = client.decide(Question.inline("Does this need review?"),
        "Please review my order.", options);
} catch (ThinkThenException error) {
    System.err.println(error.code() + ": " + error.getMessage());
}
```

`Cancel.cancel()` requests native cancellation from any thread. The handle stays cancelled once fired. It owns no process-wide signal handler. Native cancellation errors keep their native code. Closing a stream cancels its call. Cancellation may run concurrently with a blocked native pull. The handle attaches to each call under the same guard that detaches it before freeing; cancellation before attachment is applied as soon as the call opens. Cleanup waits for active host operations, including cancel, to leave before releasing the call without joining engine work. The packaged native library stays loaded for the process lifetime. Java blocking calls belong on an application's worker thread; Kotlin callers use the suspending layer.

Packaging and dependency choices are in [PACKAGING.md](PACKAGING.md).
