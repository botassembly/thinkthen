# Correct stale command-page checks

The 0154 command checkpoint exposed two documentation defects left by the landed 0152 vocabulary change. This Quick Fix changes no product behavior and adds no product completion to the item table.

Commit `bbf9d6ce` removes the hardcoded 1,735-byte transform-catalog assertion. The catalog is now 1,726 bytes, while the immediately preceding `cmp` already proves every emitted byte matches the source file. The catalog-list and unknown-name checks remain. Commit `91a9be1d` shortens demo 19's threshold sentence. Vocabulary commit `1a97fb91` had replaced one word with two twice, growing the page from 899 to 901 words. The 900-word rule and all executable examples remain unchanged.

The fresh independent 0154 reviewer accepted each correction in session `01a0e4d5-fcfd-7510-8b4d-bdbd4f50568b`. The corrected transform page passed. The complete demo segment passed with 21 green and zero red pages. The exact segmented results and unchanged-source evidence are recorded in [0154-build.md](0154-build.md). No new test was added; one weaker duplicate assertion was deleted.

## What the build taught us

A mechanical vocabulary replacement can change both an embedded catalog's byte length and a page's enforced word budget. Preparation must check these downstream constraints. Keep the exact catalog comparison and delete its redundant size assertion instead of maintaining two assertions for the same bytes. Preserve passed checks and rerun the affected page segment after a prose-only correction.
