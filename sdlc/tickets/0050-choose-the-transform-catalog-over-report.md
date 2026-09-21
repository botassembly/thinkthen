---
flow: build
priority: 39
opens: sdlc/planning specification/roadmap.md transforms/README.md
---

# 0050: Choose the transform catalog over report

Status: proposed

## Outcome

The completed transform stage ends with one clear verdict on Ian’s five outcomes: no `report` command. The repository keeps the separate `jq` transforms and carries them through the command with the already accepted read-only `thinkthen transform list` and `thinkthen transform show NAME` surface. The catalog lands during release preparation, after the one-crate move fixes its package location.

## Current facts and decisions

ADR 0010 names five possible outcomes. ADR 0015 already accepts the fifth when three transform families exist. They now exist: whole-run metrics, a row policy, and a reviewed-action monitor. Ten `.jq` files total 1,408 lines. Seven transform checks and nineteen green how-tos run them without a model call. Comparison and human-label sweeps have awkward invocations, but their output contracts work as files; a `report` command would have to duplicate several distinct reports and their validation.

This ticket records these conclusions. Ian can overturn the timing and the verdict; the catalog surface itself was accepted in ADR 0015.

1. Outcome 5 wins: a general way to carry transforms inside the tool. Full and partial `report` stay declined. “No report at all” and “transforms alone” also lose because a release binary does not give an installed user the repository paths that every how-to currently names.
2. The general way remains exactly `thinkthen transform list` and `thinkthen transform show NAME`. Both print and exit. The tool never starts `jq`, interprets a transform, manages user files, or combines reports. A user runs the printed source with their own `jq` process.
3. The catalog carries every public `.jq` transform that ships with the repository, including the example-specific triage policy. Listing all shipped files is less surprising than a hidden distinction between reusable and example-specific files. The later build ticket fixes names and byte identity from the final package tree.
4. The catalog implementation waits until release preparation, after ADR 0017’s one-crate move. The one published crate must own the embedded files. Moving or duplicating them now would create packaging work that the accepted merge immediately moves again.
5. The transform stage is complete after this verdict. The next builder work is ADR 0017 section 8 step 1: fold to one crate and move machinery into the private engine without changing behavior. `transform list/show` remains a named item in the release pass, not an unowned idea.

## Scope

Write ADR 0031 with the evidence and verdict. Update the report section of the roadmap, both active plans, and the transform index. Mark planning slice 10b done and put the one-crate move next. Make no product-code, transform, demo, or command change.

Excluded: implementing the catalog, choosing its final embedded file layout, adding `report`, changing any transform output, adding a jq engine, moving crates, release packaging, and live calls.

## Acceptance

- ADR 0031 names all five outcomes, the observed evidence, why outcome 5 wins, the catalog’s exact boundary, and the one-crate packaging dependency. It says which decisions Ian can overturn.
- `specification/roadmap.md`, ADRs 0010, 0012, 0015, and ADR 0031 tell one story: `report` remains held; transforms remain executable files; the command will only list and print them; it never runs them.
- Both active plans mark slice 10b complete, name the catalog in release preparation, and put ADR 0017 section 8 step 1 next. They do not create the future implementation ticket early.
- `transforms/README.md` states that the repository files are the current distribution and the read-only catalog follows the one-crate move. It makes no command claim in the present tense.
- Counts of ten `.jq` files, 1,408 lines, seven transform checks, and nineteen green how-tos are reproduced by command. Documentation checks, all four repository rungs, and `git diff --check` pass.

## Dependencies

Ticket 0049 and accepted ADRs 0010, 0015, and 0017.

## Complexity

- Contract: 1
- State and timing: 0
- Reach: 1
- Proof: 1
- Cost of error: 1
- Total: 4
- Minimum level floor: none
- Final level: 2
- Reasons: this closes one held product decision across planning and roadmap files. It changes no executable behavior, but a wrong verdict would send the next stage toward the wrong public surface.
- Selected model: `gpt-5.6-luna` with high reasoning.

Re-score if review requires command code, transform relocation, or a new report shape.

## Review

Pending independent design and documentation review.
