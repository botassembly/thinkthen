---
flow: build
priority: 85
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance sdlc/ratchet.json sdlc/planning
---

# 0085: Build the real engine facade

Status: built on `ticket/0085-real-engine-facade`; code review pending. Record: `sdlc/records/0085-build-real-engine-facade.md`. Owner: Claude.

## Outcome and authority

Build one private, blocking, host-neutral, typed facade over the production engine for all ten functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `annotate`, `find`, `recognize`, and `relate`. Route the command through that facade. Ticket 0086 exposes the later public Rust API over the same facade. This ticket does not publish an API.

ADR 0017 fixes one crate, one blocking engine, six error kinds, shared call controls, one process width cap, cache and counter ownership, and one shape across hosts. Tickets 0084 and 0095 freeze the Rust contract names and typed shapes this ticket implements privately. Tickets 0080 and 0081 fix the ninth and tenth functions and their shared relation planner. Tickets 0076, 0077, and 0078 fix whole-call deadlines, process width, and host signal ownership before this work opens another caller. Ticket 0096 adds fork recovery after it. The current engine, core, command, and `conformance/` files are production authority.

The `surfaces` branch is read-only shape evidence. Its contract crate, stand-in connector, surface methods, and tests show conversions and host expectations. They are not production code, do not prove real-engine behavior, and must not be copied into main. Where that branch conflicts with ADR 0017, ticket 0084, the accepted 0080/0081 tickets, or current conformance, main's accepted records control.

Ian can overturn the facade boundary, the private type grouping, and the budgets below. The one-engine ownership, blocking execution, ten-function list, control contracts, and no-copy rule remain controlled by ADR 0017 and their owning tickets unless he replaces those decisions.

## Boundary

Add one private completion layer inside `engine`. It accepts the typed questions, records, entities, relation rules, immutable engine settings, and per-call options fixed by ticket 0084. It returns host-neutral typed values, ordered bulk outcomes, result metadata, usage snapshots, and the six structured failure kinds. No facade signature accepts command arguments, standard streams, exit codes, rendered JSON rows, host objects, dynamic JSON, or a language binding type.

The facade owns orchestration only. It delegates each concern to its existing owner:

- The core parses and validates question files and typed values, constructs plans, interprets answers, applies thresholds, ranks and finds, assembles recognition, and plans and assembles relations.
- The private engine prepares one canonical request identity, splits under backend limits, schedules bounded work, sends and retries, records and replays, caches, counts, cancels, applies deadlines, and enforces process width.
- The command keeps argument and environment lookup, credential lookup, standard-input framing, CSV and TSV decoding, detached input reading, terminal rendering, diagnostics, signals, and exit codes.

The command converts its resolved inputs into facade values and converts facade outcomes into its existing output. It must not retain a parallel direct route into a scheduler, prepared request, recorder, HTTP client, recognition pipeline, or relation planner. Ticket 0086 later wraps these same private calls and types. It must not reconstruct a request path beside them.

Keep the low-level engine modules private. Add no C symbol, connector trait, dynamic dispatch requirement, plugin door, host callback ABI, or public re-export. A narrow internal trait is allowed only as a deterministic test seam over transport or fault injection that the current engine does not already provide. It must not become a second engine interface.

## Facade contract

One immutable engine value holds resolved engine settings. Construction validates all local settings before external effects and preserves explicit versus omitted width for ticket 0077. It holds no resident worker thread. The engine value keeps its retained pool, width state, counters, and recorder and cache coordinators behind one private state accessor from the start. Ticket 0096 lands after this ticket and puts its PID guard in that accessor. Nothing outside the accessor holds a raw reference to that state.

Every function has one typed call path. Convenience within the command may reduce to shared helpers, but one function never re-parses another function's rendered output. Scalar and bulk forms use the same typed judgment path. `filter` and `rank` retain record identity through input indexes or typed borrowed records. `find` returns its selected input and all candidate probabilities. `annotate` preserves named-question order and answered-or-failed members. `recognize` and `relate` consume the 0080 planner and result owners without another tokenizer, splitter, planner, threshold rule, or edge assembler.

Every successful logical result carries the production metadata its contract requires: model, question identity, logical request digests in construction order, successful send count, cache status, provider usage when present, and the probabilities needed to derive its bare value. Reading a bare value, details, a record mapping, or already-returned metadata from that immutable result sends nothing. The existing standalone `details(question, evidence)` meaning remains a new judgment call if ticket 0084 retains it.

Bulk calls return completed outcomes in input order, independent of worker completion order. A valid unresolved answer remains distinct from a failed judgment. A logical failure preserves already completed good outcomes and the exact failure marker or structured cause required by conformance. Cancellation, deadline, local failure, or backend failure preserves only results the owning scheduler considers complete, carries existing stopped-at metadata, dispatches no later work after the stop checkpoint, and joins every engine worker before returning. The facade does not collect an unbounded input merely to simplify a host result.

Each call accepts the shared cancel token and an optional one-instant deadline. Cancellation and deadline precedence, gate waits, retry waits, sent-attempt completion, and worker joining remain exactly as tickets 0073, 0076, and 0077 fix them. Keep the cancel and deadline poll in one function. Ticket 0097 adds 0095's interrupt check there after this ticket lands.

The facade uses the one process width state from ticket 0077. An omitted width stays omitted. A conflicting explicit width fails locally before input dispatch, key lookup, cache or recording mutation, request accounting, connection, or send. The facade installs no process signal handler. The command alone retains its SIGINT and `SIGXFSZ` behavior.

Cache, replay, recording, and request coalescing keep their production meanings. A stored answer reports zero sends and increments the cache-answer counter under the existing rule. A live retry counts every started attempt. Process usage snapshots remain cumulative and reset in a forked child as ticket 0096 specifies; durable command totals remain the command's count-only files. The facade does not add a counter reset. The CLI `status` and cache commands continue to read the same cache and counter owners rather than a facade-side mirror.

The six facade failure kinds are `usage`, `backend`, `local`, `cancelled`, `deadline`, and `defect`. Preserve the structured cause and backend retryability fixed by ticket 0084. Prose belongs at the host edge. Invalid settings, invalid questions, malformed record or entity values after host framing, impossible profile limits, request-budget refusal, spent deadlines, pre-fired cancellation, and conflicting width fail before a send. A local refusal must not be inferred from an error alone; tests count every external-effect boundary.

### Error-index rows this ticket carries

`sdlc/issues/2026-09-23-surfaces-branch-error-index.md`, as sorted by the port guide: R2-21 (a refused connection is not retried), R5-19 (no leftover send after a cancelled batch), and on every facade path the 0089 rule behind R5-4 and R6-15. The G1 rows marked `*` in the port guide wait for ticket 0097. The record names one planted bug per carried row and shows its test turning red: R2-21 marks `Refused` as retried; R5-19 lets a worker dispatch one more queued item after the cancel; G2 resends after a close that follows the body.

## No copied machinery

Before the first code change, inventory the post-0084 call graph for all ten functions and name the owners of parsing, planning, scheduling, transport, cache, recording, counters, controls, and result assembly in the implementation record. Delete or fold any command-only adapter made obsolete by the facade. Do not paste code from `surfaces`, create a facade-specific question parser, copy either scheduler, wrap the HTTP client with a second transport, encode or digest a request twice, add a second cache, or serialize typed results only to parse them again.

Mechanical checks must keep the boundary from drifting. Extend the existing policy script to reject production HTTP construction outside the one gated transport owner and to reject direct CLI imports of the low-level scheduler, prepared-request, recorder, and transport modules. A small allowlist may name the facade module and existing engine-internal owners. Plant and refuse one representative violation of each new rule in the policy self-test.

## Which thread sends

Ticket 0078 masks host signals on engine worker threads only. A send on the host's calling thread can end as `Transport(Other)` when a host signal arrives (R6-3). Decision: every live attempt, including single judgments and `find`, sends on an engine worker thread. A single call spawns one scoped worker for its live attempt and joins it. A cache hit or replay spawns nothing. The calling thread does not send. The cost is one thread spawn per live single call, which is small against a network round trip. Ian can overturn this.

## Offline conformance and acceptance

- Observe focused red tests before implementation. Start with one scalar function, one ordinary bulk function, grouped `annotate`, aggregate `find`, `recognize`, and `relate`. Each must fail because no facade route exists, then pass through the same production owners the command uses.
- Run every case in `conformance/cases.json`, `backend-profiles.json`, and `record-values.json` through the private facade with no key and no network. Tickets 0080 and 0081 must first add their accepted cases to main. Use the existing production grammar, request encoder, decoder, digest, answer rules, ranking, find selector, recognition assembler, relation planner, and edge assembler. Add no second conformance parser or canonicalizer.
- For all ten functions, compare facade results with the command's existing contract at the typed seam: bare value, detailed metadata, logical request order, send counts, cached state, provider usage, error kind and cause, and record identity. Rendering and exit-code tests remain command tests. The facade tests do not snapshot terminal prose.
- Prove deterministic ordering under deliberately reversed worker completion for ordinary records, split requests, grouped annotation, recognition stages, and relation questions. Retries add sends but never reorder logical request identities. Equal logical requests keep separate result positions even when one stored exchange serves both.
- Prove partial results with an early success, a valid unsure or null answer, a failed logical answer, and a later undispatched item. Keep completed outcomes in input order, preserve the failed marker or cause, report the exact stopped position, and never turn absence after failure into a negative judgment. Repeat the ownership seam for `annotate`, `recognize`, and `relate` without duplicating their internal planner tests.
- Count loopback requests and filesystem effects for malformed typed input, impossible backend limits, request-budget refusal, conflicting width, pre-fired cancellation, and a zero or spent deadline. Each sends nothing. Local preflight creates no cache entry, recording entry, durable usage delta, or request count. A cache hit and replay also send nothing but retain their accepted cache and counter meanings.
- Exercise live-path behavior only against synchronized loopback listeners. Across mixed concurrent calls to all ten functions, the maximum in-flight attempts never exceeds ticket 0077's one process cap. Cancellation and deadlines stop new attempts at their established checkpoints. Started attempts finish under the existing rule. Retry attempts reacquire the shared permit and increment sends and counters exactly once each.
- A test pins the private state accessor as the only door to retained pool, width, counter, and coordinator state. Ticket 0096 guards that accessor.
- Prove that reading bare values, details, metadata, usage snapshots, cache status, and CLI status after a completed call causes no extra request. A counted listener must remain unchanged. The standalone details call, if retained by 0084, is tested separately as an ordinary cache or network judgment.
- Sending thread: a `cfg(test)` seam records the sending thread ID. A single judgment and `find` send on a thread other than the caller. The test first installs a do-nothing `SIGUSR1` handler through the existing `signal_hook::flag::register`, because the default action kills the test process. A `SIGUSR1` sent to the calling thread during a held single send leaves the call answering with exactly one send. Planted bug: send on the calling thread, and the thread-ID test turns red.
- Transport resend (0089, G2): on every facade path, a loopback close after the request body left produces zero resends and the backend kind. A refused connection fails at once and is not retryable (R2-21). After a cancelled batch, the next single call sends only its own request (R5-19). The R5-19 test holds one reply across the cancel, releases it, and counts sends after the release.
- Bulk forms (0095, G4): the private bulk row carries the yes probability that `details` returns for the same record. A one-question `annotate` over `choose`, `score`, and `tag` returns the values the scalar calls return on 0091's cases.
- Prove all engine-owned workers and feeders have joined when every facade call returns. An endless or long generated input remains bounded by effective width and the existing queue window. No test depends on sleep for ordering; channels, barriers, and held loopback replies establish each phase.
- Search facade values, `Debug` output, command output, errors, recordings, cache entries, durable count files, and fixtures for credentials and unredacted secret-bearing failure text on every new success and failure path. No key reaches the core, a digest, a cache or recording file, a counter, or a returned result.

## Scope and exclusions

Allowed: one private engine value and completion facade; typed settings, call options, outcomes, metadata, usage snapshots, and six-kind errors fixed by 0084; all ten private function entries; command conversion to and from the facade; consolidation or deletion of superseded command adapters; reuse of the real parser, planner, scheduler, transport, cache, recording, counter, cancellation, deadline, and width owners; one offline facade runner over the shared conformance files; focused unit, integration, loopback, policy, secrecy, ordering, partial-result, bounded-memory, and worker-lifetime tests; exact ratchet, ticket, queue, and record updates during implementation.

Excluded: any `pub` user API, the interrupt check (0097), fork recovery (0096), public `engine` module, semver promise, C ABI, language or database binding, Polars door, `surfaces` merge or edit, connector or stand-in import, API or HTTP service, command grammar or output change, new setting, new result field, new error kind, new parser, new scheduler, new splitter, new transport, async runtime, resident worker, cache format change, counter reset, signal handler, dependency, workflow, installer, publication, live call, paid call, or quality-policy change for recognition or relations.

This ticket may change at most sixteen production Rust files (the owner lifted this to 29 on 2026-09-24 after the code review, because only 8 files carry new logic) and add at most 1,200 nonblank production Rust lines. Focused tests may add at most 2,100 nonblank Rust lines. The combined change may add at most 3,300 nonblank Rust lines and may not raise any file above 500 nonblank lines. Delete superseded command plumbing before raising the exact ratchet. The implementation record must name production additions, deletions, net growth, every touched owner, and where duplication was removed or deliberately retained. Add no dependency.

## Dependencies and stop conditions

Implementation starts from main after 0080 and 0081 have landed the real ninth and tenth functions, 0082 and 0083 have closed the command and transform work that can add request paths, 0076, 0077, 0078, and 0089 have landed and passed their private-control proofs, 0091 has merged the branch conformance cases, 0092 has moved the loopback harness these tests build on, and 0084 and 0095 have frozen the Rust contract used here. Rebase first, rewrite the call-path inventory against that exact tree, and make the first red test prove the missing facade rather than an obsolete branch shape.

Ticket 0086 depends on this ticket and exposes the public Rust API without another engine path. C and the separately reviewed language and database integration follow 0086. The `surfaces` branch remains untouched until that integration ticket.

Stop and redesign if the post-0084 tree requires a public type decision beyond 0084 and 0095, a second parser or scheduler, a transport outside the shared gated boundary, collection of an unbounded input, a weaker partial-result contract, a changed request or recording byte under an unchanged input, a new dependency, a host signal handler in the facade, more than the stated file or line budget, or an edit to `surfaces`.

## Gates, review, and completion

Run focused facade and command tests continuously, then policy, exact ratchet, formatting, Clippy, documentation, and `git diff --check`. An independent design reviewer checks the post-0084 inventory, type boundary, all-ten coverage, no-copy proof, control composition, conformance plan, and budgets before implementation. An independent code reviewer checks every live path, ordering and partial results, no-send boundaries, cache and counter meanings, signal ownership, the sending thread, worker joining, secrecy, and deletion of parallel command routes.

The coordinator then runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially with the key and base-address variables unset. No gate opens a non-loopback socket. No live or paid call runs. Completion records the exact tested revision and confirms that the command and facade passed the same offline cases through the real engine.

## Complexity and routing

- Contract score: 3
- State and timing score: 4
- Reach score: 4
- Proof score: 4
- Cost of error score: 4
- Total: 19
- Minimum and final level: 4
- Reasons: this private seam reaches every function and every paid request path, composes cancellation, deadlines, width, cache, counters, replay, and partial completion, and becomes the only implementation path for the command and later public API. A wrong seam can double-send, reorder results, lose good partial answers, bypass a process limit, deadlock a forked host, or freeze copied behavior into every binding.
- Selected implementation: Claude builds (Opus subagent). A fresh Claude session reviews design and code. A research subagent may take the post-0084 call-path inventory, never the contract.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-engine.md`) asked to split the interrupt check out (now 0097), route to Claude, order 0092 first, and plant bugs per row. All applied. The re-review (`sdlc/records/2026-09-24-rereview-engine.md`) asked for a sharper R5-19 plant, the private bulk row in G4, and no fork owner before 0096; all applied. The sending-thread decision comes from the 0078 re-review (`sdlc/records/2026-09-24-rereview-near.md`). Confirmation accepted it.
- Code review: `sdlc/records/0085-code-review.md` returned findings. The fixes and the lifted file budget are in `sdlc/records/0085-build-real-engine-facade.md`.
