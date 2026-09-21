# 0039: Coalesce cache misses by digest

Ticket 0039 makes concurrent callers that share a cache send one backend request for an equal digest. The first caller that misses takes an operating-system file lock, checks the entry again, and owns sending, decoding, and recording. A waiter reads the installed entry as a replay. Record-only and replay-only behavior stays unchanged.

## Implementation

`cache_lock.rs` is the engine-ready filesystem boundary. It accepts a folder and digest and returns `io::Result<CacheLock>`. Its small generic coordinator owns the first read, lock, second read, and fill sequence. It has no command-line type, output, `Failure`, or exit code. The standard library owns locking, and no dependency entered. Empty private lock files remain under `.locks`; they hold no process identity or other data. Closing the file releases ownership after normal return, failure, or process death.

The request path reads once before locking and once after locking. It reads the key and sends only after the second miss. The guard lives through reply decoding and immutable entry installation. A process killed after the backend accepts a request but before installation can still cause a second send because it left no answer to replay. The ticket deliberately adds no pending state or recovery protocol.

ADR 0020 now carries the later cache decision without rewriting its immutable recording history. The recording and records specifications describe the lock, replay result, interruption window, and record-only exception. The main and prospective plans put `find` next. The language plans qualify their deduplication claims with a shared cache.

## Red then green

The rewritten two-process test first failed with `a keyless waiter finished before the owner installed the entry`. After the lock and second read landed, the keyed owner and keyless waiter both succeeded, exactly one loopback request arrived, and their detailed rows reported one live answer and one replay.

The focused matrix proves one request for sixteen equal rows at jobs 1, 4, and 32, one request through annotate's grouped path, and concurrent requests for different digests. A deterministic unit test uses positive channel events and the real file lock to prove that the second caller takes the production coordinator's lock, rereads, and never fills. The compiled keyless-waiter case would fail if the request path skipped locking. Its complete detailed result equals the owner's after normalizing only `meta.replayed`; a third keyless run replays the same value without another request. Backend, decode, recording, and killed-process cases release ownership and let a waiter proceed without recovery. A child enters an unwind-safe kill-and-wait guard immediately after spawn, before stdin handling can fail. Lock setup failure uses a valid cache with a blocked `.locks` path, occurs before key reading, and sends zero requests. Replay alone creates no lock directory. A pre-existing cache keeps its mode while the lock directory and files become private.

## Validation and size

The four repository rungs pass. The Rust test rung passes 410 tests with none failed or ignored, plus all three probability-probe self-tests. The spec rung passes its 26 and 2 page checks, all committed replay checks, and all 18 green how-tos. `git diff --check` passes.

The Rust ceiling rises from 21,709 to 22,336 nonblank lines. Product code accounts for 178 net lines: the neutral lock module and coordinator, recorder selection, and the request-path integration. The remaining 449 lines are unit and compiled process, failure, privacy, jobs, and annotate proof. Existing scheduler, listener, recording, and secrecy helpers were reused; no second cache or request implementation was added.

## Review

The design review rejected its first draft until the one-send promise was qualified, failure outcomes were exact, a keyless waiter was proved, and lock setup preceded the key. The revised ticket was accepted. Code review rejected the first implementation proof because the mode test did not preserve an existing cache mode, setup failed at the wrong path, waiting relied on a 50 ms inference, and assertion failure could strand a child. The first remediation added exact mode and setup cases, a positively synchronized test over the production coordinator, the compiled keyless-waiter proof, and an unwind-safe child reaper. The second review required full normalized value equality, a later keyless replay, and child ownership before any post-spawn failure. The second remediation adds all three. The same reviewer accepted the final tree with no remaining finding.
