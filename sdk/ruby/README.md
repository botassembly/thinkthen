# Ruby SDK

The Ruby SDK asks questions about text from your Ruby code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

Install it with CRuby 3.3 or later:

```sh
gem install thinkthen --version 0.2.0
```

Or run `bundle add thinkthen --version 0.2.0`. It carries its own engine. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```ruby
require "thinkthen"

ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?",
                             "Please refund my order.")
  puts is_refund
end
```

Output:

```text
true
```

`true` means yes: the message asks for a refund.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. `decide?` raises `ThinkThen::UnsureError` when decide is not sure, and `ThinkThen::RecordError` when ThinkThen could not judge the message, so neither one passes for `false`. Send both to a person.

```ruby
require "thinkthen"

message = "I was charged twice for one invoice."
ThinkThen::Client.open do |client|
  is_refund = client.decide?("Does the customer request a refund?",
                             message, threshold: 0.2..0.8)
  puts is_refund
rescue ThinkThen::UnsureError => unsure
  puts "Not sure (#{unsure.probability}): ask a person."
rescue ThinkThen::Error => failure
  puts "#{failure.code}: #{failure.message}"
end
```

Output:

```text
Not sure (0.7): ask a person.
```

## Next

The [Ruby SDK page](https://thinkthen.dev/sdks/ruby/) shows all ten functions. The [Ruby guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/ruby.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/ruby/API.md) cover every option.

## Usage

Copy cumulative usage from an explicit client:

```ruby
require "thinkthen"

ThinkThen::Client.open do |client|
  client.decide("Ready?", "A message")
  snapshot = client.usage
  puts snapshot.attempts
  puts snapshot.input_tokens.reported
end
```

Snapshots are independent values. Token totals count only provider-reported usage; finished attempts with missing usage are counted separately. Active calls and attempts can still add spending. Use a separate client when you need isolated totals. No prices or per-row token allocation are inferred.

After `client.close`, `client.usage` returns the settled copy after owned calls were cancelled.

## Request allowance

```ruby
require "thinkthen"

ThinkThen::Client.open(max_requests: 100) do |client|
  client.decide("Ready?", "A message")
end
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

## Local response storage

Explicitly select `cache`, `record`, or `replay` storage; leaving it unset retains no text. For an existing recording:

Create `ThinkThen::ResponseStore.new(path: "./answers", mode: :replay, namespace: "example", model_generation: "model-v1")` and pass it to `ThinkThen::Client.new(response_store: store)`. Read `client.usage.replay_hits` for the copied hit count. This assumes that `./answers` already contains a matching recording.


Store paths must name private local directories. This opt-in feature retains input and output text without encryption. Namespace and model generation are explicit nonsecret labels; change the generation when the provider's model binding changes. Replay opens existing recorded answers without credentials or network fallback, and a missing answer fails with `local.replay-miss`. Cache/replay hit counters count materialized asks, not output rows, and do not add historical token usage to current spending. Optional limits use the engine defaults: 64 MiB logical payload, 4096 eligible asks, 86400 seconds cache age, and 5000 ms busy timeout. Recordings do not expire or get evicted to continue a recording. Filesystem allocation may exceed the logical payload limit. No environment variable enables retention.
