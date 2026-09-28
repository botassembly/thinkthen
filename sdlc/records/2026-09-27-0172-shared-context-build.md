# 0172: shared context build

Status: candidate awaiting fresh code review and separately authorized live proof. Base: `0dcc48a4` (landed 0171). Ticket: [0172](../tickets/0172-a-shared-context.md). ADR: [0087](../planning/adr/0087-a-shared-context-stops-at-the-record-that-overflows.md).

## Result and proof

`--context FILE` now serves record-mode `decide`, `filter` and `rank`. The command reads exact UTF-8 bytes once, sends them as shared evidence in every request, and includes their SHA-256 in each detailed row. `meta.batch` remains present for context batches of one. The context changes the request digest but not the question digest. A too-large context and question fail before a send; a later record that cannot fit closes and sends the earlier batch, then stops without sending that record. A backend 413 keeps the context in both halves.

The eight focused command tests cover separate boundaries: exact pinned wire bytes and raw hash; batch-one metadata and dry-run; `filter` and `rank` dry-run adapters; 413 halves; exact late-prefix output and no refused-record send; the document/file/question shape table; request-size and profile-limit refusal table; and two-context digest/replay identity. The two refusal tables consolidate five cases each under one listener/count/output contract. The first compiled `--context` command failed before the option existed and passed after the implementation. A temporary late-stderr placeholder was replaced with the observed fixed sentence. No test-only product hook was added. Existing batching cases cover no-context bytes; the focused batching module is the retained regression proof. The paid 306-title check is prepared in `probes/context/` but has not been run.

## Measured growth and duplication review

The measured Rust delta against `0dcc48a4` is **641 nonblank lines**: 174 product and 467 tests, versus the original estimates of 116 and 232 (348 total). The coordinator authorized a measured amendment before completion. The product groups are 46 for context reading and raw-byte digest, 89 for typed evidence-free refusal mapping, and 39 for argument/metadata adapters and the late-close change. The tests add one module registration and a 466-line outside-in file. The largest current files are `cli/asking.rs` 500, `cli/failure.rs` 499, `core/batch.rs` 496, and the new test file 466; none exceeds 500.

I checked existing `Batcher`, `RequestMeta`, result constructors, listener and recording helpers before raising the ratchet. The code reuses the core `Evidence`, `Batcher`, request digest and result metadata instead of adding a second request form. The two refusal tables share loopback setup and pin distinct winning limits and failure phases. The batch-one/dry-run and 413 cases keep separate assertions because omitting context in one path would leave the other green. The prior no-context-only `refused` mapper was removed and replaced by the typed mapper; no duplicate cause mapper remains. No accepted boundary or readable code was cut merely to meet the stale estimate. The exact ratchet is 80,221.

## Validation and remaining work

The focused context command tests passed 8/8 and the full related batching module passed 29/29. All-feature, all-target strict Clippy, formatting, tickets, pages and policy checks passed; the exact ratchet is updated after formatting. No paid call or full `test`, `spec` or `surfaces` rung was run for this ticket. The coordinator will name the related-ticket checkpoint for 0170–0172.

The proposed live call is `sdlc/scripts/live --max-tokens 150000 probes/context/job.sh /home/ian/workspace/repos/beatles-bench NAME` after a clean, current build. The helper verifies 306 titles and 18 Abbey Road first albums, runs three one-request `decide --lines --details --threshold 0.7 --context` calls, grades each scratch output with `audit`, prints counts only, and stops before a repeat after 120,000 counted tokens or any score under 297. The accepted estimate is about 66,300 input tokens and $0.003 at the recorded input rate; the charged ceiling is 150,000 tokens. This is a proposal, not authorization or a claim of measured accuracy.
