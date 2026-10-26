# ThinkThen in Kotlin

Record methods suspend and return one row; `Stream` methods and filter/rank return Flows. A List passed as a single record remains one JSON array record. Flow completion or cancellation closes its operation; close clients with `use`.

Check `META-INF/README.md` inside the JAR for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Use Kotlin/JVM with Java 22 or later. Add the supplied `thinkthen-kotlin-0.2.0.jar`, matching Java engine JAR and supplied runtime dependencies to your application's dependencies. In Gradle Kotlin DSL, a local package folder can use `implementation(fileTree("packages") { include("**/*.jar") })`. The package requires kotlinx.coroutines and kotlinx.serialization at its packaged versions. Enable native access with `--enable-native-access=ALL-UNNAMED`. The matching Java dependency contains the native engine.

Set `THINKTHEN_API_KEY` through the environment before creating a normal client. Keep keys out of source and reports. `ThinkThen { }` selects the default backend. `Backend.Stub` needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```kotlin
import io.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.flow.collect
import kotlinx.serialization.json.Json

fun main() = runBlocking {
    val records = File("records.jsonl").readLines().filter { it.isNotBlank() }
        .map { Json.parseToJsonElement(it) }
    ThinkThen { backend = Backend.Stub }.use { client ->
        val array = client.decide("Does the customer request a refund?", listOf("first", "second"))
        println("array ${array.status}")
        client.decideStream("Does the customer request a refund?", records, evidence = Pointer("/text"))
            .collect { row ->
                val message = if (row is Row.Failed) row.error.message else ""
                val answer = if (row is Row.Answered) row.answer else null
                println("${row.position} ${row.status} $answer $message")
            }
    }
}
```

Save that example as Example.kt. With the supplied Kotlin JAR and its dependencies under `packages/`, compile for JVM 22 and run with the matching Java runtime:

```sh
CP=$(printf '%s:' packages/*.jar packages/dependencies/*.jar)
kotlinc -jvm-target 22 -classpath "$CP" Example.kt -d example.jar
java --enable-native-access=ALL-UNNAMED -classpath "example.jar:$CP" ExampleKt
```

The JVM target must match the library's JVM 22 bytecode; the compiler's default target is too old for its inline functions.

## Records and the ten functions

Strings remain raw text. Serializable Kotlin values or JsonElement values supply structured records; null and JsonNull become missing-record rows. Custom values use the configured Json serializers or an explicit KSerializer overload. Structured records can carry exact string or integer IDs. `evidence = Pointer("/text")` selects a field; pointers escape `/` as `~1` and `~` as `~0`.

| Call on an open client | Answer type |
| --- | --- |
| `decide(question, record)` | Boolean |
| `choose(question, record, choices = listOf("billing", "shipping", "other"))` | String |
| `tag(question, record, labels = listOf("billing", "shipping"))` | List of strings |
| `score(question, record, levels = listOf("routine", "urgent"))` | Double from 0 to 1 |
| `filter(question, records)` | Flow of Filtered original items and rows |
| `rank(question, records, levels = listOf("routine", "urgent"))` | Flow of Ranked original items and Long ranks |
| `find(question, record)` | FindAnswer.Match or FindAnswer.NoMatch |
| `recognize(record, kinds = listOf("person", "organization"))` | List of Entity |
| `relate(record, rules = rules)` | List of Relation |
| `annotate(questions, record)` | Annotations |

Use literal wording such as `"Does the customer request a refund?"`. Recognize and relate take domains without prompts. Construct rules with `RelationRule(name = "works_for", source = "person", target = "organization")`; optional `reads` supplies wording and `either` makes the relation undirected.

Decide, choose and filter accept `Threshold.cut(0.8)` or `Threshold.band(0.2, 0.8)`. Tag, find, recognize and relate accept cuts only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Score and rank levels run lowest to highest. Rank reads the collection, puts highest scores first and retains failures when `top` limits successful ranks.

A rank row's answer is its rank; `position` is its input order. Filter emits every outcome with its original `item`, `row` and `kept`; `keptItems()` projects only answered true items. Preserve original IDs or input positions when saving results.

## Rows, flows and questions

Rows are sealed `Row.Answered`, `Row.Unsure` and `Row.Failed` cases. Answered rows have `answer`, optional `probability` and typed `detail`. Failed rows have `RowError`. All rows retain `function`, `position`, `id`, `source` and `status`. RecordId preserves text, signed and unsigned integer variants. Source identifies the question, backend and served model. Detail carries label and level probabilities, recognition pieces and relation pairs. Find's NoMatch is an answered outcome. Spans use UTF-16 offsets from zero with an exclusive end. Find selects a whole nonblank line; keep line breaks.

Single calls suspend. Their `…Stream` counterparts accept an Iterable or Flow and return a cold Flow. Filter and rank already take collections. Collect each Flow once: collecting again executes another call. Collections supply finite input and preserve batching. Avoid one call per record. `ThinkThen { batch = 16 }` sets packing; byte and backend limits also apply. One record can require several questions.

```kotlin
client.decideStream(
    "Does the customer request a refund?", records,
    evidence = Pointer("/text")
).collect { row ->
    if (row is Row.Failed) error("${row.error.code}: ${row.error.message}")
    println(row.position)
}
```

Typed classes such as `Decide`, `Choose` and `Recognize` store wording, domains and options. Use `client.ask(question, record)` or `askStream(question, records)` for single-question types. Filter and rank use their own methods. `Toolbox(mapOf("refund" to Decide("Does the customer request a refund?")))` stores named entries; `toolbox.decide("refund")` selects one. `Toolbox.fromJson(text)` loads the saved `entries` format, and `toJson()` saves it. `AnnotationQuestions(mapOf(...))` excludes rank and annotate. An answered annotation can hold unsure or failed children; inspect each entry's EntryOutcome.

Use Kotlin/Java file and CSV or spreadsheet libraries for IO. There is no shell toolbox-run API. The public result row is not annotated with a general JSON serializer; export the fields your application needs rather than assuming `encodeToString(row)` works. Preserve exact IDs and nested failures in the saved result.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```kotlin
val box = Toolbox.fromJson(File("toolbox.json").readText())
File("toolbox-copy.json").writeText(box.toJson())
val topic = requireNotNull(box.choose("topic"))
ThinkThen { backend = Backend.Stub }.use { client ->
    client.askStream(topic, records).collect { row ->
        if (row is Row.Failed) error("${row.error.code}: ${row.error.message}")
        println(row)
    }
}
```

Use this code inside the coroutine from the first example. `box["topic"]` returns a nullable general question; typed selectors such as `box.filter("ready")` and `box.choose("topic")` return nullable typed questions. Check absence before the call. Collect each returned Flow once.

Call `relate` or `relateStream` alone when you need names and relations from the same evidence. On `Row.Answered`, unwrap its nullable detail with `val detail = requireNotNull(row.detail)`. `detail.names` contains all admitted names, including names with no returned relation, and `detail.pairs` contains scored relation pairs. Read accepted relations from `row.answer`. Do not run recognize first for that task.

## Failures and cancellation

Record and backend failures return Row.Failed, including single calls. They do not throw automatically. Test for failure before saving or chaining; throw or end collection if the workload requires stopping. Inspect annotation children independently.

Call-level errors throw ThinkThenException with a stable code, message and optional HTTP status. Cancelling the collecting coroutine cancels native work and raises CancellationException. Flow short-circuiting closes its call. `use` closes the client and cancels active collectors.

Make questions unambiguous. Define the evidence criterion, distinguish an unstated fact from a stated negative and use distinct choices. The stub checks installation, not question quality.
