# Ruby API

Require `thinkthen` and open a client with `ThinkThen::Client.open { |client| ... }`; the block form closes it. A client accepts `backend`, `model`, `url`, `api_key`, `batch` and `retry`. Without `backend:`, the client reads `THINKTHEN_BACKEND`, and without that it uses the default backend; an unknown name raises `UsageError` with code `:"usage.backend"`. The key comes from `api_key:` or the environment. `ThinkThen.configure` sets up the shared client before its first use, and `ThinkThen.close` closes it.

Each one-record call returns one row and refuses a collection with `UsageError`. Its `_each` form takes an Enumerable and returns a `Rows` enumerator, which pulls input as it reads and can be read once; with a block it yields each row and closes the call. Close an enumerator you stop early, in `ensure`. Input files stay the caller's.

| Call | Required arguments after question and record |
| --- | --- |
| `decide`, `filter`, `find` | None |
| `choose` | `choices:` |
| `tag` | `labels:` |
| `score`, `rank` | `levels:` |
| `recognize` | Takes record and `kinds:` |
| `relate` | Takes record and `rules:` |
| `annotate` | Takes `AnnotationQuestions` and record |

`decide?` returns `true` or `false`. A row's `answer`, `yes?`, `no?` and conversions raise `UnsureError` when not sure and `RecordError` when the record failed, so no row needs a type check before reading its answer. `answered?`, `unsure?` and `failed?` test a row without raising.

`filter(question, records)` returns an Enumerator of the caller's own kept records, in input order; not-sure records are left out. After the kept records it raises one `RecordError` whose `rows` lists every failed `FilterRow`; `failures: :row` yields those rows instead. With a block, it yields each kept record. `filter_each` yields a `FilterRow` for every record, with `item`, `kept?` and the typed `row`.

`rank(question, records, levels:)` returns an Enumerator of `RankRow`s in rank order, then raises one `RecordError` for failed records, or yields them with `failures: :row`. With a block, it yields each row. A `RankRow`'s `rank` is the engine's rank number, starting at 1, and its `item` is an equal copy of the record read back from JSON, not the caller's object, so a long rank keeps only the engine's copies. A Hash record's copy always has String keys, even when the input used Symbol keys. `rank_each` yields every row, failed rows included. `top:` keeps the first ranks.

A record that has no JSON form, such as an `Object` or a cyclic Hash, becomes a failed row with code `:"record.invalid"` whose message names its class and reason, and the call goes on. Reading `item` on such a rank row raises that error. A `nil` record fails with `:"record.missing"`, and blank text with `:"record.empty"`.

Thresholds use Float. Decide, choose and filter also accept an inclusive Float Range. Score, rank and annotate have no threshold option. Field accepts a JSON Pointer String or an Array of String, Symbol and nonnegative Integer components. Decide and filter take `yes_means:` and `no_means:`; choose, tag, score and rank take `descriptions:`. Each call accepts an execution `name:` and a `cancel:` token.

Typed question classes keep their options. A matching call takes its typed question and record with only `name:` and `cancel:` (and `failures:` for `filter` and `rank`). `Toolbox` maps names to typed questions, and its `[]` raises `KeyError` for an unknown name. `AnnotationQuestions` excludes rank and annotate. Selecting a toolbox entry does not set an execution name.

Rows are immutable `Answered`, `Unsure` and `Failed` Data values. Answered and not-sure rows carry typed detail values; failed rows carry a `RowError`. Symbols name function, status, backend and error codes. `to_h` returns contract data with String keys, and `to_json` writes it.

Call and record errors include `ThinkThen::Error`: `UsageError` (an `ArgumentError`), `RecordError`, `BackendError`, `CancelledError`, `LocalError` and `InternalError`. `UnsureError` is not one of them. `CancelToken#cancel` cancels every call it is attached to. Interrupting a native pull raises `CancelledError` with the Interrupt as its cause.

The gem ships [RBS declarations](sig/thinkthen.rbs). CRuby requirements follow the package metadata. The ffi dependency does not support Ractors.
