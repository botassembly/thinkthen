# 0578: Give every published package a public README

Status: OPEN.

Milestone: 0.2.1

Depends on: 0574
Depends on: 0581
Depends on: 0582
Depends on: 0688
Depends on: 0690

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
Reviews: revision 054013a, accept
Reviews: revision 9939efe, accept
Reviews: revision f6ce54a, accept

Reviews: revision 8ac420e, accept

Reviews: revision 81ebcc8, accept

Reviews: revision 3d93e2d, accept

Reviews: revision ea33174, accept

## Outcome

- Changes: `databases/duckdb/README-PUBLIC.md`, `databases/postgresql/README-PUBLIC.md`, `databases/sqlite/README-PUBLIC.md`, `libraries/ada/README-PUBLIC.md`, `libraries/c/README-PUBLIC.md`, `libraries/cobol/README-PUBLIC.md`, `libraries/cpp/README-PUBLIC.md`, `libraries/csharp/README-PUBLIC.md`, `libraries/dart/README-PUBLIC.md`, `libraries/jvm/README-PUBLIC.md`, `libraries/objective-c/README-PUBLIC.md`, `libraries/php/README-PUBLIC.md`, `libraries/python/README-PUBLIC.md`, `libraries/r/README-PUBLIC.md`, `libraries/swift/README-PUBLIC.md`, `libraries/zig/README-PUBLIC.md`, `libraries/dart/flutter/README-PUBLIC.md`, `libraries/typescript/README.md`, `libraries/go/README.md`, `libraries/ruby/README.md`, `crates/thinkthen/README.md`, `sdlc/scripts/registry-readmes.py`, `sdlc/scripts/registry-readmes-self-test.py`.
- Each package registry page that ships with 0.2.1 shows a short README for a stranger with the shared opening, one replayed example in that language, the cost and the install line. `sdlc/scripts/registry-readmes.py` fails the release on any page that names an ADR, a ticket, an `sdlc/` path, an internal crate or a stale version, or whose example does not run.
- Each page's example reads the language's final call surface after phase 3, and a question set in an example carries no `version` field (`sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`). The install line names the 0.2.1 package; package release numbers stay.
- Ian approved the prose at 2e52b3334. The prose stays except sentences that describe a changed example. Changed prose goes back to Ian before anything publishes.
- PM decision, 2026-10-13: every public package README, the DuckDB README page, the DuckDB descriptor's `extended_description` and the PR description open with one paragraph word for word. Its single source is the root README's first paragraph from 0575, and the check compares every page against it. Only the install line and the example differ.
- PM decision, 2026-10-13: each page says the library is free and MIT licensed, that hosted models charge per token and that local models are free. It names Jev, SystemOne, OpenAI and local models as backends, shows the one line that sets the key, such as `export THINKTHEN_API_KEY=...`, and stays under 40 lines. Titles read "ThinkThen for Python". Say "DuckDB 1.5.6 and 2.0", never "preview".
- PM decision, 2026-10-13: each example shows real use: at least two functions over several records, such as decide and choose over a tickets table in SQL or a list in a library. No plan-only or single-sentence examples. Pages share recordings with 0575.
- PM decision, 2026-10-13: the root `README.md` stays the docs team's README from 0575, and Composer and Packagist render it. No public page links `sdlc/`. One DuckDB page only.
- PM decision, 2026-10-13: this team's reviewer runs the fresh-reader test under 0575, and the PM brings passing texts to Ian. Nothing posts to the DuckDB PR or any registry before Ian approves the exact words.
- Each library's in-repo developer README stays where it differs from the public one. The DuckDB descriptor and PR stay with 0577. The site and the fresh-reader test stay with 0575. Publication stays with 0425.
- Work runs per language: a language whose phase 3 tickets have landed has its example rewritten on its own lane. PostgreSQL goes last. One run of `registry-readmes.py` collects every failing check before each review.

### Proof

- On the M5: `pm job run --machine m5 0578 SHA --memory 8 -- sh -c 'python3 sdlc/scripts/registry-readmes-self-test.py && python3 sdlc/scripts/registry-readmes.py sources'` passes. The self-test's fixture README that names an ADR fails, and its broken example fixture fails.
- At the 0691 checkpoint, `python3 sdlc/scripts/registry-readmes.py examples DIR` over that revision's release packages replays every published package page and the DuckDB page. Families that do not build on the M5 run at the 0619 container checkpoint.
- Each built package's rendered description matches its source page.

## Evidence

- Starts from: the marketing lead's live registry check of 2026-10-13. PyPI thinkthen 0.1.2 opens its description with an ADR number and an `sdlc/` path, npm thinkthen 0.1.2 shows "No README data found!", and RubyGems holds only the 0.0.1 name placeholder.
- `sdlc/scripts/registry-readmes.py` maps each package to its source (`check_sources`) and replays examples from release packages (`check_examples`, `examples`): the root `README.md` for the command line and Packagist, `crates/thinkthen/README.md` for crates.io, `libraries/<lang>/README.md` for TypeScript, Go and Ruby, and `README-PUBLIC.md` in each other library and database. `sdlc/scripts/release-workflow` runs it, and `sdlc/scripts/registry-readmes-self-test.py` covers it. `site/scripts/check-binding-samples.mjs` replays the site samples the same way.
- 0581 and 0582 landed the `value`, `probability`, `results` and `facts` reads and the `thinkthen_<function>_details` SQL route. The phase 3 language tickets change each language's calls again, so examples wait for 0690, which waits on every phase 3 call ticket.
- 0597 rewrites the same example lines in Ian's style and depends on this ticket.

## Progress

PM amendment, 2026-10-14. To break the merge cycle, each 0581 or 0582 binding lane may change the public example code that its existing installed checks replay, and only as far as the move to `result.value` and `result.probability` requires. It changes no prose. This ticket and 0575 still own the example text and its fresh-reader review.

PM amendment, 2026-10-14. Work starts per language. A language or database whose 0581 or 0582 lane is merged into the 0543 final branch may have its examples and text rewritten now, on its own lane, without waiting for the other languages. The fresh-reader review still runs once all are done.

Ian approval, 2026-10-14. Ian approved the fresh-reader-accepted public text at 2e52b3334: the registry pages, the DuckDB page, the DuckDB descriptor and the listing PR description. Publishing still waits for his go.
