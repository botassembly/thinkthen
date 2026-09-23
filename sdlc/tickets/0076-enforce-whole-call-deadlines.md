---
flow: build
priority: 76
opens: crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests/backend sdlc/ratchet.json sdlc/planning
---

# 0076: Enforce whole-call deadlines

Status: ready

## Outcome and authority

Every engine call carries at most one deadline instant. The instant is made once from the caller's budget and is shared by every direct request, record, chunk, annotation group, retry, and request-path wait in that call. A spent deadline returns the private `deadline` error before key lookup, request accounting, connection, or send. Remaining budget caps each retry wait and each blocking HTTP send. When the budget ends, the engine returns `deadline`, not a backend timeout.

ADR 0017 section 4 fixes this contract. The settled surface ledger adds that a zero or already-spent deadline is legal and sends nothing. The launch-first queue authorizes design now. Implementation waits through 0083 solely because the launch-first queue and the overlapping CLI writes require that order; 0080 and 0081 are the request-path prerequisites. Ticket 0077 later adds the process-width gate and makes its gate wait observe the same deadline; this ticket supplies the deadline state and does not add a width gate.

## Current facts and design

The private engine has one cooperative cancel token across ordinary scheduling, grouped `annotate`, prepared requests, retry waits, recording folder waits, and digest-lock waits. It has only an unused unit `Error::Deadline`. `Client` gives every HTTP attempt the full `--timeout`, and ticket 0064 caps each retry wait by that same per-attempt timeout. A call can therefore spend the full timeout on every attempt and every retry wait. No current command option supplies a whole-call budget.

Add one private optional deadline value that carries the original budget and the one `Instant` computed at call entry. Never rebuild it per record, group, chunk, or retry. The existing command passes no deadline and keeps `--timeout` as the per-attempt limit. Tickets 0084 through 0086 later expose the already-proved deadline through the public Rust and binding options without adding another timer or request path.

At every existing stop checkpoint, observe cancellation first and deadline second. Precedence follows observation at that checkpoint, never an unrecorded worker completion time. A local or backend result already received by the coordinator remains that result. A result still queued behind a deadline check does not outrank the deadline merely because its worker completed first. A deadline observed before an attempt starts returns `Error::Deadline` without invoking the attempt observer. During a retry wait, wait for the least of the selected backoff, the remaining deadline, and ticket 0064's attempt-timeout cap. During an HTTP attempt, set that attempt's blocking limit to the lesser of `--timeout` and the remaining deadline. If that limit expires and the call budget is spent, return `Error::Deadline`; an independent attempt timeout remains `Error::Transport(Timeout)`. Cancellation does not interrupt an attempt already sent, so cancellation raised during that send does not rewrite its completed response or its deadline-derived timeout. A retryable backend failure may retry only while budget remains.

Both schedulers observe the same deadline while waiting for input or worker results. Once spent, they request and dispatch no more work, clear undispatched annotation groups, preserve completed ordered results and existing stop metadata, and join every engine worker. Prepared-request recording folder, recorder mutex, and digest-lock waits observe it through the existing polling seams. Cleanup or recording finalization already in progress keeps the ticket 0073 precedence: an owned-resource cleanup failure remains local, and a paid answer already obtained is finalized rather than discarded. Ticket 0077 composes its not-yet-built gate wait with this control after 0076 lands.

`Error::Deadline` remains structurally distinct from every transport and status error and retains the original budget for the later public message, `the deadline of 5 s passed before the call answered`. It says nothing about backend health. A later public-API ticket decides whether any host exposes retry advice; this ticket makes no retryability claim. This ticket adds no command diagnostic or exit-code promise because the command exposes no deadline option.

## Scope and budgets

Allowed: one private deadline value; deadline-aware polling in both schedulers, prepared-request and recording waits, retries, and HTTP sends; unbounded deadline plumbing through every post-0083 request path; structural deadline error data; deterministic unit and loopback tests; exact ratchet; and later implementation updates to the ticket, ADR, queue, and record.

Before implementation, inventory the rebased tree and name every live request path added through 0083. Direct judgments, ordinary record scheduling, grouped `annotate`, aggregate `find`, split requests, `recognize`, `relate`, and retries must converge on the same deadline-aware prepared-request and transport boundaries. Production may touch at most twelve Rust files and add at most 500 nonblank Rust lines. Focused tests may add at most 750 nonblank Rust lines. Reuse the existing 50 ms poll, schedulers, prepared-request choke point, recorder waits, and blocking HTTP client.

Excluded: a command `--deadline` option, changes to `--timeout`, public Rust or binding APIs, a process-width gate or gate-wait implementation, fork recovery, host signal ownership, interrupting cancellation's already-started attempts, retry classification, request or recording bytes, result shapes, cache semantics, counter meanings, dependencies, async code, resident threads, workflows, surfaces implementation, site work, paid calls, and publication. Add no second scheduler, transport door, timer thread, or generalized runtime.

## Deterministic acceptance

- A pre-spent deadline on every direct post-0083 request path returns the deadline kind before key lookup, request accounting, connection, or request. Count each boundary. Cover direct judgments, aggregate `find`, ordinary records, grouped `annotate`, split requests, `recognize`, and `relate`; do not infer no-send behavior from an error alone.
- One call over several records, chunks, or annotation groups shares one deadline instant. Synchronized fixtures let an early item finish, hold later work, spend the budget, and prove no undispatched request starts. Completed ordered results and stopped-at metadata remain correct, annotation clears undispatched groups, and every worker has joined when the call returns.
- A loopback listener accepts one request and withholds its reply. The call returns `Error::Deadline` before the listener releases the socket, observes exactly one send, and leaves the request count frozen after return. A control with no deadline reaches the existing backend timeout kind. Channels establish the phases; a generous outer timeout only detects deadlock.
- A retryable 503 with a retry delay longer than the remaining budget returns the deadline kind during the wait and opens no second connection. A shorter delay still retries. A second case gives the retry enough time to start but less than the per-attempt timeout; the blocking send ends as deadline, not backend timeout. Attempt and usage counts include only attempts that started.
- Synchronized held-folder, recorder-mutex, and digest-lock cases return the deadline kind without waiting for the fixture owner and send nothing. Release the owner before filesystem assertions. Existing cancellation-first and cleanup-failure precedence tests remain green.
- Scheduler, request, and HTTP unit tests pin cancellation before deadline when both are observed at a pre-send checkpoint, deadline before a not-yet-started backend operation, and backend or local failures received before either stop. A barrier race queues a worker result before the coordinator's deadline check and proves the documented checkpoint order rather than guessing worker completion time. A sent attempt that reaches its deadline-derived socket timeout returns deadline even if cancellation became set while the attempt was in flight; a completed response remains its response. The deadline error carries the original budget and maps to `Kind::Deadline`, never `Kind::Backend`.
- A private seam proves an unbounded call preserves the existing attempt timeout, retry waits, request counts, output order, cache/replay behavior, and exact request and recording bytes. Ticket 0077's accepted design names the remaining proof: a future width-gate waiter must use this deadline and send nothing after it expires.
- Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass before code review. The coordinator then runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially. No test opens a non-loopback socket or uses a paid service.

## Dependencies and order

Tickets 0055, 0064, 0072, 0073, and 0074 provide the private engine boundary, bounded per-attempt retry waits, fast refusal, cooperative cancellation, and command SIGINT behavior that this deadline must preserve. ADR 0017 section 4 and the settled surface ledger fix the whole-call and spent-deadline rules.

`ready` is the repository's canonical status for an authorized ticket that has not started; it does not mean its dependencies have landed. Tickets 0080 and 0081 are the real request-path prerequisites: `recognize` and `relate` add live request paths, so their final paths must enter the inventory and acceptance matrix before code changes begin. Ticket 0079 supplies the splitter those paths consume. Tickets 0082 and 0083 add no request path and are not technical deadline prerequisites. Implementation nevertheless waits through 0083 solely because the launch-first queue and the overlapping CLI writes require that order. Ticket 0077 follows 0076 and adds deadline-aware waits at its new process-width gate. Ticket 0078 then composes fork recovery and host ownership before tickets 0084 through 0086 expose public APIs and packages.

## Complexity and routing

- Contract score: 1
- State and timing score: 2
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 7
- Minimum level floor: level 3 for concurrency and timing
- Final level: 3
- Reasons: level 3 applies because one instant crosses concurrent schedulers, durable request locks, retries, and blocking sends. Wrong ordering can send paid work after a caller's budget, misreport a deadline as backend illness, or leave a host waiting past its bound. The contract is explicit, remains private, adds no dependency, and defers the future width gate.
- Selected route: Luna Max (`gpt-5.6-luna`, `max`) owns design, case analysis, implementation, and remediation. Independent design and code reviews use separate Sol High (`gpt-5.6-sol`, `high`) sessions under the three-ticket trial.
- Required trial measurements: record the Luna reasoning level, design outcome, substantive Sol findings, review rounds, Luna remediation passes, any Sol repair, focused and complete gate results, reopened defects, and trustworthy elapsed start-to-accept time. Record usage and cost only when the tools expose trustworthy figures.

Re-score if the post-0083 inventory reveals another transport door, the work changes a public API or command surface, requires a dependency, or cannot preserve one deadline across all paths without a broader scheduler change.

## Review

- Design review: accepted by independent Sol High after routing and dependency attribution were corrected.
- Code review: pending.

## What Ian can overturn

Ian can overturn the cancellation-before-deadline precedence, the exact later public sentence, and the implementation budgets. ADR 0017's one-instant whole-call rule, spent-deadline no-send rule, and distinct deadline kind control unless he replaces them.
