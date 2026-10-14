# 0585: Experiment: many writers on one SQLite answer file

Status: OPEN.

Milestone: 0.3

## Outcome

Learn whether one SQLite file in write-ahead-log mode, with full sync, one write per reply and 0.2's 30-second wait, serves many ThinkThen writers at once with zero lost answers and acceptable waits.

## Evidence

- Starts from: `crates/thinkthen/src/engine/store/opening.rs` and the rest of `engine/store/` for today's settings; the marketing repo's `products/thinkthen/0.3/prd/01-database.md`, "Concurrency".
- Keeps: No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work.
- Changes: `sdlc/records/0585*` only. Owned numbered experiment folder with `OWNER` reading `thinkthen/0585`.
- Method: a throwaway writer using today's store settings and a small answers table (0584's tables if ready). Run eight writer processes, two readers, and one reader holding a long snapshot, each writer storing 100,000 answers; then add four writers through the SQL extensions if main lets them open a named database, or four more processes if not, and say which. Local disk only.
- Proof: count lost rows, busy errors, 50th and 99th percentile waits, and write-ahead-log growth. Pass means zero lost rows, zero timeouts and a 99th percentile wait under one second.
- Stop rule: the pass above, or the first lost row or timeout, with its cause.
- Decision it changes: SQLite as the one store for 0.3, and whether writes must be grouped.
- Machine: the Beelink.
- Defers: the proxy's writers wait for 0.5.
