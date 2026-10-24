# thinkthen

ThinkThen gives ordinary software ten functions that judge evidence by meaning. A script, a program or a query asks a typed question about a record and branches on the answer. ThinkThen turns model probabilities into `true`, `false`, a label, a number or "not sure". A failed call never looks like an answer.

## Install and ask

Download the command archive for your operating system and processor from the [0.2.0 release](https://github.com/botassembly/thinkthen/releases/tag/v0.2.0). The command archives are `thinkthen-0.2.0-linux-x86_64.tar.gz`, `thinkthen-0.2.0-linux-arm64.tar.gz`, `thinkthen-0.2.0-macos-arm64.tar.gz` and `thinkthen-0.2.0-windows-x86_64.zip`. Each archive contains the command and its licenses. Read the release's runtime requirements before installing.

On Linux x86-64, extract the downloaded archive and call the command by its path:

```sh
mkdir thinkthen-0.2.0
tar -xzf thinkthen-0.2.0-linux-x86_64.tar.gz -C thinkthen-0.2.0
printf '%s\n' 'Please refund my order.' > message.txt
./thinkthen-0.2.0/thinkthen decide --adapter stub --details \
  'Does the customer ask for money back?' < message.txt
```

Use the matching archive name on Linux arm64 or Apple Silicon macOS. On Windows, extract the ZIP and call `thinkthen.exe` from PowerShell:

```powershell
Expand-Archive .\thinkthen-0.2.0-windows-x86_64.zip .\thinkthen-0.2.0
Set-Content -Encoding utf8 message.txt 'Please refund my order.'
Get-Content -Raw message.txt | .\thinkthen-0.2.0\thinkthen.exe decide --adapter stub --details 'Does the customer ask for money back?'
```

The stub needs no account, key or service. It prints a JSON row with `status`, `answer` and `p`. Stub answers test your integration; they do not judge the message's meaning. A yes answer exits 0, a no answer exits 1 and an unsure answer exits 3. These are successful verdicts. See the exit-code table below for failures.

To call `thinkthen` without its directory, put the extracted command directory on `PATH`. For example, from the directory above it in a POSIX shell:

```sh
export PATH="$PWD/thinkthen-0.2.0:$PATH"
```

For a model-backed answer, set `THINKTHEN_API_KEY` to your System One key in your shell and run:

```sh
thinkthen decide --adapter systemone 'Does the customer ask for money back?' < message.txt
```

## Language libraries

The libraries call the same engine and expose the ten functions. Their package guides describe installation, runtime requirements, typed results, streaming and cancellation. The native engine ships with each language package.

| Language | Package and guide |
| --- | --- |
| Python | [`thinkthen`](sdk/python/README.md), including pandas and Polars |
| TypeScript / JavaScript | [`thinkthen`](sdk/typescript/README.md) |
| Go | [`github.com/botassembly/thinkthen/sdk/go`](sdk/go/README.md) |
| Java | [ThinkThen Java](sdk/java/README.md) |
| Kotlin | [ThinkThen Kotlin](sdk/kotlin/README.md) |
| C# | [`ThinkThen`](sdk/csharp/README.md) |
| R | [`thinkthen`](sdk/r/README.md), including data.frame and tibble |
| C | [ThinkThen C](sdk/c/README.md) |
| C++ | [ThinkThen C++](sdk/cpp/README.md) |
| Ruby | [`thinkthen`](sdk/ruby/README.md) |
| Swift | [`ThinkThen`](sdk/swift/README.md) |
| Zig | [`thinkthen`](sdk/zig/README.md) |

Rust applications use the [`thinkthen` crate](https://docs.rs/thinkthen/0.2.0/thinkthen/). Use `cargo add thinkthen` for the library and `cargo install thinkthen` for the command. The default `command` feature includes `thinkthen::files` for tables and `thinkthen::runs` for saved entries. Set `default-features = false` to omit command dependencies.

## Databases

[DuckDB](databases/duckdb/README.md), [PostgreSQL](extensions/postgres/README.md) and [SQLite](extensions/sqlite/README.md) call the same ten functions over query results. Each extension guide explains installation, backend configuration, source queries, result columns and database cancellation. Read the host and version requirements in that guide before installing.

## Spreadsheets and delimited tables

The command reads CSV, TSV and Excel `.xlsx` files and appends verdict columns. Create a small CSV and ask about its `Body` column:

```sh
printf '%s\n' 'Body' 'Please refund my order.' 'Where is my delivery?' > messages.csv
thinkthen decide 'Does the customer ask for money back?' --adapter stub \
  --csv --input messages.csv --column Body --into refund
```

The output is `messages.thinkthen.csv`. Use `--tsv` or `--excel` for the other formats, repeat `--column` to select several columns, and use `--sheet` to select an Excel worksheet. `--output` names a new destination. `--in-place` keeps a backup before replacing the source. The [file guide](crates/thinkthen/src/files/README.md) describes cell handling and write behavior.

## Backends

`--adapter` takes a backend name as a string. The built-in table contains `systemone`, `openai` and `stub`; `systemone` is the default. It reads only `THINKTHEN_API_KEY`, uses `THINKTHEN_BASE_URL` when set, and otherwise uses the TypeSafe address shown above. `--url` overrides that address. `--model` selects a model; without it, System One uses `jev-1.13.0`.

For the OpenAI decisions backend, set `OPENAI_API_KEY` in your shell:

```sh
thinkthen decide --adapter openai 'Does the customer ask for money back?' < message.txt
```

OpenAI reads only `OPENAI_API_KEY`, uses `https://api.openai.com/v1` and defaults to `gpt-6-luna`. It ignores `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`. Use `--url` and `--model` to override its address and model. Keys go only to the selected backend. A library's `key_source` must name that backend's own key variable; a custom `url` does not grant access to another backend's variable. Backend addresses require HTTPS, or plain HTTP to loopback. Other schemes are refused. Plain HTTP loopback calls ignore environment proxies.

Rust callers can add an entry to `BackendTable` and create a client with `Client::from_json_with_backends`. Each `BackendEntry` holds its name, wire `Adapter`, address, key variable, default model and `Limits`. `model_limits` replaces those bounds for a selected model. Adding a name does not change a function or a language's settings type. The client resolves the name and credentials once. A question that cannot fit the selected model's bounds fails with `backend.limit` before sending; it is a backend failure.

Rust applications using the `thinkthen` crate own their logger configuration. Enabling trace logging for `ureq_proto` writes raw HTTP request headers, including backend API keys. Keep that dependency below trace level. The shipped command, SDK native libraries and database extensions compile out trace logging through the `log` crate; library callers that set `default-features = false` keep the embedding application's logging policy unchanged. The default `command` feature also compiles out trace logging.

## Output and exit codes

`--details` prints a JSON result row. `--lines` reads one text record per nonblank line, and `--jsonl` reads one JSON record per nonblank line; both print JSON rows except that `filter` and `rank` print original records unless `--details` is set. Rows keep input order except for ranking.

Rows for `decide`, `filter`, `choose` and `find` carry top-level `p` when a backend answer exists. Rows for `tag`, `score`, `rank`, `recognize`, `relate` and `annotate` omit top-level `p`. Each annotate entry follows its function's rule. Recognized names and relations carry their own `p`; `detail` holds other answer probabilities. A recognized name's `p` is a support score: the smallest `1 − OUT` among its decoded pieces, multiplied by its selected kind probability (or 1 without kinds). It does not give the probability of exact name boundaries. Relate admits names using that support, then reports each relation's own backend probability. Serialized probabilities round to two decimal places, while scores and internal calculations retain their precision. Unsure rows have `"answer": null`.

| Exit code | Meaning |
| --- | --- |
| 0 | yes, a selected answer, or a successful record stream |
| 1 | no, or find selected no line |
| 2 | invalid arguments or settings |
| 3 | not sure |
| 4 | backend failure |
| 5 | local failure, such as unreadable input or an invalid toolbox |
| 130 | interrupted |

In record mode, a no or unsure row does not fail the stream. A failed row sets the exit code to its error family. The first Ctrl-C stops new requests and retry waits, prints `thinkthen: cancelling; press Ctrl-C again to exit` to standard error, and closes open backend connections. A second Ctrl-C kills the run’s child process groups, removes its file-operation locks and temporary files, and exits with 130.

## The ten functions

| Function | It answers |
| --- | --- |
| `decide` | yes, no, or not sure |
| `choose` | the one option that fits best |
| `tag` | every label that applies |
| `score` | a number on a scale |
| `filter` | the records that pass a yes/no question |
| `rank` | the records, best first |
| `find` | the one unit of text that answers the question, or nothing |
| `annotate` | a toolbox of named questions for each record |
| `recognize` | the names in a text and what kind each is |
| `relate` | the relations between named things |

## Toolboxes

A toolbox is a JSON file of named entries, or a folder containing one JSON entry per file. An entry holds a function request or a run of function and shell steps. Use `thinkthen run TOOLBOX NAME`, `thinkthen run FILE.json`, or `thinkthen run NAME` to select an entry from the local `thinkthen.json` or `thinkthen/` folder. Function commands take literal questions.

## Batching

ThinkThen packs many records into few requests. Each question keeps its own key, so the way records group never changes an answer.

## Saved entries and run

A saved function entry uses the ordinary request contract without `records`:

```json
{"function":"decide","question":{"type":"inline","value":"Does the customer ask for money back?"},"options":{"threshold":"0.2:0.8"}}
```

Save it as `refund.json` and run `thinkthen run refund.json < records.jsonl`. Each nonblank input line is one JSON value; quote a text record as a JSON string. `--lines` accepts ordinary text lines. A named toolbox wraps entries in `{"entries":{"refund":...}}`. A run wraps function requests and literal command arguments in `{"run":[...]}`. Run prints JSON rows and returns zero when no step fails, including false and unsure answers. A shell step uses `{"shell":["program","argument"]}`; shell syntax requires an explicit interpreter. Only commands written into the toolbox run. Shell steps inherit the caller's environment, including API keys. `thinkthen run NAME` reads `./thinkthen.json` or `./thinkthen/`; inspect its entry before running it from an unfamiliar directory. The run design describes records, quoting, exit status and cancellation.

In-memory question toolboxes keep their `questions` object for `annotate` and language callers. Shared `on` selects evidence unless a question sets its own. Choose questions use `options`, tag questions use `labels`, and score questions use `levels`. The Rust API parses those definitions with `Toolbox::from_bytes`; libraries take definitions from memory. The command's `annotate` argument is an inline JSON definition. Rust callers use `thinkthen::runs` for saved entry loading and execution; other language libraries expose the ten functions.

The JSON boundary separates client settings from calls. Create a `Client` once with settings such as `{"adapter":"stub","batch":16}`. Admit calls with `SdkRequest::from_json`, for example `{"function":"decide","question":{"type":"inline","value":"Ready?"},"records":["evidence"]}`. Run the request with that client. Each function has its own request shape in the [call schema](schemas/request.schema.json); [client settings](schemas/client.schema.json) travel separately. A tagged toolbox question carries `{"type":"toolbox","value":{"questions":{"ready":{"decide":"Ready?"}}},"name":"ready"}`. A named proxy question carries `{"type":"reference","name":"ready"}`. Proxy mode and named references return a usage error until the proxy transport is available.

Answered and unsure rows carry `detail.source` with `question_identity`, `backend` and `model`. Optional `detail.probabilities` holds the function's backend detail. A question identity hashes the caller-visible function, wording, options and threshold. Routes and internal prompt templates do not change it. Default JSON evidence omits the root `id`; an explicit evidence pointer, including `/id`, selects exactly that field.

Recognition selects the most likely valid positional tag path. BEGIN and INSIDE must continue with INSIDE or END; a name is SINGLE or a BEGIN-through-END stretch. Equal path scores retain the earlier tag in BEGIN, INSIDE, END, SINGLE, OUT order. The boundary decoder uses its existing probability floor. Name support uses the minimum unfloored `1 − OUT` within the decoded stretch, multiplied by kind probability. Bracket completion retains the decoded stretch's support. Longer names cannot increase support above their least supported piece.

## Test a caller without a service

Select `--adapter stub` in the command, `Backend::stub()` in Rust, `Client(adapter="stub")` in Python or `new ThinkThen({ adapter: "stub" })` in TypeScript. This built-in backend needs no key or server, reads no backend environment variables and makes no HTTP request. It ignores URL and model settings. The ten functions still run through the ordinary engine, including thresholds, batching and dependent rounds.

The stub hashes each question's own content identity, admitted evidence, the number of its preceding answers and the ask's local position in its round. Annotation uses each named question's content and selected evidence; identical content and evidence share an answer. Each field has an unsigned eight-byte big-endian byte length before its bytes; numeric fields use unsigned eight-byte big-endian values. Read the first eight SHA-256 bytes as an unsigned big-endian integer, `n`. A yes/no ask receives `p = (n % 101) / 100`. A choice or scale selects option `n % count` in caller order, starting at zero, with probability one and every other option zero. Internal prompt wording, backend addresses and JSON record identifiers do not change default stub answers. Different keys can share an answer because the answer domains are finite. The stub measures no semantic quality.

[The shared construction cases](construction/stub.json) give complete requests and expected rows for all ten functions, including names and relation edges. Callers can use these examples to predict and compare results.

Rust hosts can open a `Client::open` operation with an `OffsetUnit`. Drive `PullCall::next_event`: send raw text with `send_text` or structured JSON with `send_json` after `NeedsInput`, call `end_input` after the last record, and read rows until `Finished`. Initial request records run before sent records. Inspect `final_error` after completion. Every call delivers all rows, including failed rows, then reports its first failure at completion. Filters return every selected record before reporting their first failure. `cancel` stops the call and closes open backend connections. Dropping it requests the same cancellation and releases the caller without waiting for a backend reply. DNS lookup and a TCP connection still being established remain subject to the attempt timeout. `PullRow` separates the row JSON from raw detail JSON so native callers can decode detail on demand. Offsets use the selected unit in answers and detail alike.

CSV and TSV preserve literal cell text. Spreadsheet applications may evaluate text beginning with characters such as `=` as formulas when opening these files. Use a text viewer or import those columns as text when the source is untrusted.

File mode writes detail JSON in each answer’s `_detail` column, including when `--details` is supplied. An Excel answer or detail that exceeds the cell text limit or contains an unsupported XML character becomes a failed outcome with an error cell. Other rows continue, and the completed file reports the failure through its exit code.

Spreadsheet `--in-place` writes retain a backup before replacing the source. Close other programs that can save the file before starting. The final source hash and Excel owner-file check can detect a conflict, but another writer can still save between that check and replacement. A detected conflict preserves the source and keeps completed answers in a separate file named in the error. Windows uses atomic replacement and flushes file data. macOS and Linux also sync the containing directory after publishing the backup and replacement.

The public contract defines error codes, settings names, canonical execution identity and fixed stub vectors. Missing credentials are a usage error with code `usage.key-missing`.

## License

MIT. See [LICENSE](LICENSE).
