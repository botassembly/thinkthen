# 0468: Report failed persistent usage writes to SDK callers

Status: OPEN. The sweep found that the deferred writer-failure issue still has a deterministic silent-failure path.

Milestone: 0.2

Owner: builder.
Severity: medium accounting correctness.

Reviews: revision 19abe6542, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

SDK and SQL callers can discover failed persistence of usage counts without exposing evidence, credentials or private filesystem paths. In-memory call facts remain correct; persistence failure never silently claims durable totals are complete.

## Evidence

- Starts from: issue 2026-09-30-a-usage-write-that-fails-after-a-good-start-is-silent-on-the-libraries.md; engine/usage.rs latches Queue.failed, and engine/facade/finish.rs discards finish's failure. CLI has a warning but the public Engine::finish_usage returns no failure. The after-sprint sweep promotes this known runtime defect into 0.2.
- Keeps: Count-only persistence, private files, the existing usage-lock deadline, no additional model call, existing in-memory facts and successful answers. No secret or authored content in warnings.
- Changes: Add nonblocking Engine::usage_persistence() and Engine::finish_usage_status() with the existing usage-lock deadline, returning UsagePersistence; retain void finish_usage() for compatibility. States are Disabled (no usage path), Pending (queued/writing), Written (this engine’s current deltas drained), and latched Failed (writer/queue failure, taking precedence). Written says nothing about future calls or other engines/processes. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Give all language engines explicit equivalent methods and the C JSON door a status request. Add thinkthen_usage_status() to all three SQL extensions without changing thinkthen_usage() output; PostgreSQL aggregates built engines and DuckDB retains failure when engines are evicted. Only fixed safe advice is exposed. Never infer durable status from call facts or turn a valid answer into a backend failure.
  Implement the shared state and facts contract first. Claim `crates/thinkthen/src/engine/usage.rs`, `crates/thinkthen/src/engine/usage/**`, `crates/thinkthen/src/engine/facade/finish.rs`, `crates/thinkthen/src/engine/facade.rs`, `crates/thinkthen/src/engine/call_facts.rs`, `crates/thinkthen/src/public/engine.rs`, `crates/thinkthen/src/public/options.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/public/results.rs`, `crates/thinkthen/src/public/results/call.rs`, `crates/thinkthen/src/public/results/complete_facts.rs` and existing usage/facts consumer tests. Name any further implementation seam individually. Generated-family adoption tickets carry this settled observation into host methods in 0.2. Subsequent C/SQL status slices name their adapter files when assigned; no current claim covers every library, database or public module.
- Proof: Fresh design/ticket and code reviews. Existing writer failure fixtures plus public consumers prove Pending during a held write, Written after successful finalization, latched Failed after an owned write failure, Disabled without storage, safe advice, unchanged call facts and exact request counts. SQL status remains callable before exit; exit hooks alone are insufficient. Applicable full checks at landing; no hosted run or paid call.
- Also changes: Under the 2026-10-08 binding architecture ruling, expose the safe persistence state in the shared facts JSON with its observation point. A result's earlier Pending snapshot cannot imply a later asynchronous write succeeded. Settle this representation before generated result readers, and have their adoption tickets consume it rather than adding independent per-binding logic.
- Defers: No monthly spending policy, ledger change, telemetry, new durable store or proof framework. Release management remains held.
