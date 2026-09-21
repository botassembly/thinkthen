---
flow: build
priority: 39
opens: sdlc/planning specification/roadmap.md transforms/README.md
---

# 0050: Apply the transform catalog ruling

Status: proposed

## Outcome

The completed transform stage records that ADR 0015's trigger has fired. Three transform families now exist, so the release will carry them through the accepted read-only `thinkthen transform list` and `thinkthen transform show NAME` surface. The `report` verb stays declined. The catalog lands during release preparation, after the one-crate move fixes its package location.

## Current facts and decisions

ADR 0010 named five possible outcomes for `report`. Ian accepted the fifth in ADR 0015 once three transform families existed: a general way to carry transforms inside the tool. They now exist as whole-run metrics, a row policy, and a reviewed-action monitor. Ten `.jq` files total 1,408 lines. The transform index contains seven executable Bash blocks, and nineteen green how-tos run them without a model call. Comparison and human-label sweeps have awkward invocations, but their output contracts work as files. A `report` command would duplicate several distinct reports and their validation.

This ticket applies the accepted ruling. Ian can overturn the implementation timing; the catalog surface and the decision to decline `report` are already accepted.

1. Outcome 5 supplies the distribution mechanism. The release binary will list and print named transforms. It will never start `jq`, interpret a transform, manage user files, or combine reports.
2. The separate `report` verb stays declined. The transforms remain executable files for `jq`; the catalog makes selected files available to an installed user without copying repository paths.
3. The release implementation ticket will choose the catalog membership, public names, byte identity, and embedded location against the final package tree. This ticket does not turn every file now under `transforms/` into a compatibility promise.
4. The catalog implementation waits until release preparation, after ADR 0017's one-crate move. The one published crate must own the embedded files. Moving or duplicating them now would create packaging work that the accepted merge immediately moves again.
5. The transform stage is complete after this record. ADR 0017 has already been rewritten and accepted. Its required build-team review comes next, followed by section 8 step 1: fold to one crate and move machinery into the private engine without changing behavior.

## Scope

Add a dated implementation note to accepted ADR 0015. Update the report section of the roadmap, both active plans, and the transform index. Mark planning slice 10b done and put the ADR 0017 build-team review next. Make no product-code, transform, demo, or command change.

Excluded: implementing the catalog, choosing its membership or public names, adding `report`, changing any transform output, adding a jq engine, moving crates, release packaging, and live calls.

## Acceptance

- ADR 0015 records the observed evidence that its three-family trigger fired, the continued refusal of `report`, and the one-crate packaging dependency. It says Ian can overturn the timing.
- `specification/roadmap.md`, ADRs 0010, 0012, and 0015 tell one story: `report` remains held; transforms remain executable files; the later command only lists and prints selected transforms; it never runs them.
- Both active plans mark slice 10b complete, correct the stale claim that ADR 0017 still needs a rewrite, name the required build-team review, name the catalog in release preparation, and put ADR 0017 section 8 step 1 after that review. They do not create the future implementation ticket early.
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

Independent design review rejected the first proposal because it duplicated ADR 0015 and prematurely made every current transform a catalog promise. This rewrite applies ADR 0015, defers the catalog boundary, and restores the required review before the one-crate move.
