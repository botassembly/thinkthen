# The library tier needs one engine per process, and the sugar must not rebuild it

Status: Open

A library caller who makes a thousand judgments should pay for DNS, TCP, and TLS once, not a thousand times. The live probe of 2026-09-21 measured the per-connection cost at about 135 ms (DNS ~14 ms, TCP ~58 ms, TLS ~64 ms) against ~145 ms of model time, so a per-call engine roughly doubles the latency of every warm call after the first.

The contract already has the object: ADR 0017's engine value owns sending, retries, the scheduler, the width gate, the cache, record and replay, the cancel token, the deadline, and the counters. The public library surfaces as the slides draw them are module-level one-shot calls (`tt.decide(...)`, `await tt.recognize(...)`), and nothing yet forbids the obvious implementation: build an engine, make one call, drop it. That implementation would make every call a cold call.

## The rule this issue asks for

1. **Every library exposes an explicit engine object as the primary API.** Constructing it is where the settings live (address, model, cache folder and cap, width, deadline defaults). The object holds the warm connection, the resolved address, the TLS state, the cache handle, and the usage counters.
2. **Module-level convenience functions stay, but they delegate to one lazily-created process-wide default engine.** Never a fresh engine per call. Zero boilerplate for the slide examples, one connection underneath.
3. **The engine may pre-warm** — resolve DNS and finish the TLS handshake at construction or on an explicit warm call, before the first question — so the first judgment costs model time, not setup time.
4. **The database extensions hang the engine off the host connection** (DuckDB and SQLite per-connection state, PostgreSQL per backend), so a warmed database session answers from one warm HTTP connection with no user action. `thinkthen_warm` becomes exactly this pre-warm plus the cache fill.
5. **Naming:** the public word is the engine (or the library's natural spelling of it: the `ThinkThen` object, the handle), not "connection." The object owns more than a connection, and the manual already teaches that the model, not the connection, is the cost a user thinks about.

## The test that pins it

Against a loopback stub that counts TCP connections: two module-level calls in one process must open exactly one connection (or the number the ruling fixes), and a second call must not re-handshake. The same test with an explicit engine object, and the same test through each database extension after a warm call. A per-call engine fails this with two connections.

## How bad it is for a user

Major for the library and database tier if implemented per call: every warm call pays a hidden ~135 ms and the width gate multiplies it. Nothing is wrong yet — the surfaces may already do this right — but nothing yet forbids doing it wrong, and the slides invite it.

Found by experiment 218, from Ian's question of 2026-09-22. The shape is the product side's to rule; the test is the quality plan's to run.
