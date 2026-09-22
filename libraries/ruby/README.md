# The Ruby surface

The eight verbs as module methods on `ThinkThen`, over the one engine
through the contract. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`,
runs as drawn through `tests/slide_sample.rb`:

```ruby
require "thinkthen"

upset = ThinkThen.filter("Is this a complaint?", reviews)
urgent = ThinkThen.rank("Is this urgent?", inbox, top: 5)
ThinkThen.score("How urgent is this?", outage, levels: [...])
```

`nil` is "not sure". A `Range` threshold is the band. Any `Enumerable`
crosses into the engine once and runs at the process width: the width is
the number of requests in flight, and each in-flight request holds its own
connection — 1,000 records at width 32 measured 33 pooled connections. The
bulk spelling is `decide_many`. The six error kinds are Ruby classes —
`UsageError` under `ArgumentError`, the rest under `StandardError` — each
carrying `kind` and `retryable`. The interrupt shape: `Thread#raise` and
Ctrl-C stop any call, tick or no tick — the interrupt fires the call's own
cancel token, sent requests finish, no new one starts, and the pending
exception re-raises when the call returns; the proofs are
`tests/test_interrupt_wire.rb` and `tests/test_interrupt_fast.rb`. A bulk
call can also run a tick each wait interval with the VM lock taken, and a
raise inside the tick cancels the engine the same way; the proof is
`tests/test_cancel.rb`.

## Question helpers, and the names the check allows

Beyond the fourteen ruled functions, the module carries exactly these
documented helpers, and `scripts/check_public_names.py` fails the tree on
any other public name:

- `ThinkThen.question(**keywords)` and `ThinkThen.set(path)` build the same
  question value from keywords and from a question file, and
  `ThinkThen.built(value)` wraps a question already built.
- `ThinkThen.with_tick(&tick)` runs a block each wait interval with the VM
  lock taken, for a host that wants progress or its own stop gesture during
  a long bulk call; a raise inside the tick cancels the call as the section
  above says. It is optional: plain calls already hear `Thread#raise` and
  Ctrl-C.
- `ThinkThen.score_with_level(question, evidence, levels:)` and
  `ThinkThen.decide_many_with_probabilities(question, records)` are the
  level-carrying and probability-carrying forms the slide check and the
  conformance runner read; the plain verbs return the bare answers.

Ruby is not installed on this machine; `./build.sh` and `./check.sh` run
everything inside the `ruby:3.4-trixie` container (removed after each
run) with the host Rust toolchain mounted read-only. The gem builds from
`thinkthen.gemspec` and is never published.
