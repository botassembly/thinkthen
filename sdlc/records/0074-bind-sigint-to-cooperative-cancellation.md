# 0074: Bind SIGINT to cooperative cancellation

Status: Accepted on the current-main integrated tree. Landing and remote-main verification remain.

## Result

The command now owns one process-lifetime SIGINT handler and one shared cancellation token. On Unix, the caller and request workers block SIGINT while one short-lived carrier thread receives it. The first signal stops later attempts that observe cancellation, lets sent attempts finish, flushes completed output and the stopped-at line, then restores normal SIGINT termination. A later signal after arming terminates immediately. A typed cancellation emits no defect diagnostic.

The carrier uses the optional Unix-only `nix` signal feature because Rust's standard library and `signal-hook` expose no safe per-thread signal-mask API. Failed registration, mask changes, carrier startup, readiness, restoration, and default emulation keep the ticket's exact local precedence. A hidden exclusive one-byte acknowledgment exists only for deterministic subprocess tests.

## Review and verification

Independent design review shaped the process-lifetime four-action handler, activation and finalization order, Unix signal affinity, dependency gating, and deterministic acknowledgment. Independent code review rejected stale cancellation before routing, incorrect source ceilings, missing routing-failure proof, incomplete cleanup ordering, fake sticky-registration proof, and later all-target Clippy failures. Remediation reset and bound the token before routing, injected each routing failure, observed arm-before-cleanup and release-after-restore, delivered SIGINT to every retained registration prefix in child processes, consolidated tests, and made integration helpers return errors. The same reviewer accepted the final diff.

Focused interrupt, backend, help, cancellation, policy, formatting, all-target Clippy, exact ratchet, and whitespace checks passed. The integrated current-main ladder ran `install`, then offline `lint`, `test`, and `spec` sequentially at `88885e7`; all exited 0. Rust reported 644 passed tests and one intentional child-harness ignore, plus two passing doctests. Replay checks and all nineteen executable how-tos passed. No paid or external call ran. GitHub Actions remains disabled by ruling.

Production changed five Rust files and added 350 net nonblank lines. The whole ticket added 899 net nonblank Rust lines. Every Rust file remains at or below 500 nonblank lines, and the exact crate ceiling is `37600/37600`. The implementation searched existing cancellation, scheduler, request, failure, and backend-test paths first and added no second scheduler or transport path.

Deadlines, process-wide width, fork recovery, recorder signal ownership, recognize, relate, public Rust functions, surfaces, release publication, and installation artifacts remain separate authorized tickets.
