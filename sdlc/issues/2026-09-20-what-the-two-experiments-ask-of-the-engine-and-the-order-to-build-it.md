# What the two experiments ask of the engine, and the order to build it

Status: Open

Ian asked on 2026-09-20 for a plan to finish the command and the Rust code under it. Two experiments finished the same day. `experiments/205-thinkthen-libs/FINDINGS.md` covers six library bindings. `experiments/207-thinkthen-db/FINDINGS.md` covers three database extensions. Both ran on a stand-in engine against a local stub, and the stand-in wraps the real `thinkthen-core` by path. This page turns their findings into an order of work. It authorizes nothing. It steers `sdlc/planning/prospective-bash-rust-python-plan.md`, and the builder writes the tickets.

## The main finding

Both experiments had to invent the same missing layer. The repository has a pure core and a command. The code every other surface needs sits inside the command: `http.rs`, `schedule.rs`, `annotate_schedule.rs`, `recorder.rs`, and the request loop in `asking.rs`. All of it is `pub(crate)`, and all of it returns `Failure` or an exit code. Nine surfaces wait on that code. This page calls the missing layer the engine.

The one crate gets three layers:

| Layer | Holds | May touch |
| --- | --- | --- |
| `core` | Questions, thresholds, framing, the wire shapes, digests | Nothing. The lint holds it pure |
| `engine` | Sending, retries, the scheduler, the width gate, the cache, record and replay, the cancel token, the counters | The network, the folders the caller named, threads |
| `cli` | Arguments, standard input and output, exit codes, messages | The terminal and the process. It sits behind a default `cli` feature, and a library user never compiles `clap` |

The prospective plan says "Hosts own files, credentials, retries, and concurrency." Ian's ruling in `sdlc/planning/libraries/README.md` reversed that: sending, retries, scheduling, and recording are written once, in Rust. The sentence needs the change.

## What changes in the order

The builder's order for the command stands: `tag`, CSV and TSV input, the correction pass, the cache locks, `find`, then page 16 and the transforms. Ticket 0023 is still `ready` and fits the correction pass.

Two changes:

1. **The ADR 0017 rewrite moves ahead of the one-crate merge.** The plan puts the rewrite after version one. The merge makes the command "the first caller of the public Rust library shape". A merge ticket with no ADR would choose that shape alone. The rewrite touches no code, and it runs beside the rest of the command work. The experiment team drafts it after its run on a blocking engine, per `2026-09-20-feedback-to-the-experiment-team-after-both-harvests.md`. The build team reviews it before the merge ticket, because it names their modules.
2. **The merge becomes four steps.** The command stays the first caller through all four, and the existing how-tos and gates prove each one.

## The engine, in four steps

**Step 1. Fold to one crate and move the machinery down.** This step only moves code. `core` becomes a module with the purity lint held over its path. The five files above move into `engine` and stop returning `Failure`. The engine gets one public error with a small set of kinds: usage, backend, local, cancelled, defect. The command's `Failure` wraps it and keeps the exit codes. The step is proven when every gate is green and no page changes.

**Step 2. The public functions.** Take the stand-in's completion layer as the draft list, because three extensions and six bindings already called it: `experiments/207-thinkthen-db/engine/src/lib.rs`. The surface is an engine value built from settings, a question built from parts, the eight verbs, `details`, and `usage`. Four points come from the findings:

- The batch spine returns every judgment. `filter` returns only what it kept, and a banded answer over a column had no bulk path. Both experiments hit this, and shims in both hand-rolled a scheduler against the anti-goals. `schedule.rs` already takes an iterator and returns results in order. Make it the spine. `decide_many` over a slice is a thin wrapper.
- A question is built from parts: text, threshold, model, options, levels. Every binding formatted question-file JSON by hand. `Threshold::cut`, `Threshold::band`, `Plan::evidence`, and `Plan::model` are `pub(crate)` today and become public. A judgment exposes its model and its digest.
- The engine exports no C symbol. The stand-in's two exports leaked into every shared library built over it. The C binding owns every exported name.
- The stand-in's `score` maps a probability to a named band. `specification/score.md` says a position on the levels. The real function follows the specification.

The twenty shared cases in `experiments/205-thinkthen-libs/shared/cases/` and `experiments/207-thinkthen-db/engine/cases2/` are data. Replayed against the real engine, they start the conformance suite every later surface reuses.

**Step 3. Four things a host needs and a command never did.** A command lives for one run and dies on Ctrl-C. A library lives inside someone else's process. Each need gets a test on the stub's wire. The null backend hid the fork hang in both experiments, and no test of these four runs on it.

| Need | What was seen | Test |
| --- | --- | --- |
| One width for the process | 100 single-row DuckDB calls ran 85 wide on 101 connections. PostgreSQL parallel workers multiplied the width again | 100 calls at once never exceed `jobs` in flight |
| A cancel token | DuckDB ran 16.6 s of paid work after Ctrl-C. A token checked between requests cut that to between 0.04 and 0.29 s, and cut SQLite's from 1.9 s to 106 ms | After a cancel no new request starts, and the call returns `cancelled` |
| A fork check | A child forked after a call hangs on the wire in Python, Ruby, and PostgreSQL | Fork after a call, and the child's call answers |
| A fast stop | A dead address crawled about 3 s for every call | 1,000 records against a dead address return after one retry cycle |

The gate covers one process. PostgreSQL's parallel workers are separate processes, and the `PARALLEL RESTRICTED` marking holds them. The experiment measured its cost at 3.7 times on the parallel scan.

The promise for cancel is exact: no new request starts, and requests already sent finish. Every page promises that and no more.

**Step 4. The cache and the counters become engine settings.** The per-digest locks from the builder's step 4 land here, or move here in step 1. They also give the databases their rule: equal pairs of question and text are asked once. `usage` counts requests, cache answers, and tokens for the process. The digest in `details` is `Exchange::digest` over the real request bytes, and the stand-in's digest gap closes with no new code. `annotate` already packs a question set into one request, and that closes the stand-in's other gap.

## Stay blocking

The stand-in engine used an async runtime, and the runtime caused three of the findings. The process-global runtime hangs after a fork. It parks 16 threads in the host for the whole session. It forced a second runtime into Python's async bridge. The real engine is a blocking client with scoped threads. It holds no thread between calls. A fork check has only the connection pool and the width gate to rebuild. **The recommendation is to stay blocking.** The cost is one thread for each request in flight, at most 32. Node runs the blocking call on a worker thread and cancels through the token.

One number proves it. The experiments' bench is 1,000 records, a 300 ms stub, and `jobs` at 32. The stand-in took 9.65 s, and about 1.3 µs a record on the null backend. The real engine runs the same bench at the end of step 3. If it misses by much, the ADR looks at a runtime again.

## Two rules for building

- A library artifact builds with `panic = "unwind"`. The workspace release profile says `abort`. That setting suits the command. Inside a host, an abort kills someone else's process. Cargo ignores a dependency's profile, and each binding's build sets its own.
- The first release is 0.1.0 on every surface, and builds before it are 0.0.N. Ian ruled it on 2026-09-20, and `2026-09-20-the-first-release-is-0-1-on-every-surface.md` has the ruling. One crate carries the binary and the library, and publishing the command publishes the function surface. A number below 1.0 lets Python call that surface before semantic versioning freezes it.

## Parked questions this page answers

| Question | Answer |
| --- | --- |
| A chunk ceiling for bulk calls, or a cancel token | The token. The whole container crosses once. No shim chunks for the sake of Ctrl-C |
| Fork policy | The process-ID check in the engine. `2026-09-20-a-process-that-forks-after-its-first-call-hangs.md` has the reasons |
| `thinkthen_warm` on a question with a band | Never an error. The threshold does not enter the request. `warm` caches the reply, and the band applies when a query reads it |
| The default flush for `thinkthen_warm` | 256 rows. The flush bounds memory, and the width gate sets the speed |
| The return shape of `thinkthen_score` | A double in all three databases, because the specification prints a number. The nearest level's name goes in `thinkthen_details`. PostgreSQL already returns the double |

Left for the database ADR: the DuckDB version pin and the PostgreSQL key channel. Left for the Python ticket: the free-threaded wheel. Left for Ian: the registry names, already on his list.

## What Ian can overturn

Everything here except the release number, and that ruling is his own. Three reach widest: staying blocking, moving the ADR 0017 rewrite ahead of the merge, and the double for `thinkthen_score`.
