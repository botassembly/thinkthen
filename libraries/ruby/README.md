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

The module methods use one engine built on first use from the environment: `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, and the rest that the command reads. `ThinkThen::Engine.new(backend:, base_url:, model:, throttle:, max_requests:, max_request_bytes:, cache:, timeout:, max_retries:, profile:, record:, replay:, batch:, max_requests_total:)` builds another. `max_requests_total:` caps the live sends every engine of the process makes together; `0` refuses the first with `UsageError`. Each omitted keyword comes from the environment. `cache: false` turns the cache off. There is no key keyword, and the key never enters a Ruby object.

`ThinkThen::Engine.new(backend: "typesafe")` selects a built-in backend or a backend in the Rust configuration. The built-ins are `liquid`, `ollama`, `openrouter`, `perplexity`, and `typesafe`. Their keys come from `LIQUIDAI_API_KEY`, `OLLAMA_API_KEY`, `OPENROUTER_API_KEY`, `PERPLEXITY_API_KEY`, and `TYPESAFE_API_KEY`, respectively. Rust captures those variables when the engine is constructed. Later environment changes do not change that engine. Omit `backend:` or pass `nil` to retain environment selection. `base_url:` and `model:` override the selected backend's address and model. Its path, question form, prices, and profile remain selected unless their supported settings override them. Backend names cannot appear in per-call settings. No Ruby keyword accepts a key.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `ThinkThen::Engine.new(cache: false)`.

## The verbs

`decide`, `decide_many`, `decide_many_with_probabilities`, `filter`, `rank`, `find`, `choose`, `choose_many`, `score`, `score_many`, `score_with_level`, `tag`, `tag_many`, `details`, `annotate`, `recognize`, `relate`, `plan`, and `usage`. `ThinkThen.question(**keywords)` builds a question from the question file's keys, and a `Range` threshold is the band. `ThinkThen.set(path)` loads a question set, and `ThinkThen.set(name: spec, ...)` builds one. `nil` means unsure.

`ThinkThen.question(file: path)` reads one named UTF-8 question file of at most 1 MiB, validates it, and returns the same question value as the keyword form. `file:` cannot be combined with other question keywords. A bad named file raises non-retryable `LocalError` before a send; bad typed keywords still raise `UsageError`. Ordinary question strings remain literal, including a leading `@`. `ThinkThen.set(path)` remains the separate question-set form.

`rank` and `find` accept plain question text or a built `decide` question containing only its text. They refuse a built question with `profile`, `threshold`, `model`, or another extra key before sending because those text-only calls cannot retain it.

A record that is not a `String` crosses as its JSON text. `nil`, invalid UTF-8, and a NUL byte refuse with `UsageError` naming the index, before any request. Every failure is a `ThinkThen::Error`. Its six kind classes are `UsageError`, `BackendError`, `LocalError`, `CancelledError`, `DeadlineError`, and `DefectError`, and each carries `kind`, `retryable`, and `code`, the C door's number from 1 (`usage`) to 6 (`defect`). `ThinkThen::YES`, `NO`, and `UNSURE` are the C outcome codes 1, 0, and 2, and `ThinkThen.outcome(answer)` maps `true`, `false`, and `nil` to them.

## Run facts

Every asking verb returns a `ThinkThen::Call`. Its `.value` is the former Ruby value, `.facts` is the completed call account, and `.details` is an ordered, immutable list of question observations. The account includes records, requests sent, cache answers, elapsed seconds, and optional provider tokens and model. A pre-accounting error has no facts. A completed Rust failure carries final facts and details on the exception. `Call#map` preserves the account when reshaping a value and attaches it to a conversion error.

`details(question, text).value` returns the command's `--details` line for one text as a `Hash`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. This document has no cost or elapsed time field; the call's `.facts[:seconds]` gives elapsed time.

Facts, details, and annotate rows are Ruby hashes read from the engine's JSON, so a member this gem does not know reads like any other. In an annotate row, `nil` is unresolved and `{"failed" => {"kind" => ..., "cause" => ...}}` is a failed question. `ThinkThen.failed(answer)` returns that failure hash, or `nil` for any value.

`plan(question, records, batch:, context:)` previews a `decide`, `choose`, `score`, or `tag` call without asking it. It returns the result schema's `plan` object: `records`, `requests`, `estimated_bytes`, `estimated_input_tokens` with `lower` and `upper`, `upper_bound`, and `first_body_utf8`. It reads no key and no cache and sends nothing.

`usage` returns this engine's running totals of requests sent, retries, cache answers and tokens. `Engine.new(batch: "max")` or a positive integer controls eligible record batching. Per-call `batch:` on `decide_many`, `filter`, `rank`, `choose_many`, `score_many`, `tag_many`, and `annotate` overrides it. `context:` shares nonblank text across those calls except `annotate`. Scalar calls, `find`, `recognize`, and `relate` reject both keywords before sending. `batch: 1` preserves former one-record request bytes; it admits one request at a time while a lazy stream waits for a row. The default packs compatible records into each request.

## Stopping a call

Every call runs on its own worker thread while the calling Ruby thread waits in 50 ms slices with the VM lock released. Ctrl-C raises `CancelledError` with the `Interrupt` as its cause. `Thread#raise`, `Thread#kill`, and a raising trap stop the call and pass through unchanged. `cancel:` takes a `ThinkThen::Cancel`, and `deadline_ms:` takes whole milliseconds: `-1` or `nil` for none, `0` for spent. Each stop returns within one slice. Requests already sent finish on the detached worker, and no new request starts. When cancellation or an interrupt makes the caller return before its worker finishes, the surfaced exception carries `.completion`. Its `done?` reports whether that worker ended; `result(timeout:)` waits for the final outcome and facts, or raises `Timeout::Error` while still pending. An error returned after the worker finishes instead carries its available final `.facts` and `.details`. `with_tick { ... }` runs a block about ten times a second during the thread's calls, and a raise inside it stops the call with that error.

## The throttle is per loaded copy

The throttle is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads this gem and another surface's native package can run up to twice the throttle (ADR 0047 item 5).

## Building and checking

The release channel uses RubyGems platform gems for the supported Linux and macOS targets. When a compatible release is available for your platform and Ruby 3.4, install it with:

```sh
gem install thinkthen
```

Each release also includes a matching-version plain Ruby fallback gem. If RubyGems selects it, installation prints the supported Ruby and platform requirements, and `require "thinkthen"` raises `LoadError`. It contains no native engine and cannot run calls. Native gems retain Ruby `>= 3.4, < 4` and the macOS 15.0 floor.

From a source checkout, `setup-ruby.sh` builds the pinned Ruby 3.4.11 once per machine, with the network, into `~/.cache/thinkthen-toolchains/ruby/3.4.11`. `toolchain.env` pins both source archives by hash. `build.sh` builds the extension and the gem offline with that Ruby. `check.sh` is this surface's entry in the surface rung. It runs the file checks, the toolchain probe, the build, Clippy, the Rust unit tests, each Ruby test file, the conformance runner, the examples, the gem check, and the slide. Each test starts its own loopback backend and runs its calls in a scrubbed child with a fake key. Without the pinned prefix, `check.sh` prints "not run" and exits 77.

Explicit files and folders use the [library reader contract](../files.md), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments.

## Complete native calls

`engine.complete` exposes all ten named functions: decide, choose, tag, score,
filter, rank, find, annotate, recognize and relate. Each returns a
`ThinkThen::Complete::Completed` with typed `results`, native final `facts`,
original `inputs` and zero-based `ordinals`.

```ruby
engine = ThinkThen::Engine.new(cache: false)
call = engine.complete.decide(
  { role: 'atomic', body: { decide: 'Is this a complaint?' } },
  { kind: 'records', records: [{ content: { kind: 'text', value: 'Please refund.' } }] },
  attempts: true
)
probability = call.results.first.answer.probability
requests = call.facts.requests_sent
```

Question sources select exactly one body, raw JSON, path, name or reference,
with an explicit role: atomic, dynamic, rank, set, find, recognize or relate.
Rank sets use set. Native loaders retain structured descriptions, author names,
wording versions, declarations and annotation members. Caller inputs may be
hashes or `ThinkThen::Complete` carriers.

File sources use `kind: 'files'`, `paths: ['report.txt']` and
`options: { reading: { unit: 'file' }, media: 'text' }`. Line/window readings
retain physical coordinates; `jsonl: true` reads records and `media: 'image'`
reads explicit image files. Records can carry ordered PNG/JPEG attachments,
explicit context and described options. Images are admitted for decide, choose
and score; the other seven functions refuse before sending.

`decide_batch`, `choose_batch`, `tag_batch`, `score_batch`, `filter_batch` and
`annotate_batch` return native lazy enumerable batches. Each row has typed
`result`, `ordinal` and `input`; final `facts` is available at exhaustion or
terminal failure. Enumeration closes on early exit; direct `next` users should
`close` when leaving early. `cancel` and the ordinary cancellation/deadline
controls stop native work. Source files and sends wait for the first pull.

Complete failures retain the six ordinary error kinds and add typed `complete`,
with available native final facts and stopped position. Native cache, record,
replay and attempts apply, including zero-send replay and changed reading
identity. Carrier inspection withholds contents. Existing bare calls retain
their behavior.

Dynamic choose batches accept a `dynamic` question source with `choose`, optional input/context/candidate pointers, and a whole ordered candidate list on every record. The native lazy dynamic choose API admits each record and supplies its own probabilities and identity. Missing later candidates yield the completed prefix, then a usage error with joined final facts.

Rank set results retain ordered `members`, each with its saved `name` and typed `RankMemberResult`. The child carries its own positive member position, answer ID, authored question, probability, source and metadata; the parent carries the final turns position and winner. Usage dimensions remain independently optional. Parent and member usage overlap; use the final call facts for invocation totals. Ordinary ranks retain absent members.
