# Quick Fix qf-jobs-width: keep the default --jobs width at 4 and give its measured rates

Status: landed. It closes `sdlc/issues/2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md`. A fresh read-only Opus review is in `sdlc/records/qf-jobs-width-review.md`.

## Result

- The default width stays at 4. The issue's decision section gives the reasons and says Ian can overturn it. No code changed.
- `specification/records.md` and `site/src/pages/reference.astro` give experiment 206's rates at widths 4 and 3 and name the accuracy record that holds them. `records.md` also names the live-probe record for the run at about 4,300 a minute. Both pages tell a user who must stay inside the documented limit on short records to set `--jobs 3`.
- `records.md` no longer says 4 is safe everywhere.

## Tests

This fix adds no test, because no behavior changed. The review planted `Width::FALLBACK = Self(3)`. `the_first_explicit_width_wins_and_only_a_different_one_is_refused` in the engine width tests went red on its expected "width 4". Two other width tests hung past 60 seconds, because each holds 4 slots. `decide_edge` stayed green: its `[default: 4]` check reads fixed help text in `cli/args.rs`, so a change to the constant must edit that text by hand. The issue's decision section says so.

## Checks

With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, each rung under `timeout 2400`, at `a9661d0c`, with the one-minute load between 3.8 and 8.0:

- `install`: exit 0.
- `lint`: exit 0, `ratchet: crates + conformance 48801/48801`.
- `test`: exit 0, 799 passed, 0 failed, 7 ignored across 18 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

Main then moved to `f5725027` with `sdlc/` files only. The merge re-measured the ratchet at 48801/48801. The whole ladder ran again at the commit that adds this section, which is the commit that lands. The merge commit on main states that run.
