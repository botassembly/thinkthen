# 0304: One question cache and one batching path

Status: design accepted; slice 1 ready. Lane claude-2. Plan: `sdlc/planning/cleanup-2026-09-30.md`, rulings 2 to 6.

## Outcome

One engine path serves all ten functions and every surface. It builds backend questions, looks each one up in the cache, packs only the misses into requests, sends them, splits the replies, stores each answer alone with a taken-at time, and returns results in input order with bounded memory.

## Design first

Write ADR 0111. It replaces ADR 0048 item 5 and settles:

- The question key: model, backend address, shared context, and one question as sent. What else changes an answer and so belongs in it.
- How `choose`, `tag`, and `score` quote each record in its own question.
- How `find`, `recognize`, and `relate` map onto question entries.
- The store: one SQLite file or another format. Locking between processes. What happens to per-digest lock files and coalescing.
- Test recordings: the simpler of one store or two, and a converter for old request-level recordings.
- Token and request accounting across only the sent questions.
- Partial replies, refusals that halve a batch, and retries.
- Where the requests-per-minute limit sits.
- Which current code the path deletes: content-cut batching, duplicate schedulers, and the batch paths in `cli`, `public`, and the facade.
- The build order in slices that each land green.

A fresh reviewer returns ACCEPT or findings before any build.

## Slices

ADR 0111 names each slice's proof. Each lands green with `cargo test --workspace`, `policy.py` and a fresh code review.

1. The quoted wire form on every surface. The shared `conformance/` cases and committed recordings are rewritten at the request level. The `spec` gate stops replaying probes. No storage or scheduler change.
2. Question key, SQLite store, sorted JSON Lines fixtures, `cache convert`, and the pipeline for the seven record functions on the command. Proof includes 100 records, then 120, sending only the 20 new questions on the loopback backend.
3. Public Rust API, Polars eager and lazy, the C door and the SQL hosts on `ask_all`.
4. `find`, `recognize` and `relate` on `ask_all`. Their fixtures convert without loss.
5. Remove the old store, locks, marker, request-level prune, both old schedulers and every committed `DIGEST.json` outside probe history.

## Retained behavior

Output order, exit codes, the null-versus-failure rule, key secrecy, no key read on replay.

## Proof

Named in the ADR. At least: a batch of 100 records followed by a run of 120 that includes them sends only the 20 new questions, counted on a loopback backend.

## Deferred gaps

Cache clearing and expiry.
