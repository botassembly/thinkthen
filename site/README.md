# thinkthen.dev

The site lives in this folder of the thinkthen repository, so the code and the site go public together (Ian, 2026-09-22). The Pages workflow at `.github/workflows/pages.yml` builds from here on a push to `main` that touches `site/`.


The site for ThinkThen. Astro, static output, served by GitHub Pages at `thinkthen.dev`.

Tagline on every page: **ThinkThen: code that knows what you mean.**

## Build it

```
npm install
npm run build
```

`npm run build` does four things in order:

1. `scripts/write-version.mjs` writes `public/version.json` with the commit it is building.
2. `astro build` writes `dist/`. It fails if any function-and-surface cell is missing or carries no status.
3. `scripts/emit-md.mjs` writes a Markdown twin of every page and `dist/llms.txt`.
4. `scripts/check-links.mjs` fails the build on a broken internal link.

`npm run dev` serves the site while you work.

## Where the examples come from

Every code sample on this site is pulled from somewhere else. Nothing is typed into a page by hand, so a page cannot drift from the code.

```
npm run pull
```

`scripts/pull-examples.mjs` reads the deck at
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands` (override with `THINKTHEN_DECK`) and writes one file per function-and-surface cell into `src/data/examples/`. Those files are committed, so a build never reaches outside this repository.

Each cell carries a status:

| Status | What it means | Where it comes from |
| --- | --- | --- |
| `run` | The command really printed this. | The command from the deck's `examples/run.sh`, the output from `examples/out/*.txt`, and the exit code from the last line of that file. |
| `drawn` | Nobody has run this. It is the shape the library is being built to. | The deck's `surfaces.md` and `recognize-surfaces.md`. Every drawn sample is labelled "drawn, not run" on the page. |
| `planned` | Nothing is written for this cell yet. | The page says so in place of code. |

The how-to pages work the same way: the commands come from the deck's `usecases/run.sh` and the output from `usecases/out/`.

`src/data/examples.mjs` checks every cell at build time. A missing file, an unknown status, a `run` cell with no example, or a `drawn` cell with no code all fail the build.

The names, the order, the one line for each function, and the option tables live in `src/data/catalog.mjs`. The one line for each function is the help text's first line, copied from the vocabulary page.

The first article is copied byte for byte into `src/articles/code-that-understands.md` from `repos/mktg/content/thinkthen/drafts/01-code-that-understands/article.md`. Edit it there and copy it again.

## The deploy proof

A docs deploy can fail while the old build keeps serving, over and over, with a green log. So every build writes the commit it came from to `/version.json`, and `.github/workflows/pages.yml` ends by polling `https://thinkthen.dev/version.json` for that exact commit. The deploy is not finished until the site serves it.

Check by hand:

```
curl -s https://thinkthen.dev/version.json
```

## The noindex switch

The site stays unlinked until launch, so every page carries `<meta name="robots" content="noindex">`. One constant turns it off:

```
src/data/catalog.mjs  ->  export const NOINDEX = true;
```

Set it to `false`, build, and the tag is gone from every page.

## The palette

Phosphor, from the site plan. Four outcomes, four colours, everywhere an answer or an exit code shows.

| Role | Dark | Light |
| --- | --- | --- |
| ground | `#0e1311` | `#faf8f1` |
| card | `#18211d` | `#ffffff` |
| ink | `#e6eee8` | `#18201c` |
| muted | `#8a9a90` | `#5d6b63` |
| think, "not sure", exit 3 | `#ffb000` | `#a56300` |
| then, "yes", exit 0 | `#2fd27a` | `#0b8046` |
| "no", exit 1 | `#ff5d5d` | `#c62828` |
| "broken", other exits | `#7d8a83` | `#6b746f` |

Dark follows the system and is the default. The Theme button in the header overrides it and the choice is kept for the visit.

## The language choice

The tabs on the home page and the cross-view on every function page share one choice, kept in `localStorage` under `tt-lang`. Pick Ruby once and every code block on every page reads Ruby, falling back to Bash where nothing is written yet. Plain JavaScript, no framework on the client.

## What is not here yet

- `/install.sh`. The download script is not built. The install page shows the line and marks it "coming with 0.1".
- Pages for Excel, Google Sheets, pandas, and any serve mode. None of them has run.
- A tutorial page, a backends page, a generated reference, and a refusals page. They come after the first version.
