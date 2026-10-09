# 0531: Account for Perplexity token usage in plans and budgets

Status: OPEN.

Milestone: 0.2

Reviews: revision 43719e4a80629187619d1cf48458f84429924656, accept

## Outcome

Perplexity plans and estimated-input budgets account for the provider's repeated shared-state token usage across questions. Other providers retain their existing estimates. Documentation distinguishes estimated tokens, reported billable usage and caller-configured prices.

## Evidence

- Starts from: Main `9086eda3e`. A reported saved comparison uses identical shared state with one, six and twelve questions: Perplexity reports 2209, 13254 and 26479 input tokens. A recognition call reports 116014 against a plan range of 26497 to 46628. The [provider's pricing documentation](https://docs.perplexity.ai/docs/decisions/quickstart#pricing) bills reported `usage.input_tokens` at $0.02 per million, with free output. The repeated-state counting formula is inferred from the reported replies, not documented as a provider guarantee.
- Keeps: One endpoint, explicit provider choice, pure core, native cache identity, saved-response tests, no-network gates, existing secrecy and token-cap refusals. Prices remain explicit caller settings; do not introduce a hardcoded tariff or silently change models.
- Changes: Inspect the existing native request estimator and route metadata, then apply repeated-state accounting to the Perplexity route before plan display and send-budget admission. Name the narrow estimator, route and test claims before coding. Correct the stale $0.04 setup example and explain the counting assumption and uncertainty. Use saved exchanges or synthetic public content only.
- Proof: Existing outside-in plan and send-budget cases retain their behavior. Add a focused shared-state/multiple-question regression for the Perplexity route, including zero sends when its revised estimate exceeds the cap, and prove another provider's estimate stays unchanged. Retain original raw replies and reported usage in existing recording storage; add no capture framework or paid gate.
- Defers: Actual invoice reconciliation and undocumented provider guarantees need provider evidence. No paid calls, clinical quality evaluation, automatic business routing, new raw-response feature or broader benchmark belongs to this repair.

## Progress

- 2026-10-09 started
