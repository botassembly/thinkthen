# 0045: Sweep decisions, picks, and scores

Date: 2026-09-21

Status: landed

## Result

One `sweep.jq` now reads detailed `decide`, `choose`, or `score` rows. Decision input keeps the prior report unchanged. Choice input shows accuracy beside coverage at each winning-probability cut, reports exact ties separately, and chooses no cut. Score input shows binary accuracy, precision, recall, and F1 at each boundary between named levels, and chooses no cut.

The transform validates the containers and fields it reads before computing a report. Its failures use fixed sentences and do not echo row data. Current backend probabilities have two decimal places, so the public page explains why neighboring twentieth cuts can tie without turning that observation into a backend promise.

## Proof and review

The focused test freezes both prior decision reports. Synthetic rows prove labeled-only choice denominators, an inclusive exact cut, tie exclusion, null denominators, score boundaries, and fixed failures for malformed containers and numeric bounds. The committed choice and score probes reproduce the ticket's outside numbers. The transform page has executable examples for both modes.

Design review made the choice denominators, tie treatment, and validation boundary exact before implementation. Code review then found one data-bearing legacy error, jq-native container errors, and a missing exact-cut assertion. The repair closes all three, and the same reviewer accepted it with no remaining finding.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass. How-to 13 is 100 lines and 887 words. No live or paid call ran.
