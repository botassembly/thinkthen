# 0460: Bound requests for repeated records when caching is disabled

Status: COMPLETE. Fresh whole-change High review accepted source 710bd20ec5ba56a256fa991cd91df9a57007d0bd after focused and forwarded checks passed. Full test, lint and specification gates run on the recorded candidate before landing.

Milestone: 0.2

Owner: lane0 builder.

## Outcome

Native and ordinary command-line planning supply a request count callers can safely use for admission when repeated records may send again with caching disabled. Every original rank record and position survives. The estimate states when it is a bound rather than an exact count.

## Evidence

- Starts from: Main 7b3f7729b and the experiments message `2026-10-07-trials10-coordinator-planner-undercounts-repeated-texts-with-cache-disabled.md`. Its recorded calls exhausted a planned request allowance before returning every record. The proposed local fixture uses 164 unique records followed by 22 repeats with batch 16, one worker, caching disabled and no retries.
- Keeps: Existing admitted planning routes, request packing, validation, in-flight coalescing, bounded execution memory, every original record identity, zero-send planning, secrecy, cache freshness and replay behavior. Refusal splits and retries retain their documented separate limits.
- Changes: Reproduce the reported planner/runtime difference with the owned loopback fixture. Make the existing planner account conservatively for repeats that execution can send again, or prove an equally small compatible correction. Reuse PlanEstimate and its existing upper_bound field where possible. Document the count and byte semantics without adding routing, a new cache, a new public function or a C ABI change.
- Proof: Extend existing behavior-named native planning tests. Preserve the report's original record count and the planned-budget failure as the prior-failing witness; the corrected request allowance must complete all records and retain their original positions. Check unique input, repeated input within and beyond the in-flight window, existing staged/byte-bound behavior and zero-send planning. Use saved replies and loopback only. Do not weaken request budgets, suppress refusals or make hosted calls.
- Defers: Application workarounds, new benchmark campaigns, provider routing and unrelated changes. This correctness fix precedes the named core-freeze checkpoint.

## Reviewed design

Fresh Astra Extra High review accepted this ticket and the bounded design on 2026-10-07. The native planner and ordinary command-line and annotate planners have the same global duplicate filter. Correct all three together without changing runtime scheduling or caches.

- Count admitted wire-question occurrences with checked arithmetic as the request admission bound. Every initial request contains at least one occurrence; pauses, window flushes and later repeats cannot exceed this bound. Refusal splits and retries remain outside it.
- Remove global duplicate filtering from the prepared-body preview. Drain each closed batch immediately and retain only the open batch, first body and counters. Set upper_bound when occurrence count exceeds packed preview-body count.
- Preserve exact byte/token estimates for the uncoalesced uninterrupted preview. These estimates are not runtime byte/token admission guarantees. The independent three-record fixture retains bytes418, tokens215..380 and first body, while requests becomes3 and upper_bound becomes true.
- Annotate request_count and group_requests continue to describe the hypothetical packed preview. Its requests field is the conservative admission bound and may differ. Recognize and relate staged bounds retain their existing algorithms.
- Record one short ADR superseding ADR0111 section2's global once-per-call promise and amendment table. Retain its section3 pending-key coalescing and normal cache eligibility after completion. Update existing channel, annotate, settings and backend-check documentation coherently. No new public field, caller setting, C layout or function is needed.

Extend existing native and CLI planning regressions, including a multi-question tag case so the bound counts wire questions rather than records. Preserve reader failure/unread-tail checks, zero-send planning and all original duplicate record positions. One fresh whole-change code review follows focused policy/lint and behavior tests; add one short record at landing.

Code review: ACCEPT, 2026-10-07; fresh whole-change read-only Sol High review of 710bd20ec5ba56a256fa991cd91df9a57007d0bd against 0d78347db.

Reviews: accept
