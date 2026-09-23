# 0073: Cooperative engine cancellation

Status: Landed on main through `7ec6f82` after independent acceptance and two complete sequential local ladders, including the current-main combined tree. Main was pushed. GitHub Actions remains manually disabled, so no hosted success is claimed.

## Result

One private atomic token now reaches both schedulers and every command request path. A 50 ms poll stops new input and undispatched work, checks every initial attempt and retry before request accounting and transport, interrupts retry and request-path lock waits, preserves completed ordered results, and joins every engine worker. Attempts already started finish through decode, usage accounting, and recording finalization. Cancellation cleanup keeps the existing local-error precedence. The command creates an unfired token only; SIGINT registration, stopped output, signal restoration, and exit 130 remain the separate follow-up.

## Review and verification

Independent design review rejected absolute race-free send claims, detached-reader and cleanup overclaims, imprecise error precedence, ignoring worker events after cancellation, and incomplete token plumbing. The accepted design defines the attempt-start observation point, keeps the detached command reader, continues draining worker events through join, threads one token through whole ordinary and annotate runs, and includes retry, folder-gate, and digest-lock waits while excluding prune and signal behavior.

Independent code review required stronger lock-contention proof through the prepared-request path, direct input-wait tests for both schedulers, cleanup-failure precedence, restoration of existing comments, and an annotate request-count assertion. The same reviewer accepted the remediations. The first full test rung then exposed two older Linux tests tied to the former `locks_lock_inode_wait` kernel state. Cooperative polling cannot enter that state. Their test-only helper now identifies the original file by device and inode while retaining liveness, no-request, lock-count, replay/send, prune-order, and later-fill assertions. Both exact tests, all 296 backend tests, and re-review passed.

Final coordinator command on the reviewed branch: `sh sdlc/scripts/install && sh sdlc/scripts/lint && sh sdlc/scripts/test && sh sdlc/scripts/spec`. Exit 0: policy, package, audit, format, Clippy, docs, exact ratchet `36701/36701`, 633 Rust tests with one intentional ignore, doctests, all schema/probe/transform/replay checks, and nineteen green how-tos. `git diff --check` also passed.

The Rust ceiling rose by 727 net nonblank lines. Production adds the private token and bounded polling at existing scheduler, request, retry, recorder, and lock seams. Most growth is synchronized positive, negative, race, cleanup, and lifetime proof. The implementation reused both scheduler state machines, the shared prepared-request choke point, existing recorder cleanup, one polling helper, existing HTTP accounting, and existing worker scopes. It added no dependency, global state, second scheduler, second request path, or cache-prune mechanism. A separate reviewer examined and accepted the ceiling increase.

No signal handler, diagnostic, exit code, public Rust API, deadline, process-wide width, fork recovery, `SIGXFSZ` ownership, cache-prune behavior, dependency, workflow, surface, site, provider, or publication behavior changed.
