---
flow: build
priority: 73
opens: crates/thinkthen/src/engine crates/thinkthen/src/cli/edge.rs crates/thinkthen/src/cli/asking.rs crates/thinkthen/src/cli/asking/request.rs crates/thinkthen/src/cli/annotate.rs crates/thinkthen/src/cli/schedule.rs crates/thinkthen/src/cli/annotate_schedule.rs sdlc/ratchet.json sdlc/planning
---

# 0073: Stop engine work cooperatively

Status: independently accepted and locally verified; landing pending

## Outcome and authority

A private caller-supplied cancel token stops both engine schedulers from reading or dispatching more work. Each initial attempt and retry observes the token at one attempt-start check immediately before request accounting and transport; no attempt starts when that check observes cancellation. Attempts already past that check finish within their existing attempt timeout. Completed results remain ordered, all engine-owned workers join, and the engine returns the existing typed cancelled cause with stop metadata. Ian authorized bounded private-engine tickets in the amended queue and ruled the cancellation behavior in ADR 0017 and the approved SIGINT issue.

## Current fact and design

Both schedulers block on `Receiver::recv`, workers drain already-buffered jobs, retry waits sleep without observing cancellation, and recording preparation can wait indefinitely on a folder or digest lock. `Error::Cancelled` exists but no production path can produce it.

Add one private cloneable atomic cancel token and one 50 ms polling interval. Thread an unfired token through the command edge without registering a signal. Both schedulers poll it, stop requesting input, clear undispatched annotate work, preserve completed ordered output, and wait for in-flight work to finish. The shared prepared-request path checks the token after recording preparation and before key acquisition. The HTTP loop defines the attempt-start linearization point: it checks the token immediately before request accounting and transport for every initial attempt and retry. Retry waits poll the token. Folder mutex acquisition, shared folder locking, and digest locking poll the same token so cancellation cannot leave the scoped worker join waiting on an unstarted request. Existing cleanup handles resources owned by a cancelled operation; cleanup failures remain reported as local failures.

An attempt past its attempt-start check ignores later cancellation until transport returns. A successful attempt proceeds through decode, usage accounting, and uncancelled recording finalization; decode or finalization failures keep their actual cause. A non-retryable failed attempt keeps its backend cause. A retryable failed attempt becomes cancelled only when the token is observed before or during the wait for its next attempt, and its sent-attempt count remains charged. If abandoning a prepared write fails, that local cleanup failure wins. Scheduler state already accepted before cancellation keeps its ordered result or failure. Once a scheduler observes cancellation, it ignores only later input events, stops undispatched work, and continues accepting every worker result until its in-flight count reaches zero. The earliest ordered cancelled worker result represents the stop. The scheduler synthesizes cancellation at the first position that cannot complete only when no dispatched result can represent that position. Recording finalization remains uncancelled so a paid answer is not discarded.

## Scope and exclusions

Allowed: a private engine cancel type; cancellation polling in ordinary and annotate schedulers, request preparation, HTTP retry waits, recorder folder gates, and request-path cache locks; inert command-edge plumbing; deterministic tests; exact ratchet; ticket, issue, queue, and record updates. One token is created in `Environment` and passed from the ordinary and annotate command call sites to both their scheduler and all request calls. The ordinary single-document and aggregate `find` paths receive the same environment token through the shared prepared-request path. Production changes may touch at most fourteen Rust files and add at most 900 nonblank Rust lines including tests. Reuse one token and polling helper; do not add global state or a generalized cancellation framework.

Excluded: SIGINT registration, default-signal emulation, exit 130, new diagnostics or exit codes, public Rust API changes, deadlines, process-wide width, fork recovery, `SIGXFSZ` ownership, interrupting an already-started HTTP attempt, cancelling recording finalization, cache-prune locking or behavior, retry/cache/recording semantic changes, dependencies, workflows, surfaces, site, paid calls, and publication. The command-owned input reader retains ADR 0017's accepted detached lifetime.

## Acceptance

- A pre-fired token causes ordinary scheduling to return cancelled stop metadata at record 1 without requesting input, dispatching work, reading a key, or sending a request.
- Synchronized tests prove cancellation while either scheduler waits for input returns without another input request. Cancellation with work in flight starts no later queued work, lets already-started work finish, emits completed results in order, reports the first unfinished record, and returns only after every engine-owned worker joins. A generous outer timeout guards against deadlock but elapsed wall time is not the assertion.
- Annotate cancellation clears undispatched groups and does not read another row. A non-stream aggregate returns the typed cancelled cause rather than stream stop metadata.
- A pre-fired token at the prepared-request boundary abandons prepared recording resources through existing cleanup and calls neither key acquisition nor send. Single-document and `find` callers use this boundary without separate cancellation logic. A cleanup failure remains a local failure.
- Cancellation during a retry wait performs no second attempt and returns `Error::Cancelled`; request accounting still counts the attempt that was sent. A successful, non-retryable, decode, or finalization result already obtained keeps its actual outcome.
- Synchronized held-folder and held-digest-lock tests prove a cancelled waiter returns without waiting for the fixture owner and sends nothing. The fixture owner is then released before filesystem assertions. Existing durability, prune, and same-digest convergence tests remain green.
- Existing failure tests prove state already accepted by a scheduler is not replaced by a later cancellation. No engine-owned worker remains after return. Cleanup either removes the cancelled operation's partial resource or returns its existing local failure; the ticket makes no claim about another process's lock file or the accepted detached command reader.
- Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass before code review. The coordinator runs `sdlc/scripts/install`, `lint`, `test`, and `spec` sequentially after acceptance. Actions remains disabled.

## Dependencies

Tickets 0055, 0061, 0064, 0065, and 0072 are landed. ADR 0017 fixes engine ownership, polling, and finish-sent-request behavior. The approved SIGINT issue fixes the later command behavior. No public surface or signal handler is needed for this ticket.

## Complexity

Contract 2; State/timing 4; Reach 3; Proof 4; Cost of error 4; Total 17. Minimum floor: level 3 for cancellation, concurrency, and timing. Final level: 3. Reasons: one token crosses two schedulers, worker queues, retry timing, and durable lock waits; a mistake can send paid work after cancellation, deadlock a host, or discard a completed answer. The ticket avoids signal ownership, public APIs, deadlines, width, and fork recovery, so it remains an independently provable level-3 slice. Selected implementation: `sol-implementer` (`gpt-5.6-sol`, medium). SWE-2 prepares research and may remediate bounded findings. Independent design and code review use separate `sol-reviewer` sessions.

## Review

Independent design review rejected the first draft's absolute send-prevention, detached-reader, cleanup, timing, and error-precedence claims. The first re-review then rejected ignoring worker events after cancellation and incomplete token plumbing. The accepted revision defines the attempt-start observation point, preserves the detached command reader, fixes a 50 ms interval without wall-clock assertions, scopes cleanup claims, preserves completed-attempt outcomes, continues accepting worker results until join, passes one environment token through both whole runs, includes request-path lock waits, excludes prune, and adds a fourteen-production-file/900-added-line implementation budget. The same reviewer accepted the corrected design and confirmed the level-3 route.

Independent code review required synchronized lock-contention proof through the prepared-request path, direct scheduler input-wait tests, cleanup-failure precedence, restoration of existing comments, and the annotate request-count assertion. The same reviewer accepted each remediation and the final Linux device-and-inode test correction. The coordinator's final sequential local ladder passed 633 Rust tests with one intentional ignore, doctests, all schema/probe/transform/replay checks, and nineteen how-tos at exact ratchet 36,701. The first full test rung failed two older Linux tests because they required the former kernel lock wait channel; both exact tests and all 296 backend tests passed after their test-only probe was updated for cooperative polling.
