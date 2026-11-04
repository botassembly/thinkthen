# Ruby functions that answer questions about text

ThinkThen 0.2.0 for Ruby gives you ten functions. Each one reads a record, answers one question about it, and returns a typed value your code can branch on.

## Install and ask your first question

Run `gem install thinkthen --version 0.2.0`, or `bundle add thinkthen --version 0.2.0`. It needs CRuby 3.3 or later and includes the native engine.

```ruby
require "thinkthen"

ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?",
                             "Please refund my order.")
  puts is_refund
end
```

<!-- output: capture -->

Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there. [Choose a service and get a key](service.md#choose-a-service-and-get-a-key) explains the backends and how to get a key.

## Handle not sure and failures

`decide?` and `answer` are the one step from a call to its answer. Neither hands back a default:

- When ThinkThen is not sure, they raise `ThinkThen::UnsureError`. Its `probability` gives the row's probability.
- When the record failed, they raise `ThinkThen::RecordError`. Its `code` names the engine error, such as `:"record.missing"`.

With decide's default cut, every answer is yes or no. A band such as `threshold: 0.2..0.8` makes the middle not sure. Call `decide` without the question mark to get the row, check `unsure?` first, and send those messages to a person:

```ruby
require "thinkthen"

message = "Please refund my order."
ThinkThen::Client.open do |client|
  refund_check = client.decide("Does the customer request a refund?", message,
                               threshold: 0.2..0.8)
  if refund_check.unsure?
    puts "Ask a person: #{message}"
  elsif refund_check.yes?
    puts "Send to refunds: #{message}"
  else
    puts "Leave in the inbox: #{message}"
  end
end
```

`yes?`, `no?`, `answer` and the conversions `to_s`, `to_f` and `to_a` all raise on a not-sure or failed row. `answered?`, `unsure?` and `failed?` test a row without raising. Blank text raises `RecordError` with `record.empty` before any request is sent.

Mistakes in the call itself raise at once. An Array passed to a one-record call raises `ThinkThen::UsageError`, an `ArgumentError` that names the `_each` call to use instead. A bad threshold or a missing key also raises `UsageError`. Call and record errors include the `ThinkThen::Error` module, so `rescue ThinkThen::Error` catches them; `UnsureError` is not one of them.

## The ten functions

Each snippet runs inside `ThinkThen::Client.open do |client| ... end`. The many-record functions read the same three support messages:

```ruby
messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]
```

### decide: answer a yes-or-no question

```ruby
is_refund = client.decide?("Does the customer request a refund?",
                           "Please refund my order.")
```

### choose: pick the one option that fits

```ruby
topic = client.choose("What is the message about?",
                      "Please refund my order.",
                      choices: ["billing", "shipping", "other"]).answer
```

### tag: label every option that fits

```ruby
topics = client.tag("Which topics appear?",
                    "I was charged twice and my parcel is late again.",
                    labels: ["billing", "shipping"]).answer
```

### score: place the text on a scale from 0 to 1

```ruby
urgency = client.score("How urgent is the request?",
                       "Please refund my order.",
                       levels: ["routine", "urgent"]).answer
```

Levels run from lowest to highest. A score orders a queue a person reads. Use decide for a yes-or-no gate.

### filter: keep the records that answer yes

```ruby
client.filter("Does the customer request a refund?", messages) do |refund_request|
  puts refund_request
end
```

`filter` yields your own records, only the ones that answer yes, in input order; without a block it returns them as an Enumerator. Records that are not sure are left out. When records fail, it delivers every kept record first, then raises one `RecordError`; its `rows` lists every failed row with its `position`. `failures: :row` hands the failed rows over after the kept records instead of raising. To see every record's outcome, call `filter_each`, which gives a `FilterRow` for each record with its `item`, `kept?` and `answer`.

### rank: order every record, highest first

```ruby
client.rank("How urgent is the request?", messages,
            levels: ["routine", "urgent"]) do |ranked|
  puts "#{ranked.rank} #{ranked.item}"
end
```

`rank` yields a `RankRow` for each record, most urgent first; without a block it returns them as an Enumerator. `ranked.rank` is the rank, starting at 1, and raises on not sure as `answer` does; a not-sure row stays in its place. `ranked.item` is a copy of your record, not the same object, and a Hash comes back with String keys, whatever keys your records used. `position` is the record's place in your input. When records fail, the ranked rows come first, then one `RecordError` lists every failed row; `failures: :row` yields the failed rows instead. `top: n` keeps the first `n` ranks.

### find: point to the line that answers

```ruby
action = client.find("Which line states the requested action?",
                     "Morgan works for Example Group.\nPlease refund my order.").answer
puts action.text if action
```

`nil` means no line answers the question. Find picks one whole line, so keep the line breaks. A `ThinkThen::Found` gives the matched `text` and its `start` and `end` as character offsets, with the end excluded.

### annotate: fill in several questions for each record

```ruby
questions = {
  "refund" => ThinkThen::DecideQuestion.new("Does the customer request a refund?"),
  "topic" => ThinkThen::ChooseQuestion.new("What is the message about?",
                                           choices: ["billing", "shipping", "other"]),
  "urgency" => ThinkThen::ScoreQuestion.new("How urgent is the request?",
                                            levels: ["routine", "urgent"]),
}
client.annotate_each(questions, messages) do |form|
  is_refund = form[:refund].answer
  topic = form[:topic].answer
  urgency = form[:urgency].answer
  puts "#{form.position} #{is_refund} #{topic} #{urgency}"
end
```

Each question has its own outcome, and its `answer` raises on not sure or failure, as a row's does. `form[name]` takes a String or Symbol and raises `KeyError` for a name you never added. For one record, `client.annotate(questions, message)` returns the row directly. An annotate form cannot hold rank or annotate questions.

### recognize: pull out each name and its kind

```ruby
names = client.recognize("Morgan works for Example Group.",
                         kinds: ["person", "organization"]).answer
names.each { |name| puts "#{name.kind}: #{name.text}" }
```

### relate: link names by the relations you allow

```ruby
works_for = ThinkThen::RelationRule.new(source: "person", target: "organization",
                                        relation: "works_for")
relations = client.relate("Morgan works for Example Group.\nPlease refund my order.",
                          rules: [works_for]).answer
relations.each do |relation|
  puts "#{relation.source.text} #{relation.relation} #{relation.target.text}"
end
```

`ThinkThen::RelationRule.new` also takes `phrase:` to word the relation and `symmetric: true` to make it undirected. Relate finds its own names, so it needs no recognize call first. [Relation rule fields](service.md#relation-rule-fields) describes each field.

## Judge many records

Every one-record function has an `_each` twin, such as `decide_each`, that takes any Enumerable: an Array, a Set, or `File.foreach("messages.txt", chomp: true)`, which sends each line as the command would. `filter` and `rank` take many records already. ThinkThen packs the records into batches; `ThinkThen::Client.open(batch: 16)` sets the size.

With a block, an `_each` call yields one row per record and closes the call when the block ends. Without a block, it returns an Enumerator you read once; its `size` is your input's size when the input has one. Call `close` on it in `ensure` if you stop early. A failed or not-sure record is a row like any other, so pattern matching keeps every record without stopping the batch:

```ruby
client.decide_each("Does the customer request a refund?", messages) do |refund_check|
  case refund_check
  in ThinkThen::Answered then puts "#{refund_check.position} #{refund_check.yes?}"
  in ThinkThen::Unsure then puts "#{refund_check.position} not sure: ask a person"
  in ThinkThen::Failed then puts "#{refund_check.position} failed: #{refund_check.error.code}"
  end
end
```

Each row also carries `id`, the record's own `id`, and `source`, the backend and model that answered. `row.to_json` and `JSON.generate(row)` write the whole row, so `out.puts(row.to_json)` inside an `_each` block saves every outcome as JSON Lines.

## Say what yes, no and each option mean

Decide and filter take `yes_means:` and `no_means:`. Choose, tag, score and rank take `descriptions:`, a Hash from each option or level to what it means. The answers keep their short names.

```ruby
is_refund = client.decide?("Does the customer request a refund?",
                           "Please refund my order.",
                           yes_means: "The customer asks for money back",
                           no_means: "The customer asks for anything else")
topic = client.choose("What is the message about?",
                      "Please refund my order.",
                      choices: ["billing", "shipping", "other"],
                      descriptions: {"billing" => "Invoices, charges and refunds",
                                     "shipping" => "Delivery and parcels",
                                     "other" => "Anything else"}).answer
```

## Records and field

A record is a String, or a Hash of JSON values with String or Symbol keys. A String stays text even when it holds JSON. A Hash record can carry an `"id"`, which comes back on its row.

`field: "/text"` judges one field of each record, and `field: [:text]` is the same pointer written as a key path. Without field, ThinkThen judges the whole record. In a pointer string, write `~1` for a `/` in a key and `~0` for a `~`.

```ruby
tickets = [
  {"id" => "a1", "text" => "Please refund my order."},
  {"id" => "a2", "text" => "Where is my parcel?"},
]
client.filter("Does the customer request a refund?", tickets,
              field: "/text") do |refund_request|
  puts refund_request["id"]
end
```

A one-record call takes exactly one record; pass many to the `_each` form. A `nil` record, or a record without the field the pointer selects, gives a failed row with `record.missing` and sends no request.

## Read the probability and details

Answered and not-sure rows carry `probability`, a Float. For decide and filter it is the probability of yes, even when the answer is no. For the other functions it is the probability of the answer given.

An answered or not-sure row also has `detail`. Choose and tag give a `LabelDetail` with each label's probability. Score and rank give a `LevelDetail` with each level's probability, in level order. Recognize gives a `RecognizeDetail`, and relate gives a `RelationDetail` with every name it found in `names` and every scored pair in `pairs`.

## Save and load a toolbox

A toolbox is a file of named questions. Save this as `toolbox.json`:

```json
{"entries":{"refund":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

Load it, pick a question by name, and pass it to its function:

```ruby
toolbox = ThinkThen::Toolbox.load_file("toolbox.json")
client.choose_each(toolbox["topic"], tickets) do |topic|
  puts "#{topic.id} #{topic.answer}"
end
```

A saved question keeps its own options, such as its field and choices, so the call takes only `name:` and `cancel:` beside it. `toolbox["name"]` raises `KeyError` for a name the toolbox lacks. `toolbox.to_json` writes the saved format, `ThinkThen::Toolbox.from_json(text)` reads it from a String, and `ThinkThen::Toolbox.new(questions: {topic: question})` makes one in code. `ThinkThen::AnnotationQuestions.load_file(path)` reads a toolbox to pass to `annotate`.

## Settings, sharing and cancellation

`ThinkThen::Client.open` takes `backend:` (such as `:openai`), `model:`, `url:`, `api_key:`, `batch:` and `retry:`. Without `backend:`, it reads `THINKTHEN_BACKEND`, as the command does. `retry: ThinkThen::Retry.new(max_retries: 2, initial_backoff: 0.5, attempt_timeout: 30.0)` takes seconds; `max_retries: 0` turns retries off. The key never shows in `inspect`.

`ThinkThen.configure { |config| config.backend = :openai }` sets up one shared client before its first use. Then `ThinkThen.decide?` and the other module calls use it, and `ThinkThen.close` closes it. A client belongs to the process that opened it, so open a new one after `fork`.

Pass `cancel: ThinkThen::CancelToken.new` to any call; `cancel` on the token stops every call it is attached to. Ctrl-C during a call stops it and raises `ThinkThen::CancelledError`. Closing a client stops its running calls.

## Reference

[Read the API reference](../../sdk/ruby/API.md). The gem ships RBS signatures for every public name in `sig/thinkthen.rbs`, so Steep and editors with RBS support check your calls.
