# Rust functions that answer questions about text

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Use Rust 1.95 or later. Run `cargo add thinkthen --no-default-features`. The crate is the engine. Set `THINKTHEN_API_KEY` in the environment first.

A one-record `engine.run` call returns `Err(thinkthen::Error)` when its record could not be judged. A successful call returns an answered or not-sure row, with `is_failed()` false and `error()` returning `None`. Many-record calls retain failed rows, so the shared row type has three states:

- **answered**: there is an answer, and `result.answer()` gives it.
- **not sure**: the model's number fell between the two bars you set, or no choice was likely enough.
- **failed**: the record could not be judged, or the service did not respond.

Three rules hold for every function on this page.

1. **Reading the answer fails when there is none.** A returned one-record row can give `Err(NoAnswer::Unsure { .. })`. A many-record row, form entry or saved row can also give `Err(NoAnswer::Failed(error))`. A missing answer never reads as "no" or as an empty value.
2. **Checking the state never fails.** `result.is_answered()`, `result.is_unsure()`, `result.is_failed()`, `result.probability()` and `result.error()` are always safe to call.
3. **Printing never fails.** `println!("{result}")` shows the answer, `unsure`, or `failed:` with the reason.

## decide

### The simplest if

Rust's `if` takes only a `bool`, so the `if` reads the answer.

```rust
use thinkthen::{Decide, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let asks_refund = Decide::new("Does the customer request a refund?")?;

    if engine.run(&asks_refund, "Please refund my order.")?.answer()? {
        println!("refund queue");
    } else {
        println!("inbox");
    }
    Ok(())
}
```

If the message could not be judged, `engine.run` returns `Err(thinkthen::Error)` and its `?` passes that failure up. The `if` never receives a failed row and never falls through to `inbox` for a failure. With one bar, a returned decide row always answers yes or no; two bars can make its answer read return not sure.

Of the two `?` marks on the `if` line, the first handles any call failure, including a failed judgement. The second is the loud answer read, which can return not sure when the question uses two bars. The later examples are the body of a `main` like this one, with `engine` already made.

### Yes, no, or not sure

Set two bars and decide can also be not sure. `is_answered()` tells you whether there is a yes or no to read.

```rust
use thinkthen::{Decide, Threshold};

let asks_refund = Decide::new("Does the customer request a refund?")?
    .threshold(Threshold::band(0.2, 0.8)?);
let is_refund = engine.run(&asks_refund, "Where is my parcel?")?;

if is_refund.is_answered() {
    println!("{}", if is_refund.answer()? { "refund queue" } else { "inbox" });
} else {
    println!("a person reads it");
}
```

You can also `match` on the read. The call above succeeded, so only not sure can occur in its error arm at runtime. Its shared `NoAnswer` type still contains `Failed` for bulk rows and form entries; a statically exhaustive match must account for that case too.

```rust
match is_refund.answer() {
    Ok(true) => println!("refund queue"),
    Ok(false) => println!("inbox"),
    Err(_) => println!("a person reads it"),
}
```

### Did the call work

```rust
let asks_refund = Decide::new("Does the customer request a refund?")?;
match engine.run(&asks_refund, "Please refund my order.") {
    Ok(is_refund) => println!("{is_refund}"),
    Err(error) => println!("the call did not complete: {error}"),
}
```

The `Err` arm handles all call failures, not only a failed judgement. Rust keeps the existing `thinkthen::Error`: the promoted row error uses that same type as usage, setup, authentication, cancellation and other call errors. Its existing code, message and HTTP status remain available; there is no new Rust `Failed` exception or new error category. `error.code()` and `error.code().family()` describe the cause without changing its classification.

Not sure is a successful call: the `Ok` arm receives its row and printing is safe. Reading that row's answer can return `NoAnswer::Unsure`; not sure never becomes `thinkthen::Error`.

### Both bars and the probability

```rust
let asks_refund = Decide::new("Does the customer request a refund?")?
    .threshold(Threshold::band(0.2, 0.8)?);
let is_refund = engine.run(&asks_refund, "Where is my parcel?")?;

if let Some(probability) = is_refund.probability() {
    println!("{}", probability.get());
}
```

- No `.threshold(...)`: one bar at 0.5. The answer is always yes or no.
- `.threshold(Cut::new(0.7)?)`: one bar. Yes at 0.7 or above, otherwise no.
- `.threshold(Threshold::band(0.2, 0.8)?)`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

`probability()` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. A returned decide row has a probability even when not sure; a failed bulk row has `None`. `.get()` gives the `f64`.

The number is the model's own. To choose bars for your data, run examples you have already labelled through `engine.rows`, save each `probability()`, and compare with your labels.

## choose with your own choices

Give your choices as an enum made with `choices!`. The answer comes back as a member of it.

```rust
use thinkthen::{Choose, Engine};

thinkthen::choices! {
    pub enum Topic {
        Billing = "billing",
        Shipping = "shipping",
        Other = "other",
    }
}

fn team_for(topic: Topic) -> &'static str {
    match topic {
        Topic::Billing => "billing team",
        Topic::Shipping => "warehouse",
        Topic::Other => "front desk",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let asks_topic = Choose::<Topic>::new("What is the message about?")?;
    let topic = engine.run(&asks_topic, "I was charged twice for one invoice.")?;

    if topic.is_unsure() {
        println!("a person reads it");
    } else {
        println!("{}", team_for(topic.answer()?));
    }
    Ok(())
}
```

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `topic.is_unsure()` before reading the answer. `.threshold(Cut::new(0.3)?)` moves the bar.

The text after each `=` (`"billing"`, `"shipping"`, `"other"`) is what the model reads, so make it a plain word or short phrase, not a code. The member names are for your code. `choices!` makes an ordinary enum that is `Copy` and prints as its text, so `println!("{}", Topic::Billing)` shows `billing`.

Add a member to `Topic` and the compiler refuses `team_for` until you handle it.

A choose result does not go in an `if`, and is not equal to its answer. `if topic {` and `topic == Topic::Billing` do not compile. Read `topic.answer()?`.

A list of strings also works, `Choose::with_strings("What is the message about?", ["billing", "shipping", "other"])?`, and the answer is then a `&str`.

## Many records

`engine.rows` takes the same question as `engine.run` and many records, and gives one result per record, in order. Each result carries your own record as `item()`.

```rust
use thinkthen::{Decide, Threshold};

let messages = vec![
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
];
let asks_refund = Decide::new("Does the customer request a refund?")?
    .threshold(Threshold::band(0.2, 0.8)?);

for is_refund in engine.rows(&asks_refund, messages.clone())? {
    let is_refund = is_refund?;
    if is_refund.is_unsure() {
        println!("a person reads it: {}", is_refund.item());
    } else if is_refund.is_failed() {
        println!("try again later: {}", is_refund.item());
    } else if is_refund.answer()? {
        println!("refund queue: {}", is_refund.item());
    } else {
        println!("inbox: {}", is_refund.item());
    }
}
```

One failed record does not stop the loop. Even a collection with one record keeps this bulk behavior; `engine.rows` does not switch to scalar semantics by counting its inputs. Each result keeps its own state, with `position()` counted from 1 and the record's `id()` when it has one. The `?` on `let is_refund = is_refund?;` is for the call itself failing, such as when it is cancelled, and the loop ends after it.

Pass the whole collection to one call so ThinkThen can send the records in batches. A worker thread reads the records, so pass a collection you own, such as a `Vec`. `into_item()` takes your record back out of a result. Dropping the results stops the call, so an early `break` is safe. `.collect::<Result<Vec<_>, _>>()?` collects every result.

To save every result, write `serde_json::to_string(&is_refund)?` as a line of JSON. It includes the state, so nothing is lost.

To stop at a failed row, use the shared row's explicit failure check:

```rust
for is_refund in engine.rows(&asks_refund, messages.clone())? {
    let is_refund = is_refund?.err_if_failed()?;
    println!("{is_refund}");
}
```

The first `?` handles an iterator-level call error, such as cancellation. `err_if_failed()` then returns the failed row's `thinkthen::Error`, or the same row unchanged. A not-sure row passes this check and prints safely. The method stays on the shared row type, but an ordinary successful scalar call needs no additional failure check.

## Many-result not sure or failed

```rust
for is_refund in engine.rows(&asks_refund, messages.clone())? {
    let is_refund = is_refund?;
    if is_refund.is_unsure() {
        println!("a person reads it");
    } else if is_refund.is_failed() {
        println!("try again later");
    } else if is_refund.answer()? {
        println!("refund queue");
    } else {
        println!("inbox");
    }
}
```

Yes and no are the two answers. Not sure and failed are the two ways a bulk row can have none. To name every case and have the compiler confirm none is missed, `match` on the read without a catch-all arm. Each arm also hands you what belongs to its state: the probability or the error.

```rust
use thinkthen::{Decide, NoAnswer, Row};

fn where_it_goes<T>(is_refund: &Row<Decide, T>) -> String {
    match is_refund.answer() {
        Ok(true) => "refund queue".to_string(),
        Ok(false) => "inbox".to_string(),
        Err(NoAnswer::Unsure { probability }) => {
            format!("a person reads it: {}", probability.get())
        }
        Err(NoAnswer::Failed(error)) => format!("try again later: {}", error.code()),
    }
}

for is_refund in engine.rows(&asks_refund, messages)? {
    let is_refund = is_refund?;
    println!("{}: {}", where_it_goes(&is_refund), is_refund.item());
}
```

Leave out an arm and the code does not compile. The shared `Row`/`NoAnswer` types still require a failed arm even if the function is given a successful scalar row, where that arm is unreachable at runtime. `status()` gives the bare state, `Status::Answered`, `Status::Unsure` or `Status::Failed`, for counting or storage. `where_it_goes` is generic over the caller's item type so it accepts rows carrying the original input as well as scalar rows.

## filter

filter means "give me the yeses". It hands back your own records, in order.

```rust
use thinkthen::Filter;

let messages = vec![
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
];
let asks_refund = Filter::new("Does the customer request a refund?")?;

for refund_request in engine.filter(&asks_refund, messages)? {
    println!("refund queue: {}", refund_request?);
}
```

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `.threshold(Cut::new(0.7)?)` moves the bar. Two bars are refused: `Filter::threshold` takes a `Cut`, so `Threshold::band` does not compile there. When you want not sure, use `engine.rows` with a `Decide` question, as in the section above.

If a record fails, you still get every kept record first. Then filter gives one `Err(FilterError)` as its last item, and its `rows()` holds every failed result with its `item()`.

```rust
for refund_request in engine.filter(&asks_refund, messages)? {
    match refund_request {
        Ok(message) => println!("refund queue: {message}"),
        Err(failure) => {
            for failed in failure.rows() {
                println!("try again later: {} ({failed})", failed.item());
            }
        }
    }
}
```

## tag

Labels work as choices do. The answer is a slice of the members that fit, and it can be empty.

```rust
use thinkthen::Tag;

thinkthen::choices! {
    pub enum Topic {
        Billing = "billing",
        Shipping = "shipping",
    }
}

let asks_topics = Tag::<Topic>::new("Which topics appear?")?;
let topics = engine.run(&asks_topics, "I was charged twice and my parcel is late again.")?;

if topics.answer()?.contains(&Topic::Billing) {
    println!("billing team");
}
for topic in topics.answer()? {
    println!("{topic}");
}
```

tag is never not sure. `.threshold(...)` sets how likely a label must be to count.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it. A scale declared with `choices!` follows the members' order in that declaration, lowest first. The words are labels, not sort keys. The same order applies to rank and score questions inside a form.

```rust
use thinkthen::Score;

thinkthen::choices! {
    pub enum Urgency {
        Routine = "routine",
        Urgent = "urgent",
    }
}

let asks_urgency = Score::<Urgency>::new("How urgent is the request?")?;
let urgency = engine.run(&asks_urgency, "Please refund my order.")?;

if urgency.answer()? > 0.5 {
    println!("answer today");
}
println!("{:.2}", urgency.answer()?);
```

The score is a position between your levels, not one of them, so it is an `f64` and there is nothing to `match`. To get exactly one level back, ask with `Choose`. score is never not sure.

## rank

rank puts many records in order along your levels, highest first.

```rust
use thinkthen::Rank;

let messages = vec![
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
];
let asks_urgency = Rank::<Urgency>::new("How urgent is the request?")?;

for ranked in engine.rank(&asks_urgency, messages)? {
    let ranked = ranked?;
    println!("{} {}", ranked.answer()?, ranked.item());
}
```

`ranked.answer()?` is the rank, starting at 1, and `ranked.item()` is your record. rank reads the whole collection before it gives anything back. `.top(5)` on the question returns the five highest-ranked records, after judging the collection. A failed record comes back after the ranked ones, with `is_failed()` true.

## find

find points to the one line that answers.

```rust
use thinkthen::Find;

let message = "Morgan works for Example Group.\nPlease refund my order.";
let asks_action = Find::new("Which line states the requested action?")?;
let action = engine.run(&asks_action, message)?;

if action.is_unsure() {
    println!("a person reads it");
} else {
    match action.answer()? {
        None => println!("no line answers"),
        Some(span) => println!("{}", span.text()),
    }
}
```

The answer is `Some` of a `Span` with `text()`, `start()` and `end()`, or `None` when no line answers. The offsets count UTF-8 bytes, as Rust strings do, so `&message[span.start()..span.end()]` is the line. Like choose, find can be not sure with no bar set. Saving a result with `serde_json` writes character offsets. Loading that saved row preserves character offsets because the original prefix text is not in the row; byte slicing applies to results just executed, not to deserialized saved rows.

## recognize

```rust
use thinkthen::Recognize;

let finds_names = Recognize::new(["person", "organization"])?;
let names = engine.run(&finds_names, "Morgan works for Example Group.")?;

for name in names.answer()? {
    println!("{} {}", name.kind(), name.text());
}
```

Each name also has `start()`, `end()` and `probability()`. The answer can be empty. recognize is never not sure.

## relate

```rust
use thinkthen::{Relate, RelationRule};

let message = "Morgan works for Example Group.\nPlease refund my order.";
let rules = [RelationRule::new("person", "organization", "works_for")?];
let finds_relations = Relate::new(rules)?;
let relations = engine.run(&finds_relations, message)?;

for relation in relations.answer()? {
    println!(
        "{} {} {}",
        relation.source().text(),
        relation.relation(),
        relation.target().text()
    );
}
```

A rule names the kind it starts from, then the kind it points to, then the relation. Each relation has `source()`, `target()`, `relation()`, `probability()` and `either()`. `relations.detail()` holds every name found, in its `names()`, including names in no relation.

## annotate

annotate asks several questions of each record and gives back one form per record.

```rust
use thinkthen::{AnnotatableQuestion, Annotate, Choose, Decide, Score};

let messages = vec![
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
];
let asks_refund = Decide::new("Does the customer request a refund?")?;
let asks_topic = Choose::<Topic>::new("What is the message about?")?;
let asks_urgency = Score::<Urgency>::new("How urgent is the request?")?;

let questions = Annotate::new([
    ("refund", AnnotatableQuestion::from(asks_refund)),
    ("topic", asks_topic.into()),
    ("urgency", asks_urgency.into()),
])?;

for form in engine.rows(&questions, messages)? {
    let form = form?;
    if form.is_failed() {
        println!("try again later: {}", form.item());
        continue;
    }
    let entries = form.answer()?;
    let is_refund = entries.decide("refund")?;
    let topic = entries.choose::<Topic>("topic")?;
    let urgency = entries.score("urgency")?;
    if is_refund.is_failed() || topic.is_failed() || urgency.is_failed() {
        println!("try again later: {}", form.item());
    } else if is_refund.is_unsure() || topic.is_unsure() || urgency.is_unsure() {
        println!("a person reads it: {}", form.item());
    } else if is_refund.answer()? {
        println!("refund queue: {} {:.2}", topic.answer()?, urgency.answer()?);
    }
}
```

Each entry of a form is a result of its own function, with its own state. `entries.decide("refund")?` is a decide result, so its `answer()?` works in an `if`. One entry can be not sure while the others answer. Asking for a name the form does not have, or with the wrong function, is a mistake in the call and returns `Err`. Here `Topic` is the three-choice enum from the choose section.

A form can be answered while an entry failed. This example checks all three entries before reading their answers: failed entries go to retry, not-sure entries go to a person, and only answered entries reach the refund branch.

For one record, `engine.run(&questions, message)` returns `Err(thinkthen::Error)` only when the parent form itself failed or the call otherwise failed. An answered parent is returned even if one, several or all of its entries failed. The scalar boundary reads only the parent's own error, never the recursive failure helper or the count of failed entries.

```rust
let form = engine.run(&questions, "Please refund my order.")?;
let form = form.err_if_failed()?;
println!("{form}");
```

The second line is optional. It explicitly returns an error for a failed form or any failed entry, and otherwise hands back the same form. Not-sure entries do not fail this check. The helper also applies to bulk forms; neither a successful parent nor its answer read authorizes reading an unchecked entry's answer.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```rust
let asks_refund = Decide::new("Does the customer request a refund?")?
    .yes_means("the customer wants money back, including for a duplicate charge")
    .no_means("the customer wants anything else");

let asks_topic = Choose::<Topic>::new("What is the message about?")?.descriptions([
    (Topic::Billing, "charges, invoices and refunds"),
    (Topic::Shipping, "parcels and delivery"),
]);
```

`Decide` and `Filter` take `yes_means` and `no_means`. `Choose`, `Tag`, `Score` and `Rank` take `descriptions`, keyed by your choices, labels or levels.

## Records and fields

A record is a `&str`, a `String`, or a `serde_json::Value` (add it with `cargo add serde_json`). `.field("/text".parse()?)` on the question picks one field of a JSON object. Without it, ThinkThen judges the whole record.

Give JSON records an `id`. Each result carries it back as `id()`.

To pass your own struct, implement `From<&YourType> for Record`. `item()` then hands your struct back.

A one-record call given a list does not compile. Use `engine.rows`. A blank record (an empty string, only spaces, or `{}`) returns `Err(thinkthen::Error)` from `engine.run` with code `record.empty`, before any service request. The same input in `engine.rows` remains a failed row, even in a one-item collection.

## Async

Add the `stream` feature: `cargo add thinkthen --no-default-features --features stream`. `into_stream()` turns the results of `engine.rows` or `engine.filter` into a `futures_core::Stream` of the same items. The crate brings no async runtime. This example uses `futures::executor::block_on` from `cargo add futures`.

```rust
use futures::StreamExt;
use thinkthen::{Decide, Engine};

async fn refunds() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let asks_refund = Decide::new("Does the customer request a refund?")?;
    let messages = vec!["Please refund my order.", "Where is my parcel?"];

    let mut refund_checks = engine.rows(&asks_refund, messages)?.into_stream();
    while let Some(is_refund) = refund_checks.next().await {
        let is_refund = is_refund?;
        println!("{is_refund} {}", is_refund.item());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(refunds())
}
```

`engine.run` has no async form. For an asynchronous many-record interface with one input, pass a list of one to `engine.rows` and use `into_stream()`. It retains bulk semantics: a failed record is a failed row, not an iterator-level `Err`. The iterator can still report call errors such as cancellation. Dropping the stream stops the call.

## Saved questions

A question is a value. Build it once with its options, and many calls can share one definition.

```rust
use serde_json::json;

let asks_refund = Decide::new("Does the customer request a refund?")?
    .threshold(Threshold::band(0.2, 0.8)?)
    .field("/text".parse()?);
let is_refund = engine.run(
    &asks_refund,
    json!({"id": "a1", "text": "Please refund my order."}),
)?;
```

There is one type per function, such as `Choose` and `Score`. A `Toolbox` keeps named questions in a JSON file:

```rust
use thinkthen::Toolbox;

let toolbox = Toolbox::from_json(&std::fs::read_to_string("toolbox.json")?)?;
let asks_topic = toolbox.choose::<Topic>("topic")?;
let topic = engine.run(&asks_topic, json!({"id": "a1", "text": "Where is my parcel?"}))?;
```

`toolbox.choose::<Topic>` returns `Err` when the saved choices are not exactly `Topic`'s.

## Settings

`Engine::from_env()` reads `THINKTHEN_API_KEY` and `THINKTHEN_BACKEND` from the environment. To choose settings yourself, build the engine from `Settings` and make the same calls on it.

```rust
use std::time::Duration;
use thinkthen::{Batch, Retry, Settings};

let engine = Settings::default()
    .model("...")
    .batch(Batch::try_from(16)?)
    .retry(Retry::new(5, Duration::from_millis(500), Duration::from_secs(120))?)
    .engine(|name| std::env::var(name).ok())?;
let is_refund = engine.run(&asks_refund, "Please refund my order.")?;
```

The options are `with_backend`, `url`, `model`, `api_key`, `batch` and `retry`. `Retry::new` takes the number of retries, the first wait and the time allowed for one attempt, as `Duration`s. `engine` takes the function that looks up any environment variable it still needs. Keep the key in the environment, not in your code.

## Usage

```rust
let engine = Engine::from_env()?;
engine.run(&asks_refund, "Please refund my order.")?;
let usage = engine.usage();
println!(
    "{} {:?} {:?}",
    usage.attempts, usage.input_tokens.reported, usage.output_tokens.reported
);
```

`engine.usage()` counts everything the engine and its clones have sent so far: `attempts`, `responses`, retries, bytes sent, and tokens in and out as the service reported them.

## Cancel

```rust
use thinkthen::Cancel;

let stop = Cancel::default();
let engine = Engine::from_env()?.cancel(stop.clone());
let is_refund = engine.run(&asks_refund, "Please refund my order.");
```

Calling `stop.cancel()` from another thread stops every call on that engine, and each returns `Err` with the code `local.cancelled`. The results of a many-record call also have a `cancel()` of their own, which stops that one call.

## Errors

| What happened | Where it shows | What you get |
| --- | --- | --- |
| A mistake in the call, such as a bad threshold | When you build the question or make the call | `Err(thinkthen::Error)` with a `usage.` code. A list given to a one-record call, or two bars given to filter, does not compile |
| The engine could not start, such as no API key | At `Engine::from_env()` | `Err(thinkthen::Error)` with `usage.key-missing` |
| The call was cancelled | At the call | `Err(thinkthen::Error)` with `local.cancelled` |
| A one-record judgement failed, including blank input or a service failure on its row | At `engine.run` | `Err(thinkthen::Error)` with the unchanged row error code and payload |
| A bulk row or form entry failed | At its answer read or `err_if_failed()` | `Err(NoAnswer::Failed(error))`, or `Err(thinkthen::Error)` from the explicit check |
| An annotate parent failed | At `engine.run`; kept as a failed form by `engine.rows` | `Err(thinkthen::Error)` at the scalar call; failed row in bulk |
| An answered annotate parent contains failed entries | The scalar call returns it; a failed entry's answer read or explicit form check reports failure | `NoAnswer::Failed(error)` from the read, or `thinkthen::Error` from `err_if_failed()` |
| Not sure | When you read the answer | `Err(NoAnswer::Unsure { probability })` |
| You print a result, store it, or ask about its state | Never | |

Two types let you handle by meaning:

- `NoAnswer` is "there was no answer to read" on a shared row or entry. On a successfully returned scalar row, the only reachable `NoAnswer` variant is unsure at runtime. It is `Unsure` or `Failed`, and `Err(_)` on a read covers both.
- `thinkthen::Error` is "something went wrong". Calls use it for existing call errors and, at `engine.run`, for a failed scalar judgement. Bulk rows and entries retain failures, and `NoAnswer::Failed` holds their error. Not sure is never a `thinkthen::Error`, because not sure is a result and not an error. So in a function that returns `Result<_, thinkthen::Error>`, `?` on an answer read does not compile until you say what not sure should do.

`thinkthen::Error` carries `code()`, `message()` and `http_status()`, and `code().family()` groups the codes into usage, record, backend, local and internal. `FilterError` carries `rows()`, the failed results. `NoAnswer::Unsure` carries `probability`.

## Upgrading from 0.1

A call now returns a result, not an `Answer`.

| 0.1 | Now |
| --- | --- |
| `Question::decide("...")?.cut()` | `Decide::new("...")?` |
| `tt.decide(&question, text)?.value()` | `engine.run(&question, text)?` gives a result, and `.answer()?` is the `bool` |
| `match` on `Answer::Yes`, `No`, `Unsure` | A failed single judgement is `Err(Error)` from `engine.run`; on a returned row match `answer()`. The shared static type still requires the `NoAnswer::Failed` arm, which remains reachable for bulk rows and entries |
| `tt.filter(&question, items)` | `engine.filter(&question, items)?.collect::<Result<Vec<_>, _>>()?` |
| a `choices!` enum | Still a `choices!` enum, named on the question: `Choose::<Topic>::new(question)?` |
| `QuestionSet::load("form.json")` and `tt.annotate` | `Annotate::new([(name, question), ...])?` and `engine.rows` |
| `--features bundled-sqlite` | `--no-default-features` |

`Engine::from_env()` keeps its name. One `Engine` runs every question.

---

