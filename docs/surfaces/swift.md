# Swift functions that answer questions about text

ThinkThen 0.2.0 for Swift gives you ten functions. Each one reads a record, answers one question about it, and returns a typed value your code can branch on.

## Install and ask your first question

Add the package to `Package.swift`:

```swift
platforms: [.macOS("15.0")],
dependencies: [
    .package(url: "https://github.com/botassembly/thinkthen", exact: "0.2.0"),
],
targets: [
    .executableTarget(
        name: "Example",
        dependencies: [.product(name: "ThinkThen", package: "thinkthen")]),
]
```

It needs Swift 6.3 or later and includes the native engine. It runs on macOS 15 or later on Apple silicon, and on Linux (x86-64 or arm64) with glibc 2.35 or later.

```swift
import ThinkThen

let client = try Client()
let isRefund = try await client.decide(
    "Does the customer request a refund?",
    record: "Please refund my order.").get()
print(isRefund)
```

<!-- output: capture -->

Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

## Handle not sure and failures

`get()` is the one step from a row to its answer. It never hands back a default:

- When ThinkThen is not sure, it throws `RowError.unsure(probability:)`, which carries the row's probability.
- When the record failed, it throws `RowError.failed(RowFailure)`. The failure's `code` names the engine error, such as `.recordMissing`, and its `message` says what went wrong.

With decide's default cut, every answer is yes or no. A band makes the middle not sure. Catch `RowError.unsure` and send those records to a person:

```swift
let message = "Please refund my order."
let refundCheck = try await client.decide(
    "Does the customer request a refund?", record: .text(message),
    options: DecideOptions(threshold: .band(low: 0.2, high: 0.8)))
do {
    if try refundCheck.get() {
        print("Send to refunds: \(message)")
    } else {
        print("Leave in the inbox: \(message)")
    }
} catch RowError.unsure {
    print("Ask a person: \(message)")
}
```

A failed record is still a row, so the call returns it and `get()` throws `RowError.failed` there. `result()` gives the same outcome as a `Result<Answer, RowError>`, and `isUnsure` and `isFailed` test without throwing. Mistakes in the call itself, such as a bad threshold or a missing key, throw `ThinkThenError` from `Client(...)` or the call.

`row.answer` is `nil` for both not sure and failure, so `row.answer == true` sends not-sure records down the "no" branch. Use `get()` or `result()` instead.

## The ten functions

Each snippet runs after `let client = try Client()`, inside an `async throws` context. The many-record functions read the same three support messages:

```swift
let messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
```

### decide: answer a yes-or-no question

```swift
let isRefund = try await client.decide(
    "Does the customer request a refund?",
    record: "Please refund my order.").get()
```

### choose: pick the one option that fits

```swift
let topic = try await client.choose(
    "What is the message about?",
    choices: ["billing", "shipping", "other"],
    record: "Please refund my order.").get()
```

### tag: label every option that fits

```swift
let topics = try await client.tag(
    "Which topics appear?",
    labels: ["billing", "shipping"],
    record: "I was charged twice and my parcel is late again.").get()
```

### score: place the text on a scale from 0 to 1

```swift
let urgency = try await client.score(
    "How urgent is the request?",
    levels: ["routine", "urgent"],
    record: "Please refund my order.").get()
```

Levels run from lowest to highest. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```swift
for try await refund in client.filter(
    "Does the customer request a refund?", records: messages) {
    print(refund)
}
```

`filter` yields your own records, only the ones that answer yes: a `String` for each string you passed, and a `Record` for each `Record`. Records that are not sure are left out. When records fail, the stream delivers every kept record first, then throws one `RowFailure` whose `failedRows` lists every failed row. To see every record's outcome, call `filterResults`, which yields a `FilterResult` for each record with its `item`, its `row` and `kept`.

### rank: order every record, highest first

```swift
for try await ranked in client.rank(
    "How urgent is the request?",
    levels: ["routine", "urgent"], records: messages) {
    if case .text(let message) = ranked.item {
        print(try ranked.get(), message)
    }
}
```

Each result holds a snapshot of your record in `item`, a `Record`. rank's items are snapshots equal as JSON values, so a number may come back in a different numeric case (for example `.number(7.0)` as `.integer(7)`); filter returns your own values. `get()` is the rank, starting at 1. `position` is the record's place in your input, starting at 1. `RankOptions(top: n)` keeps the first `n` ranks and still reports failed records.

### find: point to the line that answers

```swift
let text = "Morgan works for Example Group.\nPlease refund my order.\n"
let action = try await client.find(
    "Which line states the requested action?", record: .text(text)).get()
if let action {
    print(action.text)
}
```

`nil` means no line answers the question. Find picks one whole line, so keep the line breaks. A `TextSpan` gives the matched `text`, and `start` and `end` in Unicode code points, with the end excluded. `action.range(in: text)` turns them into a `Range<String.Index>`.

### annotate: fill in several questions for each record

```swift
let questions = AnnotationQuestions([
    "refund": .decide(DecideQuestion("Does the customer request a refund?")),
    "topic": .choose(ChooseQuestion(
        "What is the message about?", choices: ["billing", "shipping", "other"])),
    "urgency": .score(ScoreQuestion(
        "How urgent is the request?", levels: ["routine", "urgent"])),
])
for try await annotated in client.annotate(
    questions, records: messages.map(Record.text)) {
    let triage = try annotated.get()
    let isRefund = try triage.get("refund", as: Bool.self)
    let topic = try triage.get("topic", as: String.self)
    let urgency = try triage.get("urgency", as: Double.self)
    print(annotated.position, isRefund, topic, urgency)
}
```

Each question has its own outcome. `triage.get(name, as:)` throws `RowError` on not sure or failure, as a row's `get()` does, and throws `ThinkThenError` for a name you never added or a type that does not match. `triage[name]` gives the question's `AnnotationResult` without throwing, and its `get(as:)` reads the answer. For one record, `client.annotate(questions, record:)` returns the row directly. Rank cannot go in `AnnotationQuestions`.

### recognize: pull out each name and its kind

```swift
let names = try await client.recognize(
    kinds: ["person", "organization"],
    record: "Morgan works for Example Group.").get()
for name in names {
    print("\(name.kind): \(name.text)")
}
```

### relate: link names by the relations you allow

```swift
let worksFor = RelationRule(source: "person", target: "organization", relation: "works_for")
let relations = try await client.relate(
    rules: [worksFor],
    record: "Morgan works for Example Group.\nPlease refund my order.\n").get()
for relation in relations {
    print(relation.source.text, relation.relation, relation.target.text)
}
```

`RelationRule` also takes `reads:` to word the relation and `either: true` to make it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge a file of records

Create `records.jsonl` with one JSON value per line:

```text
{"id":"a1","text":"Please refund my order."}
{"id":"a2","text":"Where is my parcel?"}
null
{"id":"a4"}
[{"text":"first"},{"text":"second"}]
{"id":"a6","text":null}
```

Read every line as a record and pass them all to one call. A failed or not-sure record is a row like any other, so a `switch` on `result()` keeps every record without stopping the batch:

```swift
import Foundation
import ThinkThen

let lines = try String(contentsOfFile: "records.jsonl", encoding: .utf8)
let records = try lines.split(whereSeparator: \.isNewline).map { line in
    Record.json(try JSONDecoder().decode(JSONValue.self, from: Data(line.utf8)))
}
let client = try Client()
for try await refundCheck in client.decide(
    "Does the customer request a refund?", records: records,
    options: DecideOptions(field: "/text")) {
    switch refundCheck.result() {
    case .success(let isRefund):
        print(refundCheck.position, "answered", isRefund)
    case .failure(.unsure):
        print(refundCheck.position, "unsure: ask a person")
    case .failure(.failed(let failure)):
        print(refundCheck.position, "failed", failure.code.rawValue, failure.message)
    }
}
```

<!-- output: capture -->

Look for one line per record, numbered from 1, with the failures in place. A `null` record, a record without `text` and a `text` that is `null` all fail with `record.missing` and send nothing to the backend. The array on line 5 is one record; to judge each of its values, pass them as separate records.

Every function takes `records:` with any `Sequence` or `AsyncSequence` of `Record` and returns `Rows`, an `AsyncSequence` you read once with `for try await`. Decide, choose, score, filter, rank and find also take a sequence of `String`; for the others, pass `messages.map(Record.text)`. ThinkThen packs the records into batches; `Client(batch: 16)` sets the size. Pass one collection rather than looping over one-record calls.

`position` is the record's place in your input, starting at 1. Each row also carries `id`, the record's own `id` field, and `source`, the backend and model that answered. `collect()` reads every row into an array. Breaking out of the loop, calling `cancel()` on the rows or cancelling the task stops the native work, and a cancelled read throws `CancellationError`.

## Say what each option means

Choose, tag, score and rank take each option or level as a `Choice`. A plain string is a `Choice` with no description; `Choice(name, description:)` tells the model what it means. The answers keep the short names.

```swift
let topic = try await client.choose(
    "What is the message about?",
    choices: [
        Choice("billing", description: "Invoices, charges and refunds"),
        Choice("shipping", description: "Delivery and parcels"),
        Choice("other", description: "Anything else"),
    ],
    record: "Please refund my order.").get()
```

## Records and field

A `Record` is `.text(String)`, `.json(JSONValue)` or `.missing`. A string literal is a text record. `Record.encoding(value)` turns any `Encodable` value into a JSON record. A string stays text even when it holds JSON.

Each options type takes `field`, a JSON Pointer that picks the field to judge. Without it, ThinkThen judges the whole record.

```swift
struct Message: Encodable { let id: String; let text: String }
let record = try Record.encoding(Message(id: "a1", text: "Please refund my order."))
let isRefund = try await client.decide(
    "Does the customer request a refund?", record: record,
    options: DecideOptions(field: "/text")).get()
```

A `.missing` record, a JSON `null`, or a record without the field the pointer selects gives a failed row with `.recordMissing` and sends no request, as [Judge a file of records](#judge-a-file-of-records) shows. In a pointer, write `~1` for a `/` in a key and `~0` for a `~`.

## Read the probability and details

`probability` is a `Double?`, and `nil` for a failed row. For decide and filter it is the probability of yes, even when the answer is no. For the other functions it is the probability of the answer given.

`detail` holds the rest. Choose and tag give a `LabelDetail` with each label's probability in `labels`. Score and rank give a `LevelDetail` with each level's probability in `levels`, in level order. Recognize gives a `RecognizeDetail`, and relate gives a `RelationDetail` with every name it found in `names` and every scored pair in `pairs`.

Rows are `Codable`, so `JSONEncoder().encode(refundCheck)` saves a row with its source, details and id.

## Save and load a toolbox

A toolbox is a file of named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Load it, pick a question by name and function, and pass that question to its function:

```swift
import Foundation

let toolbox = try Toolbox(contentsOf: URL(fileURLWithPath: "toolbox.json"))
let topicQuestion = try toolbox.choose(named: "topic")
let topic = try await client.choose(
    topicQuestion, record: try Record.encoding(["text": "Please refund my order."])).get()
```

`Toolbox(contentsOf:)` and `Toolbox(json:)` read this saved format, and `jsonString()` and `write(to:)` write it. `Toolbox(["topic": .choose(topicQuestion)])` makes one in code. The typed selectors, such as `toolbox.choose(named:)`, throw `ThinkThenError` for a missing name or a different function; `toolbox["topic"]` returns an optional `ToolboxQuestion` instead. `AnnotationQuestions(toolbox)` turns a toolbox into annotate questions and throws if it holds a rank question. `AnnotationQuestions(json:)` and `AnnotationQuestions(contentsOf:)` read a bare mapping of names to questions, with no `entries` wrapper.

## Set up the client

`try Client()` reads `THINKTHEN_BACKEND` and uses System One when it is unset. Pass a backend to choose one in code: `.systemOne` or `.openAI`. The initializer also takes `url`, `model`, `apiKey: APIKey(...)`, `batch` and `retry`. An `APIKey` prints as `<redacted>`, so a logged setting never shows the key.

`Retry()` uses the engine's defaults. Set only the fields you want to change, `maxRetries`, `initialBackoff` or `attemptTimeout`, and the engine keeps its own value for the rest. A retried request can be processed and billed twice, so pass `retry: Retry(maxRetries: 0)` when that matters.

`Client` is `Sendable`, so tasks can share one. It frees the native engine when its last reference goes.

## Reference

[Read the API reference](../../sdk/swift/Sources/ThinkThen).
