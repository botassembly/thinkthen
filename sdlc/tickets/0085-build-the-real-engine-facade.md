---
flow: build
priority: 85
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance sdlc/ratchet.json sdlc/planning
---

# 0085: Build the real engine facade

Status: draft, design not reviewed. Owner: Claude. Was marked ready; implementation waits for tickets 0080, 0081, 0082, 0083, 0076, 0077, 0078, 0089, 0091, 0084, and 0095 to land

## Outcome and authority

Build one private, blocking, host-neutral, typed facade over the production engine for all ten functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `annotate`, `find`, `recognize`, and `relate`. Route the command through that facade. Ticket 0086 exposes the later public Rust API over the same facade. This ticket does not publish an API.

ADR 0017 fixes one crate, one blocking engine, six error kinds, shared call controls, one process width cap, fork recovery, cache and counter ownership, and one shape across hosts. Tickets 0084 and 0095 freeze the Rust contract names and typed shapes this ticket implements privately. Tickets 0080 and 0081 fix the ninth and tenth functions and their shared relation planner. Tickets 0076 through 0078 fix whole-call deadlines, process width, fork recovery, and host signal ownership before this work opens another caller. The current engine, core, command, and `conformance/` files are production authority.

The `surfaces` branch is read-only shape evidence. Its contract crate, stand-in connector, surface methods, and tests show conversions and host expectations. They are not production code, do not prove real-engine behavior, and must not be copied into main. Where that branch conflicts with ADR 0017, ticket 0084, the accepted 0080/0081 tickets, or current conformance, main's accepted records control.

Ian can overturn the facade boundary, the private type grouping, and the budgets below. The one-engine ownership, blocking execution, ten-function list, control contracts, and no-copy rule remain controlled by ADR 0017 and their owning tickets unless he replaces those decisions.

## Boundary

Add one private completion layer inside `engine`. It accepts the typed questions, records, entities, relation rules, immutable engine settings, and per-call options fixed by ticket 0084. It returns host-neutral typed values, ordered bulk outcomes, result metadata, usage snapshots, and the six structured failure kinds. No facade signature accepts command arguments, standard streams, exit codes, rendered JSON rows, host objects, dynamic JSON, or a language binding type.

The facade owns orchestration only. It delegates each concern to its existing owner:

- The core parses and validates question files and typed values, constructs plans, interprets answers, applies thresholds, ranks and finds, assembles recognition, and plans and assembles relations.
- The private engine prepares one canonical request identity, splits under backend limits, schedules bounded work, sends and retries, records and replays, caches, counts, cancels, applies deadlines, enforces process width, and repairs state after fork.
- The command keeps argument and environment lookup, credential lookup, standard-input framing, CSV and TSV decoding, detached input reading, terminal rendering, diagnostics, signals, and exit codes.

The command converts its resolved inputs into facade values and converts facade outcomes into its existing output. It must not retain a parallel direct route into a scheduler, prepared request, recorder, HTTP client, recognition pipeline, or relation planner. Ticket 0086 later wraps these same private calls and types. It must not reconstruct a request path beside them.

Keep the low-level engine modules private. Add no C symbol, connector trait, dynamic dispatch requirement, plugin door, host callback ABI, or public re-export. A narrow internal trait is allowed only as a deterministic test seam over transport or fault injection that the current engine does not already provide. It must not become a second engine interface.

## Facade contract

One immutable engine value holds resolved engine settings. Construction validates all local settings before external effects and preserves explicit versus omitted width for ticket 0077. It holds no resident worker thread. Every call enters ticket 0078's current-process guard before touching a pool, width waiter, cache coordinator, recorder, counter, scheduler queue, or request-path lock inherited across a fork.

Every function has one typed call path. Convenience within the command may reduce to shared helpers, but one function never re-parses another function's rendered output. Scalar and bulk forms use the same typed judgment path. `filter` and `rank` retain record identity through input indexes or typed borrowed records. `find` returns its selected input and all candidate probabilities. `annotate` preserves named-question order and answered-or-failed members. `recognize` and `relate` consume the 0080 planner and result owners without another tokenizer, splitter, planner, threshold rule, or edge assembler.

Every successful logical result carries the production metadata its contract requires: model, question identity, logical request digests in construction order, successful send count, cache status, provider usage when present, and the probabilities needed to derive its bare value. Reading a bare value, details, a record mapping, or already-returned metadata from that immutable result sends nothing. The existing standalone `details(question, evidence)` meaning remains a new judgment call if ticket 0084 retains it.

Bulk calls return completed outcomes in input order, independent of worker completion order. A valid unresolved answer remains distinct from a failed judgment. A logical failure preserves already completed good outcomes and the exact failure marker or structured cause required by conformance. Cancellation, deadline, local failure, or backend failure preserves only results the owning scheduler considers complete, carries existing stopped-at metadata, dispatches no later work after the stop checkpoint, and joins every engine worker before returning. The facade does not collect an unbounded input merely to simplify a host result.

Each call accepts the shared cancel token, an optional one-instant deadline, and ticket 0095's optional interrupt check. The check runs only on the calling thread: once before the first send and at every existing 50 ms poll while the call waits on the width gate, a retry wait, a recording or lock wait, or bulk results. A `true` return fires the call's cancel token at that moment. It never runs on a worker or during one blocking send. A panic in the check joins every engine worker, then resumes unchanged. It does not become command signal policy; the command passes no check. Cancellation and deadline precedence, gate waits, retry waits, sent-attempt completion, and worker joining remain exactly as tickets 0073, 0076, and 0077 fix them.

The facade uses the one process width state and current-process state from tickets 0077 and 0078. An omitted width stays omitted. A conflicting explicit width fails locally before input dispatch, key lookup, cache or recording mutation, request accounting, connection, or send. A child does not retain a parent pool, gate, queue, process counters, or userspace cache lock. The facade installs no process signal handler. The command alone retains its SIGINT and `SIGXFSZ` behavior.

Cache, replay, recording, and request coalescing keep their production meanings. A stored answer reports zero sends and increments the cache-answer counter under the existing rule. A live retry counts every started attempt. Process usage snapshots remain cumulative and reset in a forked child as ticket 0078 specifies; durable command totals remain the command's count-only files. The facade does not add a counter reset. The CLI `status` and cache commands continue to read the same cache and counter owners rather than a facade-side mirror.

The six facade failure kinds are `usage`, `backend`, `local`, `cancelled`, `deadline`, and `defect`. Preserve the structured cause and backend retryability fixed by ticket 0084. Prose belongs at the host edge. Invalid settings, invalid questions, malformed record or entity values after host framing, impossible profile limits, request-budget refusal, spent deadlines, pre-fired cancellation, and conflicting width fail before a send. A local refusal must not be inferred from an error alone; tests count every external-effect boundary.

### Error-index rows this ticket carries

`sdlc/issues/2026-09-23-surfaces-branch-error-index.md`, as sorted by the port guide: R2-21 (a refused connection is not retried), R5-19 (no leftover send after a cancelled batch), and on every facade path the 0089 rule behind R5-4 and R6-15. The G1 rows marked `*` in the port guide gain the interrupt check here; their surface tickets re-run them.

## No copied machinery

Before the first code change, inventory the post-0084 call graph for all ten functions and name the owners of parsing, planning, scheduling, transport, cache, recording, counters, controls, and result assembly in the implementation record. Delete or fold any command-only adapter made obsolete by the facade. Do not paste code from `surfaces`, create a facade-specific question parser, copy either scheduler, wrap the HTTP client with a second transport, encode or digest a request twice, add a second cache, or serialize typed results only to parse them again.

Mechanical checks must keep the boundary from drifting. Extend the existing policy script to reject production HTTP construction outside the one gated transport owner and to reject direct CLI imports of the low-level scheduler, prepared-request, recorder, and transport modules. A small allowlist may name the facade module and existing engine-internal owners. Plant and refuse one representative violation of each new rule in the policy self-test.

## Offline conformance and acceptance

- Observe focused red tests before implementation. Start with one scalar function, one ordinary bulk function, grouped `annotate`, aggregate `find`, `recognize`, and `relate`. Each must fail because no facade route exists, then pass through the same production owners the command uses.
- Run every case in `conformance/cases.json`, `backend-profiles.json`, and `record-values.json` through the private facade with no key and no network. Tickets 0080 and 0081 must first add their accepted cases to main. Use the existing production grammar, request encoder, decoder, digest, answer rules, ranking, find selector, recognition assembler, relation planner, and edge assembler. Add no second conformance parser or canonicalizer.
- For all ten functions, compare facade results with the command's existing contract at the typed seam: bare value, detailed metadata, logical request order, send counts, cached state, provider usage, error kind and cause, and record identity. Rendering and exit-code tests remain command tests. The facade tests do not snapshot terminal prose.
- Prove deterministic ordering under deliberately reversed worker completion for ordinary records, split requests, grouped annotation, recognition stages, and relation questions. Retries add sends but never reorder logical request identities. Equal logical requests keep separate result positions even when one stored exchange serves both.
- Prove partial results with an early success, a valid unsure or null answer, a failed logical answer, and a later undispatched item. Keep completed outcomes in input order, preserve the failed marker or cause, report the exact stopped position, and never turn absence after failure into a negative judgment. Repeat the ownership seam for `annotate`, `recognize`, and `relate` without duplicating their internal planner tests.
- Count loopback requests and filesystem effects for malformed typed input, impossible backend limits, request-budget refusal, conflicting width, pre-fired cancellation, and a zero or spent deadline. Each sends nothing. Local preflight creates no cache entry, recording entry, durable usage delta, or request count. A cache hit and replay also send nothing but retain their accepted cache and counter meanings.
- Exercise live-path behavior only against synchronized loopback listeners. Across mixed concurrent calls to all ten functions, the maximum in-flight attempts never exceeds ticket 0077's one process cap. Cancellation and deadlines stop new attempts at their established checkpoints. Started attempts finish under the existing rule. Retry attempts reacquire the shared permit and increment sends and counters exactly once each.
- Fork from warm and busy synchronized states through the facade. The child enters the PID guard before inherited mutable state, builds fresh process state, keeps the parent unchanged, and completes against its own loopback listener. Include cache-hit, counter, held-width, queued-worker, recorder wait, and digest-lock cases already owned by ticket 0078. This ticket proves that every facade entry uses that owner; it does not recreate the fork suite.
- Prove that reading bare values, details, metadata, usage snapshots, cache status, and CLI status after a completed call causes no extra request. A counted listener must remain unchanged. The standalone details call, if retained by 0084, is tested separately as an ordinary cache or network judgment.
- Interrupt check (0095): a counted loopback listener and a recorded thread id prove the check runs only on the calling thread, before the first send and at each poll during a held width gate, a retry wait, and a held bulk reply. After it returns `true`, the listener sees no new request, sent attempts finish, the call returns `Cancelled` with stop metadata, and every worker has joined. A check that panics resumes the same panic after the join. No check runs during one held single send.
- Transport resend (0089, G2): on every facade path, a loopback close after the request body left produces zero resends and the backend kind. A refused connection fails at once and is not retryable (R2-21). After a cancelled batch, the next single call sends only its own request (R5-19).
- Bulk forms (0095, G4): `Row::probability` equals the yes probability `details` returns for the same record. A one-question `annotate` over `choose`, `score`, and `tag` returns the values the scalar calls return on 0091's cases.
- Prove all engine-owned workers and feeders have joined when every facade call returns. An endless or long generated input remains bounded by effective width and the existing queue window. No test depends on sleep for ordering; channels, barriers, and held loopback replies establish each phase.
- Search facade values, `Debug` output, command output, errors, recordings, cache entries, durable count files, and fixtures for credentials and unredacted secret-bearing failure text on every new success and failure path. No key reaches the core, a digest, a cache or recording file, a counter, or a returned result.

## Scope and exclusions

Allowed: one private engine value and completion facade; typed settings, call options, outcomes, metadata, usage snapshots, and six-kind errors fixed by 0084; all ten private function entries; command conversion to and from the facade; consolidation or deletion of superseded command adapters; reuse of the real parser, planner, scheduler, transport, cache, recording, counter, cancellation, deadline, width, and fork owners; one offline facade runner over the shared conformance files; focused unit, integration, loopback, policy, secrecy, ordering, partial-result, bounded-memory, and worker-lifetime tests; exact ratchet, ticket, queue, and record updates during implementation.

Excluded: any `pub` user API (0086 exposes 0095's interrupt check), public `engine` module, semver promise, C ABI, language or database binding, Polars door, `surfaces` merge or edit, connector or stand-in import, API or HTTP service, command grammar or output change, new setting, new result field, new error kind, new parser, new scheduler, new splitter, new transport, async runtime, resident worker, cache format change, counter reset, signal handler, dependency, workflow, installer, publication, live call, paid call, or quality-policy change for recognition or relations.

This ticket may change at most sixteen production Rust files and add at most 1,200 nonblank production Rust lines. Focused tests may add at most 2,100 nonblank Rust lines. The combined change may add at most 3,300 nonblank Rust lines and may not raise any file above 500 nonblank lines. Delete superseded command plumbing before raising the exact ratchet. The implementation record must name production additions, deletions, net growth, every touched owner, and where duplication was removed or deliberately retained. Add no dependency.

## Dependencies and stop conditions

Implementation starts from main after 0080 and 0081 have landed the real ninth and tenth functions, 0082 and 0083 have closed the command and transform work that can add request paths, 0076 through 0078 and 0089 have landed and passed their private-control proofs, 0091 has merged the branch conformance cases, and 0084 and 0095 have frozen the Rust contract used here. Rebase first, rewrite the call-path inventory against that exact tree, and make the first red test prove the missing facade rather than an obsolete branch shape.

Ticket 0086 depends on this ticket and exposes the public Rust API without another engine path. C and the separately reviewed language and database integration follow 0086. The `surfaces` branch remains untouched until that integration ticket.

Stop and redesign if the post-0084 tree requires a public type decision beyond 0084 and 0095, a second parser or scheduler, a transport outside the shared gated boundary, collection of an unbounded input, a weaker partial-result contract, a changed request or recording byte under an unchanged input, a new dependency, a host signal handler in the facade, more than the stated file or line budget, or an edit to `surfaces`.

## Gates, review, and completion

Run focused facade and command tests continuously, then policy, exact ratchet, formatting, Clippy, documentation, and `git diff --check`. An independent design reviewer checks the post-0084 inventory, type boundary, all-ten coverage, no-copy proof, control composition, conformance plan, and budgets before implementation. An independent code reviewer checks every live path, ordering and partial results, no-send boundaries, cache and counter meanings, fork and signal ownership, worker joining, secrecy, and deletion of parallel command routes.

The coordinator then runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially with the key and base-address variables unset. No gate opens a non-loopback socket. No live or paid call runs. Completion records the exact tested revision and confirms that the command and facade passed the same offline cases through the real engine.

## Complexity and routing

- Contract score: 3
- State and timing score: 4
- Reach score: 4
- Proof score: 4
- Cost of error score: 4
- Total: 19
- Minimum and final level: 4
- Reasons: this private seam reaches every function and every paid request path, composes cancellation, deadlines, width, fork recovery, cache, counters, replay, and partial completion, and becomes the only implementation path for the command and later public API. A wrong seam can double-send, reorder results, lose good partial answers, bypass a process limit, deadlock a forked host, or freeze copied behavior into every binding.
- Selected implementation: `sol-implementer` (`gpt-5.6-sol`, medium reasoning). Use separate `sol-reviewer` sessions for design and code review. Use a research agent only for the post-0084 call-path inventory, never to decide the contract.

## Review

- Design review: pending.
- Code review: pending.
