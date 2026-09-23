---
flow: build
priority: 81
opens: crates/thinkthen/src/core crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/tests conformance specification spec demos sdlc/ratchet.json sdlc/planning
---

# 0081: Build `relate` over the shared relation planner

Status: design remediation complete; Option A ruled by Ian; ready for Sol re-review.

## Outcome

Add `thinkthen relate`. It reads a complete entity set, asks bounded relation questions, and emits self-contained edges. A standalone edge has only `relation`, `source.name`, `source.kind`, `target.name`, `target.kind`, and `probability`. Recognition keeps its existing endpoint offsets and strength. Both commands use one planner, question map, threshold rule, and edge assembler.

The bare edge is:

```json
{"relation":"sung_by","source":{"name":"Octopus's Garden","kind":"song"},"target":{"name":"Ringo Starr","kind":"person"},"probability":0.93}
```

`--details` uses the ruled Option A shape below: ordered self-contained entries under `answer.questions`, while `value` contains accepted edges only.

## Current facts

- `crates/thinkthen/src/core/relation.rs` currently treats rule `*` as “match every entity” and returns the pair plan before concrete-kind expansion when `source == target`. Existing wildcard tests use one entity per kind and do not expose the gap.
- The current pair planner puts `reads` and full entity references in every `Question::Decide` text. The final method-H ruling requires one numbered entity table and one relation wording copy in request state, with pair-only H questions.
- `RelationEdge` is currently tied to `RecognizedName`. Relate needs name-and-kind endpoints without copying the planner or edge assembler.

Ticket 0081 owns these shared corrections and must prove recognition compatibility. Ticket 0079 still owns request splitting, request identity, ordering, replay, cache behavior, and cancellation.

## Fixed command contract

`thinkthen relate [OPTIONS] RELATION...` and `thinkthen relate [OPTIONS] @entities.json` are the command forms. A one-way rule is `NAME=SOURCE_KIND:TARGET_KIND`; bare `NAME` means `NAME=*:*`; `--either` asks one unordered relation. A question file may provide `reads`. The command exposes no method, one-or-many, runner-up, or packing control. The threshold defaults to `0.5` and accepts the cut.

JSONL, CSV, and TSV input use `/name` and `/kind` with existing field overrides. Without a kind field, records receive synthetic kind `*`, and a rule naming a concrete kind fails locally. Both selected values are nonempty. Input order is stable. The same name with different kinds is two entities. An exact duplicate name and kind is a usage error. More than 255 local entities is a usage error. Empty input succeeds without questions or edges after syntax validation.

`--lines` assigns synthetic kind `*` to each nonempty line and accepts only a bare rule or `*:*`. It remains one same-kind set and does not expand. `--dry-run` sends nothing and reports the planned counts and fallback facts without token or price claims. Bare output emits one deterministic JSON edge per line, keeps edges at or above the inclusive cut, emits no duplicate, and preserves completed edges when a later logical question fails. A failed question never creates an edge. Existing exit codes remain in force, including 0 for a completed run with no edges.

## Shared entity and edge owner

`core/relation` owns one validated `RelationEntity { name, kind }`, one `RelationEntityView { name(), kind() }`, and one generic `RelationEdge<E> { relation, source: E, target: E, probability }`. The planner stays index-based and generic over the view. The assembler keeps the single threshold comparison, direction normalization, self exclusion, and mapping interpretation.

`RelationEntity` serializes only `name` and `kind`. `RecognizedName` implements the view and keeps offsets and strength. The relate reader constructs `RelationEntity`; its renderer serializes `RelationEdge<RelationEntity>`. Recognition serializes `RelationEdge<RecognizedName>` through its current result path. No second planner, mapping table, threshold comparison, edge type, or edge serializer is allowed.

Migration adds the standalone entity and view, generalizes the existing edge and assembler, adapts recognition, then calls the same path from relate. Tests use both endpoint types with the same mappings, prove relate omits `start`, `end`, and `strength`, prove recognition retains them, and prove equal names with different kinds remain distinct.

## Shared planner corrections

The planner scans entities once and records concrete kinds in first-seen order. A concrete rule side expands to itself. A rule `*` expands to all admitted concrete kinds in that order. It plans each expanded kind pair separately, keeps rule/kind/entity order, and excludes self-pairs. `--either` removes reverse duplicates by first-seen order; one-way `*:*` keeps both directions and same-kind one-way rules keep ordered pairs.

Same-kind pairs use H. Both-way rules ask one unordered yes/no per pair. One-way rules ask one yes/no for each ordered direction. Different-kind pairs ask from the larger side over the smaller side plus `none`; equal sides ask from the declared source side. Every non-`none` option at or above the cut becomes an edge.

If one choice would exceed 255 wire options including `none`, or an explicit profile request-byte limit, that concrete relation falls back to H. Other expanded relations keep their own method. The preflight sends nothing when a plan cannot fit. The line synthetic kind is the one-set exception.

H request state carries one numbered entity table and one copy of the relation's `reads` wording. Each H question carries only a pair statement such as `Does this hold: Item 1 and Item 2?`. Recognition keeps its original source text byte-for-byte as source context in this state; non-relation evidence remains unchanged. Request identity, recording, replay, and profile sizing use the final encoded state and questions.

Required regressions cover wildcard expansion, one-sided wildcards, `*:*`, line input, direction, duplicate suppression, H state wording, and the 255/profile boundaries. A compiled recognition proof checks source text, one-copy wording, pair-only H questions, request order, offsets, and strength. The prior instruction forbidding shared regression tests is removed; existing 0080 tests remain.

## Detailed-result ruling

Details must show mixed choice and H answers, accepted candidates, rejected candidates, failed questions, request identity, and safe failure data. Ian ruled Option A on 2026-09-23 because it matches `recognize --details`, keeps each question self-contained and ordered, and supports run comparison through each entry's request digest without making public question ids permanent.

### Option A: ordered question entries (ruled)

Keep `value` as the accepted edge array and add `answer.questions` in logical construction order. A successful choice entry is:

```json
{"request":"<digest>","relation":"works_for","method":"choice","asker":{"name":"Ada","kind":"person"},"candidates":[{"entity":{"name":"Acme","kind":"organization"},"probability":0.84,"accepted":true},{"entity":{"name":"Other","kind":"organization"},"probability":0.10,"accepted":false},{"none":true,"probability":0.06,"accepted":false}],"pick":{"name":"Acme","kind":"organization"}}
```

An H entry uses `method: "yes_no"`, `source`, `target`, `probability`, and `accepted`; a rejected H entry sets `accepted` to false. A failed entry has the request and question identity plus the existing structured `failure` object and has no probability or accepted candidate. `meta.failed_questions` keeps its existing count and includes each failed entry. Cost: about 200 production and 300 test lines, plus a result specification update.

### Option B: keyed standard answer entries

Keep `value` as the accepted edge array and add `answers` keyed by stable ids such as `r1-q1`. Each entry reuses standard `question` and `answer` fields, adds accepted and rejected candidate lists, and carries either probability data or `failure`. Cost: about 150 production and 250 test lines; public ids become part of the contract.

### Option C: separate audit and failure arrays

Keep `value` as the accepted edge array and add `audit` and `failed_questions` arrays. Audit rows carry request digest, method, entities, all candidate probabilities, and acceptance; failure rows carry digest, question identity, and `failure`. Cost: about 100 production and 220 test lines; callers must join arrays and can lose ordering.

Option A is the public shape. `value` contains accepted edges only. Exact JSON tests cover one choice entry, one yes/no entry, one rejected entry, and one failed entry.

## Scope and exclusions

Scope includes the shared entity/view and generic edge migration; wildcard and H-state corrections; relate input, syntax, field overrides, 255 guard, dry-run, help, specification, and replay-only how-to; calls through the shared planner and 0079 path; the selected detailed serializer; offline fixtures; the focused recognition regression; and focused policy, format, ratchet, test, and whitespace evidence.

Exclusions include a second planner, edge assembler, threshold rule, or splitter; unrelated recognize behavior; public Rust, C, language, library, database, or `surfaces` APIs; new-row input; runner-up questions; one-to-many controls; method flags; unmeasured Jev byte constants; credentials; production data; retained artifacts; live calls; and paid calls.

## Acceptance

- The command and input rules above pass focused no-send, ordering, duplicate, empty-input, wildcard, direction, threshold, failure, and line-mode tests.
- Shared planner tests prove first-seen wildcard expansion, hybrid routing, no self-pairs, no reverse duplicates, H state wording, and exact 255/profile fallback boundaries. Existing 0080 tests remain.
- Generic endpoint tests prove name-and-kind-only relate output and unchanged recognition offsets and strength.
- The selected detailed result shows accepted and rejected candidates, failed questions, request identity, and safe failure data in the exact chosen shape.
- Offline fixtures and fake backends cover replay, partial failure, secrecy, dry-run, and impossible-plan zero sends. No credential appears in output, logs, hashes, fixtures, or records.
- Each changed Rust file stays below 500 nonblank lines. The exact ratchet, policy, formatting, focused tests, specification cases, and `git diff --check` pass before Sol review. No live or paid call runs.

## Budget

Budgets are deltas from landed 0080. Its 15 production files, 8 test-only files, and 2,304 gross lines are not counted again.

| Owner and actual tree paths | Production files | Test-only files | Production lines | Test lines |
| --- | ---: | ---: | ---: | ---: |
| Shared relation entity/planner/state: `core/relation.rs`, `core/plan.rs`, `core/mod.rs`, one state split if needed | 4 | 3 | 360 | 320 |
| Recognition/request proof: `core/recognize.rs`, `cli/recognize.rs`, `core/adapters/systemone/request.rs` | 3 | 2 | 220 | 260 |
| Relate command: `cli/args.rs`, `cli/args/command.rs`, `cli/mod.rs`, new relate module | 4 | 3 | 520 | 420 |
| Selected result/spec owner: one new private result owner and its fixtures | 1 | 1 | 180 | 180 |
| **Maximum** | **12** | **9** | **1,100** | **900** |

The gross added Rust budget is 2,000 nonblank lines. The implementation records actual files and counts before Sol review. It splits an existing 0080 file before 500 lines. A budget increase requires a new ticket decision.

## Dependencies and review

Dependencies are landed 0079 request scheduling and landed 0080 shared relation planning. No dependency is added.

Contract 1; state and timing 2; reach 1; proof 2; cost of error 1; total 7; final level 3; Luna Max owns implementation design and code; Sol High independently reviews the design and implementation.

Sol rejected the first design for four substantive gaps: no shared name-and-kind endpoint owner; false wildcard and H-wording claims; no exact mixed detailed-result shape; and stale budgets that forbade required recognition regressions.

This remediation assigns the shared owner, scopes both planner corrections, requires recognition proof, re-estimates budgets, and records Ian's Option A ruling. The design is ready for Sol re-review. No product code, surface file, live call, paid call, implementation gate, targeted Sol repair, reopened defect, or elapsed start-to-accept time exists for this pass. The full trial record is `sdlc/records/0081-build-relate.md`.
