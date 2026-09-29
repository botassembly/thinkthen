# 0283 — Core settings schema and plan (T1)

Status: Accepted design. High preparation review accepted `8bf14799`; implementation is in progress on this ticket branch. No code outcome or issue closure is claimed yet.

## Outcome

Build `thinkthen.settings/1` and its parser once in the Rust core
(`crates/thinkthen/src/core` boundary respected: the parser is pure), beside
one core engine-settings schema shared with the C door's
`thinkthen_engine_new_with` (ADR 0037), gaining `max_requests_total`. Unify
the deadline as `deadline_ms` milliseconds at every boundary a number crosses
(Rust keeps `Duration` internally). Replace the asking verbs' `--dry-run` with
`--plan` (first request body plus a whole-input count line; `check --plan`;
`cache prune --dry-run` stays), moving in the same commit: the `spec/` pages,
the `specification/` pages and fixtures, the `specification/settings.md`
Dry-run and Deadline rows and the new schema row, the gated demo blocks (03,
06, 14, 15, 16, 21, 45), the probe self-tests
(`probes/probability-total-0038`, `probes/find-0040`), and the remaining prose
(`demos/27-test-with-no-network/README.md`, `demos/FINDINGS.md`, `AGENTS.md`).

Current C constructor enters libraries/c/src/ffi.rs and parses in libraries/c/src/settings.rs; the Rust builder is crates/thinkthen/src/public/settings.rs. The parser belongs in pure core and cannot read files, environment, clock or sockets. The C engine schema may validate max_requests_total here, but no public configure path may accept an inert value: T7 supplies active reservation and final exposure. Existing CLI preview branches include asking, batched, find, annotate, recognize, relate and check; cache prune --dry-run is a deliberate exception. At the 500 nonblank-line cap, cli/args.rs measures 473 (27 headroom), public/settings.rs 470 (30), and cli/args/command.rs 430 (70); cli/asking.rs is 419 (81) and core/question_file.rs 392 (108). Reuse modules or one cohesive private extraction rather than consuming unrelated headroom. Libraries/c/src/settings.rs is 143 nonblank lines. Remeasure all touched files and shared ratchets before review.

The changed executable blocks include demos 03, 06, 14, 15, 16, 21 and 45, probes probability-total-0038 and find-0040, specification/settings.md Dry-run/Deadline/schema rows, demos/27 and FINDINGS, plus AGENTS.md. The old spec/decide.md assertion rejecting --plan must change with the flag. The full V/I corpus belongs in one core fixture, not copied to every host.

**One preview owner.** Add `crates/thinkthen/src/core/plan_summary.rs`, exported privately through `core/mod.rs`, as the pure full-input accumulator for `records`, `requests`, `estimated_bytes` (sum of exact prepared request body lengths), and the `estimated_input_tokens` band at ADR 0105's 0.516/0.908 tokens per byte. Pin integer rounding and overflow refusal in an independent fixture. Retain the first prepared body for CLI disclosure and mark `recognize`/`relate` request counts as upper bounds where later answers determine work. CLI adapters feed validated ordered records to the summary. For ordinary record streams the summary drives `core/batch.rs::Batcher::{push,finish}` over the **whole** input, including content cuts, 4,096-member and configured batch limits; staged adapters supply their already prepared chunks and explicit upper-bound counts. Neither summary nor adapter invents a second cut/split algorithm. Non-batch paths already use `engine/prepared_request.rs::PreparedRequests::with_profile` through `engine/facade.rs::split` for request-byte, question and profile limits. If core's inward dependency rule requires that pure splitter in core, move it once and leave the engine forwarding to it. Planned counts exclude cache answers, refusal splits and retries.

Adapt all CLI entries to the same summary: `cli/asking/plan.rs` now prints one record; `cli/asking/batched.rs::planned` stops at the first closed batch; `cli/find.rs` and `cli/annotate/plan.rs` have separate first-set/group paths; `cli/recognize/dry_run.rs` stops at one record and knows staged name/pair bounds; `cli/relate/dry_run.rs` reports prepared relation chunks; `cli/check.rs` has four fixed probes. Preserve existing framing, disclosure and first-body shapes while adding whole-input totals. `core/plan_document.rs` may serialize the summary but does not calculate another estimate. Contract pages must name the upper-bound mark and measured token-band rates.

## Prerequisites and proposed files

Prerequisite: first build. Proposed exact future claim: new `crates/thinkthen/src/core/plan_summary.rs`; `crates/thinkthen/src/core/{mod.rs,batch.rs,plan_document.rs,question_file.rs,question_file/resolve.rs,threshold.rs}`; `crates/thinkthen/src/engine/{prepared_request.rs,facade.rs}` only if sharing its pure splitter needs an extraction; `crates/thinkthen/src/public/{settings.rs,options.rs}`; `libraries/c/src/{settings.rs,ffi.rs}`; `crates/thinkthen/src/cli/{args.rs,args/command.rs,asking.rs,asking/batched.rs,asking/plan.rs,find.rs,annotate.rs,annotate/plan.rs,recognize.rs,recognize/dry_run.rs,relate.rs,relate/dry_run.rs,check.rs,failure.rs}`; `crates/thinkthen/tests/settings_cases.rs`, focused CLI tests, and spec/, specification/, demos and probes named above. Current nonblank counts/cap: `core/batch.rs` 458/500, `engine/facade.rs` 468/500, `engine/prepared_request.rs` 177/500, `core/plan_document.rs` 127/500. Measure again and extract only cohesive pure code if headroom requires it. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Run V1–V11 and I1–I10 from the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) through one pure core parser test; add one C engine-schema conversion edge. Use a multi-cut input whose **independently recorded wire bodies** establish full-input records, planned requests and exact summed body bytes; assert the token band's fixed-rate rounding and unchanged first body. Include recognize/relate marked upper bounds and an invalid later record that a first-record preview would miss. A loopback listener must accept **zero** requests for invalid settings and every `--plan` route; pin exit codes/refusal sentences. Keep `cache prune --dry-run`. Run changed executable pages/demo blocks/probe self-tests with a matched binary, measure ratchets and focused format/policy/pages/tickets/diff. No broad gate or provider run belongs to preparation. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) separate installed and release qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Pure settings parser, validated engine schema and one full-input plan summary reused by all CLI adapters; 0289 activates the cap.
- Proof: Core V/I table, multi-cut exact bodies/bytes/token band, later-record refusal, staged upper bounds, zero loopback accepts and changed executable pages.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
