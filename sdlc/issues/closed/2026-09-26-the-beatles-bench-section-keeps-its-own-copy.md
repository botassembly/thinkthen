# The Beatles Bench section keeps its own copy of the pages

Status: closed 2026-09-30. Fixed by site commits `201bfcca0` and `e0989cd87`. The pages live in `site/src/data/beatles.mjs`, bench links use paths on bench main, and `site/scripts/smoke.mjs` runs every example. `site/examples/beatles/bench-pin` stays as the source commit for `pull-bench` and the slide check, as `site/README.md` records.

Filed on 2026-09-26 by bench ticket 0016. It revises `2026-09-25-a-worked-examples-section-pulled-from-beatles-bench.md`. That issue pulls the pages from a pinned bench commit. Bench ticket 0016 deletes those pages from the bench before the bench goes public.

## What changed in the bench

Bench ticket 0016 deletes `docs/walkthroughs/`, `docs/run-it-for-free.md`, `docs/context-article.md`, and `docs/README.md`. It merges `docs/the-data.md` into `data/README.md` and `docs/context-and-cost.md` into `reports/open-book.md`. The bench README now links https://thinkthen.dev/learn/beatles-bench/ for the walkthroughs. The website holds the only copy.

Bench commit 7d2468443fa6a38dc3b52942c780b07c308a852a is the last main commit before ticket 0016. It holds every page the section needs, with the same bytes as commit 2cdb6445. The site copies the pages from that commit once. The copy lives in this repository, so a later squash of the bench history breaks nothing.

## What the site needs

1. **Its own copy.** The walkthroughs and the free-run page live under `site/` and change here. The pull of pages from the bench stops.
2. **No bench pin.** The bench history may be squashed again at launch, and any pinned hash then stops resolving. The dead pin 9d7f1820 in the older issue goes. The site names no bench commit. The bench data files the section copies (recordings, cases, audit rows, and tables) keep their bytes. The copy can be refreshed from bench main when a bench ticket changes one.
3. **Links to bench files.** A link to a bench file uses its path on bench main after bench ticket 0017. Ticket 0017 renames these paths:
   - `functions/` becomes `examples/`.
   - `questions/functions/` becomes `questions/suite/`.
   - `scripts/generate/functions.py` becomes `scripts/generate/make_suite.py`.
   - `scripts/run/functions.py` becomes `scripts/run/ask_suite.py`, and `scripts/run/functions.sh` becomes `scripts/run/ask_suite.sh`.
   - `scripts/score/functions.py` becomes `scripts/score/score_suite.py`.
   Links can use `https://github.com/botassembly/beatles-bench/blob/main/PATH`.
4. **Run paths.** Bench ticket 0016 moves superseded runs to `results/archive/runs/`. The files the site preview copies stay where they were: the function folders, `results/tables/functions.tsv`, and `results/runs/2026-09-26-thinkthen-jev`.

## Deferred

Command checks for the site's copy stay open. The bench's tests no longer check the walkthrough commands, because the bench no longer holds the walkthroughs.

Done when the site builds the section from its own copy with no bench pin, and every bench link uses a path on bench main.
