# Estimate repeated shared state

The build starts from `6eaee4b2a`. Ticket [0531](../tickets/0531-perplexity-estimated-token-accounting.md) fixes Perplexity plan and estimated-input admission undercounting when several wire questions share a state.

The selected backend carries explicit accounting metadata from its adapter table through named resolution. A pure calculation reads the final packed state's encoded bytes and actual question count. It adds the state once per question after the first, then applies the existing empirical byte density. Plans and text attempt admission share that calculation. Physical body bytes, prepared bodies, cache identity, model selection and caller-configured prices remain unchanged. Image attempt admission retains its existing tile calculation.

The existing send mark retains each attempt's computed estimate in private call and process counters. Existing call facts, command facts and Tally read the retained maximum instead of reconstructing it from physical body bytes. No public field or schema changes.

The [official pricing page](https://docs.perplexity.ai/docs/decisions/quickstart#pricing), checked on 2026-10-09, states $0.02 per million input tokens and free output. Billing uses reported `usage.input_tokens`. The setup example and installation page distinguish this price from explicit caller settings. The ticket's saved one-, six- and twelve-question comparison reports 2209, 13254 and 26479 input tokens. These observations support the repeated-state assumption; they establish no provider tokenizer formula or guaranteed upper bound. This build makes no paid call and changes no existing recorded response.

## Evidence

Fresh closure review accepted `4721e09fb25761d38d9e360164a68644d48fdd96`. It confirmed the agreed scope and unchanged accounting inputs against the existing routine, lint, specification and accepted site-build evidence. Provider counting guarantees and invoice reconciliation remain outside this repair.

The outside-in regression in `public_estimated.rs` failed before the fix because the Perplexity plan retained the ordinary estimate. It now proves shared-state accounting across three actual packed questions, unchanged request bytes, a cap between the old and revised estimates with zero loopback sends, ordinary-provider admission, and successful call and Tally facts matching the admitted estimate.

Focused public estimated-admission tests, public plan tests, image estimated-admission tests, the existing pure summary case and existing Tally cases passed. Existing CLI backend plan regressions passed. A built-command synthetic three-question shared-context check proved the CLI plan keeps physical bytes and computes its upper and largest token estimates through selected-route accounting. Targeted Clippy passed with warnings denied. Policy passed before Rust review. No source file exceeds its limit. The existing admission test file crosses the warning threshold because it retains distinct cap, retry, cache, replay and concurrency behaviors beside this regression; no mechanical split adds behavior.

The agreed plan, admission, provider isolation and documentation outcomes are implemented in `4d5d8b251`. Coordinator routine tests, strict workspace lint and executable specification checks passed; their existing output is in `target/0512-routine-all/{test.log,rust-lint-after-table.log,spec-after-settings.log}`. The [0467 record](0467-settings-reference-adoption.md#current-cli-examples) records the complete passing site build, including the Perplexity page. Relevant accounting code, regression, page and backend specification remain unchanged through this revision, so these checks apply without another build.

## Source budget

Fresh read-only review accepted `1b97c9273a1ecc79471764386329a48fb01f69b4`, including the source-ceiling increase that the coordinator applied before merging. Independent policy and focused behavior checks passed. The repeated-state formula remains an estimate supported by saved observations, with no billing guarantee. Actual invoice reconciliation remains deferred because it needs provider evidence; the repair requires no paid call.

## What the build taught us

Wire shape cannot identify accounting: several providers use the same adapter. Accounting belongs to selected backend metadata.

Body-byte maxima and estimated-token maxima measure different things. Retaining the computed estimate at the existing attempt mark keeps facts consistent without retaining input text or altering the request.

Process-wide admission totals require isolated regression processes. The existing child environment helper prevents one test's successful sends from spending another test's cap.

A measured admission cap bounds the estimate. Reported billable usage can exceed it, so pricing examples and plans must state the assumption and uncertainty.
