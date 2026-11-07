# Kotlin functions that answer questions about text

ThinkThen 0.2.0 for Kotlin gives you ten functions. Each one reads a record, answers one question about it, and returns a typed value your code can branch on.

## Install and ask your first question

Add the package to Gradle:

```kotlin
dependencies {
    implementation("io.github.botassembly:thinkthen-kotlin:0.2.0")
}
```

Or to Maven:

```xml
<dependency>
  <groupId>io.github.botassembly</groupId>
  <artifactId>thinkthen-kotlin</artifactId>
  <version>0.2.0</version>
</dependency>
```

It needs Kotlin/JVM on Java 22 or later and includes the native engine. Set the Kotlin JVM target to 22, because the calls are inline functions built for Java 22. Run with `--enable-native-access=ALL-UNNAMED`.

```kotlin
import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val isRefund = client.decide(
            "Does the customer request a refund?",
            "Please refund my order.",
        ).answer()
        println(isRefund)
    }
}
```

<!-- output: capture -->

Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

## Handle not sure and failures

`answer()` is the one step from a row to its answer. `getOrThrow()` is the same call, named as Kotlin's `Result` names it. Neither hands back a default:

- When ThinkThen is not sure, it throws `UnsureException`. That is a `NoSuchElementException`, and its `probability` gives the row's probability.
- When the record failed, it throws `ThinkThenException`. Its `code` names the engine error, and `code.wireName` gives the name, such as `record.missing`.

With decide's default cut, every answer is yes or no. A band, `Threshold.band(low, high)`, makes the middle not sure. Check `isUnsure` first and send those records to a person:

```kotlin
val message = "Please refund my order."
val refundCheck = client.decide(
    "Does the customer request a refund?", message,
    threshold = Threshold.band(0.2, 0.8),
)
when {
    refundCheck.isUnsure -> println("Ask a person: $message")
    refundCheck.answer() -> println("Send to refunds: $message")
    else -> println("Leave in the inbox: $message")
}
```

A failed record still reaches `answer()` and throws there. Use `isAnswered` or `isFailed` to test without throwing, or `when` on the row's type, as [Judge many records](#judge-many-records) shows.

Mistakes in the call itself throw at once. Blank text throws `ThinkThenException` with `record.empty` before any request is sent. A `List`, `Array`, `Sequence` or `Flow` passed to a one-record call does not compile, and the compiler names the `…Stream` call to use instead.

## The ten functions

Each snippet runs inside `ThinkThen().use { client -> ... }` in a coroutine, as in the first example. The many-record functions read the same three support messages:

```kotlin
val messages = listOf(
    mapOf("id" to "a1", "text" to "Please refund my order."),
    mapOf("id" to "a2", "text" to "Where is my parcel?"),
    mapOf("id" to "a3", "text" to "I was charged twice for one invoice."),
)
val text = Pointer("/text")
```

`Pointer("/text")` tells ThinkThen to judge each message's `text` field. Without a pointer, it judges the whole record.

### decide: answer a yes-or-no question

```kotlin
val isRefund = client.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
).answer()
```

### choose: pick the one option that fits

```kotlin
val topic = client.choose(
    "What is the message about?",
    "Please refund my order.",
    listOf("billing", "shipping", "other"),
).answer()
```

### tag: label every option that fits

```kotlin
val topics = client.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    listOf("billing", "shipping"),
).answer()
```

### score: place the text on a scale from 0 to 1

```kotlin
val urgency = client.score(
    "How urgent is the request?",
    "Please refund my order.",
    listOf("routine", "urgent"),
).answer()
```

Levels run from lowest to highest. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```kotlin
client.filter(
    "Does the customer request a refund?", messages,
    field = text,
).collect { refund -> println(refund["id"]) }
```

`filter` returns a `Flow` of your own records, only the ones that answer yes. Records that are not sure are left out. When records fail, the flow delivers every kept record first, then throws one `ThinkThenException`. Its `failures` lists every failed row with its `position`. To see every record's outcome, call `filterRows`, which gives a `Filtered` for each record with its `item`, `kept` and `answer()`.

### rank: order every record, highest first

```kotlin
client.rank(
    "How urgent is the request?", messages,
    listOf("routine", "urgent"),
    field = text,
).collect { ranked ->
    val urgencyRank = ranked.answer()
    println("$urgencyRank ${ranked.item["id"]}")
}
```

rank gives a `Ranked` of your record type. Its `item` is an equal copy of your record, read back through the record's serializer, not the same object, so a long `Flow` with `top` holds only the top records. filter gives back your own objects. A missing or invalid record has no copy, so it keeps your original on its failed row. A type that cannot read back to the same JSON throws `SerializationException` naming the type, before any backend request when the first record shows it. `answer()` is the rank, a `Long` starting at 1. `position` is the record's place in your input. `top = n` keeps the first `n` ranks and still reports failed records.

### find: point to the line that answers

```kotlin
val action = client.find(
    "Which line states the requested action?",
    "Morgan works for Example Group.\nPlease refund my order.\n",
).answer()
println(action?.text ?: "no line answers")
```

The answer is a `Span?`, and `null` means no line answers the question. Find picks one whole line, so keep the line breaks. A `Span` gives the matched `text` and its `start` and `end` as `Long` UTF-16 offsets, with the end excluded.

### annotate: fill in several questions for each record

```kotlin
val questions = AnnotationQuestions(mapOf(
    "refund" to Decide("Does the customer request a refund?"),
    "topic" to Choose("What is the message about?",
        listOf("billing", "shipping", "other")),
    "urgency" to Score("How urgent is the request?",
        listOf("routine", "urgent")),
))
client.annotateStream(questions, messages, field = text)
    .collect { form ->
        val answers = form.answer()
        val isRefund = answers.decide("refund").answer()
        val topic = answers.choose("topic").answer()
        val urgency = answers.score("urgency").answer()
        println("${form.position} $isRefund $topic $urgency")
    }
```

Each question has its own outcome, and its `answer()` throws on not sure or failure, as a row's does. A selector such as `answers.decide(name)` throws `ThinkThenException` with `usage.argument` for a name you never added or a question of another function; `answers.decideOrNull(name)` returns `null` instead. For one record, `client.annotate(questions, message)` returns the row directly. Rank cannot go in `AnnotationQuestions`, and the compiler refuses it.

### recognize: pull out each name and its kind

```kotlin
val names = client.recognize(
    "Morgan works for Example Group.",
    listOf("person", "organization"),
).answer()
for (name in names) {
    println("${name.mention.kind}: ${name.mention.span.text}")
}
```

### relate: link names by the relations you allow

```kotlin
val relations = client.relate(
    "Morgan works for Example Group.\nPlease refund my order.\n",
    listOf(RelationRule("person", "organization", "works_for")),
).answer()
for (relation in relations) {
    println("${relation.source.span.text} ${relation.relation} " +
        relation.target.span.text)
}
```

`RelationRule(source, target, relation)` also takes `reads` to word the relation and `either = true` to make it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge many records

Every one-record function has a `…Stream` twin, such as `decideStream`, that takes an `Iterable` or a `Flow` and returns a cold `Flow` of rows in input order. `filter`, `filterRows` and `rank` take many records already. ThinkThen packs the records into batches; `ThinkThen { batch = 16 }` sets the size.

Each collection of a flow makes its own call, so collect it once. A failed or not-sure record is a row like any other, so a `when` keeps every record without stopping the batch:

```kotlin
client.decideStream(
    "Does the customer request a refund?", messages,
    field = text,
).collect { refundCheck ->
    val position = refundCheck.position
    when (refundCheck) {
        is Row.Answered ->
            println("$position ${refundCheck.answer}")
        is Row.Unsure ->
            println("$position not sure: ask a person")
        is Row.Failed ->
            println("$position failed: ${refundCheck.error.code.wireName}")
    }
}
```

Each row also carries `id`, the record's own `id` as a `RecordId`, and `source`, the backend and model that answered. Inside a flow, a blank record is a failed row with `record.empty`, and the rest continue.

## Say what yes, no and each option mean

Decide and filter take `yesMeans` and `noMeans`. Choose, tag, score and rank take `descriptions`, a map from each option or level to what it means. The answers keep their short names.

```kotlin
val isRefund = client.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
    yesMeans = "The customer asks for money back",
    noMeans = "The customer asks for anything else",
).answer()
val topic = client.choose(
    "What is the message about?",
    "Please refund my order.",
    listOf("billing", "shipping", "other"),
    descriptions = mapOf(
        "billing" to "Invoices, charges and refunds",
        "shipping" to "Delivery and parcels",
        "other" to "Anything else",
    ),
).answer()
```

A description for a name that is not one of the options throws `ThinkThenException` when the question is used, at the call or when you add it to a `Toolbox`.

## Records and field

A record is a `String`, a `JsonElement`, or any value kotlinx.serialization can encode, such as a `Map<String, String>` or a `@Serializable` class. A string stays text even when it holds JSON. Set `json` on the client, `ThinkThen { json = Json { serializersModule = ... } }`, to use your own serializers, or pass `serializer =` to one call.

A one-record call takes exactly one record. A `JsonArray` is one record. A `null` record, or a record without the field the pointer selects, gives a failed row with `record.missing` and sends no request. In a pointer, write `~1` for a `/` in a key and `~0` for a `~`. A record's `id` comes back exactly on its row, as text or as a whole number.

## Read the probability and details

`probability` is a `Double?`, and a failed row has none. For decide and filter it is the probability of yes, even when the answer is no. For the other functions it is the probability of the answer given.

An answered or not-sure row also has `detail`. Choose and tag give a `LabelDetail` with each label's probability in `labels`. Score and rank give a `LevelDetail` with each level's probability in `levels`, in level order. Recognize gives a `RecognizeDetail`, and relate gives a `RelationDetail` with every name it found in `names` and every scored pair in `pairs`.

## Save and load a toolbox

A toolbox is a file of named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Load it, pick a question by name and type, and pass that question to its function:

```kotlin
val toolbox = Toolbox.load(java.nio.file.Path.of("toolbox.json"))
val topicQuestion = toolbox.choose("topic")
client.askStream(topicQuestion, messages).collect { topic ->
    println("${topic.position} ${topic.answer()}")
}
```

`Toolbox.load(path)` and `Toolbox.fromJson(json)` read this saved format, and `toJson()` writes it. `Toolbox.fromDefinitions(json)` reads bare definitions with no `entries` wrapper, such as `{"topic":{"function":"choose", ...}}`. `Toolbox { "topic"(Choose(...)) }` makes one in code. The typed selectors, such as `toolbox.choose(name)`, throw `ThinkThenException` with `usage.argument` for a missing name or a different function; their `…OrNull` forms, such as `toolbox.chooseOrNull(name)`, return `null` instead. `client.ask(question, record)` asks one record, `client.filter(toolbox.filter("refund"), messages)` filters, and `toolbox.forAnnotation()` gives every question to `annotate`, refusing a toolbox that holds a rank question.

## Settings, cancellation and closing

`ThinkThen { }` takes `backend`, `model`, `url`, `apiKey`, `batch`, `retry` and `json`. With no `backend` set, the client reads `THINKTHEN_BACKEND`, as the command does, and then uses the default backend. With no `apiKey` set, it reads the backend's key variable, such as `THINKTHEN_API_KEY`. An `ApiKey` never appears in `toString`.

Cancelling the coroutine that collects a flow cancels the native work and raises `CancellationException`. Stopping a flow early, with `first()` or `take(n)`, closes its call. `use` closes the client and cancels its active collections, and a later call throws `ThinkThenException` with `usage.closed`.

## Reference

[Read the API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/kotlin/src/main/kotlin/dev/thinkthen/kotlin). Every public name has KDoc in the published sources JAR, so your editor shows it on hover.
