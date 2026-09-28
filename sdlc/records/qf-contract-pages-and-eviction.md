# Quick Fix: align answer pages and name cache eviction order

Status: candidate for fresh independent review. Branch: `ticket/qf-contract-pages-and-eviction`, based on `origin/main` at `106d3e4c`. Scope: experiment 284 registers 86 and 97. No runtime, schema, or site source changed.

## Findings on current main

Register 86 names six disagreements. Three were already resolved and need no manufactured edit:

1. `specification/threshold.md` already lists `tag`, `recognize`, `recognize --relation-threshold`, and `relate`, with the quantity each cuts. The earlier answer-ties Quick Fix landed this table. This instance is a non-issue on current main.
2. Every detailed JSON example in `specification/result.md` already includes `meta.failed_questions`; its metadata table says the field is always present. This instance is a non-issue on current main.
3. `specification/audit.md` already calls the public state `unsure`, including the count and table wording. `crates/thinkthen/src/core/measure/answer.rs::Said::text` prints `unsure`; `core/measure/audit.rs` serializes its internal `unresolved` field as `unsure`. This instance is a non-issue on current main.

The other three were page defects. `specification/README.md` still marked `result.md` as having one open point, although that page declares the settled result contract and identifies no open point. `specification/choose.md` explained ties through alphabetical order even though `crates/thinkthen/src/core/answer.rs::Distribution::leader` keeps the first highest-probability label in supplied order; `core/answer_tests.rs::the_leader_is_the_first_of_a_tie_and_a_tie_is_still_unresolved` pins the detail leader and unresolved value. `specification/question-file.md` said every detailed row has `meta.question_sha256`, but `core/result.rs::AnnotateMeta` and the `specification/result.md` command table give annotate its `meta.questions_sha256` set digest. The page's later set-digest section was already correct.

Register 97 is a page defect. `crates/thinkthen/src/engine/cache_prune.rs::Plan::selected` and `plan` sort and select by entry `modified` time and digest name on ties. `cache_prune/scan.rs::read_entry` reads that time from file metadata. Cache-hit paths in `engine/recorder.rs` read and return an existing response without touching its modification time. `specification/recording.md` already gave the selection order but did not say that use does not refresh it or what a future recency policy would require.

## Change and retained behavior

`specification/README.md` removes the stale open-point status. `specification/choose.md` now names user order and says it cannot resolve a probability tie; `answer.pick` still names the first tied option while the value remains not sure. `specification/question-file.md` scopes the singular digest claim to single-question rows and names annotate's plural set digest. `specification/recording.md` explains that the tool's last write or repair sets modification time, cache hits leave it alone, and a recency policy would need persisted last-hit times. The policy remains oldest modification time, with digest-name tie breaks. No behavior or prior pending batch contract changed.

## Checks and closure recommendation

The edits are prose only. Source and existing tests above supply the relevant tie, digest, and prune evidence; no provider, build, new runtime test, or full suite is needed. `python3 sdlc/scripts/pages` reported 1 coming and 21 green; `python3 sdlc/scripts/tickets` reported 0 evidence failures; `git diff --check` was clean. The changed pages and this record were reread together. No executable demo page with a word limit changed.

Recommend closing register 86 after review and landing: three named instances were already fixed, and this Quick Fix corrects the other three. Recommend closing register 97 after review and landing: it asked for an eviction policy note, not an LRU implementation. The plan and register remain for the coordinator to update.

## What the build taught us

The register preserved historical defects after some pages had moved on. Checking all six against one current main avoided rewriting examples and vocabulary that were already correct. A tie has both a detailed leader and an unresolved public value; the leader follows supplied order without granting that order decision authority. A plural annotate digest identifies a resolved set, so a blanket singular-digest sentence fails even while the canonical digest explanation remains valid. Modification time is a cheap deterministic eviction input, but it measures writes and repairs rather than cache usefulness. A true recency policy needs new hit state, not a prose rename.
