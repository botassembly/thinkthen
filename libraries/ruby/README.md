# The Ruby surface

The eight verbs as module methods on `ThinkThen`, over the one engine
through the contract. The acceptance sample, drawn in
the product deck's surfaces page,
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
bulk spelling is `decide_many`. Every failure is a `ThinkThen::Error`: the
six kind classes — `UsageError`, `BackendError`, `DeadlineError`,
`LocalError`, `CancelledError`, `DefectError` — inherit it, each carrying
`kind` and `retryable`, and the wrapper's own refusals raise
`UsageError`. A record or an evidence text that is not a `String` crosses
as its JSON text. The interrupt shape: every call runs on its own thread
while the calling Ruby thread waits in slices with the VM lock released.
Between slices MRI runs the host's interrupts: Ctrl-C, `Thread#raise`,
`Thread#kill`, and a raising trap fire the call's own cancel token, sent
requests finish, no new one starts, and the raise surfaces when they
have. A spurious `Thread#wakeup` and a trapped signal that raises
nothing cancel nothing, and an interrupt never fires a token the caller
shared. The call's thread blocks asynchronous signals, so a signal never
interrupts a send and a paid request is never sent twice. The proofs are
`tests/test_interrupt_fast.rb`, `tests/test_harmless_wakeups.rb`,
`tests/test_interrupt_wire.rb`, and `tests/test_signal_no_resend.rb`. A
bulk call can also run a tick about ten times a second on the watchdog
thread, and a raise inside the tick cancels the engine; the proof is
`tests/test_cancel.rb`.

## Question helpers, and the names the check allows

Beyond the fourteen ruled functions, the module carries exactly these
documented helpers, and `scripts/check_public_names.py` fails the tree on
any other public name. `check.sh` runs it over the loaded module, so a
name the native half defines counts too:

- `ThinkThen.question(**keywords)` and `ThinkThen.set(path)` build the same
  question value from keywords and from a question file, and
  `ThinkThen.built(value)` wraps a question already built.
- `ThinkThen.with_tick(&tick)` runs a block about ten times a second on
  the watchdog thread, for a host that wants progress or its own stop gesture during
  a long bulk call; a raise inside the tick cancels the call as the section
  above says. It is optional: a bulk call already hears a real
  `Thread#raise`, Ctrl-C, and `Thread#kill` without one.
- `ThinkThen.score_with_level(question, evidence, levels:)` and
  `ThinkThen.decide_many_with_probabilities(question, records)` are the
  level-carrying and probability-carrying forms the slide check and the
  conformance runner read; the plain verbs return the bare answers.
- `ThinkThen::Error` is the base of the six kind classes,
  `ThinkThen::Cancel` is the token the `cancel:` keyword takes,
  `ThinkThen::Question` and `ThinkThen::QuestionSet` are the values
  `question` and `set` return, and `ThinkThen::VERSION` is the gem version.

Ruby is not installed on this machine; `./build.sh` and `./check.sh` run
everything inside the `ruby:3.4-trixie` container (removed after each
run) with the image's own pinned Rust toolchain. The gem builds from
`thinkthen.gemspec` and is never published.
