# 0049: Sweep tag and annotate against human labels

Date: 2026-09-21

Status: landed

## Result

The sweep now checks standalone `tag` runs through one pointer to a human label set and checks selected named `annotate` answers through an ordered question-to-pointer map. Every tag label gets its own decision report. Mapped decisions and choices get the same reports as their scalar forms. The transform prints no combined score that could hide a weak label or question.

Missing or null human truth stays visible as unlabeled. Present malformed truth stops. Before a nested result reaches the arithmetic, the transform checks its question, kind, probability shape, threshold, stored value, question-set digest, case id, and cross-row definition. The valid no-threshold choice form remains supported; a missing threshold and a numeric cut of zero are refused under the settled product rules. Fixed diagnostics echo no row, question, label, or pointer value.

Page 14 now checks all six human-labeled correctness cases rather than only the four its saved band resolved. The stored probabilities choose cut 0.6 with accuracy and F1 of 1. Page 25 explains that tag labels and mapped questions remain separate reports. Both pages stay within their limits.

## Proof and review

The focused proof covers standalone tags, mixed named decision/choice/tag annotations, partial and reordered mappings, pointer escapes, missing and null truth, fixed argument refusals, exact output keys, corrupt saved results, hostile markers, and byte-identical prior reports.

Design review fixed the nested output contract, complete tag validation, argument dispatch, digest boundary, routing score, and outer-versus-nested coherence before implementation. Code review then found incomplete saved decision validation, an incorrect refusal of valid null-threshold choices, an unsafe malformed-choice path, and the forbidden zero choice cut. Three focused repairs closed those gaps, and the same reviewer accepted the result.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass on current main plus this ticket. No live or paid call ran.
