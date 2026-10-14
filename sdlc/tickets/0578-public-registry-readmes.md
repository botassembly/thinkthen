# 0578: Give every published package a public README

Status: OPEN.

Milestone: 0.2

Depends on: 0574
Depends on: 0581
Depends on: 0582

## Outcome

Each package registry page that ships with 0.2 shows a short README written for a stranger: what ThinkThen is, one example in that language, what it costs, and how to install it. No registry page names an ADR, a ticket, an `sdlc/` path, an internal crate or a stale version. A release check fails the release if one does.

## Evidence

- Starts from: the marketing lead's live check of the registries on 2026-10-13.
  - PyPI thinkthen 0.1.2 opens its description with "The binding is the unpublished crate thinkthen-python in its own Cargo workspace (ADR 0047). The design page is sdlc/planning/libraries/python.md."
  - npm thinkthen 0.1.2 shows "No README data found!".
  - RubyGems holds only the 0.0.1 name placeholder.
  - The DuckDB maintainer flagged the same problem on the community listing, which 0577 covers.
- Keeps: each library's in-repo developer README, if it differs from the published one.
- Changes:
  - Write one public README per published package: PyPI, npm, RubyGems, crates.io, Maven Central, NuGet, Go, Swift, Packagist, pub.dev, CRAN or r-universe, and the database extensions, for whichever of these 0.2 publishes. Each says what ThinkThen is, shows one example, states that the library is free and MIT licensed while hosted models charge per token and local models are free, and shows the install line.
  - Point each package's metadata at its public README, so the registry renders it.
  - Add a release check that reads every package's published README and fails on an ADR number, a ticket number, an `sdlc/` path, an internal crate or project name, or a version other than the one being released.
  - The PM sends the texts to the marketing lead for Ian's approval before anything publishes.
- Proof:
  - The release check passes on the candidate and fails on a fixture README that names an ADR.
  - Each built package's rendered description matches its approved README.
  - Ian approves the texts.
- Defers: nothing.

## Progress

PM amendment, 2026-10-13. Review of the drafts at 53f10e414 before they go to marketing. Revise every public README this way. Capitalize the language name in each title, as in "ThinkThen for Python". Delete "Failed calls remain separate from answers." Say in one sentence why a developer would use ThinkThen over calling a model directly: answers code can branch on, each question paid once through the cache, and offline replay for tests. Name Jev, SystemOne, OpenAI and local models as backends, per Ian's 2026-10-13 search ruling. Replace "Configure a model backend" with the one line that sets the key, such as `export THINKTHEN_API_KEY=...`. Show one typed answer read in each example, never `results[0]`. Keep each page under 40 lines.

PM amendment, 2026-10-13, from the marketing lead. This replaces the 457bba8fa wording rules. (1) Every public package README, the one DuckDB README page, the DuckDB descriptor's `extended_description` and the PR description open with the reviewed pitch from the docs team's README on `docs/0.2-refresh`: the ten functions, then why not call the model yourself: typed answers code branches on, not sure instead of a guess, many records packed into few requests, the cache, and any model (Jev, OpenAI or a local model) in every language and database. Reuse one opening paragraph word for word on every page; only the install line and the example differ. Keep the free, MIT and per-token lines. Delete `sdlc/records/0577-duckdb-readme-draft.md`; one DuckDB page only. (2) Each example shows real use: at least two functions over several records, such as decide and choose over a tickets table in SQL or a list in a library, read with that library's one-line typed answer. No plan-only or single-sentence examples. Say "DuckDB 1.5.6 and 2.0", never "preview". (3) The release check replays the example on every published package page and on the DuckDB page, as the site replays its samples, and fails the release on any that does not run. This ticket therefore follows 0574. (4) Send the rewritten texts to the docs team for its fresh-reader page test. Only texts that pass go to marketing for Ian. Post and push nothing to the DuckDB PR or any registry before Ian approves the exact words.

PM amendment, 2026-10-13, from the marketing lead. Replace point (4) above: send the rewritten texts to marketing, not the docs team. Marketing runs the fresh-reader page test, returns failures to the PM, and brings passing texts to Ian.

PM amendment, 2026-10-13, from Ian through the marketing lead. Replace the routing above: the fresh-reader test of these texts runs under 0575 by this team's own reviewer, and passing texts go from the PM straight to Ian.

PM amendment, 2026-10-13. Remaining work. Done: 25 public README sources and their metadata routing, and `sdlc/scripts/registry-readmes.py` wired into `sdlc/scripts/release-workflow` (e6dca4b3e) with self-tests. Merge 9a6408dcc and c17f91b4d from `ticket/0578-public-registry-readmes` into `ticket/0543-final`. Rulings: (1) The root `README.md` stays the docs team's README from `docs/0.2-refresh`. Revert the root replacement, and delete the "ThinkThen for the Command Line and PHP" page and `README-DEVELOPMENT.md`. Composer and Packagist render the root README, and the check validates only its opening paragraph and versions. (2) The shared pitch has one source: 0575 makes this ticket's `OPENING` paragraph the root README's first paragraph, and the check compares every page against it. (3) Point the root README's ADR 0055 link at the site's "Pay once" page, so no public page links `sdlc/`. Remaining work: rewrite every page example to use at least two functions over several records with the 0574 one-line typed answer; TypeScript, C, Zig and COBOL wait on 0574. Add an example replay mode to `registry-readmes.py` that replays each page's example against saved answers as `site/scripts/check-binding-samples.mjs` does, with a failing fixture page. Pages and site samples share recordings with 0575. Proof: `registry-readmes.py sources` and its self-test pass on the candidate, the ADR fixture fails, and every built package's text matches its approved source.

PM amendment, 2026-10-14. Four rejects came from examples, references, lengths and replays drifting while the 0576 language lanes changed the public APIs. Those lanes have now merged everything except PostgreSQL. Restart from current `ticket/0543-final`: rewrite each package example and reference against the merged API, replay it against the installed package, and check length before review. Do PostgreSQL last, after its whole-library review. Collect every failing check in one run, and fix all of them before the next review.

PM amendment, 2026-10-14. 0581 and 0582 change how every library and database returns values and details. Hold code example rewrites until they land, then rewrite each example against `result.value`, `result.probability` and `thinkthen_<function>_details`. Prose and openings continue now.
