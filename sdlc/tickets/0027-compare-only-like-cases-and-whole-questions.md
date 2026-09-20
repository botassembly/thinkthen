---
flow: build
priority: 87
opens: transforms/compare/compare.jq transforms/README.md demos/41-tune-a-question-file/README.md sdlc/issues sdlc/planning/plan.md
---

# 0027: Compare only like cases and whole questions

Status: ready

## Outcome

A run comparison reports question changes from the full resolved question and never counts changed evidence or labels as answer agreement or movement.

## Current Facts

`compare.jq` compares `question.text`, so the two runs in how-to 41 report no question change even though their true and false descriptions differ and their `meta.question_sha256` values prove it. The transform lists changed evidence and labels but still includes those pairs in `same` or `flips`. The existing issue on question changes and the new issue on incomparable pairs reproduce both faults.

## Scope

- Use question digests when both runs are nonempty and every row in both has one. Fall back to printed question text for an older or mixed nonempty run, and report which identity was used. Report the question change as unavailable when either run is empty.
- Keep `paired` as the shared unique-id count. Add `compared` for matching evidence and labels. Compute `same` and every flip from those comparable pairs alone.
- Update the transform header, its executable README proof, how-to 41, both issues, the plan, and the landed record. Do not change result rows or the binary.

Excluded: changing how ids pair, accepting repeated ids, adding metrics for `choose` or `score`, changing label types, or redesigning the other transforms.

## Acceptance

- How-to 41's real draft and tuned rows report `changed.question: true` and `question_by: digest` while retaining 24 paired and compared rows, 21 same answers, and the three existing flips.
- A deterministic legacy fixture with one missing digest uses text for both runs, reports `question_by: text`, and detects a printed text change.
- If either run is empty, `changed.question` is null and `question_by` is `unavailable`.
- A doctored pair whose evidence and label differ remains in both mismatch lists but enters neither `same` nor `flips`. The output proves exactly that `compared` equals `same` plus the total lengths of every flip list.
- Missing cases and repeated ids remain outside both `paired` and `compared`. The full ladder passes without a network call.

## Dependencies

ADR 0021, decided with this ticket. Ticket 0026 is landed.

## Complexity

- Contract score: 1
- State and timing score: 0
- Reach score: 1
- Proof score: 2
- Cost of error score: 1
- Total: 5
- Minimum level floor: none
- Final level: 2
- Reasons: the transform has several explicit public cases and two executable pages; modern, legacy, mixed, and empty runs need compatibility proof; a wrong result can misstate an evaluation, but the change is pure local jq over fixtures and is easy to correct.
- Selected model: `gpt-5.6-luna` with high reasoning

## Review

- Design review: accepted after one correction. The reviewer required defined empty-run semantics, an exact comparison-count invariant, and a proof score of 2 for legacy compatibility. It accepted the corrected null question change and unavailable identity for an empty side, the explicit invariant, and the level 2 Luna High route.
- Code review: pending
