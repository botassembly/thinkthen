# Estimate repeated shared state

The build starts from `6eaee4b2a`. Ticket [0531](../tickets/0531-perplexity-estimated-token-accounting.md) fixes Perplexity plan and estimated-input admission undercounting when several wire questions share a state.

The selected backend carries explicit accounting metadata from its adapter table through named resolution. A pure calculation reads the final packed state's encoded bytes and actual question count. It adds the state once per question after the first, then applies the existing empirical byte density. Plans and text attempt admission share that calculation. Physical body bytes, prepared bodies, cache identity, model selection and caller-configured prices remain unchanged. Image attempt admission retains its existing tile calculation.

The existing send mark retains each attempt's computed estimate in private call and process counters. Existing call facts, command facts and Tally read the retained maximum instead of reconstructing it from physical body bytes. No public field or schema changes.

The [official pricing page](https://docs.perplexity.ai/docs/decisions/quickstart#pricing), checked on 2026-10-09, states $0.02 per million input tokens and free output. Billing uses reported `usage.input_tokens`. The setup example and installation page distinguish this price from explicit caller settings. The ticket's saved one-, six- and twelve-question comparison reports 2209, 13254 and 26479 input tokens. These observations support the repeated-state assumption; they establish no provider tokenizer formula or guaranteed upper bound. This build makes no paid call and changes no existing recorded response.

## Evidence

The outside-in regression in `public_estimated.rs` failed before the fix because the Perplexity plan retained the ordinary estimate. It now proves shared-state accounting across three actual packed questions, unchanged request bytes, a cap between the old and revised estimates with zero loopback sends, ordinary-provider admission, and successful call and Tally facts matching the admitted estimate.

Focused public estimated-admission tests, public plan tests, image estimated-admission tests, the existing pure summary case and existing Tally cases passed. Targeted Clippy passed with warnings denied. Policy passed before Rust review. No source file exceeds its limit. The existing admission test file crosses the warning threshold because it retains distinct cap, retry, cache, replay and concurrency behaviors beside this regression; no mechanical split adds behavior.

The site build cannot run in this worktree because its installed dependencies and output are absent. The changed page and specification were checked against the current provider source. Full landing gates belong to the coordinator.

## Source budget

Propose 275 additional handwritten nonblank Rust lines over the starting ceiling of 180824, for a measured total of 181099. Generated source grows by zero lines; no generator changes. The shared estimator replaces duplicate estimate reconstruction, and the outside-in case reuses the existing child fixture. The ceiling remains unchanged for explicit reviewer acceptance.

## What the build taught us

Wire shape cannot identify accounting: several providers use the same adapter. Accounting belongs to selected backend metadata.

Body-byte maxima and estimated-token maxima measure different things. Retaining the computed estimate at the existing attempt mark keeps facts consistent without retaining input text or altering the request.

Process-wide admission totals require isolated regression processes. The existing child environment helper prevents one test's successful sends from spending another test's cap.

A measured admission cap bounds the estimate. Reported billable usage can exceed it, so pricing examples and plans must state the assumption and uncertainty.
