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
