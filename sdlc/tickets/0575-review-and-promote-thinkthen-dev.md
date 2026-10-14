# 0575: Ship thinkthen.dev and every public text with 0.2

Status: OPEN.

Milestone: 0.2

Depends on: 0543
Depends on: 0581
Depends on: 0582

Owner: the ThinkThen build team. The coordinator runs it as its own lane beside the 0.2 waves. It owns branch `docs/0.2-refresh` and its worktree `worktrees/thinkthen-docs`.

Reviews: revision 5c13e69, reject

Reviews: revision 7a04860, accept

Reviews: revision f27da0d, accept
Reviews: revision 0bb5893, accept

## Outcome

thinkthen.dev and every public text that ships with 0.2 sell one idea: put Jev-class decision models in your code, whatever model, language or database you use. ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. A first-time Python developer, DuckDB analyst or agent builder can say in one minute what they get, whether it works in their setup and what to paste. Every sample runs on the 0.2 release. Nothing goes live on thinkthen.dev, any registry or the DuckDB PR before Ian approves it.

## Evidence

- Starts from: the docs refresh that 0572 began, which this ticket supersedes, and Ian's 2026-10-13 ruling that the ThinkThen team owns the website and that it ships with 0.2.
  - The site lives on branch `docs/0.2-refresh` at 126ea45b2, last changed 2026-10-12. It is 48 commits ahead of main and 28 behind. Its source is under `site/`. `npm run build` in `site/` runs every site check. `astro preview` serves it locally on port 4329; three preview processes from the old owner still run there.
  - The menu is Start, Functions, Models, Languages, Databases and Recipes. The site has 104 listed pages, down from 138, and 169 old addresses forward to new pages.
  - On 2026-10-12 the full site build passed every check: samples, cases, flags, layout, code, words, settings, links, redirects, cards, head and recipes. All 231 language samples replayed against saved answers with no key and no network, including the ten Objective-C samples on the M5.
  - The branch carries temporary word-check allow entries for the old Settings page wording, which main fixed at 4c2268737.
  - The README's opening on the branch is the reviewed pitch: the ten functions, then why not call the model yourself. 0578 and 0577 reuse it word for word.
  - thinkthen.dev still shows the 0.1 site.
  - Ian ruled on 2026-10-13 that Jev, SystemOne, TypeSafe and OpenAI are public names to use often for search.
- Keeps: replayed samples as smoke tests, the Markdown twins and llms files, the site's build checks, and the old-address redirects.
- Changes:
  - Merge main into `docs/0.2-refresh` now, and again from `ticket/0543-final` as each wave lands. Rerun `npm run build` and every language replay after each merge.
  - Remove the temporary word-check allow entries.
  - Name Jev, SystemOne, TypeSafe, OpenAI and local models often in page text, titles and descriptions.
  - Make the home page and README answer "why not call the model yourself?" with claims that each link to evidence.
  - Update every sample to each library's one-line typed answer once 0574 lands, and replay it.
  - Run the fresh-reader page test: a fresh agent given one page plays each reader and must succeed. Run it on the home page, one function page, one language page and the DuckDB page, and on every public text outside the site: each registry README from 0578, the DuckDB README page, the DuckDB descriptor's `extended_description` and the PR description from 0577. Fix what fails. The PM brings passing texts to Ian for approval of the exact words.
  - Land the branch on main with the 0.2 release. Deploy thinkthen.dev from main after Ian's go.
  - Stop the old preview processes once the new owner's preview runs.
- Proof:
  - `npm run build` and every language replay pass on the release commit.
  - The fresh-reader test passes on every page and text listed above.
  - Ian approves the public texts.
  - After Ian's go, thinkthen.dev serves the 0.2 home page, and a sample of old addresses forward to their new pages.
- Defers: paid promotion, live-model demos, and fine-tuning, GEPA, proxy and hosted-service pages.

## Progress

PM amendment, 2026-10-13. Remaining work. `docs/0.2-refresh` is at 126ea45b2, 30 commits behind main and about 400 behind `ticket/0543-final`. Steps: merge `origin/main` in `worktrees/thinkthen-docs`. Merge `origin/ticket/0543-final`. On `README.md` keep the docs branch version, put 0578's `OPENING` paragraph first, and drop 0578's root rewrite. Regenerate `site/recordings/thinkthen.jsonl` from the merged commit instead of hand-merging it, and merge `libraries/files.md` and `site/src/pages/install/index.astro` by hand. Remove the obsolete allow entries. Run `npm run build` and every language replay, with Objective-C on the M5, and push after each fix. Once 0574 merges, move every sample to the one-line typed answer; 0578's pages and 0577's DuckDB page reuse these samples and recordings. Run the fresh-reader test listed above. The coordinator merges `docs/0.2-refresh` into `ticket/0543-final` before the freeze, because the candidate's `check-binding-samples.mjs` checks site samples; that merge is a 0543 gate. Text changes Ian asks for after the freeze are text-only: rerun the README check, `sdlc/scripts/package`, the site build and the hosted rehearsal, not the full campaign. After Ian's go, main takes the candidate SHA and thinkthen.dev deploys from main.

PM amendment, 2026-10-14. 0581 and 0582 change how every library and database returns values and details. Hold code example rewrites until they land, then rewrite each example against `result.value`, `result.probability` and `thinkthen_<function>_details`. Prose and openings continue now.

PM amendment, 2026-10-14. 0543 gates only landing and deploy; every other change starts now.

PM amendment, 2026-10-14. Work starts per language. A language or database whose 0581 or 0582 lane is merged into the 0543 final branch may have its examples and text rewritten now, on its own lane, without waiting for the other languages. The fresh-reader review still runs once all are done.
