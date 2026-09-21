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
crosses into the engine once and runs at the process width. The bulk
spelling is `decide_many`. The six error kinds are Ruby classes —
`UsageError` under `ArgumentError`, the rest under `StandardError` — each
carrying `kind` and `retryable`. The interrupt shape: a bulk call runs a
tick each wait interval with the VM lock taken, and a raise inside the
tick cancels the engine, lets sent requests finish, and re-raises; the
proof is `tests/test_cancel.rb`.

Ruby is not installed on this machine; `./build.sh` and `./check.sh` run
everything inside the `ruby:3.4-trixie` container (removed after each
run) with the host Rust toolchain mounted read-only. The gem builds from
`thinkthen.gemspec` and is never published.
