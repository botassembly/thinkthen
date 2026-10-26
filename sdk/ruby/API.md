# Ruby API

Require `thinkthen` and create a client with `ThinkThen::Client.open(backend: :stub) { |client| ... }`. Explicit clients accept `backend`, `model`, `url`, `api_key`, `batch` and `retry`. The engine supplies omitted defaults. `ThinkThen.configure` configures the shared client before its first use. `ThinkThen.close` closes it.

Each named call returns one row. Its `_each` form accepts an Enumerable and returns lazy `Rows`, or yields rows when given a block. Close external iterators in `ensure`. Block traversal and `Client.open` close owned calls. Input files remain caller-owned.

| Call | Required arguments after question and record |
| --- | --- |
| `decide`, `filter`, `find` | None |
| `choose` | `choices:` |
| `tag` | `labels:` |
| `score`, `rank` | `levels:` |
| `recognize` | Takes record and `kinds:` |
| `relate` | Takes record and `rules:` |
| `annotate` | Takes `AnnotationQuestions` and record |

Thresholds use Float. Decide, choose and filter also accept an inclusive Float Range. Score, rank and annotate have no threshold option. Evidence accepts a JSON Pointer String or an Array of String, Symbol and nonnegative Integer components. Each call accepts an independent optional execution `name:` and `cancel:` token. Rank also accepts `top:`.

Typed question classes retain their domains and evidence. A matching named call accepts its typed question and record with only `name:` and `cancel:`. `Toolbox` maps names to typed questions. `AnnotationQuestions` excludes rank and annotate. Selecting a toolbox entry does not set an execution name.

Rows use immutable `Answered`, `Unsure` and `Failed` Data variants. Narrow the variant before reading `answer`. Answered and unsure rows expose typed detail values; failed rows expose `RowError`. Symbols represent function, status, backend and error codes. `to_h` returns contract data with String keys. `FilterRow` retains the original `item`, its typed `row`, and `kept?`.

Missing and invalid records return failed rows. Call errors use `UsageError`, `BackendError`, `Cancelled`, `LocalError` and `InternalError`. `CancelToken#cancel` requests cancellation across attached calls. Interrupting a native pull raises `Cancelled` with its original Interrupt as cause.

The gem ships [RBS declarations](sig/thinkthen.rbs). CRuby requirements follow the package metadata. The ffi dependency does not support Ractors.
