# 0474: Share cache admission across one recognition call

Status: OPEN. Source confirms repeated validation and write transactions on current-schema cache opens. The reported jobs-eight recording failure and forty-second startup timing remain unconfirmed locally.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Severity: medium storage correctness and responsiveness.

Reviews: fresh Sol High ticket review, accept

Reviews: revision 62161ace2, accept

## Outcome

One recognition operation validates its cache before sending and shares that admission across its records and stages. Current-schema validation does not take a migration write transaction. Damaged saved answers still refuse before a request. Reported recording failures get a concrete reproduction and fix owner rather than being assumed solved by this optimization.

## Evidence

- Starts from: Shared mailroom `2026-10-08-tcga-demo-recognize-controls-we-need-in-0-2-with-evidence.md`, asks 11 and 12; 0462 bug sweep. Fresh read-only Sol diagnosis and Astra Extra High direction identify `engine/store/migration.rs`: Entries::read and normalization happen under transaction even when user_version is already 2. `engine/pipeline.rs` opens stores per stage; CLI record scheduling admits concurrent records. SQLite's thirty-second busy wait maps to the reported safe recording error. The exact reported failing operation is not yet established.
- Keeps: Corrupt-entry refusal before sending, validated cached-answer reuse, legacy conversion atomicity, cancellation and lock deadlines, secrecy, cache/record/replay identity, correct run facts, configured request concurrency and all saved content. Storage synchronization must not hold a lock across provider execution. Separate SDK calls get separate admissions; no engine-lifetime or process-global validation shortcut.
- Changes: Define an operation-scoped admission shared across CLI concurrent records and recognition stages, native recognize_with stages and recognize_records_complete_with records. Validate a consistent current-schema snapshot with a read transaction; acquire a write transaction only for actual migration or necessary schema/index creation, rechecking the version and index under that transaction. Store::connect currently also wraps CREATE INDEX IF NOT EXISTS in BEGIN IMMEDIATE on every open: inspect the expected index read-only first and skip the write path when already present, preserving missing-index repair with a transaction and race recheck. Retain per-answer validation at lookup. Define external-change invalidation before coding: retain an operation-owned connection/synchronization boundary or equivalent checked generation so outside writes trigger renewed validation without mistaking this operation's own writes for an external change. Do not bypass validation merely because the schema is current. Keep the state private to engine/adapters, with no core I/O or routing policy.
  Claim `crates/thinkthen/src/engine/store/**`, `crates/thinkthen/src/engine/pipeline.rs`, `crates/thinkthen/src/engine/facade/recognize/**` and `crates/thinkthen/tests/backend/**`. Native Windows checks are owed to the first authorized candidate; keep the ticket open until they pass.
- Proof: Fresh ticket and code review. Reuse synthetic stores, saved exchanges and existing loopback helpers. Reproduce jobs one/eight with fresh and populated caches, duplicate and distinct records, and capture the internal failing SQLite operation/category without exposing private data. Check one preflight per unchanged operation, fresh preflight across calls, stage/record sharing, corrupt entries and externally changed entries refused before sends, migration and missing-index races with version/index rechecks, concurrent current-schema opens without unnecessary writer locking, cancellation, concurrent loopback request progress and unchanged request/cache identities. Measure owned synthetic cache scan count and first-request time; one full initial scan may remain linear in cache size. Do not claim constant-time startup or that the reported recording error is fixed without reproducing its cause. Fix a confirmed separate cause in its own ticket. Run policy before review, applicable full tests/lint on landing source, and one short record. No paid call or release workflow.
- Defers: No lazy acceptance of unvalidated saved content, cache-format change, new store, global memoization, performance benchmark framework, proof tooling or release management. Snippet controls and larger recognize feature scope remain separate PM intake.
