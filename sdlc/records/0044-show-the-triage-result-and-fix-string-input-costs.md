# 0044: Show the triage result and fix string-input costs

Date: 2026-09-21

Status: landed

## Result

Page 16's first command now prints the six fictional tickets in id order with subject, action, and reason, then prints the existing route and agreement totals. This is a page-local TSV view assembled from the three JSONL audit files. The product and the saved files remain JSONL. The page also states the short `annotate | jq` idea before its complete staging script.

`cost.jq` no longer assumes that `input` is an object. A string or any other nonobject input, and an object with no id, uses `with no id`. Missing usage remains visible and contributes no tokens or cost.

## Proof and review

The triage self-test pins the six sorted display rows and four totals. Direct replay prints two drafts, one block, three reviews, and agreement on all six reviewed actions. How-to 28 feeds a hostile string input through the full cost transform, proves that the marker appears nowhere in its output, and pins `no_usage:["with no id"]` with zero tokens and zero cost.

Design review kept TSV limited to the page display and required both pages to stay within the how-to limits. Code review found that the first hostile-input assertion projected fields before checking for a leak. The repaired proof checks the full output first, and the same reviewer accepted it with no remaining issue.

The focused checks, `install`, `lint`, `test`, `spec`, and `git diff --check` pass. Page 16 is 93 lines and 783 words. How-to 28 is 120 lines and 862 words. All 19 green how-tos pass. No live or paid call ran.
