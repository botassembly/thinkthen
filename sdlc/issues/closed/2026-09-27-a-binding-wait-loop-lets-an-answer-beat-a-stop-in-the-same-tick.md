# A binding's wait loop lets an answer beat a stop in the same tick

Status: closed by ticket 0168 on 2026-09-27. Python and Ruby now read their caller token before taking a waiting answer. The old order returned an answer beside a fired token in both binding proofs; the new order passed 20 held-reply runs on each binding. The race is a non-issue for R, DuckDB, SQLite and PostgreSQL, whose loops read only a host interrupt, with no caller token to contradict a returned answer. Found by reading while writing ticket 0166.

The held throttle place in local experiment 284 file 76 is real and separate from the four host-interrupt loops. Ticket 0168 states the wait it can impose on a later call in `specification/settings.md`. Sent attempts already enter usage counters, and the throttle bounds concurrent work. A cancellable socket read remains a deferred possible fix for the held place; this closure does not declare that cost a non-issue.

## Behavior before 0168

Python, Ruby, R, DuckDB, SQLite and PostgreSQL run each call on a worker thread. The calling thread waits for the worker's answer in ticks of 50 or 100 ms. At each tick it reads its stop: the caller's token, a pending signal, or the host's interrupt flag. On a stop it fires its own token for the worker and returns cancellation at once.

Each loop reads the answer before it reads the stop. When the stop and the answer land in the same tick, the loop returns the answer.

- `libraries/python/src/worker.rs`, `wait`: `recv_timeout` returns the value before the caller's token and `check_signals` are read.
- `libraries/ruby/src/ffi.rs`, `cross`: `Taken::Ready` returns before the tokens are read.
- `libraries/r/thinkthen/src/rust/src/calls.rs`, `wait`: an answer returns before `pending()` is read.
- `databases/duckdb/src/worker.rs`, `wait`: `stopped()` is read, then `next()` blocks up to one tick and returns the answer.
- `databases/sqlite/src/worker.rs`, `wait`: `interrupted()` is read only on a timeout.
- `databases/postgresql/src/call.rs`, `wait`: `poll()` runs only on a timeout.

The window is one tick wide. A held reply cannot cause it, because the loop reads the stop at every tick while the reply is held. In Python and Ruby the caller can see it: the call returns an answer while the caller's token reads as cancelled. In R and the three databases a pending interrupt still raises at the host's next check.

TypeScript does not have it. Its `AbortSignal` handler rejects the promise in the event loop and detaches the worker.

## Suggested fix

Before a loop returns an answer, read the stop once more, and return cancellation when it has fired. A test for each binding fires the stop and releases the held reply in one step, then requires cancellation.
