# 0449: Keep each SDK engine on one configured route

Status: built; awaiting fresh code review and landing.

Milestone: 0.2
Owner: Codex builder, lane claude-1.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

The standing SDK contract resolves one endpoint, one effective key and one provider API type per built engine. Business routing, A/B policy, curation and automatic threshold tuning remain proxy work.

## Evidence

- Starts from: Main 399c6c7c7 and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 3; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Record an ADR and audit current/planned features for the boundary, with an explicit keep/refuse table. Amend the 0.2 tickets that imply SDK model-group interpretation or automatic route selection. No fallback provider, routing table or policy optimizer is added.
- Proof: Saved/loopback cases prove endpoint/key/API-type invariance across stages, retries and splits; direct explicit model selection retains its old precedence. Unsupported reserved proxy inputs refuse before lookup/send. Existing offline audit tools do not mutate runtime policy unless a caller explicitly requests an offline file write.
- Defers: Proxy routing, model groups, fallback providers, A/B allocation, curation and automatic threshold tuning.

## Dependencies and ownership

Precedes the revised 0442 and 0450 contract, before broad host edits. 0443/0444 implement transport/storage, not routing. 0433’s explicit SQLite model override remains required.

## Design notes

Keep named backend/configuration setup shortcuts: selection resolves once before the call and users may construct another engine explicitly. Keep direct caller-selected model names as opaque provider parameters for compatibility; the SDK neither maps them to a group nor chooses/adapts them. Nested annotate/rank-set model refusals remain. Keep transport retry, splitting/batching, cancellation, local rate/size/spend bounds, caller prices and calibration warnings because they enforce caller/transport resources and report facts. Keep caller-supplied thresholds and free offline rereading as explicit deterministic reading rules; they do not tune or install business policy. Keep runs audit/diff and transforms as explicitly invoked offline analysis. Future explicitly configured proxy calls refuse question-level model selection because the proxy owns it; this activation contract is 0.3. Direct endpoint hostnames or reply fields never switch the SDK into proxy mode. Record each retained feature and reason in repo contracts, not only a temporary plan.

## Built boundary

ADR 0119 and `specification/sdk-boundary.md` record the permanent rule and the current/planned keep/refuse table. Settings, backends and the specification index link it. The existing engine already captures one selected backend and key; no runtime refactor is needed. Existing loopback stage, retry and refusal-split cases now assert the posting path, effective fake key and literal model on every send. Named-backend precedence tests remain the model/route resolution oracle. Reserved proxy activation validation stays with 0450; metadata/storage adoption stays with 0442–0444. These implementations remain open and this ticket makes no claim that they pass.

Focused offline checks passed: three-stage recognition, status retry, refusal splitting, and both CLI/Rust every-pair named-backend precedence cases (five tests). Policy, formatting, source ratchet, settings reference and whitespace checks passed. Builds ran under a systemd resource cap with a minimal explicit environment and no paid calls. Full gates and the final record belong to landing.
