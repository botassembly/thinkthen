# 0042: Compare scalar values and honest probability movement

Date: 2026-09-21

Status: landed

## Result

`compare.jq` now compares null, boolean, string, and number values. `same` and `changed_values` form an exact partition of comparable pairs. The ordered `changes` list carries both values, while the existing yes, no, and unresolved `flips` object keeps its old shape.

Valid yes-or-no pairs also carry their before and after probabilities, their absolute delta, and `probability_delta_over_tolerance`. Every changed answer stays visible. Same-answer probability movements at or below the active tolerance are summarized instead of filling the list. The optional tolerance defaults to 0.08, accepts zero through one, and appears in `yes_no_probability`. Choice and score comparisons invent no probability.

## Proof

The focused test first failed when the old transform rejected a string value. It now covers all four scalar kinds, all six boolean-or-null directions, lexical ordering, exact partitions, tolerance boundaries and overrides, legacy rows, malformed yes-or-no answers, and invalid scalar values. Fixed diagnostics repeat no hostile id, body, or answer marker.

The read-only check over local experiment 212 compared 100 pairs. Ninety-six values stayed the same and four flipped. Sixty-three probabilities moved; 59 same-value movements were summarized, the largest was 0.08, and none exceeded the default. All four visible flips say their probability delta did not exceed the tolerance. No network or paid call ran.

How-to 41 is 119 lines and 870 words. It states the fair-pair rule and describes 0.08 as the observed maximum of one limited run rather than a regression boundary. Both active plans put per-question comparison inside `annotate` result objects next, before monitors, without creating that ticket early.

## Review and gates

Design review corrected the compatibility score, fixed the public summary names, distinguished an omitted tolerance from explicit null, and accepted the later per-change boolean. The final route was level 3 with Sol Medium.

Code review rejected input-bearing scalar errors and the missing prospective-plan follow-up. The remediation fixed both and added hostile-marker tests. The same reviewer accepted the corrected tree with no remaining finding.

On the final reviewed tree, `install`, `lint`, `test`, and `spec` all passed. The test rung ran 436 Rust tests, two documentation tests, the focused transform test, the probe self-tests, the triage tests, and `live-test`. The specification rung ran 26 specification checks, two transform checks, every probe replay, and all 19 green how-tos. `git diff --check` passed.
