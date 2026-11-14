# Ruby API

Use Ruby 3.3 or later and `require "thinkthen"`. `ThinkThen::Client.open { |client| ... }` closes the client at the end of the block. The functions on `ThinkThen` use a shared default client. `ThinkThen.configure` changes it before its first use; `ThinkThen.close` closes it.

Each result is immutable and has `function`, `status`, `probability`, `error`, `position`, `id`, `source`, `detail` and `item`. Status is `:answered`, `:unsure` or `:failed`. State checks, printing, inspection, hashing and serialization never read the answer. `.answer` raises `UnsureError` or `FailedError` when there is no answer. Only `DecideRow` has `.yes?`, with the same loud read. Ruby treats every result object as true in an `if`; always call `.yes?` or read `.answer`.

| Function | Answer | Required options |
| --- | --- | --- |
| `decide` | Boolean | None |
| `choose` | String or Symbol | `choices:` |
| `tag` | Frozen Array of Strings or Symbols | `labels:` |
| `score` | Float from 0 to 1 | `levels:` |
| `find` | `Span`, or answered `nil` | None |
| `recognize` | Frozen Array of `Name` | Record first, `kinds:` |
| `relate` | Frozen Array of `Relation` | Record first, `rules:` |
| `annotate` | Named full results under `.answer` | `AnnotationQuestions.new(refund: ..., topic: ...)` |

A scalar call raises `FailedError` at the call when its own parent row failed, including blank or missing input. A successful scalar result is answered or unsure. Scalar annotate still returns an answered parent with one, several or all entries failed; only an explicit `raise_if_failed` scans those entries.

Each has an `_each` form taking an Enumerable. An `_each` call remains many-record even with one input, retaining its failed row. With a block it yields results and closes the call, including after `break` or an exception. Without a block it returns a closable `Rows` Enumerator. Every result's `.item` is the caller's original object. A one-record call refuses an Array and names its `_each` alternative.

`filter(question, records)` yields only the caller's original kept records. It takes one threshold cut and refuses a band with guidance to `decide_each`. It yields kept records before raising one `FailedError` whose `rows` contains all failed results. There is no `filter_each`, `FilterRow`, `kept?` or failures option.

`rank(question, records, levels:, top:)` yields `RankRow` results in rank order. `.answer` is the rank, starting at 1. Failed results follow the ranked ones. Every `.item` is the original object, including invalid records. Keeping originals means retaining the inputs until ranking finishes, even with `top:`. There is no `rank_each` or second rank read.

Choices, labels and levels accept an Array of Symbols or an Array of Strings. A symbol domain returns symbols for choose and tag; strings return strings. The supplied Array's order defines a scale, lowest first. `descriptions:` is keyed by those words. Decide and filter take `yes_means:` and `no_means:`. `threshold:` is a Float; decide also accepts an inclusive Float Range.

`DecideQuestion`, `ChooseQuestion` and the other question classes save their options. `Toolbox.load_file(path)` loads canonical saved questions; `.choose(:topic)` and the other typed selectors reject a missing name or wrong function. `AnnotationQuestions` excludes filter, rank and annotate. `RelationRule.new(source:, target:, relation:, reads: nil, either: false)` uses named fields. A name has `kind`, `text`, `start`, `end`, `probability`. A relation has `source`, `target`, `relation`, `probability`, `either`.

Records are strings or JSON-like values, including Hashes with String or Symbol keys and caller classes converted through `to_h`. Conversion never calls `to_json`. `field:` selects a JSON Pointer; the reference also accepts a key path such as `[:text]`. Blank input, missing input and invalid records fail at the scalar call; many-record APIs retain them as failed results. IDs are strings, signed 64-bit integers or unsigned 64-bit integers. Invalid IDs are absent from failed results; the original `.item` remains available.

Results have value equality and hashing over their nine own fields, excluding `item` throughout nested forms. Comparing a result with a bare answer raises `TypeError` naming `.answer`; `eql?` returns false. `to_h` and `to_json` write the canonical wire row, omit `item` and preserve character offsets. Printing uses the command's words and folds CR, LF and tab inside string leaves to spaces; tabs separating labels or form entries remain.

`raise_if_failed` returns the same result unless it failed or an annotate entry failed. A failed outer form takes precedence; otherwise the first failed child in the engine's question-name order supplies the error. Unsure entries do not fail a form. Child results retain the parent's position, ID, source and original item.

`Client.open` accepts `backend:`, `model:`, `url:`, `api_key:`, `batch:`, `retry: RetryPolicy.new(...)` and the reference-only `max_requests:` allowance. Retry times use seconds. Every call also accepts `name:` and `cancel: Cancel.new`. Calls release Ruby's interpreter lock while waiting, and one client can serve several threads. Create a new client after `fork`. `client.usage` returns an immutable cumulative snapshot, including after close.

`NoAnswerError` is included by `UnsureError` and `FailedError`. `Error` covers engine and binding failures, including `UsageError` (also an `ArgumentError`), `SetupError`, `CancelledError`, `InternalError`, `UnknownCodeError` and `FailedError`; it excludes uncertainty. `UnsureError` carries its `row` and `probability`; `FailedError` carries `rows`, `code` and `http_status`. HTTP 401/403 always stop the call; an all-failed first batch with any HTTP 4xx also stops it. These remote call refusals raise `SetupError`, while other service failures fail at scalar calls and remain failed results in many-record APIs.

The gem ships [RBS signatures](sig/thinkthen.rbs). The ffi dependency does not support Ractors. The [Ruby guide](../../docs/surfaces/ruby.md) supplies runnable examples.
