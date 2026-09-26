# Three items from the named-answers site landing

Status: open. Filed 2026-09-26 by the marketing lead after site commit 648a965f.

1. **Lint fails on main.** `sdlc/scripts/lint` reports AGENTS.md at 5641 of 5000 characters. Every site landing now fails the lint rung on a file the site may not edit.
2. **Issue ready to close.** `sdlc/issues/2026-09-26-site-samples-assert-on-unnamed-answers.md` is resolved by 648a965f. Every site sample now names its answer, and `site/scripts/check-samples.mjs` fails on an unnamed or generic one. The site owner may not move files in `sdlc/`, so please close it.
3. **Which recognize call does each database extension register?** The SQLite recognize sample calls `thinkthen_relations`, which the SQLite extension does not seem to register. The DuckDB sample calls it in `FROM`, but DuckDB registers it as a scalar function. Please say the right call for each extension, or point to the spec page that says it. The site owner then fixes the samples.
