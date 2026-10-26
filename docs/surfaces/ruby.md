# ThinkThen in Ruby

Named methods return one row; `_each` methods return or yield a row stream. Array values remain valid single records in named methods. Block traversal closes streams; close external iterators in `ensure` and use `Client.open` for client ownership.

Check `README.md` inside the gem for the supplied package's operating-system floor and runtime requirements before installation. Use the package for your operating system and architecture.

Install the supplied matching platform gem with `gem install --local /path/to/thinkthen-0.2.0-platform.gem`. The platform gem includes the native engine. Its `ffi` dependency must also be installed. Use CRuby 3.3 or later. Require `thinkthen` in your program.

Set `THINKTHEN_API_KEY` through the environment before opening a normal client. Keep keys out of code and reports. `ThinkThen::Client.open` selects the default backend; `backend: :stub` needs no key or network and returns synthetic installation-check answers.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```ruby
require "json"
require "thinkthen"

records = File.readlines("records.jsonl", chomp: true).reject { |line| line.strip.empty? }.map { |line| JSON.parse(line) }
ThinkThen::Client.open(backend: :stub) do |client|
  array = client.decide("Does the customer request a refund?", ["first", "second"])
  puts "array #{array.status}"
  client.decide_each("Does the customer request a refund?", records, evidence: "/text") do |row|
    message = row.failed? ? row.error.message : ""
    puts "#{row.position} #{row.status} #{row.answered? ? row.answer : nil} #{message}"
  end
end
```

## Records and the ten functions

A record is text or a JSON-compatible Ruby value. JSON object keys in record Hashes must be Strings, not Symbols. Hash records can carry a string or integer `id`. `evidence: "/text"` selects a JSON field; an Array of key/index components is also accepted. In a pointer, escape `/` as `~1` and `~` as `~0`.

```ruby
records = [{"id" => "a1", "text" => "Morgan works for Example Group.\nPlease refund my order."}, {"id" => "a2", "text" => "Where is my parcel?"}]
record = records.first
questions = ThinkThen::AnnotationQuestions.new(questions: {
  "refund" => ThinkThen::DecideQuestion.new("Does the customer request a refund?"),
})
# Each expression is a separate call; reuse an open client.
client.decide("Does the customer request a refund?", record, evidence: "/text")
client.choose("Which topic?", record, choices: ["billing", "shipping", "other"], evidence: "/text")
client.tag("Which topics appear?", record, labels: ["billing", "shipping"], evidence: "/text")
client.score("How urgent is the request?", record, levels: ["routine", "urgent"], evidence: "/text")
client.filter_each("Does the customer request a refund?", records, evidence: "/text") { |item| puts item.item if item.kept? }
client.rank_each("How urgent is the request?", records, levels: ["routine", "urgent"], evidence: "/text", top: 5) { |row| puts row.answer if row.answered? }
client.find("Which line states the requested action?", record, evidence: "/text")
client.recognize(record, kinds: ["person", "organization"], evidence: "/text")
client.relate(record, rules: [ThinkThen::RelationRule.new(source: "person", target: "organization", relation: "works_for")], evidence: "/text")
client.annotate(questions, record, evidence: "/text")
```

| Function | Answer when answered |
| --- | --- |
| decide, filter | Boolean |
| choose | Choice String |
| tag | Array of matching labels |
| score | Float from 0 to 1 |
| rank | Integer rank from 1 |
| find | Span or nil |
| recognize | Array of names with text, offsets, kind and probability |
| relate | Array of relations with endpoints, relation, probability and symmetric |
| annotate | Named typed outcomes |

A rank row's answer is its rank; `position` is its input order. Rank uses lowest-to-highest levels, returns highest scores first and keeps failures when `top:` limits successful ranks. Find selects a whole nonblank line; preserve line breaks. Span offsets count Unicode code points from zero with an exclusive end.

Rows are immutable `Answered`, `Unsure` and `Failed` variants. Check the variant before reading `answer`. They retain `position`, exact `id`, source identity and status. Source identifies the question, backend and served model. Answered/unsure detail values hold label or level probabilities, recognition pieces or relation pairs. `FilterRow` wraps its typed `row`, retains the original `item`, and has `kept?` true only for answered true.

## Streams, questions and files

Every named method returns one row. Its `_each` version takes an Enumerable and returns lazy `Rows`, or yields with a block. Prefer a single `_each` call over the collection to a loop of single calls. Arrays use finite input; file Enumerables can feed incrementally. Block traversal closes its call. Close an external iterator in `ensure` when traversal ends early. Input files remain yours to close.

Clients accept `batch:`, `retry:`, `backend:`, `model:`, `url:` and `api_key:`. Engine defaults apply to omitted settings. Batching also observes byte and backend limits; one record can require multiple questions.

Typed question classes such as `ThinkThen::DecideQuestion` retain domains and evidence. `ThinkThen::Toolbox` maps names to typed questions. Its `toolbox["name"]` selector retrieves a question for the matching client method; selecting it does not set an execution name. `AnnotationQuestions` accepts per-record questions and excludes rank and annotate. Inspect nested statuses independently of the outer annotation.

`retry:` requires a `ThinkThen::Retry`, not a Hash. Set `max_retries: 0` when a task prohibits automatic retries. A typed question owns its evidence, threshold and required lists; pass those options to its constructor. A matching call with that question accepts only `name:` and `cancel:`. The same ownership applies to questions stored in a toolbox or annotation collection:

```ruby
retry_policy = ThinkThen::Retry.new(max_retries: 0)
refund = ThinkThen::FilterQuestion.new("Does the customer request a refund?", evidence: "/text")
ThinkThen::Client.open(backend: :stub, retry: retry_policy) do |client|
  client.filter_each(refund, records) do |item|
    puts [item.row.id, item.row.status, item.kept?].inspect
  end
end
```

The retry setting also applies to a normal client. Do not repeat evidence or threshold at the call when the saved question already owns its options.

Use Ruby `JSON` or `CSV` to load records and save results. `row.to_h` exports contract data with String keys, including typed answers, details, source and errors. `JSON.generate(row.to_h)` serializes it. Preserve IDs when joining to input; rank changes output order. Read and write XLSX with your application's spreadsheet library. There is no shell toolbox-run API in this SDK.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```ruby
box = ThinkThen::Toolbox.from_json(File.read("toolbox.json", encoding: "UTF-8"))
File.write("toolbox-copy.json", box.to_json)
ThinkThen::Client.open(backend: :stub) do |client|
  client.choose_each(box["topic"], records) do |row|
    raise "#{row.error.code}: #{row.error.message}" if row.failed?
    p [row.id, row.answer] if row.answered?
  end
end
```

Use `box["ready"]` with `filter_each` for the loaded filter question. A selected question retains its evidence and domain; do not repeat them at the call.

Call `relate` or `relate_each` alone when you need names and relations from the same evidence. After checking that the row has details, `row.detail.names` contains all admitted names, including names with no returned relation; `row.detail.pairs` contains scored relation pairs. Read accepted relations from `row.answer`. Do not run recognize first for that task.

## Failures and cancellation

Missing and invalid records return failed rows, including single calls. Inspect `row.failed?` and its error before saving or chaining. Inspect `item.row` for filter failures and every annotation child. Raise or break and close the iterator when the workload must stop on failure; a failed row does not raise automatically.

Call-level failures raise `UsageError`, `BackendError`, `LocalError`, `InternalError` or `Cancelled`. Errors retain stable codes and library messages. `ThinkThen::CancelToken.new` supplies a token through `cancel:`; its `cancel` method cancels attached calls. Interrupt during native pull raises `Cancelled` with the original Interrupt as cause. `Client.open` closes the client after the block.

Decide, choose and filter accept a Float cut or inclusive Float Range such as `0.2..0.8`. A decide/filter band yields false below low, unsure inside and true at or above high. Choose needs its winning choice to reach the upper cut. Tag, find, recognize and relate accept Float cuts only.

Make each question unambiguous: specify the exact evidence criterion and use distinct choices. Distinguish an unstated fact from a stated negative. Use the stub only to check installation.
