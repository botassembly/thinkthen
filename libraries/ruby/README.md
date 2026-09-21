# The Ruby surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the shim:

```ruby
require "thinkthen"

ThinkThen.decide("Does the customer ask for a refund?", text)  # => true
refund = ThinkThen.question(decide: "...", threshold: 0.2..0.8)
ThinkThen.decide(refund, "I was charged twice. Can you fix this?")  # => nil
complaints = ThinkThen.filter("Is this a complaint?", reviews)
rows = ThinkThen.annotate("form.json", tickets)
```

`nil` is "not sure". Any `Enumerable` crosses once. The bulk spelling is
`decide_many`. Cancel lands here in the brief's item 7.
