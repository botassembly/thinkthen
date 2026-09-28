# Quick Fix qf-readme-tutorial-pass: tighten the README and name the batch default

Ian asked for major README updates on 2026-09-28. This is a wording and structure pass on `README.md` alone. No claim changes meaning, and no example changes.

## Result

- The command block keeps its five examples. The five sentences that followed collapse into one line per answer and one link to each contract.
- The cache paragraph, seven sentences in one block, becomes four bullets: what an entry holds, who controls a folder, the address binding, and `--no-cache`. Every sentence survives. `cache prune` is the only removal path, from `specification/recording.md`, is added as one clause.
- The `status` paragraph keeps its content and drops its connectors. The sidecar sentence about retry totals stays.
- One bullet is added to "What it will and will not do": `--batch max` fills each request and is the default, `--batch 1` sends one record per request, records that share a request can affect each other's answers, and a threshold tuned at one setting warns at another. Sources: `specification/records.md` lines 93 and the batch table, `specification/question-file.md` line 103, and ADR 0048 item 12. The README had no batching line at all, and batching is the shipped default.
- The two "not for" bullets lose their duplicated wording. The `coprocess` guidance, `decide --lines --batch 1`, `filter`'s dropped-line behavior, and `rank`'s wait-for-input behavior all stay; they are the fix from register 55.
- The "Four names" and "Where to read" sections lose their internal curation history (ADR 0018 and ticket 0088 bookkeeping). The front window keeps its seven rows untouched: same titles, same order, same links.
- `sdlc/planning/` pages move under a new `## Contributing` heading. `## Gates` is unchanged.

## Checks

- `python3 sdlc/scripts/pages`: pass, "1 coming, 22 green". The front window and every relative link hold.
- `sdlc/scripts/lint`: pass. It reports `CLAUDE.md 5532/5000 characters`, which predates this change and is not touched here.

## What this fix does not do

- No change to `demos/`, `specification/`, or any page the site pulls. The word-count budget for site pages and the other punch-list items stay open.
- No fresh reviewer ran on this pass. The author checked every kept claim against `specification/` and the register items named above.

## Build lesson

The first draft of this pass nearly reverted `c128c7ff`'s install-truthfulness change. A parallel read raced the checkout's fast-forward and showed the README's older release-tarball first run, so the rewrite restored it. `git diff` against HEAD caught it before landing, and the source-checkout first run is kept exactly as `c128c7ff` wrote it. Read and write in one step when another lane is pushing.
