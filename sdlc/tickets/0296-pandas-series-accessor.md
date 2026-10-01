# 0296 — pandas Series accessor (F5)

Status: deferred until after the 0.1 release, by the queue batches Quick Fix of 2026-09-30, a coordinator default Ian can overturn. Python already takes a pandas `Series` in the four verbs and returns one with the caller's index and name (`libraries/python/README.md`); the `.tt` accessor is additive sugar, so shipping it later breaks no consumer. Draft from Codex branch `ticket/0283-sql-frame-redesign-preparation`.

Milestone: later

## Outcome

Opt-in import thinkthen.pandas registers series.tt decide/choose/score/tag, returning a Series with original index/name through one packed engine call. Base import stays independent of pandas; pandas remains eager. Use chunked reader examples for bounded memory and explicit tally for facts.

## Prerequisites and proposed files

Prerequisite: F1/F2. Proposed file families: `libraries/python/thinkthen/{pandas.py,__init__.pyi}; libraries/python/tests/{test_pandas.py,test_call.py}; Python pyproject/README/ratchets`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

pandas 2.3.3 and 3.0.6 Series replay preserves nulls/name/index and one exact body. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Opt-in pandas Series accessor preserving index and name.
- Proof: pandas 2.3.3 and 3.0.6 Series replay preserves nulls/name/index and one exact body. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
