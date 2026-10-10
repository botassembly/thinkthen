# 0510: Keep only ranked records at or above a probability cutoff

Status: COMPLETE.

Milestone: 0.2

Depends on: 0475
Depends on: 0476

Reviews: revision 1d15f4c6f, accept

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision a087f6dc3, accept

Reviews: revision ac60aee9d21141d4961ab7ee9027da2fc031ebdf, accept

Reviews: revision 0011f905b3632815c2044050839f4a8c4be5c85d, accept

Landed: 1cc8564

## Outcome

Callers can use `rank --threshold P` to keep records whose yes probability is at least P, in the existing ranked order. The shared Request carries the same optional cutoff, so the Rust API, MCP and every migrated language get it without their own cutoff logic. Omitting it preserves current rank behavior. The cutoff reads stored probabilities and makes no extra model call.

## Evidence

- Starts from: Ian's 2026-10-09 approval in `inbox/thinkthen/2026-10-09-pm-thinkthen-ian-approves-rank-threshold-for-0-2.md` and the retained [2026-10-01 issue](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md).
- Keeps: The ten functions, default uncut rank, stable ties, original records and positions, observations, existing model questions, request counts and plain rank's cache and replay answers. The cutoff is a reading, not question wording or routing.
- Changes: Amend `specification/rank.md`, `specification/threshold.md` and ADR 0007's no-selection consequence. Admit one optional inclusive cutoff with the domain `0 < P <= 1` and reject bands. Apply it to plain and saved decide rank. Filter below-cut records before top; top keeps the eligible order. Reject a supplied or authored cutoff on saved-score and question-set routes before any send, and keep their uncut behavior. A weighted score is not a probability. Admit the cutoff in Request options and native admission, generate its request schema with the existing generation, and let CLI and MCP forward it. Claim:
  - Core and CLI: `crates/thinkthen/src/core/order.rs`, `crates/thinkthen/src/core/question_file.rs`, `crates/thinkthen/src/cli/args.rs`, `crates/thinkthen/src/cli/asked/rank.rs`, `crates/thinkthen/src/cli/schedule.rs`, `crates/thinkthen/src/cli/request.rs`.
  - Public and Request: `crates/thinkthen/src/public/bulk/complete.rs`, `crates/thinkthen/src/public/complete/rank.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/options.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/execution.rs`, `crates/thinkthen/src/public/request/definition.rs`, `crates/thinkthen/src/public/request/tests.rs`, `specification/request.schema.json`.
  - MCP: `crates/thinkthen/src/mcp/request.rs`, `crates/thinkthen/src/mcp/tests/execution.rs`.
  - Tests and documents: `crates/thinkthen/tests/backend/keeping/rank_top.rs`, `crates/thinkthen/tests/backend/native_results/rank.rs`, `crates/thinkthen/tests/public_batches/ranks.rs`, `crates/thinkthen/tests/native_complete/aggregates.rs`, `specification/rank.md`, `specification/threshold.md`, `sdlc/planning/adr/0007-flat-verbs-bare-values-and-one-threshold.md`.
  Confirm the smallest public reading seam before coding, and change claims if it needs other files.
  Review confirms these additional owning paths: `crates/thinkthen/src/cli/args/command.rs`, `crates/thinkthen/src/cli/judge.rs`, `crates/thinkthen/src/core/mod.rs`, `crates/thinkthen/src/core/question_file/resolve.rs`, `crates/thinkthen/src/public/complete.rs`, `crates/thinkthen/src/public/question.rs`, `crates/thinkthen/src/public/rank_question.rs`, `crates/thinkthen/tests/backend/keeping/graded_rank.rs`, `crates/thinkthen/tests/backend/rank_set.rs`, `crates/thinkthen/tests/backend/refused.rs`, `crates/thinkthen/tests/backend/threshold_args.rs`, `crates/thinkthen/tests/native_rank_question.rs`, `demos/06-top-search-hits/README.md`, `demos/13-pick-a-threshold/README.md`, `sdlc/ratchet.json`, `specification/question-file.md` and `specification/settings.md`.
- Proof: Outside-in CLI, Rust and MCP cases keep an exactly-at-cut record, drop below-cut records, preserve probability order and stable ties, compose with top, and reject invalid cutoffs and refused routes before sends. Run plain rank first, then the cut reading against the same cache or recording, and count zero extra model requests with the same question and cache identities. With no cutoff, existing literal output and error cases stay unchanged.
- Defers: A new function, a grep alias, calibration, extra model calls and proxy business policy need no ticket. Release management stays with Ian.

## Progress

- 2026-10-09 started
- 2026-10-09 landed 1cc856481d5383afe888d385a4d14f914f213363; next: Inclusive rank cutoff is reviewed and landed. Run combined functional checks before closing; cache reuse and refused routes pass focused CLI, native and MCP cases.
- 2026-10-09 landed 6800aaca5; next: Rank cutoff and the corrected saved-band diagnostic are landed. The full default Rust suite passes; finish the combined library-only and downstream checks before whole closure.
