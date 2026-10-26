# ThinkThen in Swift

The `record:` overloads return one awaited row; `records:` overloads return Rows streams. A JSON array inside one Record remains one record. Iterator release ends early traversal; client resources follow Swift ownership.

Check `README.md` at the archive root for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture. Swift packages require macOS 15 or later; Linux packages require glibc 2.35 or later and GLIBCXX_3.4.30 support.

Use Swift 6.3 or later. Extract the supplied Swift 0.2.0 archive and add its local package to Package.swift with `.package(path: "/absolute/path/to/thinkthen-swift")`; use the extracted directory containing Package.swift. Add its `ThinkThen` product to your target. Keep the packaged native engine in place. Build and run with Swift Package Manager.

Set `THINKTHEN_API_KEY` through the environment before creating a normal client. Keep keys out of code and reports. `try Client()` selects the default backend. The stub needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```swift
import Foundation
import ThinkThen

let lines = try String(contentsOfFile: "records.jsonl", encoding: .utf8)
let records = try lines.split(whereSeparator: \.isNewline).map { line in
    Record.json(try JSONDecoder().decode(JSONValue.self, from: Data(line.utf8)))
}
let client = try Client(.init(backend: .stub))
let array = try await client.decide("Does the customer request a refund?",
    record: .json(.array([.string("first"), .string("second")])))
switch array.outcome {
case .answered: print("array answered")
case .unsure: print("array unsure")
case .failed(let failure): print("array failed", failure.message)
}
for try await row in client.decide("Does the customer request a refund?",
    records: records, options: .init(evidence: "/text")) {
    switch row.outcome {
    case .answered(let answer, _, _): print(row.position, "answered", answer)
    case .unsure: print(row.position, "unsure")
    case .failed(let failure): print(row.position, "failed", failure.message)
    }
}
```

Client settings accept `retry: Retry(...)`. When a task requires no automatic retries, set `maxRetries` to zero. The initializer also requires backoff and attempt-timeout Durations:

```swift
let client = try Client(.init(
    backend: .stub,
    retry: Retry(maxRetries: 0, initialBackoff: .milliseconds(100), attemptTimeout: .seconds(30))
))
```

A normal client accepts the same retry setting. The timeout limits each attempt; zero retries does not change batching or permit another execution of a task.

## Records and the ten functions

Use `.text(string)` for raw text, `.json(JSONValue)` for structured data and `.missing` for missing records. `Record.encoding(value)` converts an Encodable value. Structured records can carry exact string or integer IDs. Each options type has `evidence` for a JSON Pointer such as `"/text"`; escape `/` as `~1` and `~` as `~0` inside keys.

| Call on an open client | Answer |
| --- | --- |
| `decide(question, record: record)` | Bool |
| `choose(question, choices: choices, record: record)` | String |
| `tag(question, labels: labels, record: record)` | Array of strings |
| `score(question, levels: levels, record: record)` | Double from 0 to 1 |
| `filter(question, records: records)` | FilterResult stream |
| `rank(question, levels: levels, records: records)` | Int rank stream |
| `find(question, record: record)` | TextSpan? |
| `recognize(kinds: kinds, record: record)` | Array of RecognizedName |
| `relate(rules: rules, record: record)` | Array of Relation |
| `annotate(questions, record: record)` | Annotations |

Pass literal wording such as `"Does the customer request a refund?"`, choices such as `["billing", "shipping", "other"]`, levels such as `["routine", "urgent"]`, and kinds such as `["person", "organization"]`. Recognize and relate have no question prompt. RelationRule names source and target kinds and a relation, with optional `reads` and `either`.

Decide, choose and filter thresholds accept a numeric value or `.band(low: 0.2, high: 0.8)`. Tag, find, recognize and relate accept numeric cuts only. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Score and rank levels go lowest to highest. Rank reads the collection, places highest scores first and keeps failures when RankOptions.top limits successful ranks.

A rank row's answer is its rank; `position` is its input order. FilterResult retains its original `item`, typed `row` and `kept`. Every filter outcome appears, including false, unsure and failed.

## Rows, streams and saved questions

Rows retain `position`, exact `id`, `source` and `outcome`. Source identifies the question, backend and served model. Outcome is `.answered(answer, probability, detail)`, `.unsure(probability, detail)` or `.failed(RowFailure)`. Switch on outcome to distinguish false, unsure, failed and answered no-match. Typed detail holds label or level probabilities, recognition pieces or relation pairs. Find can answer nil; `row.answer` adds an outer optional for unanswered status. Find selects a whole nonblank line; preserve line breaks. Span offsets count UTF-8 bytes from zero with an exclusive end.

Single calls use `record:` and are awaited. Collection overloads use `records:` with a Sequence or AsyncSequence and return Rows. Filter and rank take collections. Iterate once with `for try await`; `.collect()` stores all rows in memory. Arrays supply finite input. Passing one collection lets the engine batch; avoid a loop of single-record calls. Settings.batch controls packing, with byte and backend limits also applied. One record can need several backend questions.

```swift
let rows = client.decide("Does the customer request a refund?", records: records)
for try await row in rows {
    if case .failed(let failure) = row.outcome { throw failure }
    print(row.position)
}
```

Typed question structs such as DecideQuestion retain options. Toolbox holds a named dictionary of ToolboxQuestion values. Decode a saved question toolbox with `JSONDecoder().decode(Toolbox.self, from: data)` and select with `try toolbox.decide(named: "refund")`; pass the result to the matching method. AnnotationQuestions holds eligible per-record questions and excludes rank. An answered annotation can have failed children. Iterate its `entries` and inspect each AnnotationResult's outcome.

Rows and toolboxes are Codable. `try JSONEncoder().encode(row)` exports an owned row with its source, details and exact IDs. Use Foundation file IO or your CSV/spreadsheet package to read data and write results. The SDK has no shell toolbox-run or spreadsheet IO API. Preserve IDs when joining results; rank may reorder them.

Call `relate(rules:records:)` or `relate(rules:record:)` alone when you need names and relations from the same evidence. Answered relation detail exposes `names` and `pairs`. The names use the existing recognized-name shape and include names with no returned relation. Read accepted relations from the answer; do not run recognize first.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```swift
let box = try Toolbox.fromJSON(String(contentsOfFile: "toolbox.json", encoding: .utf8))
try box.toJSON().write(toFile: "toolbox-copy.json", atomically: true, encoding: .utf8)
let topic = try box.choose(named: "topic")
for try await row in client.choose(topic, records: records) {
    switch row.outcome {
    case .answered(let answer, _, _): print(answer)
    case .unsure: print("unsure")
    case .failed(let failure): throw failure
    }
}
```

This code reuses the first example's client and records. `box["topic"]` returns an optional general question; `try box.filter(named: "ready")` and other typed selectors return matching question values. `JSONEncoder` and `JSONDecoder` use the same saved entries format for Toolbox's Codable conformance.

## Failures and cancellation

Record and backend failures return failed rows, including single calls. Inspect them before saving or chaining. Throw or end iteration when the task requires stopping after failure; inspect annotation children too. `row.result()` converts a failed row to a Result failure.

Call-level problems throw ThinkThenError. RowFailure retains a stable code, message and optional HTTP status. Cancel a Rows operation with `rows.cancel()` or cancel its consuming Task. Cancelled operations close native work. A Rows value permits one traversal; creating another iterator is an iteration error. Client resources follow Swift ownership; keep the client alive while using its calls.

Make questions unambiguous. Define the evidence criterion, distinguish unstated facts from explicit negatives and use distinct choices. The stub checks installation, not judgment quality.
