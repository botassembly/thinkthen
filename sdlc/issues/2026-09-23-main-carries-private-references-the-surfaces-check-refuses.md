# Main carries private references that the surfaces check refuses

Status: Fixed on branch `surfaces-wave7`. Main keeps them until the branch merges.

Found by the wave-7 gate on 2026-09-23, after the merge of main at `1b894d9`. The branch's `scripts/check_no_private_refs.py` scans every tracked file outside `sdlc/`. Main's `site/` folder and one test module failed it with 13 findings.

- `site/README.md`, `site/scripts/pull-examples.mjs`, and `site/src/pages/blog/code-that-understands.astro` named the private repository that holds the deck and the article drafts.
- `site/scripts/pull-examples.mjs` defaulted to an absolute path in the builder's home.
- `crates/thinkthen/src/cli/config.rs` used home-style paths as test inputs. They were made up and leaked nothing, but the check cannot tell them from a real home.

The fix on the branch:

- The site pages name "the deck repository" in place of the private name.
- `pull-examples.mjs` reads the deck folder only from `THINKTHEN_DECK` and stops with a message when it is unset.
- The config test uses `/srv/person` as the home. `rustfmt` then joined one wrapped line, and the crates ceiling fell from 38,070 to 38,068.

Main's own gate does not run this check, so main can add such a reference again before the merge. Ian can overturn the environment-only deck path.
