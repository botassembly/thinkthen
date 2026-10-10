# 0470: Bound DuckDB complete-file admission

Status: OPEN. The review confirmed that file descriptors accumulate before native admission; memory exhaustion is a traced risk, not a reproduced crash.

Milestone: 0.2

Depends on: 0503

Owner: builder.
Severity: medium resource correctness.

Reviews: revision caddaecbc, accept

Reviews: revision 56af78e68, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision ec9580145528656502244889a38a340875a214c6, reject

Reviews: revision 41545205e11b00ee794b6a39ea42a54024c8048b, accept

Reviews: revision ece6f886028471cf76817ac655aba4343dd62f99, reject

## Outcome

DuckDB complete-file calls feed descriptors to native admission through 0503's bounded session. Engine limits and cancellation stop further file reads. Retained results and generic whole-set operations keep their existing memory contracts; this ticket makes no total-memory guarantee.

## Evidence

- Starts from: the 0462 SQL review at 7ea661c1e. `databases/duckdb/cpp/src/complete_files.cpp` accumulates every reader descriptor into one string before the native call. `databases/duckdb/cpp/src/files_manifest.cpp` caps path bytes only. Native text and image limits apply per item, so repeated admitted paths do not bound total retained content.
- Keeps: DuckDB-authorized FileSystem access, order, duplicates, original locations, secrecy, cancellation, and native per-item and whole-set limits. Static declaration refusals still come before any file open or read. Add no invented hard cap that rejects supported record streams.
- Changes: 0503 and [ADR 0129](../planning/adr/0129-owned-json-sessions.md) own the shared session, C handles, queues and sink dispatch. This ticket consumes them and builds no second queue or handle family.
  - Keep authorized DuckDB file handles and readers on the calling thread. Copy descriptors into the session. Never send DuckDB handles or context pointers.
  - When the session is full, check DuckDB interruption and wait only in short stop-aware steps. Close on end of input, reader failure or cancellation.
  - Apply ADR 0129's read-ahead rule: runtime closure may race one already-read unaccepted descriptor. After Closed, read no further. Preserve completed prefixes and refuse whole-set overflow before reading suffixes.
  - On cancellation, signal native cancellation and drop host endpoints without waiting for a provider call. No live worker references a freed host handle. Independent owned native state lives until the provider call settles under the existing detach and deadline policy, which 0503 and ADR 0129 own.
  - Claim the DuckDB calling-thread producer in `databases/duckdb/cpp/src/complete_files.cpp`, its native bridge under `databases/duckdb/bridge/src/ffi/complete_files/`, and the existing file and cancellation tests. Name the exact test files before coding.
- Proof: A fresh design review and code review, with a High review for unsafe ownership changes. The existing reader, complete and cancellation fixtures pass. One deterministic bounded-admission case fails on the accumulator path and passes on the session path. No memory-exhaustion campaign, paid calls or hosted workflow.
- Defers: DuckDB feature redesign, provider limit tuning and release management. Process exhaustion stays labeled as inferred. Changes to generic rank or total result-memory contracts need their own reviewed design. 0495 follows this feed for the DuckDB Request migration.

## Progress

- 2026-10-10 started
- 2026-10-10 landed 4b08e6986; next: Reviewed DuckDB file admission design is landed. Implement shared 0503 owned feed controls and eager session admission first, then the calling-thread DuckDB producer; release checks remain held.
