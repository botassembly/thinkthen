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
- Changes: Design the smallest native reporting path using existing warnings/facts or an additive explicit finalization result. Adopt that path across SDK and SQL consumers that persist usage. Record async timing semantics so a call does not falsely promise that pending writes succeeded; do not convert a valid answer into a backend failure.
- Proof: Fresh design/ticket and code reviews. Existing writer failure fixtures plus a deterministic public consumer with a failed owned persistence path, correct call facts, safe warning and exact request count. Applicable full checks at landing; no hosted run or paid call.
- Defers: No monthly spending policy, ledger change, telemetry, new durable store or proof framework. Release management remains held.
