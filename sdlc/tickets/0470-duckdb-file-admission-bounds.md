# 0470: Bound DuckDB complete-file admission

Status: OPEN. The review confirmed that file descriptors accumulate before native admission; memory exhaustion is a traced risk, not a reproduced crash.

Milestone: 0.2

Depends on: 0503

Owner: builder.
Severity: medium resource correctness.

Reviews: revision caddaecbc, accept

Reviews: revision 56af78e68, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

DuckDB complete-file calls bound descriptor staging before native admission, so engine limits and cancellation can stop further file reads. This does not promise a constant bound for retained results or generic whole-set operations; their existing contracts remain.

## Evidence

- Starts from: 0462 SQL review at 7ea661c1e; cpp/src/complete_files.cpp accumulates every reader descriptor into one string before the native call. files_manifest.cpp caps path bytes only. Native text/image limits are per item, so repeated admitted paths do not bound total retained content.
- Keeps: DuckDB-authorized FileSystem access, order, duplicates, original locations, secrecy, cancellation and native per-item/whole-set limits. No invented hard cap that needlessly rejects otherwise supported record streams.
- Changes: Keep authorized DuckDB file handles/readers on the calling thread. Give an owned native call handle begin/try_push/finish/cancel/free operations and a capacity-one descriptor channel to a Rust worker; copy descriptors, never send DuckDB handles/context pointers. try_push is stop-aware and nonblocking, returning accepted/full/closed; on full the producer checks DuckDB interruption and waits only in short stop-aware steps. Close on EOF, reader failure or cancellation. Stop the producer when native admission closes the receiver; preserve completed prefixes and refuse whole-set overflow before reading suffixes. On cancellation, signal native cancellation and drop host endpoints without waiting for a provider call; retain independent owned native state until that call settles under the existing detach/deadline policy. No live worker can reference a freed host handle. Join only an admission helper whose completion does not depend on provider execution, if one is introduced. Bound pre-engine read-ahead without inventing an aggregate cap for generic rank. Native retained output and non-incremental collection still scale with accepted data; this fix makes no total-memory guarantee.
  Claim `databases/duckdb/**`, `crates/thinkthen/src/public/**` and `libraries/c/src/**`. Coordinate session ownership with 0503.
- Proof: Fresh design/ticket and code reviews, with High review for unsafe ownership changes. Existing reader/complete/cancellation fixtures plus a deterministic bounded-admission case that fails on the accumulator path. No memory-exhaustion campaign, paid calls, hosted workflow or new proof framework.
- Defers: No DuckDB feature redesign, provider limit tuning or release management. Process exhaustion is inferred and must remain labeled that way. Changes to generic rank or total result-memory contracts require a separate reviewed design, not an arbitrary cap in this adapter.

## Surface assessment amendment

0503 and ADR 0129 own the shared session, C handles, queues and sink dispatch. Consume that interface here; do not build a second one. This supersedes this ticket's broad public/C implementation claim. Claim the actual DuckDB calling-thread producer, native bridge and existing file/cancellation tests in the implementation slice. Apply ADR 0129's precise read-ahead rule: runtime closure may race one already-read unaccepted descriptor; after Closed, advance no further. Static declaration refusals still precede file opens and reads. 0495 follows this bounded feed for its Request migration.
