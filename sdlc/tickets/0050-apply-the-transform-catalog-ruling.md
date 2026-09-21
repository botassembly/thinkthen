---
flow: build
priority: 39
opens: sdlc/planning specification/roadmap.md transforms/README.md
---

# 0050: Apply the transform catalog ruling

Status: landed

## Outcome

The completed transform stage records that ADR 0015's trigger has fired. Three transform families now exist, so the release will carry them through the accepted read-only `thinkthen transform list` and `thinkthen transform show NAME` surface. The evidence-based agent decision declines the `report` verb, and Ian can overturn it. The catalog lands during release preparation, after the one-crate move fixes its package location.

## Current facts and decisions

ADR 0010 named five possible outcomes for `report`. Ian accepted the fifth in ADR 0015 once three transform families existed: a general way to carry transforms inside the tool. They now exist as whole-run metrics, a row policy, and a reviewed-action monitor. Ten `.jq` files total 1,408 lines. The transform index contains seven executable Bash blocks, and nineteen green how-tos run them without a model call. Comparison and human-label sweeps have awkward invocations, but their output contracts work as files. A `report` command would duplicate several distinct reports and their validation.

This ticket applies the accepted catalog ruling and records the agent's evidence-based decision to decline `report`. Ian can overturn either the report decision or the implementation timing.

1. Outcome 5 supplies the distribution mechanism. The release binary will list and print named transforms. It will never start `jq`, interpret a transform, manage user files, or combine reports.
2. The separate `report` verb is declined by the agent's evidence-based decision, and Ian can overturn it. The transforms remain executable files for `jq`; the catalog makes selected files available to an installed user without copying repository paths.
3. The release implementation ticket will choose the catalog membership, public names, byte identity, and embedded location against the final package tree. This ticket does not turn every file now under `transforms/` into a compatibility promise.
4. The catalog implementation waits until release preparation, after ADR 0017's one-crate move. The one published crate must own the embedded files. Moving or duplicating them now would create packaging work that the accepted merge immediately moves again.
5. The transform stage is complete after this record. ADR 0017 has already been rewritten and accepted. The next order is to record and address build-team findings in ADR 0017, land Job 3's conformance cases, prove Job 2's DuckDB interrupt inside Python, then take section 8 step 1: fold to one crate and move machinery into the private engine without changing behavior.

## Scope

Add a dated implementation note to accepted ADR 0015. Update the report section of the roadmap, both active plans, and the transform index. Mark planning slice 10b done and put the ADR 0017 build-team review next. Make no product-code, transform, demo, or command change.

Excluded: implementing the catalog, choosing its membership or public names, adding `report`, changing any transform output, adding a jq engine, moving crates, release packaging, and live calls.

## Acceptance

- ADR 0015 records the observed evidence that its three-family trigger fired, the agent's decision to decline `report`, Ian's ability to overturn that decision, and the one-crate packaging dependency.
- `specification/roadmap.md`, ADRs 0010, 0012, and 0015 tell one story: the agent declines `report` from the completed evidence and Ian can overturn that decision; transforms remain executable files; the later command only lists and prints selected transforms; it never runs them.
- Both active plans mark slice 10b complete, correct the stale claim that ADR 0017 still needs a rewrite, name the required build-team review, then Job 3 and Job 2, name the catalog in release preparation, and put ADR 0017 section 8 step 1 after those steps. They do not create the future implementation ticket early.
- `transforms/README.md` states that repository files are the current distribution and that a read-only catalog follows the one-crate move. It makes no present-tense command claim and no promise that every current file becomes built in.
- Counts of ten `.jq` files, 1,408 lines, seven executable Bash blocks in the transform index, and nineteen green how-tos are reproduced by command. Documentation checks, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0049 and accepted ADRs 0010, 0015, and 0017.

## Complexity

- Contract: 0
- State and timing: 0
- Reach: 1
- Proof: 1
- Cost of error: 1
- Total: 3
- Minimum level floor: none
- Final level: 2
- Reasons: this closes one held product decision across planning and roadmap files by applying an already accepted rule. It changes no executable behavior or new public promise.
- Selected model: `gpt-5.6-luna` with high reasoning.

Re-score if review requires command code, catalog membership, transform relocation, or a new report shape.

## Review

Independent design review rejected the first proposal because it duplicated ADR 0015 and prematurely made every current transform a catalog promise. The rewrite applied ADR 0015, deferred the catalog boundary, and restored the required review before the one-crate move.

Independent implementation review then found two planning errors. The first draft skipped ADR 0017's conformance and DuckDB interrupt jobs. It also treated the evidence-based refusal of `report` as Ian's accepted ruling. The repair restored the four-step order and names the refusal as an agent decision Ian can overturn. The same reviewer accepted the repaired result.
