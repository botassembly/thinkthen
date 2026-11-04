# Java functions that answer questions about text

ThinkThen 0.2.0 for Java gives you ten functions. Each one reads a record, answers one question about it, and returns a typed value your code can branch on.

## Install and ask your first question

Add the package to Maven:

```xml
<dependency>
  <groupId>io.github.botassembly</groupId>
  <artifactId>thinkthen-java</artifactId>
  <version>0.2.0</version>
</dependency>
```

Or to Gradle: `implementation("io.github.botassembly:thinkthen-java:0.2.0")`. It needs Java 22 or later and includes the native engine. Run with `--enable-native-access=ALL-UNNAMED` on the class path, or `--enable-native-access=dev.thinkthen` on the module path.

```java
import dev.thinkthen.*;

public class Example {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            boolean isRefund = client.decide(
                "Does the customer request a refund?",
                "Please refund my order.").answer();
            System.out.println(isRefund);
        }
    }
}
```

<!-- output: capture -->

Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

## Handle not sure and failures

`answer()` is the one step from a row to its answer. It never hands back a default:

- When ThinkThen is not sure, it throws `UnsureException`. That is a `NoSuchElementException`, as `Optional.orElseThrow()` throws, and its `probability()` gives the row's probability.
- When the record failed, it throws `ThinkThenException`. Its `code()` names the engine error, such as `record.missing`.

With decide's default cut, every answer is yes or no. A band, `Threshold.band(low, high)`, makes the middle not sure. Check `isUnsure()` first and send those records to a person:

```java
String message = "Please refund my order.";
var options = DecideOptions.builder()
    .threshold(Threshold.band(0.2, 0.8)).build();
var refundCheck = client.decide(
    "Does the customer request a refund?", message, options);
if (refundCheck.isUnsure()) {
    System.out.println("Ask a person: " + message);
} else if (refundCheck.answer()) {
    System.out.println("Send to refunds: " + message);
} else {
    System.out.println("Leave in the inbox: " + message);
}
```

A failed record still reaches `answer()` and throws there. Call `throwIfFailed()` to throw it sooner, and `isAnswered()` or `isFailed()` to test without throwing. Mistakes in the call itself throw at once: a bad threshold or a missing key throws `ThinkThenException`, and a collection or an unknown object passed to a one-record call throws `IllegalArgumentException`.

## The ten functions

Each snippet runs inside `try (var client = ThinkThen.builder().build()) { ... }`. The many-record functions read the same three support messages, with `java.util.List` and `java.util.Map` imported:

```java
List<Map<String, String>> messages = List.of(
    Map.of("id", "a1", "text", "Please refund my order."),
    Map.of("id", "a2", "text", "Where is my parcel?"),
    Map.of("id", "a3", "text", "I was charged twice for one invoice."));
var text = Pointer.of("/text");
```

`Pointer.of("/text")` tells ThinkThen to judge each message's `text` field. Without a pointer, it judges the whole record.

### decide: answer a yes-or-no question

```java
boolean isRefund = client.decide(
    "Does the customer request a refund?",
    "Please refund my order.").answer();
```

### choose: pick the one option that fits

```java
String topic = client.choose(
    "What is the message about?",
    "Please refund my order.",
    List.of("billing", "shipping", "other")).answer();
```

### tag: label every option that fits

```java
List<String> topics = client.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    List.of("billing", "shipping")).answer();
```

### score: place the text on a scale from 0 to 1

```java
double urgency = client.score(
    "How urgent is the request?",
    "Please refund my order.",
    List.of("routine", "urgent")).answer();
```

Levels run from lowest to highest. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```java
try (Stream<Map<String, String>> refunds = client.filter(
        "Does the customer request a refund?", messages,
        FilterOptions.builder().field(text).build())) {
    refunds.forEach(refund -> System.out.println(refund.get("id")));
}
```

`filter` returns your own records, only the ones that answer yes. Records that are not sure are left out. When records fail, the stream delivers every kept record first, then throws one `ThinkThenException`. Its `failures()` lists every failed row with its `position()`. To see every record's outcome, call `filterRows`, which returns a `Filtered<T>` for each record with its `item()`, `kept()` and `answer()`.

### rank: order every record, highest first

```java
try (Stream<Ranked<JsonValue>> byUrgency = client.rank(
        "How urgent is the request?", messages,
        List.of("routine", "urgent"),
        RankOptions.builder().field(text).build())) {
    byUrgency.forEach(ranked -> {
        long urgencyRank = ranked.answer();
        var message = (JsonObject) ranked.item();
        var id = (JsonString) message.values().get("id");
        System.out.println(urgencyRank + " " + id.value());
    });
}
```

rank gives back each record as a `JsonValue` snapshot (text arrives as `JsonString`); filter gives back your own objects. A JSON record arrives as its JSON type, such as `JsonObject`, whose `values()` map holds its fields, and a missing or invalid record arrives as `JsonNull`, never Java `null`. `answer()` is the rank, starting at 1. `position()` is the record's place in your input. `RankOptions.builder().top(n)` keeps the first `n` ranks and still reports failed records.

### find: point to the line that answers

```java
Optional<Span> action = client.find(
    "Which line states the requested action?",
    "Morgan works for Example Group.\nPlease refund my order.\n").answer();
action.ifPresent(span -> System.out.println(span.text()));
```

An empty `Optional` means no line answers the question. Find picks one whole line, so keep the line breaks. A `Span` gives the matched `text()` and its `start()` and `end()` as `long` UTF-16 offsets, with the end excluded.

### annotate: fill in several questions for each record

```java
var questions = AnnotationQuestions.builder()
    .put("refund", DecideQuestion.of("Does the customer request a refund?"))
    .put("topic", ChooseQuestion.of("What is the message about?",
        List.of("billing", "shipping", "other")))
    .put("urgency", ScoreQuestion.of("How urgent is the request?",
        List.of("routine", "urgent")))
    .build();
var options = AnnotateOptions.builder().field(text).build();
try (Stream<Annotated<Map<String, String>>> forms =
        client.annotateStream(questions, messages, options)) {
    forms.forEach(form -> {
        Annotations answers = form.answer();
        boolean isRefund = answers.decide("refund").answer();
        String topic = answers.choose("topic").answer();
        double urgency = answers.score("urgency").answer();
        System.out.println(form.item().get("id") + " " + isRefund + " " + topic + " " + urgency);
    });
}
```

Each `Annotated` keeps your own record in `item()`. Each question has its own outcome, and its `answer()` throws on not sure or failure, as a row's does. A selector such as `answers.decide(name)` throws `ThinkThenException` with code `usage.argument` for a name you never added or a question of another function, and its message lists the names you added. For one record, `client.annotate(questions, message)` returns the row directly. Rank cannot go in `AnnotationQuestions`, and the compiler refuses it.

### recognize: pull out each name and its kind

```java
List<Entity> names = client.recognize(
    "Morgan works for Example Group.",
    List.of("person", "organization")).answer();
for (Entity name : names) {
    System.out.println(name.mention().kind() + ": " + name.mention().span().text());
}
```

### relate: link names by the relations you allow

```java
List<Relation> relations = client.relate(
    "Morgan works for Example Group.\nPlease refund my order.\n",
    List.of(RelationRule.of("person", "organization", "works_for"))).answer();
for (Relation relation : relations) {
    System.out.println(relation.source().span().text() + " "
        + relation.relation() + " " + relation.target().span().text());
}
```

`RelationRule.builder(source, target, relation)` adds `reads(...)` to word the relation and `either(true)` to make it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge many records

Every one-record function has a `…Stream` twin, such as `decideStream`, that takes an `Iterable` or a `java.util.stream.Stream` and returns a `Stream` of rows in input order. `annotateStream` returns `Annotated` items instead, which keep each record with its row. `filter`, `filterRows` and `rank` take many records already. ThinkThen packs the records into batches; `ThinkThen.builder().batch(16)` sets the size.

Close every stream with try-with-resources. Closing it early cancels the work still running. A failed or not-sure record is a row like any other, so a `switch` keeps every record without stopping the batch:

```java
try (var refundChecks = client.decideStream(
        "Does the customer request a refund?", messages,
        DecideOptions.builder().field(text).build())) {
    refundChecks.forEach(refundCheck -> {
        long position = refundCheck.position();
        switch (refundCheck) {
            case Answered<Boolean, NoDetail> answered ->
                System.out.println(position + " " + answered.answer());
            case Unsure<Boolean, NoDetail> notSure ->
                System.out.println(position + " not sure: ask a person");
            case Failed<Boolean, NoDetail> failed ->
                System.out.println(position + " failed: " + failed.error().code().wireName());
        }
    });
}
```

Each row also carries `id()`, the record's own `id` as a `JsonValue`, and `source()`, the backend and model that answered.

## Say what yes, no and each option mean

Decide and filter take `yesMeans` and `noMeans`. Choose, tag, score and rank take `descriptions`, a map from each option or level to what it means. The answers keep their short names.

```java
var refundOptions = DecideOptions.builder()
    .yesMeans("The customer asks for money back")
    .noMeans("The customer asks for anything else")
    .build();
var topicOptions = ChooseOptions.builder()
    .descriptions(Map.of(
        "billing", "Invoices, charges and refunds",
        "shipping", "Delivery and parcels",
        "other", "Anything else"))
    .build();
String topic = client.choose("What is the message about?",
    "Please refund my order.", List.of("billing", "shipping", "other"),
    topicOptions).answer();
```

## Records and field

A record is a `String`, a `JsonValue`, a `Map` with string keys, a `List`, a boxed number or `Boolean`, or `null`. A string stays text even when it holds JSON. `JsonValue.parse(json)` reads a JSON record. A record of another type needs a `RecordCodec` set on the builder with `recordCodec(...)`.

A one-record call takes exactly one record. Pass many records to the `…Stream` form. A `null` record, or a record without the field the pointer selects, gives a failed row with `record.missing` and sends no request. In a pointer, write `~1` for a `/` in a key and `~0` for a `~`.

## Read the probability and details

`probability()` is an `OptionalDouble`. For decide and filter it is the probability of yes, even when the answer is no. For the other functions it is the probability of the answer given.

An `Answered` or `Unsure` row also has `detail()`. Choose and tag give a `LabelDetail` with each label's probability. Score and rank give a `LevelDetail` with each level's probability, in level order. Recognize gives a `RecognizeDetail`, and relate gives a `RelationDetail` with every name it found in `names()` and every scored pair in `pairs()`.

## Save and load a toolbox

A toolbox is a file of named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Load it, pick a question by name and type, and pass that question to its function:

```java
Toolbox toolbox = Toolbox.load(Path.of("toolbox.json"));
ChooseQuestion topicQuestion = toolbox.choose("topic").orElseThrow();
try (var topics = client.chooseStream(topicQuestion, messages)) {
    topics.forEach(topic -> System.out.println(topic.position() + " " + topic.answer()));
}
```

`Toolbox.load(path)` and `Toolbox.fromJson(json)` read this saved format, and `toJson()` writes it. `Toolbox.fromDefinitions(json)` reads bare definitions with no `entries` wrapper, such as `{"topic":{"function":"choose", ...}}`. `Toolbox.builder().put(name, question).build()` makes one in code. The typed selectors, such as `toolbox.choose(name)`, return an empty `Optional` for a missing name or a different function.

## Run calls in the background

`client.async()` gives every one-record call as a `CompletableFuture`, with the same names. Cancelling the future stops the native work. Interrupting a thread that waits on a call cancels the call, keeps the thread's interrupt flag and throws `CancellationException`. Closing the client cancels its running calls and refuses new ones.

## Reference

[Read the API reference](../../sdk/java/API.md). Every public type and method has Javadoc in the published sources JAR, so your editor shows it on hover.
