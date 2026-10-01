# The site smoke is red, and most site samples run under no check

Status: open for parts 3 and 4, both marketing's. Parts 1 and 2 are done, as marketing reported in `closed/2026-09-30-site-fixtures-converted-and-plan-examples-moved.md`. Found on 2026-09-30 by running the site smoke against main `acebf3044`. Marketing owns `site/` (`sdlc/planning/ownership.md`); converting the recordings is ours. Our part, the conversion proof, landed with ticket 0304 slice 5; marketing runs the conversion. Owner of the rest: marketing. Merged on 2026-09-30 with the site samples issue, now in `closed/`.

Kind: debt

Pay when: 0304 slice 5 lands, before the site goes public with 0.1.

Debt: 007

Severity: medium

Keeping it leaves the site smoke red, so a new site failure hides among 79 known ones, and a library or database tab can drift from its surface with no check failing.

## 1. Fifteen replay folders have no fixture: done

Marketing converted the fifteen folders with `thinkthen cache convert`, and no entry was skipped. Each folder now holds only `thinkthen.jsonl`. Our conversion proof landed with ticket 0304 slice 5. Git history holds the folder list and the command.

## 2. The five `--dry-run` examples: done

The five examples and the smoke wrapper use `--plan`. The site smoke passes 97 of 97 examples, and `npm run build` passes.

## 3. Run the library and database samples (marketing)

`site/scripts/smoke.mjs` runs only the command-line `.sh` samples. `site/examples/SKIP` excludes every `.py`, `.ts`, `.rb`, `.R`, `.rs`, `.c` and `.sql` sample, and `site/README.md` lists an installed-host smoke runner under "What is not here yet".

Done when every sample on the site is run by its surface's check or by a site runner against the installed package, and prints the output the page shows.

## 4. Other site checks and pages (marketing)

The site rebuild (`201bfcca0`, `fa3e9848e`, `e0989cd87`) removed the status words, moved samples to `site/examples/functions/`, added the Beatles Bench section, corrected the prune line, moved the Ruby, TypeScript and R samples to `.value`, and qualified the SQL warm blurb with `thinkthen_batch(1)`. Three items remain.

- **A check that refuses status words.** Ian, 2026-09-24: every surface ships together, so no copy says one part is done and another is not. The pages no longer show preview, planned, beta, ships first, not run yet or comes with 0.1, but no check keeps them out. Done when a site check fails the build on any of those words.
- **A pandas page.** The catalog lists pandas among the bindings, but the site has no pandas page. `site/README.md` lists it under "What is not here yet".
- **A stray code tag after a table.** An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag after `</table>`, and the rest of the page renders in monospace. The smallest case is `<table><tr><td><code>{v}</code></td></tr></table>`, which builds to `</table><code> <p>After the table.</p>`. Site tables avoid it by building rows with `.map()`. Done when a check over `dist/`, run beside `check-links.mjs` in `npm run build`, fails on `</table>` followed by a start tag.
