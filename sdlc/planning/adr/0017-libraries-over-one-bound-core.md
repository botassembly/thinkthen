# ADR 0017: Ten surfaces over one bound engine

- Status: Accepted by Ian on 2026-09-21: "We can always fix it." The build team still reviews this page before the merge ticket, and a review finding amends it
- Date: 2026-09-21. This page rewrites the 2026-09-19 draft whole, on the strength of experiments 205 (six language bindings), 207 (three database extensions), and 211 (a blocking engine bound into Python and PostgreSQL)

## Context

The 2026-09-19 draft bound the pure core into each language and left the sending, the retries, the scheduling, and the recording to the host language. Ian's two rules of 2026-09-20 reversed that ownership: fast, and the least code to maintain, with everything possible pushed down into Rust. Two experiments then found the same missing layer from two directions. The code every surface needs sits inside the command as private modules, and each experiment built its own stand-in copy just to test a binding. Experiment 211 built the layer a third time, blocking, with no async runtime, and measured it inside the two hosts that fork. This rewrite binds that layer, not the core alone.

## Decision

### 1. Three layers in one crate

One crate named `thinkthen` holds three layers. Nothing named `thinkthen-core` or `thinkthen-cli` is ever published. The pure core remains a module path inside the crate, and the purity lint holds over that path as it does today. The command sits behind a default `cli` feature, so a library user never compiles the argument parser.

| Layer | Holds | May touch |
| --- | --- | --- |
| `core` | The question grammar, thresholds and bands, the wire shapes, the digests, the recording key | Nothing. The lint holds it pure |
| `engine` | Sending, retries, the scheduler, the width gate, the cache, record and replay, the cancel token, the deadline, the counters | The network, the folders the caller named, threads |
| `cli` | Arguments, standard input and output, exit codes, messages | The terminal and the process |

A shim per surface converts arguments, calls one engine function, converts the result, and maps a failure to the host's own error. The command is the first shim. This reverses item 3 of the 2026-09-19 draft, which gave the host language the sending and the recording. It also settles the sentence in the prospective plan that says hosts own files, credentials, retries, and concurrency: the engine owns the retries, the scheduling, and the recording, and the host owns the folders it names and the key it holds in a variable.

### 2. The engine stays blocking

The engine is a blocking client over scoped threads. It holds no async runtime, no resident thread, and no runtime-created state a fork can orphan. Experiment 211 measured this shape against the async stand-in of 205 and 207:

| Measure | Async stand-in | Blocking engine, 211 |
| --- | --- | --- |
| 1,000 records at 300 ms, `jobs` 32 | 9.65 s | 9.66 s in Rust, 9.658 s through Python, 9.673 s inside PostgreSQL, all at 32 in flight |
| Null backend, cost a record | 1.32–1.37 µs | 1.21–1.59 µs in Rust |
| The same through Python | 1.91–1.97 µs | 1.47–1.52 µs |
| Threads held between calls | 16 | 0 |
| A child forked after a call | hung past 30 s | answers |
| Ctrl-C during a batch | deaf 8.07 s, or 0.23 s with chunking | heard in 0.21 s with no chunking |

Blocking matches the async stand-in on the width bench and beats it into Python. The costs that came with the runtime leave with it: the fork hang, the parked threads, the second runtime the Python bridge had to spawn, and the deaf Ctrl-C.

Four rules follow, each with its evidence.

- **One width gate for the process.** Every request path, single or bulk, acquires one process-wide gate. One hundred calls at once held 32 in flight, where DuckDB's per-batch gate had run 85 wide on 101 connections. The connection pool is sized to the gate, because the client's default of ten idle connections churned 488 connections where 33 serve.
- **The fork check is by construction.** The engine stamps its process ID into its state at build. Every call entry compares, and a mismatch rebuilds the pool and the gate. The state sits behind an atomic slot, the rebuild takes no lock a request path can hold, and retired state is leaked rather than torn down. A child forked during a live 32-wide batch answered its own wire call in about 353 ms across ten rounds. The first design held its state behind a mutex and passed its test only because the inherited-lock window is narrow; the fix removes the lock from the rebuild path instead of trusting the window. In PostgreSQL, the lazy arm answers with state built in the backend's own process, and the shared-preload arm answers from a backend that inherited the postmaster's stamp. A postmaster-built live pool stays untested, because the postmaster's own init may not touch the wire.
- **The calling thread polls.** A bulk call waits on a timeout loop, and each tick of 50 to 100 ms checks the cancel token and runs a callback on the calling thread. Python passes a closure that re-takes the interpreter lock and checks signals, so Ctrl-C raises `KeyboardInterrupt` 0.21 s after the signal with nothing served after the return. PostgreSQL runs its interrupt check in the same callback, and `pg_cancel_backend` returns in 0.22 s. SQLite's progress handler is the same shape. The promise is exact and no larger: no new request starts after a cancel, and requests already sent finish.
- **Nothing is held between calls.** Zero threads outlive a call; the async stand-in parked 16 for the session. The transient spike is measured and named: the engine's own share is one worker per in-flight request plus a feeder, and the rest is the HTTP client's default resolver, which races one short-lived thread per simultaneous dial even for a numeric address. A batch on the wire peaks near twice the width during the ramp, settles to the width plus two once the pool is warm, and returns to the caller's thread after; the spike scales with the width, never with the record count, and never persists. The real engine resolves numeric addresses synchronously and halves the ramp. The allocator keeps about 14 MB of working set after a 100,000-record batch, and the note says so rather than fighting it. Idle pool connections prune at 15 s, and a dead pooled connection is retried on a fresh one without an error reaching the caller.

### 3. One error shape

The engine has one public error with six kinds. The command wraps it in `Failure` and keeps the exit codes of ADR 0007; which code carries `cancelled` and `deadline` is a merge-ticket decision recorded there.

| Kind | What it says | A second try could help |
| --- | --- | --- |
| `usage` | The request was wrong, and nothing was sent | No |
| `backend` | The wire failed or refused. Busy replies (429, 500, 502, 503, 504, 529), timeouts, resets, and closed idle connections say yes. A refused connection, 401, 402, 403, 404, 422, and a reply that does not decode say no | Yes for the first list, no for the second |
| `local` | A file or recording the caller named failed | No |
| `cancelled` | The token fired. Sent requests finished | No |
| `deadline` | The caller's own budget ran out. It says nothing about the backend's health | A new call with a new budget is the caller's decision, not a signal about the backend |
| `defect` | A bug in the engine | No |

The `deadline` kind is Ian's ruling of 2026-09-21: a caller with a fallback treats "my five seconds ran out" and "the backend is busy" differently, so the spent budget never files as a backend error. The retryable reading inside `backend` comes from the tool-search page, whose caller falls back on a timeout and a rate limit and stops on a reply it cannot trust.

The engine counts requests, cache answers, and tokens for the process. Requests counts every send. A retried send after a dead connection counts twice, because a judgment is safe to repeat and the bill pays for both; the pages say so. The second send reaches the user in two places: `usage` carries the process's send count, and a judgment's `details` carries the number of sends that produced it, so a bill showing two requests never meets a tool showing one. A library reads the counters through `usage`, and the command keeps none between runs.

### 4. Call options

Every verb takes the same two options beside its arguments: a cancel token and a deadline. A deadline is an instant, set from a budget at call time, and it caps the gate wait, the retries, and one blocking send. A past deadline sends nothing. A 100 ms budget capped one call at 104.3 ms against a 300 ms stub, and a 1 s budget expired a 60-record batch at 1.009 s with the stub's count frozen. The error's message names the limit that ran out and its value — the deadline of 5 s passed — so a reader knows which setting to raise. The poll callback of section 2 belongs to the binding, not the public surface: it is how a host hears an interrupt while the engine waits.

### 5. Engine settings

The settings a host can reach, with one spelling each:

| Setting | Command | Library and database | Default |
| --- | --- | --- | --- |
| The address | hidden `--url`, then `THINKTHEN_BASE_URL` | the engine value's setting, then the same variable | the built-in address |
| The key | `THINKTHEN_API_KEY` | the same | none |
| The model | `--model` | the question, then the engine value | the alias |
| The width | `--jobs N` | the engine value | 4 |
| The request limit | `--max-requests N` | `max_requests` on the engine value | no limit |
| The cache folder | `--cache DIR`, or `THINKTHEN_CACHE=DIR` | `cache` on the engine value | the XDG cache home |
| The cache cap | applied by `thinkthen cache prune DIR` | `cache_bytes` on the engine value | 100 MB |

The request limit is stateless. It refuses a run before its first request when the input holds more than `N` records, it writes nothing, and `--dry-run` prints the request count the run would make. It earns its place in the databases, where one `WHERE` over a hundred million rows is a real bill.

The cache and the configuration live in the XDG homes, by Ian's ruling of 2026-09-21: "XDG is definitely the strategy for storing configuration and caching by default." The cache lives at `$XDG_CACHE_HOME/thinkthen` (falling back to `~/.cache/thinkthen`), `~/Library/Caches/thinkthen` on macOS, and `%LOCALAPPDATA%\thinkthen\cache` on Windows. A folder the user names — `--cache DIR` or `THINKTHEN_CACHE=DIR` — always wins, and `--no-cache` turns the cache off for one run. The configuration lives at `$XDG_CONFIG_HOME/thinkthen`; this brings back a configuration file Ian removed on 2026-09-19, and the build team designs the smallest file that serves: address, model, cache on or off, and the cache size limit. The repository rule that the tool never writes a file the user did not name gains one stated exception, the tool's own XDG cache and configuration folders, and the rule text changes in the same commit as the code. The API key never enters either folder, and `thinkthen status` prints both folders, the cache size, and the entry count.

Read as ruled, the cache is also on by default at that folder. Three guards hold that reading honest, and they ship with it: the 100 MB cap and the prune land with the default location, never after it; `--no-cache` and a configuration switch turn it off; and the manual's first note says answers and the texts behind them are saved on disk, because people will pipe customer messages through the tool. The on-by-default reading is Ian's alone to overturn — if he meant only the location, one word flips it and the guards stay. The state home is for logs and history and holds no part of this.

The store is one JSON file per request, named by the SHA-256 over adapter, address, and request bytes, sharded by the first two digest characters once a folder can hold a million entries. Measured at a million entries: a 5.7 µs median hit, a 30.2 s fill, 4 GB. The flat folder fails first touch at that size, 93.6 µs median with a 1.1 ms tail. One SQLite file is the fallback for a host that cannot afford the file count: 11 µs hits, 662 MB, a 0.3–0.4 ms open, and a miss costs what a hit costs. Foyer was measured as a fourth arm on 2026-09-21 and did not change the store: it fills ten times faster and answers a warm read in 0.31 µs from its memory tier, but it has no blocking API, parks a runtime, takes about 430 ms to recover on open, drops best-effort writes until its queue is paced, and hangs a forked child that reads through it. The cache page carries the four-question answers. Ian ruled the default at 100 MB on 2026-09-21. A small entry takes one 4 KB block as a plain file, so 100 MB holds about 25,000 answers. The limit is a setting in the configuration file, and each database page tells the reader to raise it before `thinkthen_warm` runs over a larger table, because a warm pass that outgrows the limit evicts its own answers.

Three rules hold the cache honest. A request reads one file or writes one file and never deletes anything; removal is `thinkthen cache prune DIR`, by age or by the cap, oldest-written first, and a prune of a million entries is a twenty-second job that belongs to a person or a cron line. With plain files the cap is applied by prune; a store with native byte-bounded eviction applies it itself. The lock files leave when the entry lands, or they shard with the entries, so a million entries is never two million files; the million-entry numbers counted no lock files. Asking once inside one call is memory and always on: equal pairs of question and text in one batch are one judgment, and only the bill can observe it.

### 6. One shape across the ten surfaces

The one-shape page made ten picks. This ADR adopts each, with the objections stated where the evidence or the planning pages disagree.

1. **The same names everywhere.** Adopted. The public list is the eight verbs (`decide`, `choose`, `score`, `tag`, `filter`, `rank`, `find`, `annotate`), `question`, and `details`. The prefix follows the host: `tt.` in Python and TypeScript, `ThinkThen.` in Ruby, `tt_` in R, `thinkthen::` in Rust, `thinkthen_` in C and SQL. No surface renames a verb or adds one, and Ruby's `decide?` goes.
2. **The first argument is the question, as text or as a built question.** Adopted. `Threshold::cut`, `Threshold::band`, `Plan::evidence`, and `Plan::model` leave `pub(crate)` and become the builder. Experiment 205 found every shim formatting question JSON by hand, and the engine builds it once. Loading a file and building from keywords give the same value.
3. **"Not sure" is the host's own empty value.** Adopted, with the objection on record. The planning pages for Python, TypeScript, and Ruby give a band its own outcome type, because an empty value is falsy and `if` would read unsure as no. The one-shape page overturns them for one rule across ten surfaces, and this ADR follows, because the command already made the same choice: in a shell `if`, exit 3 takes the `else` branch. A user who set a band opted into three answers. The mitigation is teaching, and it is required: every library page leads the band example with the empty-value check, the way the command's help shows `case $?`, and the conformance suite carries a band case. The one-shape page calls this the pick most worth a second look, and the overturn is cheap.
4. **The public word is "unsure".** Adopted. The Rust arm is `Answer::Unsure` and the C constant is `THINKTHEN_UNSURE`, and the header ships with one word. The specification keeps "unresolved" in its own grammar. No page mixes the two.
5. **A band is the host's pair.** Adopted: a tuple in Python, a list in TypeScript, a range in Ruby, `c()` in R, `.band()` on the builder in Rust. The `"0.2:0.8"` string stays in question files and on the command line.
6. **`score` returns a number on every surface, and the nearest level's name rides in `details`.** Adopted. The number is the specification's probability-weighted position from 0 to K−1. The three database experiments returned three shapes, and they collapse to this one. Job 3 rewrites the score case to it.
7. **Rust is blocking.** Adopted, with the page fix: `rust.md` shows `.await` today, and it loses it. An `Engine` value is built from the environment, and plain calls return `Result`. Experiment 211 matched the async bench at 9.666 s.
8. **Bulk is the same verbs over the host's container.** Adopted. `filter`, `rank`, and `annotate` take the container and cross once. R and SQL keep a vectorized `decide`, which is their habit. Python, TypeScript, and Ruby spell the bulk form `decide_many`, because a string is also a sequence there and guessing is a trap. `decide_many` is `decide`'s bulk spelling, not a ninth verb, and the surface check admits it by name on those three surfaces. The C door carries the same shape as `thinkthen_decide_many`, because every language that loads the C library needs one bulk entry point, and the surface check admits that name there too.
9. **SQL names a question file as `'@refund.json'`.** Adopted, the command's own spelling. Where the file may be read from is the database ADR's to rule.
10. **`thinkthen_warm` is answered, and all three databases ship it.** Experiment 207 proved it in DuckDB, SQLite, and PostgreSQL. The DuckDB page still lists it open and carries the correction. SQLite needs it most, because a query there judges row by row.

The slides leave off details, counters, cancel tokens, deadlines, and the cache setting. They exist on every surface with one spelling each, and the reference pages own them. The C surface stays the door to the rest: one call that takes a request as JSON text and returns the answer as JSON text, and one call that frees it, behind one generated header.

### 7. The conformance cases

The twenty shared cases from experiments 205 and 207 become `conformance/`, one data file that every surface reads, so a pick that changes in review is one edit: each case carries the verb, the arguments, the evidence, the recorded exchange, the expected bare answer, the expected details, and the expected failure kind. Job 3 fixes them before any surface runs them: the score case to the specification, the error kinds to the six of section 3, and the question grammar of `specification/question-file.md` for `choose` and `score` as well as `decide`. Every surface runs every case under replay, with no network and no key, and the command runs them too. A new verb adds its cases once. A language tests only its shim: the conversion of types, the mapping of errors, an interrupt while Rust waits, the release of the host's lock, and safety across threads and forks. One matrix runs all ten on any change to the core or the engine.

### 8. The merge, in four steps

1. **Fold to one crate and move the machinery down.** The five files leave the command for the engine and stop returning `Failure`. Code moves, no page changes, every gate stays green.
2. **The public functions.** The stand-in's completion layer is the draft list: an engine value built from settings, a question built from parts, the eight verbs, `details`, and `usage`. The batch spine returns every judgment; `schedule.rs` becomes the spine, and `decide_many` over a slice is a thin wrapper. A judgment exposes its model and its digest. `find` returns the whole ranking with its probabilities. The engine exports no C symbol; the C binding owns every exported name.
3. **The four things a host needs.** The width gate, the cancel token, the fork check, and the fast stop, each with a test on the wire. The interrupt test ships in the same commit as any options refactor; 211's one regression was a dropped cancel token, and the test caught it.
4. **The cache and the counters become engine settings.** The per-digest locks land here or move here, and they follow the lock rule of section 5.

The command stays the first caller through all four steps. The build team reviews this ADR before the merge ticket. Job 3 lands the cases, and job 2 proves the DuckDB interrupt inside Python, in that order after this review.

### 9. Building and release

A library artifact builds with `panic = "unwind"`. The workspace release profile says `abort`, which suits the command and kills someone else's process inside a host. Cargo ignores a dependency's profile, and each binding sets its own.

The first release is 0.1.0 on every surface, together, and builds before it carry 0.0.N. One crate carries the binary and the library, and publishing the command publishes the function surface. A number below 1.0 lets Python call that surface before semantic versioning freezes it. A user installs a prebuilt package and needs no Rust toolchain; the command installs through the curl installer, and the C surface ships the shared library, the static library, and the header from the same releases.

The registry names are Ian's alone and were on his list before this rewrite.

## Consequences

- The prospective plan's sentence that hosts own files, credentials, retries, and concurrency changes to the ownership of section 1.
- The libraries and databases planning pages take the hand-off changes from experiments 205 and 207, one commit per folder, each change with its evidence. Python, TypeScript, and Ruby lose the band outcome type and gain `decide_many`. `rust.md` drops `.await`. The DuckDB page marks `thinkthen_warm` answered, and rule 5 of `databases/README.md` says the disk cache waits for a named folder.
- Job 3 rewrites the twenty cases to the picks of section 6 before any surface runs them.
- Items 3 and 9 of the 2026-09-19 draft are replaced by sections 1 and 9 here. The port-per-language design of the sdk study stays on record as the fallback if the merge fails, and nothing in it is built.
- Exit codes, `--quiet`, and `--raw` stay in the command. A library returns values and raises its own errors.

### Ruled after acceptance, 2026-09-21: the data frame is Polars

Ian ruled on 2026-09-21 that Python's data frame container is Polars, not pandas: `annotate` and `recognize` take and return Polars DataFrames, the bulk form accepts a Polars column, Polars rides as an optional dependency behind `pip install thinkthen[polars]`, and pandas leaves the surface. The ruling is recorded with its reasons in `sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md`.

## What Ian can overturn

All of it. Three reach widest. Staying blocking is measured now, and overturning it costs the fork story, the zero-thread holding, and the cheaper Python shim. Pick 3, the empty value for "not sure", is the one the one-shape page flagged for a second look, and its mitigation is teaching rather than code. The cache default, 100 MB by Ian's ruling and prune-enforced under plain files, changes with one number. The on-by-default reading of the cache ruling is Ian's alone to flip; the three guards stay either way. The registry names were his before this page and stay his.

## Amendment, 2026-09-21: the step-1 engine boundary

The build review required before the one-crate move found that section 8 step 1 cannot move the five named command files whole. This amendment replaces that step and its proof obligations. Ian can overturn any item.

1. **Extract through a private bounded bridge.** The command keeps argument parsing, environment and credential lookup, input framing, its detached standard-input reader, output, diagnostics, and exit codes. It sends framed input events to the engine. The engine receives resolved questions, backend settings, records, and run settings, then returns typed per-row result events through a callback or equivalent bounded bridge. It does not collect a whole run. Scheduling, transport, recording, and `cache_lock.rs` move into the engine.
2. **Keep the step-1 errors private and exact.** A private engine failure carries one of the future six kinds plus the structured recording or backend cause and the ordered-stop metadata needed to reproduce the current command message and count. The command alone maps it to `Failure` and an exit code. Step 1 fixes no public `thinkthen::Error` type.
3. **Preserve every command contract.** Step 1 changes no request, retry rule, order, width bound, memory bound, recording, output byte, diagnostic, or exit code. The current retry of every transport failure remains. Refused connections gain their new fast-stop behavior only in section 8 step 3.
4. **Keep core purity mechanical.** The `core` module carries module-level `forbid` attributes for the accepted disallowed APIs. A policy check scans the whole `core` source tree, refuses dependency paths to `engine` or `cli`, and allows only the runtime dependencies in the former `thinkthen-core` manifest: `serde`, `serde_json`, `sha2`, and `thiserror`. Its self-tests plant one prohibited standard API, one reverse module reference, and one outer-only dependency such as `ureq` in `core`, then prove that each fails.
5. **Prove an independent library package.** Every CLI-only dependency, including `clap` and `csv-core`, is optional, and the binary requires the `cli` feature. The gate builds the library with default features off, checks that resolved dependency graph, packages it without a sibling crate, and runs the retained core doctests. A release library proves `panic = "unwind"`; only the command release path opts into abort through its build command or an equivalent packaging step.
6. **Bound only engine-owned lifetimes.** Engine request workers and internal feeders are scoped and joined before its call returns. The command-owned input reader may remain detached because it lets a blocked standard-input read coexist with completed output and an early closed-pipe exit. A test calls the engine inside a process that remains alive and proves that no engine worker remains. Section 8 step 2 may replace the private bridge when it fixes the public iterator design.

The remaining order is this amendment, Job 3's shared conformance cases, then the section 8 step-1 merge. Job 2's DuckDB interrupt proof inside Python is already complete in `sdlc/planning/databases/duckdb.md` and experiment 207. It supplies evidence for the accepted interrupt design and no longer sits in the remaining order. This sentence supersedes section 8's earlier final sentence that placed Job 2 after Job 3; the accepted history stays above.

## Amendment, 2026-09-21: the shared case contract

Ticket 0052 turns Job 3's experiment evidence into `conformance/cases.json`. This amendment fixes the meanings that every later runner uses. Ian can overturn any item.

1. The initial file has twenty-five cases. It covers the eight verbs and the six error kinds. The schema declares its actual length without fixing a permanent count.
2. A `rank` result is an ordered list of zero-based input indexes beside their yes probabilities. A `find` result is a zero-based selected input index or null beside every candidate probability in stable input order, with `none` last. Each host maps those indexes back to its own container.
3. A successful exchange embeds its exact request bytes as a JSON string and its response as structured JSON. `captured` names a stable repository recording whose request and response match. `synthetic_contract` names a stubbed or shaped exchange. Neither kind carries headers or credentials.
4. A fault case names one of the six public error kinds and a deterministic injection. `local`, `deadline`, and `defect` remain schema contracts in this ticket. The section 8 step-1 runner proves their behavior through its private engine boundary. Faults do not pretend to be replayable product recordings.
5. The offline pure-core integration test uses the production question grammar, request encoder, response decoder, digest, answer rules, ranking, and find selector. It contains no second product canonicalizer or probability formula. The section 8 step-1 merge adds the first command runner over the same file.

Job 3 is complete. Section 8 step 1 is next.
