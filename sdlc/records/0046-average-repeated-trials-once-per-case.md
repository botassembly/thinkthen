# 0046: Average repeated trials once per case

Date: 2026-09-21

Status: landed

## Result

`trials.jq` reads detailed `decide`, `choose`, or `score` observations and writes one compact derived row per case in first-seen order. It averages stored probabilities, recomputes the value under the shared threshold, records live and replay counts, and carries stable provenance without claiming that one backend call made the averaged answer.

`counts`, `score`, `sweep`, `band`, and `calibration` now refuse any repeated present case id with a fixed message directing the user through `trials.jq`. This prevents a case tried several times from silently receiving more metric weight. `compare` continues to list repeated ids, and `cost` continues to count every paid observation.

## Proof and review

Synthetic rows prove decision cuts and both band edges, choice means, first-option tie handling, choice cuts, score means and weighted values, output order, replay counts, validation boundaries, compact JSONL, and safe failures. Existing unique-input metric reports stay byte-for-byte unchanged.

The outside check combines the two 100-row runs from experiment 212 with their matching experiment-206 rows. Three hundred observations become 100 cases. At cut 0.5 the report has 11 true positives, 10 false positives, 74 true negatives, and 5 false negatives: accuracy 0.85, precision 0.5238, recall 0.6875, and F1 0.5946.

Design review made the derived schema, provenance, validation boundary, and invocation exact. Code review found multiline output under the documented command, a too-small floating-point allowance, and missing band-boundary proof. The repair closes all three and broadens duplicate detection to every JSON id type. The same reviewer accepted it with no remaining finding.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass. No live or paid call ran.
