# The site smoke is red, and most site samples run under no check

Status: open. Found on 2026-09-30 by running the site smoke against main `acebf3044`. Marketing owns `site/` (`sdlc/planning/ownership.md`); converting the recordings is ours. Our part, the conversion proof, landed with ticket 0304 slice 5; marketing runs the conversion. Owner of the rest: marketing. Merged on 2026-09-30 with the site samples issue, now in `closed/`.

Kind: debt

Pay when: 0304 slice 5 lands, before the site goes public with 0.1.

Debt: 007

Severity: medium

Keeping it leaves the site smoke red, so a new site failure hides among 79 known ones, and a library or database tab can drift from its surface with no check failing.

## 1. Fifteen replay folders have no fixture (ours, 0304 slice 5)

Since 0304 slice 2, replay reads only `thinkthen.jsonl` or `thinkthen.sqlite` (`engine/store.rs:127-133`). Fifteen tracked folders under `site/` hold only old `DIGEST.json` entries:

- `site/recordings`, which the smoke passes to every function call (`site/scripts/smoke.mjs:101`).
- Eight Beatles Bench examples: `site/examples/beatles/bench/examples/{annotate,choose,decide,filter,find,rank,score,tag}/recording`.
- One bench results run: `site/examples/beatles/bench/results/runs/2026-09-26-thinkthen-jev/recording`.
- Three Beatles pages: `site/examples/beatles/{recognize,relate,score-bands}/files/recording`.
- Two Bash how-tos: `site/examples/how-tos/bash/{agent-tool-guard,long-lived-loop}/files/recording`.

The folder list comes from `git ls-files`: folders with digest-named `.json` files and no `thinkthen.jsonl`, outside `probes/`.

The smoke run, `cd site && THINKTHEN_BIN=../target/debug/thinkthen node scripts/smoke.mjs`, with no key or address set and no network, exited 1: 79 of 97 examples failed and 18 passed. 74 failed because the replay folder holds no answer for the question key; that count includes three asserts that failed after a miss. The other 5 failed with `--dry-run was renamed --plan`: `beatles/backends/1-check.sh`, `beatles/recognize/1-find.sh`, `install/backends/1-dry-run.sh`, `install/settings/1-environment.sh` and `install/settings/2-flag.sh`. The smoke's wrapper also still matches `--dry-run`. The smoke log was kept as a local scratch file.

Slice 5 converted each folder in a scratch copy of `site/` with a build of its branch. Every folder converted with no `--quote` and skipped no entry. The smoke over that copy, with the old files deleted, failed 5 of 97 examples, all five the `--dry-run` examples of part 2. Slice 5 does not edit `site/`, so marketing runs the conversion. The command is in ticket 0304, slice 5 evidence:

```sh
for dir in site/recordings \
  site/examples/beatles/bench/examples/{annotate,choose,decide,filter,find,rank,score,tag}/recording \
  site/examples/beatles/bench/results/runs/2026-09-26-thinkthen-jev/recording \
  site/examples/beatles/{recognize,relate,score-bands}/files/recording \
  site/examples/how-tos/bash/{agent-tool-guard,long-lived-loop}/files/recording; do
  thinkthen cache convert "$dir"
done
```

Then delete each folder's digest-named `.json` files and `.thinkthen-backend.json`, and rerun the smoke. Part 1 is done when those folders hold only `thinkthen.jsonl`.

## 2. The five `--dry-run` examples (marketing)

Change the five examples above and the smoke wrapper to `--plan`, review any `.out` change the converted fixtures cause, and keep the smoke green afterwards.

## 3. Run the library and database samples (marketing)

`site/scripts/smoke.mjs` runs only the command-line `.sh` samples. `site/examples/SKIP` excludes every `.py`, `.ts`, `.rb`, `.R`, `.rs`, `.c` and `.sql` sample, and `site/README.md` lists an installed-host smoke runner under "What is not here yet".

Done when every sample on the site is run by its surface's check or by a site runner against the installed package, and prints the output the page shows.

## 4. Other site checks and pages (marketing)

The site rebuild (`201bfcca0`, `fa3e9848e`, `e0989cd87`) removed the status words, moved samples to `site/examples/functions/`, added the Beatles Bench section, corrected the prune line, moved the Ruby, TypeScript and R samples to `.value`, and qualified the SQL warm blurb with `thinkthen_batch(1)`. Three items remain.

- **A check that refuses status words.** Ian, 2026-09-24: every surface ships together, so no copy says one part is done and another is not. The pages no longer show preview, planned, beta, ships first, not run yet or comes with 0.1, but no check keeps them out. Done when a site check fails the build on any of those words.
- **A pandas page.** The catalog lists pandas among the bindings, but the site has no pandas page. `site/README.md` lists it under "What is not here yet".
- **A stray code tag after a table.** An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag after `</table>`, and the rest of the page renders in monospace. The smallest case is `<table><tr><td><code>{v}</code></td></tr></table>`, which builds to `</table><code> <p>After the table.</p>`. Site tables avoid it by building rows with `.map()`. Done when a check over `dist/`, run beside `check-links.mjs` in `npm run build`, fails on `</table>` followed by a start tag.
