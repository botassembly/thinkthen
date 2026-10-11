---
flow: build
priority: 81
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli/recognize crates/thinkthen/tests specification/recognize.md sdlc/ratchet.json sdlc/planning
---

# 0081: Build the shared relation foundation

Status: COMPLETE.

Opened as: 2026-10-11. on remote main at merge `4229bbfa`.

## Outcome

Make the relation implementation landed by 0080 genuinely reusable before a public `relate` command exists. The pure core owns a generic entity view, a standalone `RelationEntity`, a generic self-contained edge, concrete wildcard expansion, one question map and assembler, and the exact typed relation request state. Recognition moves onto that foundation without changing its public JSON or non-relation requests.

Ticket 0081 adds no `relate` command, argument, help, `relate` specification page, detailed result, dry-run report, framing path, or how-to. It updates only the landed recognition specification for the shared behavior it changes. Ticket 0088 consumes this foundation and owns the public relate surface. The complete settled contract remains in `sdlc/planning/relate-design.md`.

## Current facts

- Landed 0080 binds planning and `RelationEdge` directly to `RecognizedName`. Wildcards match entities directly instead of expanding one rule into ordered concrete-kind plans.
- Same-kind pair questions repeat names, kinds, and relation wording in instructions. Relation requests use the original text state rather than the ruled typed relation state.
- The fixed 255-option fallback exists. Profile `max_options`, exact request-byte fallback, final H preflight, and the boundary between split-only and fallback limits remain spread across recognition and generic request preparation.
- These defects can be proved through recognition and pure-core tests. They do not require a public `relate` parser or renderer.

## Scope

`core/relation` owns `RelationEntity { name, kind }`, `RelationEntityView`, `RelationEdge<E>`, ordered concrete wildcard expansion, method selection, mappings, assembly, and relation state. `RecognizedName` implements the view. Recognition returns `RelationEdge<RecognizedName>` with its current complete endpoints.

Wildcard sides expand to admitted concrete kinds in first-seen entity order. Planning runs once per expanded concrete pair in rule, kind, and entity order. The synthetic line kind remains reserved for 0088. Same-kind plans use H. Different-kind plans use choice from the larger side over the smaller side plus `none`; equal counts ask from the declared source side. Either rules normalize to input order. Directed same-kind and `*:*` plans retain both legal directions. The assembler alone applies the inclusive cut, excludes self-edges, interprets mappings, and normalizes direction.

Every relation request uses the exact state and H instructions in `relate-design.md`. Recognition includes its unchanged normalized source under `evidence`; the state then contains every entity with stable `i1`-based ids and one expanded concrete relation. H sends no criteria. Directed H asks `Does the relation hold from iN to iM?`; either H asks `Does the relation hold between iN and iM?`.

The effective choice ceiling is `min(255, backend_profile.max_options)` when `--profile FILE` supplies that runtime limit. Equality stays choice and one over changes only that expanded concrete relation to H. `max_questions` splits without fallback. `max_request_bytes` first permits 0079 splitting and falls back only when one complete choice question for that concrete relation cannot fit alone. Sibling concrete relations expanded from the same wildcard rule keep their independently selected method. `max_evidence_bytes` refuses without fallback. The complete H replacement is encoded and preflighted again; an impossible H plan refuses before replay, cache, key access, or a send.

Relation request bytes, recording digests, and cache identities intentionally change. Non-relation request bytes and identities do not change. Recognition's public entity and edge JSON, offsets, strength, ordering, aggregate metadata, failure behavior, and command grammar remain unchanged.

## Exclusions

No public `relate` command or file grammar; no standalone entity input; no Option A result; no partial-output exit handling; no relate dry run, help, specification, replay page, secrecy route, or fixture; no public Rust or other surface; no second planner, assembler, threshold, splitter, scheduler, or request encoder; no dependency, credential, live call, or paid call.

## Owners and budget

Production owners are `core/relation.rs` and behavior-local submodules, `core/mod.rs`, and a split `cli/recognize/relation.rs` called by `cli/recognize.rs`. `core/text.rs`, `core/backend_profile.rs`, `engine/prepared_request.rs`, and the generic System One encoder may change only if the exact state or fallback cannot use their existing typed seams. Documentation ownership includes `specification/recognize.md`, whose relation section must say fallback applies independently to each concrete relation after wildcard expansion. Test owners are relation unit tests plus the existing backend recognize, profile, cache-identity, recording/replay, and refusal suites.

Change or add at most 10 production Rust files and 7 test-only Rust files. Add at most 800 nonblank production Rust lines and 700 nonblank test Rust lines, 1,500 gross. Keep every Rust file at or below 500 nonblank lines and add no dependency. Split the 491-line `cli/recognize.rs` before adding relation behavior. Do not add the result to 499-line `core/result.rs`, add relation logic to 458-line request encoding, or widen generic `Plan` when a behavior-local owner suffices. The implementation record lists actual files and gross additions and explains any variance before code review.

## Acceptance gates

1. An independent Sol reviewer returns `ACCEPT` on this ticket and the 0081 sections of `relate-design.md` before product code starts.
2. Red then green: `cargo test --locked -p thinkthen --lib relation` pins generic ownership, concrete wildcard order, same-kind and cross-kind selection, direction, self exclusion, inclusive cuts, exact 255/backend-profile boundaries, and all exact relation-state and H bytes.
3. Red then green: `cargo test --locked -p thinkthen --test backend recognize` pins public recognize JSON compatibility, complete source preservation, exact request order, backend-profile equality and one-over fallback, splittable choice bytes, unsplittable choice fallback, impossible final H with zero sends, changed relation request digests, replay/cache identity, and one unchanged non-relation request body. Two mixed wildcard cases prove exact per-concrete behavior: an option limit makes one expanded relation H while a sibling remains choice, and an unsplittable one-choice byte limit does the same. The requests and edges remain in concrete expansion order.
4. Red then green: `cargo test --locked -p thinkthen --test backend refusals` counts zero loopback requests for every new local relation refusal and proves refusal occurs before key access where applicable.
5. Independent Sol code review checks that one generic planner, mapping, assembler, edge type, fallback decision, and exact state serve recognition and future relate. It also checks budgets, request identity, cache/replay behavior, and unchanged public recognition output.
6. The coordinator runs `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially from the exact candidate revision with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and no competing Rust build, then runs `git diff --check`. Every command exits 0. No live or paid call runs.

## Dependencies and route

Landed tickets 0079 and 0080 are the only dependencies. Ticket 0088 depends on landed 0081.

Contract 1; state and timing 1; reach 1; proof 2; cost of error 1; total 6; final level 2. Exact-byte identity, per-concrete backend-profile fallback, and recognition compatibility set the proof score. Ian ended the Luna trial after two 0081 remediation passes and ordered Sol Medium to drive the redesigned 0081 and 0088. A separate Sol reviewer remains independent. Stop and re-score if implementation needs a public surface, changed non-relation bytes, a second scheduler or encoder, a dependency, or more than this budget.
