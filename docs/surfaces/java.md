# ThinkThen in Java

Record methods return one row; their `Stream` counterparts and filter/rank return streams. A List passed to a record method is one JSON array record. Close streams and clients with try-with-resources.

Check `META-INF/README.md` inside the JAR for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Use Java 22 or later. Put the supplied `thinkthen-java-0.2.0.jar` and its supplied runtime dependencies on your classpath. The JAR includes the native engine. Compile with `javac -cp '/path/to/packages/*:/path/to/packages/dependencies/*' Example.java` and run with `java --enable-native-access=ALL-UNNAMED -cp '.:/path/to/packages/*:/path/to/packages/dependencies/*' Example`. Windows uses `;` between classpath entries. A module-path application enables native access for `io.thinkthen`.

Set `THINKTHEN_API_KEY` in the environment before creating a normal client. Keep keys out of source and reports. `ThinkThen.builder().build()` selects the default backend; `Backend.STUB` needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```java
import io.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;

public class Example {
    public static void main(String[] args) throws Exception {
        var records = Files.readAllLines(Path.of("records.jsonl")).stream()
            .filter(line -> !line.isBlank()).map(JsonValue::parse).toList();
        var options = DecideOptions.builder().evidence(Pointer.of("/text")).build();
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
             var rows = client.decideStream("Does the customer request a refund?", records, options)) {
            var array = client.decide("Does the customer request a refund?", JsonValue.parse("[\"first\",\"second\"]"));
            System.out.println("array " + array.status());
            rows.forEach(row -> System.out.println(row.position() + " " + row.status() + " "
                + (row instanceof Answered<Boolean, NoDetail> answered ? answered.answer() : null) + " "
                + (row instanceof Failed<Boolean, NoDetail> failed ? failed.error().message() : "")));
        }
    }
}
```

## Records and the ten functions

A record may be String text, `JsonValue`, a string-keyed Map, List, boxed JSON scalar or null. Other objects need a `RecordCodec` in the client builder. Strings stay text even if they contain JSON. Structured records can carry string or exact integer `id` values. Function options use `evidence(Pointer.of("/text"))` to select a field. Escape `/` as `~1` and `~` as `~0` in pointers.

| Call on an open client | Answer type |
| --- | --- |
| `decide(question, record)` | Boolean |
| `choose(question, record, List.of("billing", "shipping", "other"))` | String |
| `tag(question, record, List.of("billing", "shipping"))` | List of strings |
| `score(question, record, List.of("routine", "urgent"))` | Double from 0 to 1 |
| `filter(question, records)` | Stream of original items and Boolean rows |
| `rank(question, records, List.of("routine", "urgent"))` | Stream of original items and Long ranks |
| `find(question, record)` | `Optional<Span>` |
| `recognize(record, List.of("person", "organization"))` | List of Entity |
| `relate(record, rules)` | List of Relation |
| `annotate(questions, record)` | Annotations |

Questions are literal wording such as `"Does the customer request a refund?"`. Recognize and relate take domains without an invented prompt. Build relation rules with `RelationRule.builder("person", "organization", "works_for").build()`. Optional `reads` supplies wording and `either` makes a relation undirected.

Each call accepts a final function-specific options object. For example:

```java
var options = DecideOptions.builder()
    .evidence(Pointer.of("/text"))
    .threshold(Threshold.band(0.2, 0.8)).build();
var row = client.decide("Does the customer request a refund?",
    java.util.Map.of("id", "a1", "text", "Please refund my order."), options);
row.raiseForStatus();
```

Decide, choose and filter accept `Threshold.cut()` or `Threshold.band()`. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Tag, find, recognize and relate take numeric cuts only. Score and rank levels run from lowest to highest. Rank reads the collection and places highest scores first; `RankOptions.top` limits successful ranks and retains failures.

A rank row's answer is its rank; `position()` is its input order. Filter returns every outcome, including false, unsure and failed. `ItemRow` preserves the exact original object and its typed row. Preserve IDs or input positions when saving results.

## Rows, streams and saved questions

`Row<A,D>` has sealed `Answered`, `Unsure` and `Failed` cases. Pattern match before reading answers. Answered rows expose `answer()`, optional probability and optional typed detail. Failed rows expose `RowError` with stable code, library message and optional HTTP status. Every row retains status, function, input position, exact JSON ID and optional source. Source identifies the question, backend and served model. Details include label and level probabilities, recognition pieces and relation pairs. Find can be answered with `Optional.empty()`. Spans use UTF-16 offsets from zero with an exclusive end. Find selects a whole nonblank line, so preserve line breaks.

Single-record methods have `…Stream` counterparts over `Iterable<?>`. Filter and rank already take collections. Streams open on traversal and close on exhaustion or error. Use try-with-resources for early exit:

```java
try (var rows = client.decideStream("Does the customer request a refund?", records, options)) {
    rows.forEach(row -> {
        row.raiseForStatus();
        System.out.println(row.position());
    });
}
```

Pass a collection as one stream to batch it. `builder().batch(16)` controls packing; byte and backend limits also apply. One record can require several backend questions. The async view, `client.async()`, supplies CompletableFuture single calls. Cancelling a future cancels native work.

`Toolbox.fromJson(json)` loads the saved `entries` format, and `toJson()` saves it. `Toolbox.builder().put(name, question).build()` constructs one. Factories such as `DecideQuestion.of(text, options)` create typed questions; matching toolbox selectors return Optional typed questions. Selecting a toolbox entry does not assign an execution name. Pass the selected question to the matching method. Build `AnnotationQuestions` with its builder from eligible typed questions. Rank is excluded. An answered outer annotation can contain failed children; inspect every `AnnotationOutcome` independently.

The SDK has no shell toolbox-run or spreadsheet IO API. Use Java file and table libraries to load records, parse a saved question toolbox, batch the records and write results. There is no public contract-row JSON exporter. Save fields through your application's serializer while preserving IDs and nested statuses.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```java
var box = Toolbox.fromJson(Files.readString(Path.of("toolbox.json")));
Files.writeString(Path.of("toolbox-copy.json"), box.toJson());
var topic = box.choose("topic").orElseThrow();
try (var client = ThinkThen.builder().backend(Backend.STUB).build();
     var rows = client.chooseStream(topic, records)) {
    rows.forEach(row -> {
        row.raiseForStatus();
        System.out.println(row);
    });
}
```

Use this code inside your application's method with the imports and records from the first example. `box.get(name)` returns an Optional general question; `box.filter("ready")` and other typed selectors return Optional typed questions. Close the selected question's stream after traversal.

Call `relate` or `relateStream` alone when you need names and relations from the same evidence. On an answered row, unwrap its optional detail with `var detail = answered.detail().orElseThrow()`. `detail.names()` contains all admitted names, including names with no returned relation, and `detail.pairs()` contains scored relation pairs. Read accepted relations from `answered.answer()`. A preceding recognize call is unnecessary.

## Failures and cancellation

Record and backend failures return `Failed`, including single calls. `row.raiseForStatus()` explicitly throws on a failed row. Use it before saving or chaining and inspect annotation children too. Stop later calls when your task requires stopping after a failure.

Call-level problems throw `ThinkThenException` with typed code, message and optional HTTP status. Input iterator and codec exceptions propagate. Interrupting a blocking caller cancels work, preserves its interrupt flag and throws `CancellationException`. Closing a stream cancels active work; closing the client cancels active calls and rejects new ones.

Make questions unambiguous. Define the evidence criterion, distinguish absence from explicit negatives and use distinct choices. The stub checks installation, not judgment quality.
