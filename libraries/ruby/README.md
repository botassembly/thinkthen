# ThinkThen for Ruby

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
gem install thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```ruby
require "thinkthen"

messages = ["Please refund the extra fee on my invoice.",
            "My parcel went to the wrong address.",
            "I cannot reset my password."]
refund_question = {decide: "Does the customer ask for a refund?",
                   true: "refund", false: "keep"}
team_question = {choose: "Which team owns this?", options: {
  "billing" => "Invoices, fees, and refunds.",
  "shipping" => "Parcels and delivery.",
  "account" => "Logins and passwords."
}}
ThinkThen::Client.open do |client|
  refund_routes = client.decide(refund_question, messages).value
  teams = client.choose(team_question, messages).value
  messages.zip(refund_routes, teams).each do |text, route, team|
    puts "#{team}: #{route}: #{text}"
  end
end
```

Read `.value` in one line. The refund routes are the authored strings `refund` and `keep`, or `nil` for not sure. Keep the call to read `.probability`, `.results` for each record and `.facts` for request costs. A failure raises an exception.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/ruby/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
