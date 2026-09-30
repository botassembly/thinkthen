# The graded rank tests rewrite one question file in place

Status: closed on 2026-09-30 by ticket 0340. `question()` writes a per-process name and renames it into place. Under load, 35 of 800 runs failed before and 0 of 800 after.

Kind: debt

Pay when: the next change to `crates/thinkthen/tests/backend/keeping/`, or before 0.1.

Keeping it risks a false red that teaches builders to rerun reds.

## What happened

`crates/thinkthen/tests/backend/keeping/graded_rank.rs:26` `question()` writes `graded-rank-question.json` in place with `fs::write`, and every graded rank test calls it. Nextest runs each test as its own process, so one test can read the file while another rewrites it. `graded_rank_orders_weighted_positions_and_keeps_the_earlier_top_tie` failed once at load 22 with "the question file is not valid JSON" and exit 5.

## What should happen

The helper writes a new name and renames it into place, as ticket 0324 did for the annotate tests (cleanup plan lesson 5).
