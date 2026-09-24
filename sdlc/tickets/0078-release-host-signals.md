---
flow: build
priority: 78
opens: crates/thinkthen/Cargo.toml crates/thinkthen/src/main.rs crates/thinkthen/src/cli crates/thinkthen/src/engine/recorder.rs crates/thinkthen/src/engine/workers.rs crates/thinkthen/tests/recording_durability.rs sdlc/scripts/policy.py sdlc/ratchet.json
---

# 0078: Release host signals

Status: draft, revised after design review; needs re-review. Owner: Claude.

This ticket was "0078a" in the 2026-09-24 spine review. It keeps the number 0078 and the host-signal half of the earlier draft. Fork recovery moved to ticket 0096, which lands after 0085, because the pools it rebuilds do not exist before the facade (`sdlc/records/2026-09-24-spine-review-controls.md`, 0078 finding 2).

## Outcome and authority

The engine library installs no process-wide signal handler, and a host signal never fails a call. The command keeps ticket 0061's exact Unix behavior under `RLIMIT_FSIZE`: a recording or cache write that reaches the file-size limit returns the fixed recording-storage diagnostic and exit 5 through normal cleanup. That `SIGXFSZ` policy moves to the command edge. An embedded Python, Ruby, R, C, database, or Rust host keeps the signal disposition it chose. ADR 0017 and the C plan forbid a signal handler in an embedded library. Ian can overturn whether the command claims `SIGXFSZ` on read-only paths.

## Current facts

The recorder calls `signal_hook::flag::register(SIGXFSZ, ...)` once from engine code before a write. That changes process-wide policy for the rest of the process. It suited the standalone command in 0061 and conflicts with the library and C plans. `signal-hook` is a normal dependency of the library. The command already blocks SIGINT on its worker threads with `thread_swap_mask`.

After 0089, an `EINTR` from a socket read becomes `Transport(Other)` and fails the call with no resend. A socket with a timeout returns `EINTR` even under `SA_RESTART` (signal(7)). A host signal delivered to an engine worker can therefore fail a call as a backend error (error-index row R6-3 in its post-0089 form).

## Design

- Remove `SIGXFSZ` registration and its `OnceLock` from the recorder and from every build without `cli`. Make `signal-hook` CLI-only unless another CLI-only path needs it. The engine reports ordinary write, sync, and install errors and never installs, replaces, ignores, chains, or restores a disposition.
- The CLI installs the same process-once `SIGXFSZ` handler before any path can create a recording or cache write permit, and before input, key lookup, or transport. Setup failure stays the fixed local recording failure and sends nothing. Version, help, status, cache inspection, prune, and replay-only work need not claim it.
- Engine-created worker threads block asynchronous host signals for their lifetime, the way the command already masks SIGINT. The calling thread keeps the host's mask, so the host still hears its own signals there.

## Acceptance

- R6-3, one pinned outcome: a test installs a no-op `SIGUSR1` handler with `signal_hook::flag::register` and delivers it with nix's safe `pthread_kill` to an engine worker whose thread ID a `cfg(test)` seam publishes, during a held send and during a retry wait. The call returns its normal answer, and the listener counts exactly one send.
- The existing command `RLIMIT_FSIZE` subprocess (the `ulimit -f 1` pattern at `recording_durability.rs:188`) still exits 5, prints the fixed line exactly, sends no extra request, and leaves no final or temporary entry. Signal setup failure still happens before key lookup and transport.
- A library build without `cli`, in a child process under a file-size limit, keeps a host-installed `SIGXFSZ` action before, during, and after recording. A returning host action lets the engine report the local storage failure and clean up. A separate child with the default action proves the engine does not replace it.
- `cargo tree -p thinkthen -e normal --no-default-features` has no `signal-hook`, unless the record names the CLI-only path that keeps it.
- Planted-bug proof: the record names one planted bug per carried row and shows its test turning red. For R6-3, remove the worker mask. For `SIGXFSZ`, register the handler from the recorder again.
- Existing 0074 SIGINT subprocesses, 0076 deadlines, and recording durability tests stay green. Focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check` pass. The coordinator runs `install`, `lint`, `test`, and `spec` in order. No proof opens a non-loopback socket or uses a paid service.

## Scope

About four production files and 150 nonblank production lines; at most 250 test lines. No new dependency.

Excluded: fork recovery (ticket 0096), public types, binding code, any signal other than the command's `SIGXFSZ` policy and the worker mask, `pthread_atfork`, and paid calls.

## Dependencies

After 0076. It may build beside 0077: their write sets meet only at `policy.py` and `ratchet.json`. Before 0084's `Engine` reconciliation.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Review

- Design review: `sdlc/records/2026-09-24-spine-review-controls.md` found the fork proofs unbuildable and the R6-3 test unable to fail. This rewrite answers both. Re-review pending.
- Code review: pending.
