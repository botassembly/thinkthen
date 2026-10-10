# ThinkThen for Ruby

These calls require the development 0.2 gem; the public release remains 0.1.2. Install a supplied development gem with `gem install --local path/to/thinkthen-0.2.0-platform.gem`. The gem supports Ruby 3.4 on Linux and macOS. An unsupported platform selects a diagnostic fallback that refuses to load; candidate checks qualify platform packages.

```ruby
require "thinkthen"
decision = ThinkThen::Client.open(cache: false) do |client|
  client.decide("Is this urgent?", "Please respond today")
end
puts decision.value
puts decision.facts.requests_sent
```

`Client` has the ten named methods: `decide`, `filter`, `rank`, `find`, `choose`, `score`, `tag`, `annotate`, `recognize`, and `relate`. Questions accept literal text, ordinary hashes containing the native definition, or `Client.question_file`, `question_name`, and `question_reference` selectors. Strings are literal wording; references are explicit. Rust validates questions, settings and reading options.

Pass a scalar, an array, an enumerable, or `Client.files(paths, unit: "line")`. `Client.item(value, context: ..., options: ..., images: ...)` carries per-record fields separately from the original. Image descriptors use native `bytes` with base64 data and media type, or `file` with a path. File and image support follow the shared request contract. Enumerable producers are advanced with backpressure and closed when they support `close`.

Results own their native values after client cleanup. `result.results` contains generated typed objects, `result.facts` contains typed observations, and `result.terminal` carries final settlement. Accessors and `key?` distinguish absent members from present null; false stays false. Unknown native fields remain accessible. Inspect output withholds caller text. `value` selects the convenient value; filter preserves original records, rank retains typed rows, and recognize preserves all aggregate chunks.

Failures raise a typed `ThinkThen::Error` subclass. Native terminal failures retain `facts`, `complete`, `results` for the completed prefix, and `terminal`. Admission failures have no account. `Cancel.new` and `cancel:` stop reads and submissions; `deadline_ms:` is a native request option. Native polling yields through Ruby's sleep and fiber scheduler. Block-scoped `Client.open` and `start` close operations on success, exceptions and interruption. No new Ruby asynchronous framework is introduced.

`client.plan(function, question, records, **options)` returns the native preview without calls, keys or cache reads. It supports the native atomic and rank preview contract and refuses unsupported functions. `usage`, `usage_persistence`, and `finish_usage_status` read the owned engine's observations.

## Migration

Replace module-level verbs and `Engine`/`Engine#complete` calls with the same named method on `Client`. Replace `decide_many`, `choose_many`, `score_many`, and `tag_many` with their named method and an array. Replace old question/input/complete carriers with ordinary Ruby values and `Client.item` or `Client.files`. Replace `Call#value` with the owned result's `value`; use generated result accessors and facts instead of old copied hashes. The old public API, handwritten complete readers and private host dispatchers have been removed.

## Development

Build with the pinned toolchain using `build.sh`. `check.sh` installs the native gem in an empty gem folder and runs owned-session cases plus the routine shared cases through named typed calls. `THINKTHEN_ARTIFACT` checks the supplied gem. `THINKTHEN_TEST_PROFILE=full` selects the complete shared inventory only at a candidate. Checks use isolated homes and local fake providers; no paid service is used.
