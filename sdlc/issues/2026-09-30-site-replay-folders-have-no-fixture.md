# Fifteen site replay folders have no fixture, and the site smoke is red

Status: open. Found on 2026-09-30 by running the site smoke against main `acebf3044`. Marketing owns `site/` (`sdlc/planning/ownership.md`); the conversion is ours. Owner of our part: ticket 0304 slice 5.

## The problem

Since 0304 slice 2, replay reads only `thinkthen.jsonl` or `thinkthen.sqlite` (`engine/store.rs:127-133`). Fifteen tracked folders under `site/` hold only old `DIGEST.json` entries:

- `site/recordings`, which the smoke passes to every function call (`site/scripts/smoke.mjs:101`).
- Eight Beatles Bench examples: `site/examples/beatles/bench/examples/{annotate,choose,decide,filter,find,rank,score,tag}/recording`.
- One bench results run: `site/examples/beatles/bench/results/runs/2026-09-26-thinkthen-jev/recording`.
- Three Beatles pages: `site/examples/beatles/{recognize,relate,score-bands}/files/recording`.
- Two Bash how-tos: `site/examples/how-tos/bash/{agent-tool-guard,long-lived-loop}/files/recording`.

The smoke run, `cd site && THINKTHEN_BIN=../target/debug/thinkthen node scripts/smoke.mjs`, with no key or address set and no network, exited 1: 79 of 97 examples failed and 18 passed. 74 failed because the replay folder holds no answer for the question key. That count includes three asserts that failed after a miss. The other 5 failed with `--dry-run was renamed --plan`: `beatles/backends/1-check.sh`, `beatles/recognize/1-find.sh`, `install/backends/1-dry-run.sh`, `install/settings/1-environment.sh` and `install/settings/2-flag.sh`. The smoke's wrapper also still matches `--dry-run`.

## What should happen

- **Ours, 0304 slice 5.** Run `cache convert` on each of the fifteen folders, as slice 1 requoted them by script (ticket 0304, slice 1 lessons: "Recordings under `site/examples` and `site/recordings` were requoted mechanically"). Rerun the smoke and hand marketing the converted fixtures and the run's result. Delete the old files only after marketing accepts. Ticket 0304 slice 5's proof list adds the site smoke.
- **Marketing's.** Change the five `--dry-run` examples and the smoke wrapper to `--plan`, review any `.out` change the converted fixtures cause, and keep the smoke green afterwards.

## Evidence

The folder list comes from `git ls-files`: folders with digest-named `.json` files and no `thinkthen.jsonl`, outside `probes/`. The smoke log was kept as a local scratch file.
