# ThinkThen for Ruby

Run `gem install thinkthen --version 0.2.0`. The platform gem includes the native engine.

The example uses the stub backend and needs no key or service.

```ruby
require "thinkthen"

ThinkThen::Client.open(backend: :stub) do |client|
  client.decide_each("Does the message request a refund?", ["Please refund my order."]) do |row|
    puts row.answer if row.answered?
  end
end
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/ruby/API.md).
