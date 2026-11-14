# Ruby functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use Ruby 3.3 or later. Run `gem install thinkthen`. The gem includes the engine. Set `THINKTHEN_API_KEY` in the environment first.

A one-record call raises `ThinkThen::FailedError` at the call when its record could not be judged. A successful call returns an answered or unsure result, with `failed?` false and its own `error` set to `nil`. Many-record calls keep failures in their results, so the shared result type has three states:

- **answered**: there is an answer, and `result.answer` is it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer raises when there is none.** On a returned one-record result, `result.answer` can raise `ThinkThen::UnsureError`. On a many-record result, form entry or saved row, it can also raise `ThinkThen::FailedError`. A missing answer never reads as "no" or as an empty value.
2. **Checking the state never raises.** `result.answered?`, `result.unsure?`, `result.failed?`, `result.probability` and `result.error` are always safe to call.
3. **Printing never raises.** `puts result` and `"#{result}"` show the answer, `unsure`, or `failed:` with the reason. A yes prints as `true` and a no as `false`, the same words the `thinkthen` command prints.

## decide

### The simplest if

Ask a decide result `yes?` in an `if`.

```ruby
require "thinkthen"

if ThinkThen.decide("Does the customer request a refund?", "Please refund my order.").yes?
  puts "refund queue"
else
  puts "inbox"
end
```

If the message could not be judged, `ThinkThen.decide` raises `ThinkThen::FailedError` at the call. The `if` never receives a failed result and never falls through to `inbox` for a failure. With one bar, a returned decide result is yes or no; two bars can make its answer read raise `ThinkThen::UnsureError`.

Always write `yes?`. A result is an object, and Ruby counts every object except `nil` and `false` as true, so `if is_refund` takes the yes branch every time. Ruby gives a library no way to change that. Steep can report an unreachable-branch hint when it knows the type, but its default hint does not make the build fail.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `answered?` tells you whether there is a yes or no to read.

```ruby
require "thinkthen"

is_refund = ThinkThen.decide(
  "Does the customer request a refund?",
  "Where is my parcel?",
  threshold: 0.2..0.8,
)

if is_refund.answered?
  puts is_refund.yes? ? "refund queue" : "inbox"
else
  puts "a person reads it"
end
```

You can also let the read raise and rescue `ThinkThen::UnsureError`. The call above has already succeeded, so this read cannot report a failed result.

```ruby
begin
  puts is_refund.yes? ? "refund queue" : "inbox"
rescue ThinkThen::UnsureError
  puts "a person reads it"
end
```

### Did the call work

```ruby
begin
  is_refund = ThinkThen.decide("Does the customer request a refund?", "Please refund my order.")
  puts is_refund
rescue ThinkThen::FailedError => failure
  puts "could not judge it: #{failure}"
rescue ThinkThen::Error => failure
  puts "the call did not complete: #{failure}"
end
```

`ThinkThen::FailedError` reports failed judgements. `ThinkThen::Error` also covers usage, setup, cancellation and internal call errors, keeping their existing classifications. Printing a returned result does not read its answer.

Not sure counts as a successful call: ThinkThen did its job and returned an unsure result. Its answer read can raise `ThinkThen::UnsureError`, which neither rescue above handles.

### Both bars and the probability

```ruby
is_refund = ThinkThen.decide(
  "Does the customer request a refund?",
  "Where is my parcel?",
  threshold: 0.2..0.8,
)
puts is_refund.probability
```

- No `threshold:`: one bar at 0.5. The answer is always yes or no.
- `threshold: 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `threshold: 0.2..0.8`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. It is present even for an unsure decide result. A failed many-record result has `nil` probability.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `ThinkThen.decide_each`, save each `probability`, and compare with your labels.

## choose with your own choices

Give your choices as a frozen list of symbols. The answer comes back as one of them.

```ruby
require "thinkthen"

TOPIC = %i[billing shipping other].freeze

def team_for(topic)
  case topic
  in :billing then "billing team"
  in :shipping then "warehouse"
  in :other then "front desk"
  end
end

topic = ThinkThen.choose(
  "What is the message about?",
  "I was charged twice for one invoice.",
  choices: TOPIC,
)

if topic.unsure?
  puts "a person reads it"
else
  puts team_for(topic.answer)
end
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `topic.unsure?` before reading the answer. `threshold: 0.3` moves the bar.

The symbols (`:billing`, `:shipping`, `:other`) are the words the model reads, so make them plain words, not codes. The same symbols are what your code matches on.

Add a symbol to `TOPIC` and `team_for` raises `NoMatchingPatternError` the first time that topic arrives, so a new choice can't slip through as `nil`. Ruby does not check this before the program runs. A one-line test does: `TOPIC.each { |topic| team_for(topic) }`.

A choose result has no `yes?`, and is not equal to its answer. `topic.yes?` raises `NoMethodError`. `topic == :billing` raises `TypeError` and tells you to read `topic.answer`. Written the other way round, `:billing == topic` is quietly false, because Ruby asks the symbol and not the result. So always compare `topic.answer`.

Value equality and hashing cover only `function`, `status`, `answer`, `probability`, `error`, `position`, `id`, `source` and `detail`, using this language's names. They never include `item`, including on entries inside a form. Changing the caller's record cannot change a result's equality or hash.

A list of strings also works, `choices: ["billing", "shipping", "other"]`, and the answer is then a `String`. Use strings when a choice is a phrase with spaces.

## Many records

Each one-record function has an `_each` form that takes many records and gives your block one result per record, in order. Each result carries your own record as `.item`.

```ruby
require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]

ThinkThen.decide_each(
  "Does the customer request a refund?", messages, threshold: 0.2..0.8
) do |is_refund|
  if is_refund.unsure?
    puts "a person reads it: #{is_refund.item}"
  elsif is_refund.failed?
    puts "try again later: #{is_refund.item}"
  elsif is_refund.yes?
    puts "refund queue: #{is_refund.item}"
  else
    puts "inbox: #{is_refund.item}"
  end
end
```

One failed record does not stop the call. Each result keeps its own state, with `position` counted from 1 and the record's `id` when it has one. An `_each` call remains many-record even with a one-item collection: its failed result is yielded. Collection length never chooses the scalar rule.

To stop on an encountered failed result, use its explicit check:

```ruby
ThinkThen.decide_each(
  "Does the customer request a refund?", messages, threshold: 0.2..0.8
) do |is_refund|
  puts is_refund.raise_if_failed
end
```

`raise_if_failed` raises `ThinkThen::FailedError` for a failed result and otherwise returns that same result. Not sure does not raise here, and printing is safe. The method stays on the shared result class; it is redundant after a successful ordinary scalar call.

Pass the whole collection to one call so ThinkThen can send the records in batches. The call is closed when the block ends, even after an early `break`. Without a block, the call gives an Enumerator, and `.to_a` collects every result and closes it.

To save every result, write `result.to_json` as a line of JSON. It includes the state, so nothing is lost.

## Many-result not sure or failed

Here each result comes from a many-record call, where all three states remain possible.

```ruby
ThinkThen.decide_each(
  "Does the customer request a refund?", messages, threshold: 0.2..0.8
) do |is_refund|
  if is_refund.unsure?
    puts "a person reads it #{is_refund.probability}"
  elsif is_refund.failed?
    puts "try again later #{is_refund.error.code}"
  elsif is_refund.yes?
    puts "refund queue"
  else
    puts "inbox"
  end
end
```

Yes and no are the two answers. Not sure and failed are the two ways of having none. To name every state, use `case` with `in` and no `else`, in a method that returns a value:

```ruby
def where_it_goes(is_refund)
  case is_refund.status
  in :answered then is_refund.yes? ? "refund queue" : "inbox"
  in :unsure then "a person reads it"
  in :failed then "try again later"
  end
end
```

Ruby does not check this before the program runs. What `in` gives you is a loud miss: a state with no branch raises `NoMatchingPatternError`, where `case` with `when` would quietly give `nil`.

Call the routing method on each result:

```ruby
ThinkThen.decide_each(
  "Does the customer request a refund?", messages, threshold: 0.2..0.8
) do |is_refund|
  puts "#{where_it_goes(is_refund)}: #{is_refund.item}"
end
```

`ThinkThen::NoAnswerError` groups `ThinkThen::UnsureError` and `ThinkThen::FailedError` for answer reads on many-record results, form entries and saved rows. Rescue it when either state means there is no answer to use; rescue them separately when their routes differ. Scalar and many-record calls share the same function-specific result class: only a scalar result's own failed state is excluded at runtime.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```ruby
require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]

ThinkThen.filter("Does the customer request a refund?", messages) do |refund_request|
  puts "refund queue: #{refund_request}"
end
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `threshold: 0.7` moves the bar. Two bars are refused with `ThinkThen::UsageError`: when you want not sure, use `ThinkThen.decide_each`, as in the section above.

If a record fails, you still get every kept record first. Then filter raises one `ThinkThen::FailedError`, and its `rows` holds every failed result with its `.item`.

```ruby
begin
  ThinkThen.filter("Does the customer request a refund?", messages) do |refund_request|
    puts "refund queue: #{refund_request}"
  end
rescue ThinkThen::FailedError => failure
  failure.rows.each do |failed|
    puts "try again later: #{failed.item} #{failed.error.code}"
  end
end
```

## tag

Labels work as choices do. The answer is a frozen Array of the symbols that fit, and it can be empty.

```ruby
require "thinkthen"

TOPIC = %i[billing shipping].freeze

topics = ThinkThen.tag(
  "Which topics appear?",
  "I was charged twice and my parcel is late again.",
  labels: TOPIC,
)

puts "billing team" if topics.answer.include?(:billing)
topics.answer.each do |topic|
  puts topic
end
```

tag is never not sure. `threshold:` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. The order of the supplied Array defines the scale.

```ruby
require "thinkthen"

URGENCY = %i[routine urgent].freeze

urgency = ThinkThen.score(
  "How urgent is the request?",
  "Please refund my order.",
  levels: URGENCY,
)

puts "answer today" if urgency.answer > 0.5
puts urgency.answer.round(2)
```

The score is a position between your levels, not one of them, so it is a `Float` and there is nothing to match with `case`. To get exactly one level back, ask with `ThinkThen.choose`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```ruby
require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]

ThinkThen.rank("How urgent is the request?", messages, levels: URGENCY) do |ranked|
  puts "#{ranked.answer} #{ranked.item}"
end
```

`ranked.answer` is the rank, starting at 1, and `ranked.item` is your record. rank reads the whole collection before it gives anything back. `top: 5` selects the five highest-ranked records after judging the collection. A failed record comes back after the ranked ones, with `failed?` true.

## find

find points to the one line that answers.

```ruby
require "thinkthen"

message = "Morgan works for Example Group.\nPlease refund my order."
action = ThinkThen.find("Which line states the requested action?", message)

if action.unsure?
  puts "a person reads it"
elsif action.answer.nil?
  puts "no line answers"
else
  puts action.answer.text
end
```

The answer is a `ThinkThen::Span` with `text`, `start` and `end`, or `nil` when no line answers. `message[span.start...span.end]` is the line. Like choose, find can be not sure with no bar set.

## recognize

```ruby
require "thinkthen"

names = ThinkThen.recognize(
  "Morgan works for Example Group.",
  kinds: ["person", "organization"],
)

names.answer.each do |name|
  puts "#{name.kind} #{name.text}"
end
```

Each name also has `start`, `end` and `probability`. The answer can be empty. recognize is never not sure.

## relate

```ruby
require "thinkthen"

message = "Morgan works for Example Group.\nPlease refund my order."
rules = [ThinkThen::RelationRule.new(source: "person", target: "organization", relation: "works_for")]

relations = ThinkThen.relate(message, rules: rules)

relations.answer.each do |relation|
  puts "#{relation.source.text} #{relation.relation} #{relation.target.text}"
end
```

Each relation has `source`, `target`, `relation`, `probability` and `either`. `relations.detail.names` holds every name found, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record.

```ruby
require "thinkthen"

messages = [
  "Please refund my order.",
  "Where is my parcel?",
  "I was charged twice for one invoice.",
]

questions = ThinkThen::AnnotationQuestions.new(
  refund: ThinkThen::DecideQuestion.new("Does the customer request a refund?", threshold: 0.2..0.8),
  topic: ThinkThen::ChooseQuestion.new("What is the message about?", choices: TOPIC),
  urgency: ThinkThen::ScoreQuestion.new("How urgent is the request?", levels: URGENCY),
)

ThinkThen.annotate_each(questions, messages) do |form|
  if form.failed?
    puts "try again later: #{form.item}"
    next
  end
  entries = form.answer
  is_refund, topic, urgency = entries[:refund], entries[:topic], entries[:urgency]
  if is_refund.failed? || topic.failed? || urgency.failed?
    puts "try again later: #{form.item}"
  elsif is_refund.unsure? || topic.unsure?
    puts "a person reads it: #{form.item}"
  elsif is_refund.answered? && topic.answered? && urgency.answered? && is_refund.yes?
    puts "refund queue: #{topic.answer} #{urgency.answer.round(2)}"
  end
end
```

`form.answer` holds one entry per question, under the name you gave it. Each entry is a result of its own function, with its own state. `form.answer[:refund]` is a decide result, so it has `yes?`. One entry can be not sure or failed while the others answer. Check every entry used by a branch before reading its answer; an answered form alone does not mean its entries all answered. Here `TOPIC` is the three-choice list from the choose section, and `URGENCY` is the list from the score section.

For one record, `ThinkThen.annotate(questions, message)` raises `ThinkThen::FailedError` only when the parent form itself failed. An answered parent is returned even when one, several or all entries failed. The scalar boundary does not run the recursive form check to decide whether to return it.

```ruby
form = ThinkThen.annotate(questions, "Please refund my order.")
form.raise_if_failed
```

The optional second line explicitly raises for a failed entry as well as a failed parent. An unsure entry is not a failure; otherwise this check returns the same form unchanged. It also works on forms yielded by `annotate_each`. Checking only the parent never authorizes reading an unchecked entry's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```ruby
is_refund = ThinkThen.decide(
  "Does the customer request a refund?",
  "I was charged twice for one invoice.",
  yes_means: "the customer wants money back, including for a duplicate charge",
  no_means: "the customer wants anything else",
)

topic = ThinkThen.choose(
  "What is the message about?",
  "I was charged twice for one invoice.",
  choices: TOPIC,
  descriptions: {
    billing: "charges, invoices and refunds",
    shipping: "parcels and delivery",
  },
)
```

decide and filter take `yes_means:` and `no_means:`. choose, tag, score and rank take `descriptions:`, keyed by your choices, labels or levels.

## Records and fields

A record is a String, or a Hash or other JSON-like value. `field: "/text"` picks one field of a Hash. Without it, ThinkThen judges the whole record. A record of your own class works when it has `to_h`, as a `Struct` or a `Data` does.

Give Hash records an `id`. Each result carries it back as `.id`.

A one-record call given an Array raises `ThinkThen::UsageError` and names the `_each` form. A blank record (an empty string, only spaces, or `{}`) raises `ThinkThen::FailedError` at the scalar call with code `record.empty`; a `nil` record does so with code `record.missing`. Neither sends that input to the service. In the `_each` forms, the same inputs remain failed results, even with one input.

## Saved questions

A question object holds a question and its options, so many calls can share one definition. Pass it in place of the question text.

```ruby
asks_refund = ThinkThen::DecideQuestion.new(
  "Does the customer request a refund?",
  threshold: 0.2..0.8,
  field: "/text",
)
is_refund = ThinkThen.decide(asks_refund, {id: "a1", text: "Please refund my order."})
```

There is one class per function, such as `ThinkThen::ChooseQuestion` and `ThinkThen::ScoreQuestion`. A `ThinkThen::Toolbox` keeps named questions in a JSON file:

```ruby
toolbox = ThinkThen::Toolbox.load_file("toolbox.json")
topic = ThinkThen.choose(toolbox.choose(:topic), {id: "a1", text: "Where is my parcel?"})
```

## Settings

The functions on `ThinkThen` use a shared client that reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, open a client and call the same functions on it.

```ruby
ThinkThen::Client.open(model: "...", batch: 16, retry: ThinkThen::RetryPolicy.new(max_retries: 5)) do |client|
  is_refund = client.decide("Does the customer request a refund?", "Please refund my order.")
end
```

The options are `backend:`, `url:`, `model:`, `api_key:`, `batch:` and `retry:`. Times in `ThinkThen::RetryPolicy` are in seconds. Keep the key in the environment, not in your code. The block closes the client when it ends. Open a new client after `fork`.

Ruby has no `async` and `await`, so there is no second client. A call waits without holding Ruby's interpreter lock, so other threads keep running, and one client is safe to share between threads.

## Usage

```ruby
ThinkThen::Client.open do |client|
  client.decide("Does the customer request a refund?", "Please refund my order.")
  usage = client.usage
  puts "#{usage.attempts} #{usage.input_tokens.reported} #{usage.output_tokens.reported}"
end
```

`client.usage` counts everything the client has sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them.

## Cancel

```ruby
stop = ThinkThen::Cancel.new
is_refund = ThinkThen.decide("Does the customer request a refund?", "Please refund my order.", cancel: stop)
```

Calling `stop.cancel` from another thread stops the call, which raises `ThinkThen::CancelledError`. Ctrl-C does the same.

## Errors

| What happened | When it raises | What is raised |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold or an Array given to a one-record call | At the call | `ThinkThen::UsageError`, which is also an `ArgumentError` |
| The client could not start, such as no API key | At the call | `ThinkThen::SetupError` |
| The call was cancelled | At the call | `ThinkThen::CancelledError` |
| A one-record judgement failed, including blank/nil input or a service failure on its row | At the scalar call | `ThinkThen::FailedError` |
| A many-record result or form entry failed | At its answer read, `yes?` or `raise_if_failed` | `ThinkThen::FailedError` |
| An annotate parent failed | At the scalar call; retained as a failed form by `annotate_each` | `ThinkThen::FailedError` at the scalar call or later failed-form read/check |
| An answered annotate parent contains failed entries | The call returns it; reading a failed entry or explicitly checking the form raises | `ThinkThen::FailedError` |
| Not sure | When you read the answer | `ThinkThen::UnsureError` |
| You print a result, store it, or ask about its state | Never | |

Two parents let you rescue by meaning:

- `rescue ThinkThen::NoAnswerError` catches `ThinkThen::UnsureError` and `ThinkThen::FailedError`: there was no answer to read.
- `rescue ThinkThen::Error` catches the library's call errors, including `ThinkThen::FailedError`. It does not catch `ThinkThen::UnsureError`, because not sure is a result and not an error. Ruby's own argument and file errors keep their usual classes.

HTTP 401 or 403 stops the call. The call also stops when every request in its first batch fails and at least one failure has an HTTP status from 400 through 499. These remote call refusals raise `ThinkThen::SetupError`. Other service failures become failed rows: a scalar call promotes its own failed row to `ThinkThen::FailedError`, while `_each` calls retain it as a result.

`ThinkThen::FailedError` retains the former `ThinkThen::RecordError` payload: `code`, `http_status` and `rows`, the failed results. Only the concrete host exception name changes; result classes, status symbols, wire codes such as `record.empty` and engine error families do not change. `ThinkThen::UnsureError` carries `probability` and `row`.

## Upgrading from 0.1

The baseline is the [published 0.1.2 package/source](https://rubygems.org/gems/thinkthen-0.1.2-x86_64-linux.gem). Replace its call envelopes with the results shown above.

| 0.1.2 | Now |
| --- | --- |
| `ThinkThen.decide(q, text).value`; `nil` for not sure | Scalar failures raise `ThinkThen::FailedError` at the call; a returned result is answered or unsure, with `.unsure?` and the loud answer read |
| `ThinkThen::Engine`, `ThinkThen.question(...)` | Client and typed saved questions/toolbox |
| Per-call `.facts` | Client usage |
