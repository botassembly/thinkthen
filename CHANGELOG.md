# Changelog

Every release of every surface shares one version number.

## Unreleased: 0.1.0

The first release. The command, the Rust, C, Python, TypeScript, Ruby, and R libraries, and the SQLite, DuckDB, and PostgreSQL extensions.

The command handles SIGTERM like Ctrl-C, reports a signal as the cause when a sent request later fails, and gives the finished count without naming a record that may not exist.

The command reads at most 1 MiB of a question file or question set, as the libraries do. A larger file, such as `/dev/zero`, exits 5 with `the question file is too large` before any request.

The SQL extensions add `thinkthen_try_details` so a recoverable bad row yields a safe typed JSON failure and later good rows continue. SQLite adds a connection-scoped ThinkThen time budget. DuckDB retires idle engine plans while retaining cumulative usage and its 16-plan cap. PostgreSQL refuses a changed explicit throttle. DuckDB's whole-query budget follows in ticket 0201.

Every surface paces its HTTP attempts to 1,000 a minute for each `https://` address, under the vendor's published 1,200. `THINKTHEN_REQUESTS_PER_MINUTE` sets another rate. The pacer counts within one process (ticket 0308).

Every surface reads `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL`, the estimated input admission total from ticket 0299. A request whose estimate would pass it is refused before it is sent. The command flag, the Rust setter, and the C key outrank the variable (ticket 0311).

The SQL extensions cache only in a named folder and refuse a folder another user owns or others can write (ticket 0318).

Named backends let one environment hold several providers' keys. `--backend NAME`, `THINKTHEN_BACKEND`, the configuration file's `backend`, and `EngineBuilder::backend` select one. The built-ins are `typesafe` (`TYPESAFE_API_KEY`) and `liquid` (`LIQUIDAI_API_KEY`, then `LIQUID_API_KEY`); the configuration file's `backends` adds more by URL, key variable name, and model. Each backend sends only its own key, and a built-in's key never goes to the other built-in's host. With no backend named, every command behaves as before, except that `status` prints `backend` and `key_variable`, `status --json` moves to `thinkthen.status/2`, and the `check` no-address sentence names `--backend`. The bindings' and SQL `backend` setting follows in ADR 0114 build slice 2 (ticket 0334, ADR 0114).

A third built-in backend, `ollama`, reaches Ollama at `http://localhost:11434/v1` with model `nimble`. At that loopback base it sends no key; at another base it reads `OLLAMA_API_KEY`, and `OLLAMA_API_KEY` never goes to the `typesafe` or `liquid` host. A loopback built-in base no longer counts as another built-in's host, so `--backend liquid --url http://localhost:8080/v1` still passes. Ollama refuses object descriptions today, so `ollama` alone sends each description object as its `what` text, a temporary workaround tracked as debt. `thinkthen check --backend ollama` warns about the lost detail instead of failing, and `--plan` says so once on standard error. Every other backend sends descriptions exactly as before. The unknown-backend sentence lists three names. A configuration file that already named an entry `ollama` under `backends` is now refused at exit 5, because the name belongs to a built-in; rename the entry (ticket 0339, ADR 0115).

Every engine built from the environment adds its requests, retries, live tokens and cache answers to the command's count-only usage totals, so `thinkthen status` shows one combined total for every surface. The SQL extensions write their counts when their process exits. PostgreSQL writes to the server user's usage folder; its request total and token cap still bind per backend (ticket 0322, ADR 0113).

A relate question file may mark a rule `"single": true` when each source has at most one target. That rule asks one menu per source, listing every allowed target and `none`, so a source can end with no edge; every other rule asks its yes/no pairs as before. The relate page states relate's measured precision and points to `recognize --relation` for relations a text states (ticket 0342).

The C door adds `thinkthen_plan_json`, a no-send preview of a judgment call that needs no key. It returns the planned records, requests, body bytes, input-token band and first request body as the result schema's `plan` object, which the schema now generates from the Rust type (tickets 0291 and 0314).

The Go and C++ libraries gain a public `plan` over `thinkthen_plan_json` and read facts, recognize and relate results as plain JSON. Go drops its `Facts` struct; C++ drops `CallFacts`, `Entity`, `Recognized`, `RelatedEntity` and `Edge`. Go names its error kinds, and both read an annotate member as unresolved, a value or a failure. C++ `call`, `recognize` and `relate` take a deadline (tickets 0291 and 0314).

The C#, JVM and Dart libraries gain a public `plan` over `thinkthen_plan_json` and read facts, recognize and relate results as plain JSON. C# drops its `Facts` record and `CallResult`; the JVM drops `Door.Facts` and `ResultEnvelope`'s strict key sets, keeping a small tolerant reader as `thinkthen.Json`; Dart drops `CallFacts`, `CallResult`, `Entity`, `Edge`, `Recognition`, `Relations` and `Annotation`. Each reads an annotate member as unresolved, a value or a failure. C# `Relate`, JVM `recognize` and `relate`, and Dart `call`, `recognize` and `relate` take a deadline (tickets 0291 and 0314).

The Swift, Objective-C and COBOL libraries gain a public `plan` over `thinkthen_plan_json` and read facts, recognize and relate results as JSON text. Swift drops `CallFacts`; Objective-C drops `TTCallFacts`, `tt_call_facts_clear`, `parsedAnswer` and TTJSON's envelope and answer shape checks; COBOL drops its seven-key facts check and `TT-VALIDATE-RESULT` and gains `TT-JSON-MEMBER` to read named members. Each reads an annotate member as unresolved, a value or a failure. Swift and Objective-C `recognize` and `relate`, and COBOL `TT-DECIDE` and `TT-CALL`, take a deadline (tickets 0291 and 0314).

The Ada and Zig libraries gain a public `plan` over `thinkthen_plan_json`. Ada reads facts, recognize and relate results as JSON text and drops `Run_Facts`, `JSON_Result`, `Entity`, `Relation_Edge`, `Entities`, `Edges`, `Call_Facts` and `Call_Value`, gaining `Member` and `Element` to read named members. Zig reads facts as `std.json` values and drops `CallFacts`; its `Outcome` carries the C values 1, 0 and 2, and `readField` reads an annotate member as unresolved, a value or a failure (tickets 0291 and 0314).

The Ruby, PHP and TypeScript libraries gain a public `plan`. Ruby's `plan` and TypeScript's `plan` preview through the engine directly, and PHP's `plan` wraps `thinkthen_plan_json`. PHP drops its strict facts check, so facts read as plain JSON. TypeScript now writes facts, call details and usage through the engine's own serialized types. Each library adds `YES`, `NO` and `UNSURE` codes and a `failed` helper that tells an annotate failure from an unsure `null`. Ruby and TypeScript errors carry their C code. Ruby's `Engine.new(max_requests_total:)` and TypeScript's `new Engine({maxRequestsTotal})` cap the process's live sends. Ruby's deadline keyword is now `deadline_ms:`, in milliseconds, replacing `deadline:` in seconds (tickets 0291 and 0314).

The default model is the pinned version `jev-1.13.0`, not the alias `jev-latest`, so a vendor's move of its alias moves no default answer (ticket 0159).

Python and Ruby calls now raise cancellation when the caller's token fires before a held reply reaches the call, including when both happen in one wait tick (ticket 0168).

`recognize` finds names in three steps, as ADR 0056 decides. Each name prints `text`, `start`, `end`, `length`, `kind` and `strength` on every surface. The default kinds are gone, so a run with no kinds prints every name as `ENTITY`. `--max-text-bytes` refuses a text over 600,000 bytes before any request (ticket 0147).

`relate` asks one yes/no question per allowed pair and keeps every edge at the cut. All rules share one entity state and requests of at most 400 questions; this changes relate request bodies and recording digests. Its version-one plan keeps one entry per rule with the requests carrying that rule's questions. Wildcard edges now print in question order (ticket 0167).

`decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` send one quoted form on every surface. The state is the fixed sentence `Each question quotes the text it asks about.` or the context, and each question quotes its own record, a batch of one included. The record-list state and the unquoted single-record request are gone, so request bodies and recording digests change and an old cache entry is never found again. A profile's evidence limit still bounds each record, and it now also counts the fixed sentence or the context, so a limit below 44 bytes refuses every such request (ticket 0304, ADR 0111).

On the command, `decide`, `filter`, `rank`, `choose`, `tag`, `score` and `annotate` keep one answer for each question in `thinkthen.sqlite`, so a longer run over the same records sends only the new records' questions. A partial reply keeps its good answers, and a rerun asks only the failed question. `meta.requests` and each annotate answer's `request` now name question keys, and `meta.batch` and `meta.batches` are gone from these rows. `--facts` counts `cache_answers` as questions. An old cache of digest entries is ignored by these commands until you run `thinkthen cache convert DIR`, which writes `thinkthen.jsonl`. It marks each answer read from an old entry with origin `converted`, or `quoted` for an entry already in the quoted form or one `--quote` rewrote (ticket 0304, ADR 0111).

`find`, `recognize` and `relate` keep one answer for each backend question in the same store, on the command and every library. A rerun sends only the questions no earlier run answered, a new relate rule sends only its own pairs, and a new recognize line sends only its own questions. Their details list question keys in `meta.requests` and in each answer's `request`. A failed answer is never stored, so replaying a relate run that failed one pair exits 5 and names that question, where it exited 6 before. `find --record` no longer stops at a conflicting entry (ticket 0304 slice 4, ADR 0111).

The Rust library's `decide_many`, `filter`, `rank`, `choose_many`, `score_many`, `tag_many`, `details_many` and `annotate` run on the command's question pipeline. Their rows list question keys in `meta.requests` and carry no `meta.batch`. A longer call over the same records sends only the new records' questions, and an annotate record's groups pack together. `Max` now sends the open request after 50 ms with no new record, and a 413 sends both halves of the refused request (ticket 0304 slice 3a, ADR 0111).

The Rust Polars door writes each answer into its output column as the answer arrives, instead of collecting every row first. `annotate_frame` over 100,000 rows peaked at 47 MB instead of 109 MB. With a cache folder, a lazy frame collected again sends nothing (ticket 0304 slice 3c).

`thinkthen decide --plan` and the library's plans count each question's real options, so a plan's option count changes where it printed 0 before. Two concurrent single calls for the same question no longer share one send: a call coalesces only its own questions, so each call sends its own request (ticket 0304 slice 3a).

On Linux, the C archive's `libthinkthen.a` defines only the header's `thinkthen_` functions as global names, so a program can link its own SQLite or another Rust static library beside it. The R package on Linux exports no SQLite name either. The macOS archive still exports SQLite's names. The SQLite extension refuses a cache, record or replay folder when the host's SQLite is single-threaded, and a call that names no folder still answers. In DuckDB's `thinkthen_annotate`, a record missing an `on` part fails its statement, and the other records of the same vector are still sent and stored. It now refuses more records than `thinkthen_max_requests` before any send (ticket 0304 slice 3b).

The PostgreSQL and DuckDB extensions count `max_requests_total` against the engine's one process total instead of their own counters, and PostgreSQL leaves the throttle conflict to the engine. Settings and refusal sentences are unchanged. The Rust library adds `CallOptions::max_requests_total`, a call's cap on that process total, and `process_requests_sent` (ticket 0304 slice 3e).

`diff` compares two `recognize` or `relate` runs, or two cuts on one. Each changed record lists the names or edges it gained, lost, or changed in kind, and a key runs McNemar on the key names or edges only one side matched. `--match strict|overlap` pairs names as `audit` does (ticket 0165).

Every surface refuses an API key holding any control character, not only a line break, and says `the API key contains a control character` (ticket 0321).

`audit` and `diff` treat results whose lines name no `meta.batch.setting` as an unknown setting. `audit --write` then leaves a question file's `batch` member alone and reports nothing about it, and neither command warns about mixed settings because of such a run. Rows from the command's record functions carry no setting (ticket 0304).

### Breaking changes

This is the first release. These changes break earlier builds from main:

- `--record` no longer stops at a conflicting entry. It replaces the answer stored under the same question key (ticket 0304).
- `meta.batch` and `meta.batches` are gone from the rows of the command's record functions (ticket 0304).
- `meta.batch` is gone from the Rust library's batch rows, and `meta.requests` names question keys (ticket 0304).
- `thinkthen cache convert DIR` deletes `thinkthen.sqlite` after it merges the file's answers into `thinkthen.jsonl` (ticket 0304).
