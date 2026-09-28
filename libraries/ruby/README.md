# The Ruby surface

The `thinkthen` gem exposes the engine's verbs as module methods on `ThinkThen`, over the public Rust API through a magnus extension. The Ruby slide example runs through `tests/slide_sample.rb`; the site's copy awaits its separately owned migration.

```ruby
require "thinkthen"

upset = ThinkThen.filter("Is this a complaint?", reviews).value
urgent = ThinkThen.rank("Is this urgent?", inbox, top: 5).value
score = ThinkThen.score("How urgent is this?", outage, levels: ["Routine.", "Soon.", "Immediate."])
puts score.value
puts score.facts[:requests_sent]
```

The module methods use one engine built on first use from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and the rest that the command reads. `ThinkThen::Engine.new(base_url:, model:, throttle:, max_requests:, max_request_bytes:, cache:, timeout:, max_retries:, profile:, record:, replay:, batch:)` builds another. Each omitted keyword comes from the environment. `cache: false` turns the cache off. There is no key keyword, and the key never enters a Ruby object.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `ThinkThen::Engine.new(cache: false)`.

## The verbs

`decide`, `decide_many`, `decide_many_with_probabilities`, `filter`, `rank`, `find`, `choose`, `choose_many`, `score`, `score_many`, `score_with_level`, `tag`, `tag_many`, `details`, `annotate`, `recognize`, `relate`, and `usage`. `ThinkThen.question(**keywords)` builds a question from the question file's keys, and a `Range` threshold is the band. `ThinkThen.set(path)` loads a question set, and `ThinkThen.set(name: spec, ...)` builds one. `nil` means unsure.

`rank` and `find` accept plain question text or a built `decide` question containing only its text. They refuse a built question with `profile`, `threshold`, `model`, or another extra key before sending because those text-only calls cannot retain it.

A record that is not a `String` crosses as its JSON text. `nil`, invalid UTF-8, and a NUL byte refuse with `UsageError` naming the index, before any request. Every failure is a `ThinkThen::Error`. Its six kind classes are `UsageError`, `BackendError`, `LocalError`, `CancelledError`, `DeadlineError`, and `DefectError`, and each carries `kind` and `retryable`.

## Run facts

Every asking verb returns a `ThinkThen::Call`. Its `.value` is the former Ruby value, `.facts` is the completed call account, and `.details` is an ordered, immutable list of question observations. The account includes records, requests sent, cache answers, elapsed seconds, and optional provider tokens and model. A pre-accounting error has no facts. A completed Rust failure carries final facts and details on the exception. `Call#map` preserves the account when reshaping a value and attaches it to a conversion error.

`details(question, text).value` returns the command's `--details` line for one text as a `Hash`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. This document has no cost or elapsed time field; the call's `.facts[:seconds]` gives elapsed time.

`usage` returns this engine's running totals of requests sent, retries, cache answers and tokens. `Engine.new(batch: "max")` or a positive integer controls eligible record batching. Per-call `batch:` on `decide_many`, `filter`, `rank`, `choose_many`, `score_many`, `tag_many`, and `annotate` overrides it. `context:` shares nonblank text across those calls except `annotate`. Scalar calls, `find`, `recognize`, and `relate` reject both keywords before sending. `batch: 1` preserves former one-record request bytes; it admits one request at a time while a lazy stream waits for a row. The default packs compatible records into each request.

## Stopping a call

Every call runs on its own worker thread while the calling Ruby thread waits in 50 ms slices with the VM lock released. Ctrl-C raises `CancelledError` with the `Interrupt` as its cause. `Thread#raise`, `Thread#kill`, and a raising trap stop the call and pass through unchanged. `cancel:` takes a `ThinkThen::Cancel`, and `deadline:` takes seconds. Each stop returns within one slice. Requests already sent finish on the detached worker, and no new request starts. An exception after worker start carries `.completion`. Its `done?` reports whether that worker ended; `result(timeout:)` waits for the final outcome and facts, or raises `Timeout::Error` while still pending. `with_tick { ... }` runs a block about ten times a second during the thread's calls, and a raise inside it stops the call with that error.

## The throttle is per loaded copy

The throttle is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads this gem and another surface's native package can run up to twice the throttle (ADR 0047 item 5).

## Building and checking

`setup-ruby.sh` builds the pinned Ruby 3.4.11 once per machine, with the network, into `~/.cache/thinkthen-toolchains/ruby/3.4.11`. `toolchain.env` pins both source archives by hash. `build.sh` builds the extension and the gem offline with that Ruby. `check.sh` is this surface's entry in the surface rung. It runs the file checks, the toolchain probe, the build, Clippy, the Rust unit tests, each Ruby test file, the conformance runner, the examples, the gem check, and the slide. Each test starts its own loopback backend and runs its calls in a scrubbed child with a fake key. Without the pinned prefix, `check.sh` prints "not run" and exits 77. The gem is never published.
