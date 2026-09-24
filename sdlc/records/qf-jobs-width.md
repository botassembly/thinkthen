# Quick Fix qf-jobs-width: keep the default --jobs width at 4 and give its measured rates

Status: landed. It closes `sdlc/issues/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md`. A fresh read-only Opus review is in `sdlc/records/qf-jobs-width-review.md`.

## Result

- The default width stays at 4. The issue's decision section gives the reasons and says Ian can overturn it. No code changed.
- `specification/records.md` and `site/src/pages/reference.astro` give experiment 206's rates at widths 4 and 3 and name the accuracy record that holds them. `records.md` also names the live-probe record for the run at about 4,300 a minute. Both pages tell a user who must stay inside the documented limit on short records to set `--jobs 3`.
- `records.md` no longer says 4 is safe everywhere.

## Tests

This fix adds no test, because no behavior changed. The review planted `Width::FALLBACK = Self(3)`. `the_first_explicit_width_wins_and_only_a_different_one_is_refused` in the engine width tests went red on its expected "width 4". Two other width tests hung past 60 seconds, because each holds 4 slots. `decide_edge` stayed green: its `[default: 4]` check reads fixed help text in `cli/args.rs`, so a change to the constant must edit that text by hand. The issue's decision section says so.
