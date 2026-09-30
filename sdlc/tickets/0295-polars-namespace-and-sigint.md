# 0295 — Polars namespace and SIGINT (F4)

Status: deferred until after the 0.1 release, by the queue batches Quick Fix of 2026-09-30, a coordinator default Ian can overturn. Python already judges a Polars `Series` in one engine call; the expression namespace is an additive opt-in import, so shipping it later breaks no consumer. ADR 0111 slice 3 has landed, so only the 0.1 order holds it. Draft from Codex branch `ticket/0283-sql-frame-redesign-preparation`.

## Outcome

Opt-in import thinkthen.polars registers Expr.tt decide/choose/score/tag via map_batches(is_elementwise=True) over the existing Arrow door. Probability Struct only on decide/choose; token and deadline_ms bind at build, deadline applies per morsel. Lead with cheap filters then with_columns; a judged filter and subsequent head can judge the full scan. Install the SIGINT sequence chain on first expression call, recheck disposition and process id, and preserve ordinary calls.

The expression deadline applies to each morsel, not the whole query. RSS/load/churn is opt-in.

## Prerequisites and proposed files

Prerequisite: F1/F2 and F3. Proposed file families: `libraries/python/thinkthen/{polars.py,__init__.pyi}; libraries/python/src/{frame.rs,engine.rs}; Python pyproject/README/ratchets and focused Polars tests`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Pinned Polars judged rows and output parity, SIGINT child/recovery/re-chain, per-morsel deadline and tally. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Polars expression namespace, per-morsel controls and SIGINT sequence chain.
- Proof: Pinned Polars judged rows and output parity, SIGINT child/recovery/re-chain, per-morsel deadline and tally. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
