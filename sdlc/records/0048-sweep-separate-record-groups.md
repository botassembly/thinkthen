# 0048: Sweep decision rows by record group

Date: 2026-09-21

Status: landed

## Result

The existing sweep now fits one decision cut inside each string-valued record group when the caller passes `--arg group POINTER`. Every group reuses the established nineteen-cut report and highest-F1 tie rule. Groups appear in lexical order, and the report has no pooled pick that could let a large group choose a smaller group’s cut.

The pointer follows RFC 6901, including the difference between the empty pointer and `/`, escaped slashes and tildes, and empty member names. Missing, null, or non-string group values fail with fixed messages that echo no row data. Grouped choice and score input is refused because those reports fit no automatic cut. A non-null ordinary input now fails clearly and tells the caller to use `jq -n`.

How-to 13 replaces text to show when separate group cuts make sense and stays within its limit at 102 lines and 858 words. Both active plans keep the remaining answer-group work explicit: the next ticket maps human truth and sweeps every `tag` label and every threshold-bearing named `annotate` answer.

## Proof and review

The focused proof freezes the prior valid reports byte for byte and covers independent group picks, lexical order, unlabeled rows, tied cuts, empty input, thirty added rows that leave another group unchanged, current id behavior, fixed failures, and the pointer edges above.

Design review first made the separate answer-group obligation explicit, corrected claims about current validation, aligned the pointer contract with RFC 6901, and required the page to replace text. Code review then found that `/` and the empty pointer shared one parse. The repair added the missing case and preserved the empty reference token. The same reviewer accepted the repair with no remaining finding.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass on current main plus this ticket. No live or paid call ran.
