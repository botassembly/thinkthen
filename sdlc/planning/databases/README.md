# Database extensions: DuckDB, SQLite, and PostgreSQL

Status: a study. Ian ruled the three extensions in on 2026-09-20. No code is authorized by this page.

Ian's words: "I want to make extensions for all three using Rust. That'll be the next set of experiments after the libraries are done. I won't hold up the launch for the database extensions, but I definitely want to do them as a fast follow."

They are surfaces eight, nine, and ten over the one Rust engine. `../libraries/README.md` holds the two rules every surface obeys: as fast as possible, and the least code to maintain, with everything possible pushed down into Rust. Both rules bind here. One page per database sits beside this one: [duckdb.md](duckdb.md), [sqlite.md](sqlite.md), and [postgres.md](postgres.md).

## Why this, and why soon

A table is where most text already sits, and SQL users will not leave it to call a command. The wish is proven. Jev opened on 2026-09-15, and five days later these were public, read by survey on 2026-09-20:

| Project | What it is | What to know |
| --- | --- | --- |
| `realZachi/pg-jev` | PostgreSQL functions in PL/Python, 243 stars | The design the others copy. It sends whole rows, packs many rows into one request, and measured that accuracy falls past 20 to 25 rows in a request |
| `colliber/duckdb-jev` | A DuckDB extension in C++, 14 stars | Takes an explicit text argument. Checks the question when the query is planned. Keeps the key in a DuckDB secret. Warns that the same call in `WHERE` and in `SELECT` is billed twice |
| `recodelabs/duckdb-jev` | A DuckDB extension in C++, a port of `pg-jev` | Packs 40 rows a request by default, past the cliff that `pg-jev` measured. Installs unsigned |
| A DuckDB extension by a well-known DuckDB developer | Announced on 2026-09-16, "about 10sec for 1k rows" | No public source found |
| `jev-pandas` | A data frame wrapper in Python | No stars yet |

Nobody has shipped one for SQLite. Nobody has shipped one for PostgreSQL in Rust. None of them records and replays, none keeps answers on disk between sessions, and none reports which model and which request produced an answer.

## The SQL surface, the same in all three

The question comes first and the text second, as in every other surface. The text is always a value the caller names. The extension never sends a whole row.

| Function | Returns | Notes |
| --- | --- | --- |
| `thinkthen_decide(question, text)` | A boolean, and `NULL` for "not sure" | SQL already has three-valued logic. `WHERE thinkthen_decide(...)` keeps the yes rows, and `IS NULL` finds the rows for a person |
| `thinkthen_rank(question, keyed_json)` | Rows of key, rank and probability, best first | The SQL form of `rank`, on all three extensions (ticket 0378). It replaced the per-row `thinkthen_probability` |
| `thinkthen_choose(question, text, options)` | Text, and `NULL` under the threshold | |
| `thinkthen_score(question, text, levels)` | A double — the specification's probability-weighted position from 0 to K−1. The nearest level's name rides in `thinkthen_details` (ADR 0017 pick 6) | |
| `thinkthen_tag(question, text, labels)` | A list of text. JSON text in SQLite | One request for all the labels |
| `thinkthen_annotate(question_set, text)` | A struct, `jsonb`, or JSON text, one member per question | One request for every question about one row. This is the 20.8 times saving measured on 2026-09-20 |
| `thinkthen_details(question, text)` | The `--details` object | The probability, the model version, the request digest, and the number of sends that produced the judgment, so a bill showing two requests never meets a tool showing one (ADR 0017 section 3) |
| `thinkthen_usage()` | A table | Requests, answers from the cache, and tokens for this session |
| `thinkthen_warm(question, text)` | A count | An aggregate. It judges the rows the database already scans at full width and fills the cache on disk. The query that follows reads the cache. It changes no answer |

`filter`, `rank`, and `find` get no functions. `WHERE`, `ORDER BY`, and `LIMIT` are those verbs, and SQL users already know them. Nine functions are the whole surface. The review of 2026-09-20 added the ninth, `thinkthen_warm`, as the one bulk form all three databases can spell. Experiment 207 proved it in all three databases, and all three ship it (ADR 0017 pick 10); SQLite needs it most, because a query there judges row by row. The name comes from "warming a cache": filling it before it is needed, so the next read is fast. PostgreSQL ships a module named `pg_prewarm` for its own cache, so the word is known there. How it takes the options for `choose`, `score`, and `tag` is open. The JSON form of a question can carry them.

A question is either the bare sentence or JSON text with the keys a question file uses, such as `{"decide": "...", "threshold": 0.9}`. That form works in all three databases and adds no new grammar. The per-database pages may offer a native struct beside it. They may not drop it.

## Rules every extension keeps

1. **No rule lives in the extension.** Thresholds, bands, scoring, request building, retries, and the cache are the engine's. The extension maps SQL types in and out.
2. **Every bulk form the database offers is used.** Ian ruled on 2026-09-20 that each surface maximizes performance with whatever its engine supports: vectors, batches, or any other scheme. `../libraries/README.md` has the rule for all ten surfaces. DuckDB hands a function up to 2,048 rows at a time, and the whole chunk crosses into the engine once and runs at full width there. DuckDB's stable C API does not say whether a vector is constant or a dictionary, read on 2026-09-20. The shim groups each chunk by the pair of question and text instead, and a repeated value costs one judgment. DuckDB's worker threads share one pool, so the width is one number for the process. SQLite and PostgreSQL call a scalar function one row at a time, and their pages call that form serial. The bulk form all three share is the aggregate `thinkthen_warm`. PostgreSQL adds a second bulk form, array overloads of the same names, and `postgres.md` says why. Equal pairs of question and text are asked once. Each page lists every such mechanism its database has, and each experiment measures rows a second beside the engine's own number.
3. **The key never appears in SQL text.** No function takes it as an argument. Statement logs, `pg_stat_statements`, and shell history would hold it. PostgreSQL uses a setting that only a superuser sets and that no view shows. SQLite reads the environment variable. DuckDB's secret store is out of reach for a Rust extension on the stable C API, read on 2026-09-20. Its key path is open between a setting and the environment variable, and `duckdb.md` holds the question. The environment variable is the safe default, because a `SET` statement puts the key into SQL text.
4. **The question is checked when the query is planned** wherever the database allows it, so a bad threshold fails before the first paid request.
5. **Answers are kept on disk, in the XDG cache home by default.** Ian ruled on 2026-09-21 that "XDG is definitely the strategy for storing configuration and caching by default", and ADR 0017 section 5 carries the ruling: the cache lives at `$XDG_CACHE_HOME/thinkthen` (the platform equivalent elsewhere), a folder the user names always wins, the 1 GiB cap and the prune ship with the location, and the on-by-default reading carries three guards. The engine's cache is keyed by the whole request. The same call in `WHERE` and in `SELECT` is asked once. Tomorrow's run of the same query costs nothing, because the folder exists by default. A recording folder makes a SQL test run with no network and no key.
6. **A failure is an error, never a `NULL`.** `NULL` means "not sure". A setting may turn a failed row into `NULL` for a long query, and it is off by default.
7. **A query can be cancelled.** A wait on the network checks for the database's interrupt, and a statement timeout is honored.
8. **The function is marked volatile** unless a page proves a weaker marking safe. The cache stops a second bill. The planner cannot be trusted to. In PostgreSQL every judging function is also `PARALLEL RESTRICTED`. Each parallel worker is its own process and would open its own width, and four workers would run four times past the vendor's limit.

## The engine under these pages

The engine stays a blocking client with scoped threads and no async runtime. ADR 0017 section 2 rules it on experiment 211's numbers: the width bench matched the async stand-in at 9.66 s through Rust, Python, and PostgreSQL, all at 32 in flight; zero threads are held between calls where the stand-in parked 16; the process-ID check rebuilds the pool in a forked child, which is the fork story a PostgreSQL backend needs; and a poll callback on the calling thread carries each database's interrupt, `pg_cancel_backend` returning in 0.22 s with the wire frozen. Where an older page says "runtime", read "the engine's pool of threads and connections".

## Rows in one request: measured, and one record per request stays the rule

Every project above packs many rows into one request, and that is why they report thousands of rows in a few seconds. ThinkThen sends one request per record, and `specification/annotate.md` says records never share a request. Inside the vendor's documented rate limit that is about 980 short rows a minute and about 1.2 US cents per thousand. `pg-jev` reports 2,000 rows in 3.5 seconds for 1.2 US cents.

Our own run on 2026-09-20 used the largest project's layout on the same 1,000 SMS messages as the accuracy round. `sdlc/issues/closed/2026-09-20-packing-rows-into-one-request-measured.md` has the tables.

- At 10 rows per request the accuracy matched one row per request, 0.968, for 3.3 times fewer tokens and ten times fewer requests.
- At 20 rows the accuracy fell to 0.941. At 40 rows the recall fell from 0.94 to 0.43, and the later rows in a request did worst. One public DuckDB extension defaults to 40.
- At 10 rows, dealing the same messages into different groups changed the answer for 34 rows of 1,000. A row's answer depends on its neighbors.

So packing keeps the totals and loses the single answers. It also breaks replay, because a recording is keyed by the whole request. One record per request stays the rule and the default on every surface. If packing is ever offered it is an engine option for an ADR, off by default, capped at ten, for `filter` and `rank` over short rows only. No page here assumes it.

## Shared goals

- One `INSTALL` or one file, with no Rust toolchain on the user's machine.
- The shared conformance cases run as SQL under replay, with the same expected answers as every library.
- The extension's own code stays a thin shim with a line ceiling in `sdlc/ratchet.json`.
- The name is `thinkthen` in every registry, with no suffix.

## Shared anti-goals

- No whole-row sending, and no function that reads a table the caller did not name.
- No text generation, no embeddings, no vector search. Other extensions do those.
- No function that writes to a table or a file.
- No background worker, queue table, or daemon in the first release.
- No difference in names or meanings between the three databases beyond what a type system forces. PostgreSQL's array overloads are the one forced difference so far: SQLite has no array type.

## The order

DuckDB first. Its vectors fit the engine's width, its community repository signs and ships the binary, and it is where the public interest is. SQLite second, because nobody has done it and it is the smallest. PostgreSQL third. It is the hardest: each connection is its own forked process, managed services allow only approved extensions, and the realistic first user runs their own server or a Docker image.

The experiments run as local experiment 207 in the workspace, after experiment 205 on the libraries, and they bind the same stand-in engine and stub.

## What Ian can overturn

Everything but his own ruling that the three are built in Rust and do not hold the launch. The function list, the order, and the packing question are open.
