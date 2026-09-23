---
flow: build
priority: 81
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance specification spec demos sdlc/ratchet.json sdlc/planning
---

# 0081: Build `relate` over the shared relation planner

Status: second design remediation complete; Ian ruled partial output at exit 6; ready for Sol re-review.

## Outcome

Add `thinkthen relate`. It reads one complete entity set, asks bounded relation questions through the shared planner, and emits self-contained edges. Standalone endpoints contain only `name` and `kind`; recognition endpoints retain offsets and strength. Bare output is one edge per line. Detailed output uses Ian's ruled Option A: one `thinkthen.result/1` object whose `value` is the accepted edge array and whose `answer.questions` is the ordered audit of every successful or recoverably failed logical relation question.

The exact current contract is `sdlc/planning/relate-design.md`. Product code may start only after Sol accepts the amended design.

## Current facts from the second Sol rejection

- The first remediation gave only one successful choice example. It did not define the outer result, reversed choice asking, directed and either H entries, or failed choice and H entries.
- It conflated the fixed 255-option ceiling with profile `max_options`, did not state exact-limit behavior, and did not separate split limits from fallback limits.
- It contradicted the ruled structured field defaults by assigning synthetic kind `*` when no override was present.
- It described H state conceptually without fixing its JSON keys, order, entity ids, concrete wildcard expansion, recognize evidence field, or exact H instructions.
- It underestimated work around files already at 491, 497, 499, and 500 nonblank lines and omitted the full gate and shared secrecy matrix.
- It promised partial output without settling the outward-facing exit code.

## Fixed input mapping

The command forms are `thinkthen relate [OPTIONS] RELATION...` and `thinkthen relate [OPTIONS] @links.json`. `@links.json` is a relation question file; entities still come from standard input or `--input FILE`.

JSONL, CSV, and TSV records default to name pointer `/name` and kind pointer `/kind`. Exactly one `--field POINTER` may replace the name pointer, and exactly one `--kind-field POINTER` may replace the kind pointer. Both pointers resolve independently against the original parsed record. CSV and TSV headers form the object they address. Each selected value must be a nonempty JSON string. A missing pointer, another JSON type, blank value, duplicate name-and-kind identity, absent concrete rule kind, or 256th entity refuses the complete set at exit 2 before any request.

`--lines` refuses both field options, uses the complete nonempty line as `name`, assigns synthetic kind `*`, and accepts only bare or `*:*` rules. Structured input never receives a synthetic kind. Empty structured or line input succeeds after rule syntax validation with no request and no edge.

The resolved detailed `question.fields` is `{"name":NAME_POINTER,"kind":KIND_POINTER}` for structured input and `null` for lines. Field mapping participates in `question_sha256`; entities do not.

## Shared planner and request corrections

`core/relation` owns one `RelationEntity { name, kind }`, one `RelationEntityView`, one generic `RelationEdge<E>`, wildcard expansion, question mappings, and assembly. `RecognizedName` implements the view. Relate uses `RelationEdge<RelationEntity>` and recognition uses `RelationEdge<RecognizedName>`. No second planner, mapping table, threshold comparison, edge type, or edge serializer is allowed.

Rule wildcards expand to admitted concrete kinds in first-seen order before method selection. Same-kind concrete relations use H. Different-kind concrete relations use choice from the larger side over the smaller side plus `none`; equal sides ask from the declared source side. `--either` removes reverse duplicates. One-way same-kind relations retain ordered directions. The line synthetic kind remains one same-kind set.

The fixed choice ceiling is 255 total options including `none`. A selected profile lowers the effective ceiling to `min(255, max_options)`. Exact equality passes; one over changes that whole concrete relation to H. `max_questions` only splits. `max_request_bytes` first splits through ticket 0079 and causes H fallback only when one choice question with complete state cannot fit alone. `max_evidence_bytes` never falls back. The final H plan is preflighted again; an impossible H request exits 2 with zero sends.

Every relation request uses the exact typed state fixed in `relate-design.md`. For recognize it carries the original normalized source text under `evidence`; standalone relate omits that key. It then carries all entities as stable `i1`-based ids and one concrete relation object. Directed H asks exactly `Does the relation hold from iN to iM?`; either H asks exactly `Does the relation hold between iN and iM?`. H sends no criteria and repeats no name, kind, or `reads` text in instructions. Non-relation request bytes remain unchanged.

## Ruled Option A detailed result

The outer object, resolved question, successful source-asking and target-asking choice entries, directed and either H entries, rejected candidates, and failed choice and H entries are exact in `relate-design.md`. The public direction values are `source_to_target` and `either`. A choice entry also carries `asker.role`, so a target-side question never reverses the relation edge.

Candidate `accepted` means that candidate produces an edge at the selected inclusive cut. `none` is always `accepted: false`. `pick` names the first highest-probability wire option before the cut. Failed entries preserve question identity and request digest but omit probabilities, `accepted`, and `pick`. `meta.failed_questions` is always present. `meta.requests`, sends, usage, cache truth, and model agreement follow existing aggregate rules.

## Ian ruling: partial-failure exit behavior

This decision applies only when decoding produced at least one valid logical relation answer and at least one recoverable failed logical relation answer. A reply with no valid answer remains exit 4. Transport, status, replay, local, output, cancellation, and defect failures retain their existing codes and never become partial success.

Ian chose exit 6 with partial output on 2026-09-23. Buffer the aggregate until all relation requests finish. Emit successful bare edges, or the complete Option A object with failed entries, and exit 6 when any recoverable logical question failed. This matches `annotate`, preserves paid valid answers, and tells bare-output callers that the graph is incomplete.

Exit 4 with no output and exit 0 with partial output are rejected alternatives. A reply with no valid logical answer remains exit 4. Transport, status, replay, local, output, cancellation, and defect failures retain their existing codes.

## Scope and exclusions

Scope includes the shared entity/view and generic edge migration; exact relation state; wildcard and fallback corrections; relation question-file parsing and digest; entity input and field mapping; relate command, result, dry-run, help, specification, and replay-only how-to; 0079 request execution; the ruled Option A serializer; offline fixtures; focused recognition compatibility; full shared secrecy/refusal coverage; exact ratchet; and durable review records.

Exclusions include a second planner, edge assembler, threshold rule, scheduler, or splitter; unrelated recognize behavior; public Rust, C, language, library, database, or `surfaces` APIs; incremental or two-set input; runner-up questions; one-to-many controls; method flags; an unmeasured Jev byte constant; dependencies; credentials; production data; retained test artifacts; live calls; and paid calls.

## Acceptance

- Focused red/green tests pin structured and line field mapping, complete-set validation, wildcard expansion, method routing, direction, duplicate suppression, state bytes, Option A JSON, inclusive thresholds, partial output at exit 6, and exact 255/profile boundaries.
- Profile tests prove 255 passes without a lower profile, 256 falls back, `max_options = N` passes at N and falls back at N+1, `max_questions` splits without fallback, a splittable byte overflow stays choice, an unsplittable one-choice byte overflow falls back, and impossible H or evidence limits send zero requests.
- Compiled recognize regressions pin unchanged endpoint JSON, source text under relation state, exact H instructions, request order, fallback behavior, and non-relation request bytes.
- The shared secrecy route matrix adds relate bare/details, lines/JSONL/CSV/TSV, success, dry-run, default and explicit cache, record, replay, replay miss, missing key, transport/status/decode and mixed logical failures, profile and field refusals, hostile and damaged recordings, storage failure, and the authorization-header-only key path. It inspects stdout, stderr, every `Debug` value, request bodies, recording/cache files, and fixtures. Relate gets no smaller command-only secrecy substitute.
- The shared refusal sweep pins exact safe messages and loopback request count zero for malformed rules/entities/pointers, duplicate identities, absent kinds, 256 entities, impossible profiles, dry-run, replay miss before key access where applicable, and every other local no-send path.
- After implementation and independent code review, the coordinator runs `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset and no competing Rust build. The exact ratchet, per-file limit, policy, formatting, Clippy, package checks, all offline tests, replay checks, executable how-tos, and `git diff --check` must pass. No live or paid call runs.

## Budget

These are gross additions from landed 0080 and include Option A, the shared corrections, mandatory near-limit file splits, and full secrecy proof. They do not count 0080's 2,304 lines again.

- Change or add at most 18 production Rust files and 12 test-only Rust files.
- Add at most 1,650 nonblank production Rust lines and 1,550 nonblank test Rust lines, for 3,200 gross nonblank Rust lines.
- Keep every Rust file at or below 500 nonblank lines and add no dependency.
- Split `cli/recognize.rs` before relation migration, `cli/failure.rs` before relate failures, and `tests/backend/secrecy.rs` before adding the relate matrix. Do not add the relate result to 499-line `core/result.rs`; give it a private relate owner. Prefer a new relate args owner over filling `cli/args.rs`, and keep relation state beside `core/relation` rather than widening generic `Plan` or the request encoder.
- Expected owners are `core/relation.rs` plus relation submodules, `core/recognize.rs`, `core/digest.rs`, `core/mod.rs`, relate question-file/config owners, the recognize relation split, relate args/command/input/result/dry-run/failure modules, and shared backend secrecy/refusal/profile/result tests. The implementation record must list actual counts and explain every variance before Sol code review.

## Dependencies, complexity, and review

Dependencies are landed 0079 request scheduling and landed 0080 relation planning. No dependency is added.

Contract 2; state and timing 2; reach 1; proof 2; cost of error 1; total 8; minimum level 3 because partial failure, cache/replay identity, and secrecy interact; final level 3; Luna Max owns design, implementation, and remediation; Sol High independently reviews design and code.

Sol has rejected two design rounds. This is Luna remediation pass 2, the trial limit. The second rejection required the corrections above. Ian ruled partial output at exit 6, and the design is ready for Sol re-review. No product code, surface file, live call, paid call, implementation gate, targeted Sol repair, reopened implementation defect, or trustworthy elapsed start-to-accept time exists. `sdlc/records/0081-build-relate.md` holds the concise trial record.
