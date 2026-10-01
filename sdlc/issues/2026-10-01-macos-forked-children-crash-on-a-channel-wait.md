Status: open. Found on the M5 on 2026-10-01 by the quick fix that closed `closed/2026-09-30-duckdb-check-fails-four-tests-on-macos.md`. Owner: the queue owner.

Kind: bug

Severity: medium

# On macOS, a forked child can crash on a channel wait

## The problem

On macOS, Rust's thread parking waits on a libdispatch semaphore. The semaphore's Mach port does not survive `fork()`. When a thread has parked once in the parent, its first blocking park in the child aborts the child with SIGTRAP. The crash report reads "BUG IN CLIENT OF LIBDISPATCH: Use-after-free of dispatch_semaphore_t" and "crashed on child side of fork pre-exec". A park that never blocks passes, so the crash comes and goes. Linux parks on a futex, which survives fork.

`std::sync::mpsc` waits, `thread::scope` ends and `thread::park` all park the calling thread. The forking thread is the one thread a child inherits, and a host usually forks from the thread that called ThinkThen.

The DuckDB case `a_forked_child_answers_from_a_zero_total` proved it. At main `d3391ee40`, the child crashed in 7 of 10 runs, inside `park_timeout` under the DuckDB bridge's `run_detached`. The quick fix replaced that one wait with a mutex and condition variable, which use pthread calls that need no Mach port. The case then passed 19 of 20 runs; the one failure came in the parent before the fork (Debt 020's connection race).

## Where else it may happen

These waits run on the calling thread through a channel. None is proven to crash yet; each needs a host that forks after a call and then calls again in the child on macOS.

- The engine: `crates/thinkthen/src/engine/workers.rs` `on_worker` and `scoped`, `engine/pipeline/run.rs` `receive`, and `public/pull.rs`. The C door and the native hosts over it call these on the caller's thread.
- Python: `libraries/python/src/worker.rs` and `src/stream.rs`.
- R: `libraries/r/thinkthen/src/rust/src/calls/worker.rs`.
- SQLite: `databases/sqlite/src/worker.rs`.
- PostgreSQL: `databases/postgresql/src/call.rs`. A backend process is a fork of the postmaster.

## A fix

Give the crate one small wait for the calling thread built on `Mutex` and `Condvar`, with the timeout and disconnect results the callers use now. Use it on each path above, and end scoped workers by joining each one before the scope closes. Prove each surface with a real fork on macOS, as the DuckDB case does.
