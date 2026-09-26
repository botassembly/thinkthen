# The site pull names a private deck folder

Status: Closed 2026-09-26 by the site landing. `site/scripts/pull-examples.mjs` is gone, and the README and the blog page name no private folder. `sdlc/scripts/lint` with the private-name list finds no name.

Filed on 2026-09-25 from a read of `site/` on main at 20e9b8d4.

## What happens

This repository is public. The site's example pull script and its README name a private deck folder by its full path. The path must go.

- `site/scripts/pull-examples.mjs`, lines 21 and 22. `DECK` defaults to an absolute path on one machine. The path points at a private deck folder.
- `site/README.md`, lines 34 and 35. The README says the pull reads that private deck folder and gives its path.

Two more places name the same private repository. They point at a private drafts folder for the first article.

- `site/README.md`, line 51.
- `site/src/pages/blog/code-that-understands.astro`, lines 2 and 3.

## What to do

1. Remove the default path from `pull-examples.mjs`. The script stops with a plain message when `THINKTHEN_DECK` is unset.
2. Rewrite the README lines to say the pull reads the folder named in `THINKTHEN_DECK`. Name no private path.
3. Rewrite the article lines to say the article is copied from its source draft. Name no private path.
4. Item 5 of `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md` moves the pull to the landed surface files. The path can go with that change. It must not outlive it.

## Proof

A grep of `site/` for `/home/` and for the private repository's name prints nothing.
