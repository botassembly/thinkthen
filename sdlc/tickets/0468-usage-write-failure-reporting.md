# 0468: Report failed persistent usage writes to SDK callers

Status: OPEN. The sweep found that the deferred writer-failure issue still has a deterministic silent-failure path.

Milestone: 0.2

Owner: builder.
Severity: medium accounting correctness.

## Outcome

SDK and SQL callers can discover failed persistence of usage counts without exposing evidence, credentials or private filesystem paths. In-memory call facts remain correct; persistence failure never silently claims durable totals are complete.

## Evidence

- Starts from: issue 2026-09-30-a-usage-write-that-fails-after-a-good-start-is-silent-on-the-libraries.md; engine/usage.rs latches Queue.failed, and engine/facade/finish.rs discards finish's failure. CLI has a warning but the public Engine::finish_usage returns no failure. The after-sprint sweep promotes this known runtime defect into 0.2.
- Keeps: Count-only persistence, private files, bounded finalization, no additional model call, existing in-memory facts and successful answers. No secret or authored content in warnings.
- Changes: Add nonblocking Engine::usage_persistence() and bounded Engine::finish_usage_status(), returning UsagePersistence; retain void finish_usage() for compatibility. States are Disabled (no usage path), Pending (queued/writing), Written (this engine’s current deltas drained), and latched Failed (writer/queue failure, taking precedence). Written says nothing about future calls or other engines/processes. Give all language engines explicit equivalent methods and the C JSON door a status request. Add thinkthen_usage_status() to all three SQL extensions without changing thinkthen_usage() output; PostgreSQL aggregates built engines and DuckDB retains failure when engines are evicted. Only fixed safe advice is exposed. Never infer durable status from call facts or turn a valid answer into a backend failure.
- Proof: Fresh design/ticket and code reviews. Existing writer failure fixtures plus public consumers prove Pending during a held write, Written after successful finalization, latched Failed after an owned write failure, Disabled without storage, safe advice, unchanged call facts and exact request counts. SQL status remains callable before exit; exit hooks alone are insufficient. Applicable full checks at landing; no hosted run or paid call.
- Defers: No monthly spending policy, ledger change, telemetry, new durable store or proof framework. Release management remains held.
