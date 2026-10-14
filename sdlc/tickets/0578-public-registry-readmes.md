# 0578: Give every published package a public README

Status: OPEN.

Milestone: 0.2

Depends on: 0574
Depends on: 0581
Depends on: 0582

Reviews: revision 6d46fa0, reject

Reviews: revision 149aa23, reject

Reviews: revision 65a4d09, reject

Reviews: revision e9274bd, reject

Reviews: revision e9f6785, reject

Reviews: revision cd0ce97, accept

Reviews: revision 79f6574, reject

Reviews: revision a7e9612, accept

Reviews: revision 0f48d9c, accept
Reviews: revision 2d335ef, accept
Reviews: revision 94ac2e5, accept
Reviews: revision bd3952c, accept
Reviews: revision 3504fe7, accept
Reviews: revision da5724b, accept
Reviews: revision 661fd00, accept

Reviews: revision 198f1ed, accept

Reviews: revision 8d42622, accept
Reviews: revision 63ea29c, accept
Reviews: revision bb85d53, accept

Reviews: revision c551f75, accept
Reviews: revision df9981c, accept

Reviews: revision 5974c75, accept

Reviews: revision 8ac420e, accept

## Outcome

Each package registry page that ships with 0.2 shows a short README for a stranger with the shared opening, one replayed example in that language, the cost and the install line, and `sdlc/scripts/registry-readmes.py` fails the release on any page that names an ADR, a ticket, an `sdlc/` path, an internal crate or a stale version, or whose example does not run.

## Evidence

- Starts from: the marketing lead's live registry check of 2026-10-13. PyPI thinkthen 0.1.2 opens its description with an ADR number and an `sdlc/` path, npm thinkthen 0.1.2 shows "No README data found!", and RubyGems holds only the 0.0.1 name placeholder.
- `sdlc/scripts/registry-readmes.py` maps each package to its source: the root `README.md` for the command line and Packagist, `crates/thinkthen/README.md` for crates.io, `libraries/<lang>/README.md` for TypeScript, Go and Ruby, and `README-PUBLIC.md` in each other library and database. `sdlc/scripts/release-workflow` runs it, and `sdlc/scripts/registry-readmes-self-test.py` covers it.
- 0581 makes every library call return `value`, `probability`, `results` and `facts`. 0582 gives each SQL function one `thinkthen_<function>_details` route. Both change every code example.

## Decisions

- PM decision, 2026-10-13: every public package README, the DuckDB README page, the DuckDB descriptor's `extended_description` and the PR description open with one paragraph word for word. Its single source is the root README's first paragraph from 0575, and the check compares every page against it. Only the install line and the example differ.
- PM decision, 2026-10-13: each page says the library is free and MIT licensed, that hosted models charge per token and that local models are free. It names Jev, SystemOne, OpenAI and local models as backends, shows the one line that sets the key, such as `export THINKTHEN_API_KEY=...`, and stays under 40 lines. Titles read "ThinkThen for Python". Say "DuckDB 1.5.6 and 2.0", never "preview".
- PM decision, 2026-10-13: each example shows real use: at least two functions over several records, such as decide and choose over a tickets table in SQL or a list in a library. No plan-only or single-sentence examples. The release check replays every page's example as `site/scripts/check-binding-samples.mjs` replays the site samples, and pages share recordings with 0575.
- PM decision, 2026-10-13: the root `README.md` stays the docs team's README from 0575, and Composer and Packagist render it. No public page links `sdlc/`. One DuckDB page only.
- PM decision, 2026-10-13, from Ian through the marketing lead: this team's own reviewer runs the fresh-reader test of these texts under 0575, and the PM brings passing texts straight to Ian. Nothing posts to the DuckDB PR or any registry before Ian approves the exact words.
- PM decision, 2026-10-14: code example rewrites wait until 0581 and 0582 merge into `ticket/0543-final`. Then each example reads `result.value` and `result.probability` in libraries and `thinkthen_<function>_details` in SQL. Prose and openings continue now.

## Scope

- Changes: `libraries/*/README-PUBLIC.md`, `libraries/dart/flutter/README-PUBLIC.md`, `libraries/typescript/README.md`, `libraries/go/README.md`, `libraries/ruby/README.md`, `databases/*/README-PUBLIC.md`, `crates/thinkthen/README.md`, `sdlc/scripts/registry-readmes.py`, `sdlc/scripts/registry-readmes-self-test.py` and `sdlc/records/0578*`.
- Rewrite each page's prose against the merged APIs on `ticket/0543-final` now. After 0581 and 0582 merge, rewrite each code example and reference, replay it against the installed package and check its length before review. Do PostgreSQL last.
- Collect every failing check in one run of `registry-readmes.py` and fix all of them before asking for the next review.
- Keeps: each library's in-repo developer README where it differs from the public one.

## Proof

- `sdlc/scripts/registry-readmes.py sources` and its self-test pass on the candidate, and the self-test's fixture README that names an ADR fails.
- `sdlc/scripts/registry-readmes.py examples` over the candidate's release packages replays every published package page and the DuckDB page, and the self-test's broken example fixture fails.
- Each built package's rendered description matches its approved source.
- Ian approves the exact texts.

## Defers

The DuckDB descriptor and PR stay with 0577. The site and the fresh-reader test stay with 0575. Publication stays with 0425.

## Progress

PM amendment, 2026-10-14. To break the merge cycle, each 0581 or 0582 binding lane may change the public example code that its existing installed checks replay, and only as far as the move to `result.value` and `result.probability` requires. It changes no prose. This ticket and 0575 still own the example text and its fresh-reader review.

PM amendment, 2026-10-14. Work starts per language. A language or database whose 0581 or 0582 lane is merged into the 0543 final branch may have its examples and text rewritten now, on its own lane, without waiting for the other languages. The fresh-reader review still runs once all are done.
