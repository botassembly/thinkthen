# A worked-examples section on the site, pulled from Beatles Bench

Status: Open.

Filed on 2026-09-25 by bench ticket 0010, on Ian's request of that day. Ian asked for a section of about 15 pages on the ThinkThen docs site. Each page names the two or three files a reader needs, gives the command that runs them, and shows why Jev got one answer right and another wrong. The pages now live in the Beatles Bench repository. The site change is this issue.

## What exists

Bench commit 9d7f1820 holds the pages. `docs/README.md` lists 16 of them in reading order:

1. Beatles Bench (`README.md`)
2. How to run the bench for free (`docs/run-it-for-free.md`)
3. The data (`docs/the-data.md`)
4. to 13. One how-to per function, from decide to relate (`examples/01-decide/README.md` to `examples/10-relate/README.md`)
14. How to find your bar with audit (`examples/11-audit/README.md`)
15. How to see what changed with diff (`examples/12-diff/README.md`)
16. Context and cost (`docs/context-and-cost.md`)

Each function page follows ADR 0011's how-to form, adapted to the bench. Its title starts with "How to", and it has "What can go wrong" and related pages. Every command answers from a committed recording with `--replay recording`, so a reader needs no key. The bench's `tests/test_examples.py` runs every command on every page and checks that it prints the block below it, byte for byte.

## What the site needs

1. **A pull script.** A new `site/scripts/pull-bench.mjs`, beside `pull-examples.mjs`, reads a bench checkout at a pinned commit. It copies each page in `docs/README.md`, in order, into `site/src/data/bench/`, with the images each page links (`slide.png`). The pulled files are committed, so a build never reaches outside the repository.
2. **A pin.** One committed file names the bench commit the pages came from. The pull refuses a checkout at another commit. The page footer names the commit.
3. **Links.** A link to another listed page becomes that page's site route. Any other relative link, such as `outputs.jsonl` or `data/songs.tsv`, becomes a GitHub link at the pinned commit. `check-links.mjs` then covers the section.
4. **A section.** A "Worked examples" entry in the site's navigation, with the 16 pages in order. The how-to index can list the ten function pages beside the thinkthen demos.
5. **Credits and licenses.** The bench data is CC BY-SA 4.0, because it is compiled from Wikipedia. The choose and relate pages credit the Commons photo in their slides. The pulled pages keep those lines, and the section names the bench's license.

The site runs none of the commands. The bench's tests prove the blocks at the pinned commit.

## Ian's rulings that apply

- Ian, 2026-09-24: "We're not doing the marketing and not publishing the website until everything's done." The section ships with the rest of the site.
- The pages carry no status words.

## Open questions for the site's owner

- The pages say "Run this in `examples/08-annotate`". On the site, a reader has no checkout until page 2. The pull could prefix each function page with a one-line pointer to page 2.
- `./run.sh` needs a build with `audit`: thinkthen main at 02dc0b96 or later. The pages say so until a release carries it. After the release, the bench pages can name the release, and the pin moves.

Done when the site builds the section from the pinned bench commit, `check-links.mjs` passes over it, and a bench pin bump is one pull and one commit.
