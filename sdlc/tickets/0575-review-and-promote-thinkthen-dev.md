# 0575: Review and promote thinkthen.dev with the 0.2 release

Status: OPEN.

Milestone: 0.2

Depends on: 0543, 0572

## Outcome

thinkthen.dev shows the refreshed 0.2 docs on the day 0.2 ships. An independent reviewer reads the whole site as a first-time reader before it goes live, every sample runs on the released 0.2 libraries, and the release announcement points readers to the site.

## Evidence

- Starts from: the docs team's 2026-10-12 refresh under 0572.
  - The site lives on branch `docs/0.2-refresh` in this repo, pushed to GitHub at 126ea45b2. Its source is under `site/`. The docs team builds it in the worktree `worktrees/thinkthen-docs` and previews it locally with `astro preview` on port 4329.
  - The branch is 48 commits ahead of main and 12 behind.
  - The top menu is Start, Functions, Models, Languages, Databases and Recipes. The site has 104 listed pages, down from 138, and 169 old addresses forward to new pages.
  - The full site build passes its checks: samples, cases, flags, layout, code, words, settings, links, redirects, cards, head and recipes.
  - All 231 language samples replay against saved answers with no key and no network, including the ten Objective-C samples on macOS.
  - The home page opens with a measured saving: 306 song titles cost 11,468 input tokens in one request instead of 88,933.
  - thinkthen.dev still shows the 0.1 site.
- Keeps: the docs team's ownership of the site content under 0572, and Ian's go before anything publishes.
- Changes:
  - Rebase the branch on the 0.2 candidate and rerun the full site build and every language replay against the candidate libraries.
  - Have one independent reviewer read every page as a first-time Python developer, DuckDB analyst and agent builder. The reviewer checks that each claim links to its evidence, each sample matches the candidate's output, and every page uses present-tense reader wording.
  - Name Jev, SystemOne, TypeSafe and other public model and product keywords often in page text, titles and descriptions, for search. Ian ruled on 2026-10-13 that these names are public.
  - Remove the docs branch's temporary word-check allow entries once the 0.2 fixes they cover have landed.
  - Land the branch on main with the 0.2 release, and deploy thinkthen.dev from main after Ian's go.
  - Write the release announcement and link it to the home page, the install page and the recipes.
- Proof:
  - The full site build and every language replay pass on the release commit.
  - The reviewer's findings are fixed or ruled on in Progress.
  - thinkthen.dev serves the 0.2 home page, and a sample of old addresses forward to their new pages.
- Defers: paid promotion and any live-model demo.

## Progress
