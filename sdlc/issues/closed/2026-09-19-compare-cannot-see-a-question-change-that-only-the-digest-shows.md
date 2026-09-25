# `compare.jq` cannot see a question change that only the digest shows

Status: Closed by ticket 0027

Found on 2026-09-19 while writing how-to 41 for ticket 0017.

## What happens

`transforms/compare/compare.jq` reports `changed.question` as `false` for two runs that asked two different questions. The runs judged the same twenty-four expense claims. The first used a question file with the question text alone, and the second added `true` and `false` to the same file. Three answers flipped from yes to no, and `changed` named nothing as the cause.

```
{"paired":24,"same":21,"flips":{"yes to no":["C-06","C-12","C-18"]},
 "changed":{"question":false,"model":false,"threshold":false}}
```

## Why

`changed.question` compares `question.text` across the two runs. The text was identical. What moved was what yes and what no mean, which ticket 0017 added, and `result.md` prints those nowhere in `question` for a `decide` row.

The same rows do carry the change. `meta.question_sha256` was `ad25f6f9...` in the first run and `c2c9a714...` in the second, because the two texts are part of the canonical form the digest is taken over, as `specification/question-file.md` fixes it.

## Why it matters

The purpose of `changed` is to stop a reader blaming the wording for a flip that the model version caused, and the other way round. A reader who trusts it today can conclude that nothing about the question moved when the question is exactly what moved.

## The fix, for whoever takes it

Ticket 0027 makes a complete nonempty comparison read `meta.question_sha256`. The digest covers the verb, the text, the two texts, the options with their descriptions, the levels, and the threshold, so one comparison replaces three. A row older than ticket 0017 carries no digest, so the transform falls back to the text and says which test it used.

How-to 41 names the digest and fallback rules in its traps section.
