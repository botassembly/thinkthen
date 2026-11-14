# Kotlin functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use Kotlin on Java 22 or later. Add `io.github.botassembly:thinkthen-kotlin` to Gradle or Maven, and set the Kotlin JVM target to 22. The package includes the engine. Run with `--enable-native-access=ALL-UNNAMED`. Set `THINKTHEN_API_KEY` in the environment first.

A one-record call throws `FailedException` at the call if its record could not be judged. A successful call returns an answered or unsure result, with `isFailed` false. Many-record calls retain failed results, so the shared result type still has three states:

- **answered**: there is an answer, and `result.answer()` is it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer throws when there is none.** On a successfully returned one-record result, `result.answer()` can throw `UnsureException`. On a many-record result, form entry or saved row, it can also throw `FailedException`. A missing answer never reads as "no" or as an empty value.
2. **Checking the state never throws.** `result.isAnswered`, `result.isUnsure`, `result.isFailed` and `result.probability` are always safe to read.
3. **Printing never throws.** `println(result)` shows the answer, `unsure`, or `failed:` with the reason. A yes prints as `true` and a no as `false`, the same words the `thinkthen` command prints.

## decide

### The simplest if

Kotlin has no truthiness, so the `if` reads the answer. On a decide result, `answer()` is `true` or `false`.

```kotlin
import dev.thinkthen.kotlin.*
import java.nio.file.Path
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*

suspend fun main() {
    ThinkThen().use { client ->
        if (client.decide("Does the customer request a refund?", "Please refund my order.").answer()) {
            println("refund queue")
        } else {
            println("inbox")
        }
    }
}
```

If the message could not be judged, `client.decide` throws `FailedException` at the call. The `if` never receives a failed result and never falls through to `inbox` for a failure. With one bar, a returned decide result is yes or no; two bars can make its answer read throw `UnsureException`.

One-record calls are `suspend` functions and run in a coroutine; many-record calls return cold flows. Later statement snippets run inside this `use` block. Put enum declarations and helper functions at file scope. The choose and tag sections show separate `Topic` definitions; annotate uses the three-choice definition from choose.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `isAnswered` tells you whether there is a yes or no to read.

```kotlin
val isRefund = client.decide(
    "Does the customer request a refund?",
    "Where is my parcel?",
    threshold = Threshold.band(0.2, 0.8),
)

if (isRefund.isAnswered) {
    println(if (isRefund.answer()) "refund queue" else "inbox")
} else {
    println("a person reads it")
}
```

You can also let the read throw and catch `UnsureException`. The scalar call has already succeeded, so this read cannot report a failed result.

```kotlin
try {
    println(if (isRefund.answer()) "refund queue" else "inbox")
} catch (unsure: UnsureException) {
    println("a person reads it")
}
```

### Did the call work

```kotlin
try {
    val isRefund = client.decide("Does the customer request a refund?", "Please refund my order.")
    println(isRefund)
} catch (failure: FailedException) {
    println("could not judge it: ${failure.message}")
} catch (failure: ThinkThenException) {
    println("the call did not complete: ${failure.message}")
}
```

`FailedException` reports a failed judgement, including a service failure returned for that record. `ThinkThenException` retains the other engine call errors, such as usage, setup and internal errors; it is not the parent of `FailedException`. Argument mistakes and coroutine cancellation keep their standard exceptions. Printing the successfully returned result does not read its answer.

Not sure counts as a successful call: ThinkThen did its job and returned an unsure result. Its answer read can throw `UnsureException`, which neither catch above handles.

### Both bars and the probability

```kotlin
val isRefund = client.decide(
    "Does the customer request a refund?",
    "Where is my parcel?",
    threshold = Threshold.band(0.2, 0.8),
)
println(isRefund.probability)
```

- No threshold: one bar at 0.5. The answer is always yes or no.
- `Threshold.cut(0.7)`: one bar. Yes at 0.7 or above, otherwise no.
- `Threshold.band(0.2, 0.8)`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. It is a `Double?`, present even for an unsure decide result. A failed many-record result has `null` probability.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `client.decideStream`, save each `probability`, and compare with your labels.

## choose with your own choices

Give your choices as an `enum class`. The answer comes back as a constant of it.

```kotlin
enum class Topic(private val text: String) {
    BILLING("billing"),
    SHIPPING("shipping"),
    OTHER("other");

    override fun toString() = text
}

fun teamFor(topic: Topic): String = when (topic) {
    Topic.BILLING -> "billing team"
    Topic.SHIPPING -> "warehouse"
    Topic.OTHER -> "front desk"
}
```

```kotlin
val topic = client.choose(
    "What is the message about?",
    "I was charged twice for one invoice.",
    Topic::class,
)

if (topic.isUnsure) {
    println("a person reads it")
} else {
    println(teamFor(topic.answer()))
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `topic.isUnsure` before reading the answer. `threshold = Threshold.cut(0.3)` moves the bar.

What each constant prints (`billing`, `shipping`, `other`) is the word the model reads, so make `toString()` give plain words or short phrases, not codes. The constant names are for your code. An enum with no `toString()` of its own sends its constant names as written, such as `BILLING`.

Add a constant to `Topic` and the compiler points at `teamFor` until you handle it.

A choose result does not go in an `if`, and is not equal to its answer. `if (topic)` and `topic == Topic.BILLING` do not compile. `topic.equals(Topic.BILLING)` is always `false`, because a result equals only another result. Compare `topic.answer()`.

Value equality and hashing cover only `function`, `status`, `answer`, `probability`, `error`, `position`, `id`, `source` and `detail`, using this language's names. They never include `item`, including on entries inside a form. Changing the caller's record cannot change a result's equality or hash.

A list of strings also works, `listOf("billing", "shipping", "other")`, and the answer is then a `String`.

## Many records

Each one-record function has a `Stream` form, such as `decideStream`, that takes many records and gives one result per record, in order. Each result carries your own record as `item`.

```kotlin
val messages = listOf(
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
)

val refundChecks = client.decideStream(
    "Does the customer request a refund?", messages,
    threshold = Threshold.band(0.2, 0.8),
)
refundChecks.collect { isRefund ->
    when {
        isRefund.isUnsure -> println("a person reads it: ${isRefund.item}")
        isRefund.isFailed -> println("try again later: ${isRefund.item}")
        isRefund.answer() -> println("refund queue: ${isRefund.item}")
        else -> println("inbox: ${isRefund.item}")
    }
}
```

One failed record does not stop the stream. Each result keeps its own state, with `position` counted from 1 and the record's `id` when it has one. A `decideStream` call stays many-record even with a one-item collection: its failed result is returned.

To stop on an encountered failed result, use the shared result's explicit check:

```kotlin
client.decideStream(
    "Does the customer request a refund?", messages,
    threshold = Threshold.band(0.2, 0.8),
).collect { isRefund -> println(isRefund.throwIfFailed()) }
```

`throwIfFailed()` throws `FailedException` for a failed result and otherwise returns that same result. Not sure does not throw here, and printing is safe. The method remains on the shared result type; it is redundant for a successfully returned scalar result.

Pass the whole collection to one stream so ThinkThen can send the records in batches. The stream is a `Flow`, and the records can be a collection or a `Flow` of their own. Nothing is sent until you collect it, and each collection makes its own call, so collect it once. Stopping early, with `first()` or `take(n)`, closes the call. `.toList()` collects every result.

To save every result, write `result.toJson()` as a line of JSON. It includes the state, so nothing is lost.

## Many-result not sure or failed

Here `isRefund` is a result from the flow, where all three states remain possible.

```kotlin
client.decideStream(
    "Does the customer request a refund?", messages,
    threshold = Threshold.band(0.2, 0.8),
).collect { isRefund ->
    when {
        isRefund.isUnsure -> println("a person reads it ${isRefund.probability}")
        isRefund is Row.Failed -> println("try again later ${isRefund.error.code}")
        isRefund.answer() -> println("refund queue")
        else -> println("inbox")
    }
}
```

Yes and no are the two answers. Not sure and failed are the two ways of having none. A failed result is a `Row.Failed`, and only a `Row.Failed` has an `error`, so there is never an empty error to check for.

A result is one of three types: `Row.Answered`, `Row.Unsure` or `Row.Failed`. To name every state and have the compiler confirm none is missed, use a `when` that returns a value:

```kotlin
fun whereItGoes(isRefund: Row<Boolean, String>): String = when (isRefund) {
    is Row.Answered -> if (isRefund.answer()) "refund queue" else "inbox"
    is Row.Unsure -> "a person reads it"
    is Row.Failed -> "try again later"
}
```

`Row<Boolean, String>` is a yes-or-no result about a `String` record. The same sealed type serves scalar calls and flows. A compiler-checked `when` still needs its `Row.Failed` branch even for a row from a successful scalar call; that branch is unreachable for that scalar result at runtime.

Call the routing function on the actual flow results:

```kotlin
client.decideStream(
    "Does the customer request a refund?", messages,
    threshold = Threshold.band(0.2, 0.8),
).collect { isRefund -> println("${whereItGoes(isRefund)}: ${isRefund.item}") }
```

`NoAnswerException` groups `UnsureException` and `FailedException` for answer reads on many-record results, form entries and saved rows. Catch it when either state means there is no answer to use; catch them separately when they need different routes.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```kotlin
val messages = listOf(
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
)

val refundRequests = client.filter("Does the customer request a refund?", messages)
refundRequests.collect { refundRequest -> println("refund queue: $refundRequest") }
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold = Threshold.cut(0.7)` moves the bar. Two bars are refused: `Threshold.band` does not compile there, and a saved filter question that holds two bars throws `UsageException`. When you want not sure, use `client.decideStream`, as in the section above.

If a record fails, you still get every kept record first. Then filter throws one `FailedException`, and its `rows` holds every failed result with its `item`.

```kotlin
try {
    refundRequests.collect { refundRequest -> println("refund queue: $refundRequest") }
} catch (failure: FailedException) {
    for (failed in failure.rows) {
        println("try again later: ${failed.item} ${failed.error.code}")
    }
}
```

## tag

Labels work as choices do. The answer is a `List` of the constants that fit, and it can be empty.

```kotlin
enum class Topic(private val text: String) {
    BILLING("billing"),
    SHIPPING("shipping");

    override fun toString() = text
}
```

```kotlin
val topics = client.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    Topic::class,
)

if (Topic.BILLING in topics.answer()) {
    println("billing team")
}
for (topic in topics.answer()) {
    println(topic)
}
```

tag is never not sure. `threshold = Threshold.cut(0.7)` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. With an enum type, ThinkThen uses the declaration order of its entries; write `ROUTINE` before `URGENT`.

```kotlin
enum class Urgency(private val text: String) {
    ROUTINE("routine"),
    URGENT("urgent");

    override fun toString() = text
}
```

```kotlin
val urgency = client.score(
    "How urgent is the request?",
    "Please refund my order.",
    Urgency::class,
)

if (urgency.answer() > 0.5) {
    println("answer today")
}
println("%.2f".format(urgency.answer()))
```

The score is a position between your levels, not one of them, so it is a `Double` and there is nothing to match with `when`. To get exactly one level back, ask with `client.choose`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```kotlin
val messages = listOf(
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
)

val byUrgency = client.rank("How urgent is the request?", messages, Urgency::class)
byUrgency.collect { ranked -> println("${ranked.answer()} ${ranked.item}") }
```

`ranked.answer()` is the rank, starting at 1, and `ranked.item` is your record. rank reads the whole collection before it gives anything back. `top = 5` selects the five highest-ranked records after judging the collection. A failed record comes back after the ranked ones, with `isFailed` true.

## find

find points to the one line that answers.

```kotlin
val message = "Morgan works for Example Group.\nPlease refund my order."
val action = client.find("Which line states the requested action?", message)

if (action.isUnsure) {
    println("a person reads it")
} else {
    println(action.answer()?.text ?: "no line answers")
}
```

The answer is a `Span?`. A `Span` has `text`, `start` and `end`, and the answer is `null` when no line answers. `message.substring(span.start, span.end)` is the line. Like choose, find can be not sure with no bar set.

## recognize

```kotlin
val names = client.recognize(
    "Morgan works for Example Group.",
    listOf("person", "organization"),
)

for (name in names.answer()) {
    println("${name.kind} ${name.text}")
}
```

Each name also has `start`, `end` and `probability`. The answer can be empty. recognize is never not sure.

## relate

```kotlin
val message = "Morgan works for Example Group.\nPlease refund my order."
val rules = listOf(RelationRule("person", "organization", "works_for"))

val relations = client.relate(message, rules)

for (relation in relations.answer()) {
    println("${relation.source.text} ${relation.relation} ${relation.target.text}")
}
```

Each relation has `source`, `target`, `relation`, `probability` and `either`. `relations.detail` is a `RelationDetail`, and its `names` holds every name found, including names in no relation. Reach it with `relations.detail as? RelationDetail`.

## annotate

annotate asks several questions of each record and gives back one form per record.

```kotlin
val messages = listOf(
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
)

val questions = AnnotationQuestions(mapOf(
    "refund" to DecideQuestion("Does the customer request a refund?"),
    "topic" to ChooseQuestion("What is the message about?", Topic::class),
    "urgency" to ScoreQuestion("How urgent is the request?", Urgency::class),
))

val forms = client.annotateStream(questions, messages)
forms.collect { form ->
    if (form.isFailed) {
        println("try again later: ${form.item}")
        return@collect
    }
    val entries = form.answer()
    val isRefund = entries.decide("refund")
    val topic = entries.choose("topic", Topic::class)
    val urgency = entries.score("urgency")
    if (isRefund.isFailed || topic.isFailed || urgency.isFailed) {
        println("try again later: ${form.item}")
    } else if (isRefund.isUnsure || topic.isUnsure) {
        println("a person reads it: ${form.item}")
    } else if (isRefund.isAnswered && topic.isAnswered && urgency.isAnswered && isRefund.answer()) {
        println("refund queue: ${topic.answer()} ${"%.2f".format(urgency.answer())}")
    }
}
```

`form.answer()` holds one entry per question, under the name you gave it. Each entry is a result of its own function, with its own state. `entries.decide("refund")` is a decide result, so its `answer()` goes in an `if`. One entry can be not sure or failed while the others answer. Check every entry used by a branch before reading its answer; an answered form alone does not mean its entries all answered. Here `Topic` is the three-choice enum from the choose section.

For one record, `client.annotate(questions, message)` throws `FailedException` only when the parent form itself failed. An answered parent is returned even when one, several or all of its entries failed. The scalar boundary does not run the recursive form check to decide whether to return it.

```kotlin
val form = client.annotate(questions, "Please refund my order.")
form.throwIfFailed()
```

The optional second line explicitly throws for a failed entry as well as a failed parent. An unsure entry does not fail this check; otherwise it returns the same form unchanged. It also works on flow forms. Checking only the parent never authorizes reading an unchecked entry's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```kotlin
val isRefund = client.decide(
    "Does the customer request a refund?",
    "I was charged twice for one invoice.",
    yesMeans = "the customer wants money back, including for a duplicate charge",
    noMeans = "the customer wants anything else",
)

val topic = client.choose(
    "What is the message about?",
    "I was charged twice for one invoice.",
    Topic::class,
    descriptions = mapOf(
        Topic.BILLING to "charges, invoices and refunds",
        Topic.SHIPPING to "parcels and delivery",
    ),
)
```

decide and filter take `yesMeans` and `noMeans`. choose, tag, score and rank take `descriptions`, keyed by your choices, labels or levels.

## Records and fields

A record is a `String`, a `Map`, a `JsonElement` or another value that kotlinx.serialization can write. For one JSON array, use a `JsonArray`; a Kotlin `List` belongs in the `Stream` form. `field = Pointer("/text")` picks one field of a `Map`. Without it, ThinkThen judges the whole record. A record of your own class needs `@Serializable`. For a class you can't mark, set `json` once on the client with your own serializers.

Give `Map` records an `id`. Each result carries it back as `id`.

A one-record call given a `List`, an array, a `Sequence` or a `Flow` does not compile, and the compiler names the `Stream` form. One that reaches the call typed as `Any` throws `IllegalArgumentException`, which names it too. A blank record (an empty string, only spaces, or an empty `Map`) throws `FailedException` at the scalar call with code `record.empty`; a `null` record does so with `record.missing`. Neither sends that input to the service. The same inputs remain failed results in the `Stream` forms, even with one input. Collection length never chooses the scalar rule.

## Async

Every call on this page is already asynchronous. A one-record call suspends instead of blocking a thread, and a many-record call is a `Flow`. So there is no separate async client, and the results are the same. To ask two questions at once, start each with `async`.

```kotlin
import dev.thinkthen.kotlin.*
import kotlinx.coroutines.*

suspend fun main() {
    ThinkThen().use { client ->
        coroutineScope {
            val isRefund = async { client.decide("Does the customer request a refund?", "Please refund my order.") }
            val names = async { client.recognize("Morgan works for Example Group.", listOf("person", "organization")) }
            println(if (isRefund.await().answer()) "refund queue" else "inbox")
            println(names.await().answer().size)
        }
    }
}
```

A failed scalar judgement throws `FailedException` from the suspend call; an unsure judgement returns successfully. Each successfully returned row has `isFailed` false even though the shared static `Row` type retains its failed case.

In this `coroutineScope`, a failure in either `async` child cancels its sibling and the enclosing scope; the scope rethrows the original failure. Handle an operation's failure around `coroutineScope`, not only around one `await()`. Kotlin propagates the exception directly rather than wrapping it in Java's `CompletionException`. Reading an unsure answer after `await()` is a separate step that can throw `UnsureException`.

Cancelling the coroutine stops the call.

From code that is not in a coroutine, wrap the call in `runBlocking { }`.

## Saved questions

A question object holds a question and its options, so many calls can share one definition. Pass it in place of the question text.

```kotlin
val asksRefund = DecideQuestion(
    "Does the customer request a refund?",
    threshold = Threshold.band(0.2, 0.8),
    field = Pointer("/text"),
)
val isRefund = client.decide(asksRefund, mapOf("id" to "a1", "text" to "Please refund my order."))
```

There is one class per function, such as `ChooseQuestion` and `ScoreQuestion`. A `Toolbox` keeps named questions in a JSON file:

```kotlin
val toolbox = Toolbox.load(Path.of("toolbox.json"))
val topic = client.choose(
    toolbox.choose("topic"),
    mapOf("id" to "a1", "text" to "Where is my parcel?"),
)
```

`Toolbox.load` throws `IOException` when the file can't be read. `toolbox.choose(name)` throws `UsageException` when the toolbox has no choose question of that name.

## Settings

A client built with no settings reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, set them in the block and call the same functions.

```kotlin
ThinkThen {
    model = "..."
    batch = 16
    retry = RetryPolicy(maxRetries = 5)
}.use { client ->
    val isRefund = client.decide("Does the customer request a refund?", "Please refund my order.")
}
```

The options are `backend`, `url`, `model`, `apiKey`, `batch`, `retry` and `json`. Times in `RetryPolicy` are `kotlin.time.Duration` values. Keep the key in the environment, not in your code.

One client is safe to share between coroutines and threads. Closing it stops its running calls.

## Usage

```kotlin
ThinkThen().use { client ->
    client.decide("Does the customer request a refund?", "Please refund my order.")
    val usage = client.usage
    println("${usage.attempts} ${usage.inputTokens.reported} ${usage.outputTokens.reported}")
}
```

`client.usage` counts everything the client has sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them.

## Cancel

Kotlin already has the way to stop work, and ThinkThen follows it: cancel the coroutine that is making the call.

```kotlin
coroutineScope {
    val stop = launch {
        client.decideStream("Does the customer request a refund?", messages)
            .collect { isRefund -> println(isRefund) }
    }
    stop.cancel()
}
```

- Cancel the `Job`, or the scope it runs in.
- Put a time limit on a call with `withTimeout`.
- Stop collecting a `Flow` early, with `first()` or `take(n)`.

The stopped call throws `CancellationException`, as every cancelled suspend function does. Nothing more is sent to the service.

## Errors

| What happened | When it throws | What is thrown |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold | At the call | `UsageException`, `IllegalArgumentException` |
| The client could not start, such as no API key | When the client is built | `SetupException` |
| The call was cancelled | At the call | `CancellationException` |
| A one-record judgement failed, including blank/null input or a service failure on its row | At the scalar call | `FailedException` |
| A many-record result or form entry failed | At its answer read or `throwIfFailed()` | `FailedException` |
| An annotate parent failed | At the scalar call; retained as a failed form in a flow | `FailedException` at the scalar call or a later failed-form read/check |
| An answered annotate parent contains failed entries | The call returns it; reading a failed entry or explicitly checking the form throws | `FailedException` |
| Not sure | When you read the answer | `UnsureException` |
| You print a result, store it, or ask about its state | Never | |

A list given to a one-record call, two bars given to filter, and a result compared with a bare answer are caught earlier still: they do not compile.

Two catches let you catch by meaning:

- `catch (e: NoAnswerException)` catches `UnsureException` and `FailedException`: there was no answer to read.
- `catch (e: FailedException)` followed by `catch (e: ThinkThenException)` catches failed judgements and engine call errors. `ThinkThenException` is the parent of `UsageException`, `SetupException` and `InternalException`. A Kotlin class has one parent, and `FailedException` has `NoAnswerException`. Kotlin has no catch that names two classes, so this takes two `catch` clauses. Neither catches `UnsureException`, because not sure is a result and not an error. Standard `IllegalArgumentException` and coroutine `CancellationException` remain separate; do not absorb cancellation as a failed judgement.

`FailedException` retains the former `RecordException` payload: `code`, `httpStatus` and `rows`, the failed results. Only the concrete host exception name changes; the sealed `Row.Failed` case remains, and wire codes such as `record.empty` and existing engine error families do not change. `UnsureException` carries `probability` and `row`.

## Upgrading from 0.1

The baseline is the [published 0.1.2 package/source](https://repo.maven.apache.org/maven2/io/github/botassembly/thinkthen-jvm/0.1.2/). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| Maven `thinkthen-jvm` with `kotlin` classifier; `KotlinFacade(Door)` | The current package and typed client |
| `facade.decide(q, text).value().outcome()` | Scalar judgement failures throw `FailedException` at the call; a returned result is answered or unsure, with loud `answer()` for unsure |
| `decideAsync(...).await()` | The suspend call shown above |
