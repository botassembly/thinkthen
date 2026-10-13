# ThinkThen for ruby

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
gem install thinkthen
```

## Example

Configure a model backend before running this example.

```ruby
require "thinkthen"

result = ThinkThen::Client.open(cache: false) do |client|
  client.decide("Does this ask for a refund?", "Please refund my order.")
end
puts result.value
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
