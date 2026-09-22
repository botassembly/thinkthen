# 0061: Repair recording and cache durability

Date: 2026-09-22

Status: landed

## Result

A recording writer now prepares private storage before it reads a key or sends a request. It writes and syncs a complete temporary entry, closes it, then installs it atomically. Record and cache modes repair damaged entries after one successful answer. Replay alone remains read-only and refuses damage locally. Valid entries remain immutable.

Missing and damaged entries share one digest lock. A completed owner removes the lock only after proving a valid final entry exists. Existing waiters stay on the original inode and recheck the installed answer. Record-only callers against a valid entry remain independent and each send once. Storage failures use one fixed path-free diagnostic.

On Unix, `signal-hook` 0.3.18 installs one safe process-wide `SIGXFSZ` handler before writer preflight. A file-size limit now reaches the structured exit-5 cleanup path instead of terminating the process. The standard library has no safe signal-installation interface, and the workspace forbids unsafe code. The dependency is pinned in policy and documented in the Rust standards.

The Rust ceiling rose from 28,528 to 29,242 nonblank lines. The prepared recording lifecycle, safe signal handling, storage-fault injection, real process concurrency proof, exact byte-preservation checks, command secrecy coverage, and necessary module splits account for the increase.

## Review and proof

Independent design review rejected two drafts for an unsafe lock-unlink rule, contradictory mode and cleanup promises, incomplete valid-entry handling, missing signal policy scope, incomplete secrecy and concurrency proof, and an invalid complexity score. The final design locks only missing or damaged entries, fixes one valid-final invariant, states realistic cleanup, and requires a real original-inode waiter handshake. The same reviewer accepted it.

Independent code review rejected the first implementation because a final-read storage error could be swallowed, the process test did not prove the waiter opened the original inode, and the injected fault table did not preserve a valid old entry. The repair propagates the read error, observes the waiter descriptor inode and kernel lock-wait state, and snapshots exact old bytes for every applicable fault and conflict. The same reviewer accepted it.

The final install, lint, test, and specification rungs passed with the key and base-address variables unset. The test rung passed 188 library tests, 239 backend tests, and every command, demo, transform, and local live-script suite. The specification rung passed 27 page checks, seven transform checks, every committed replay, and all 19 green demos. Dependency, license, package, file-size, secrecy, exact 29,242-line ratchet, and `git diff --check` checks passed. No outside network or paid call ran.
