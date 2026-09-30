# thinkthen.dev

The site lives in this folder of the thinkthen repository, so the code and the site go public together (Ian, 2026-09-22). The Pages workflow at `.github/workflows/pages.yml` builds from here when someone runs it by hand.

The site for ThinkThen. Astro, static output, served by GitHub Pages at `thinkthen.dev`.

Tagline on every page: **ThinkThen: code that knows what you mean.**

## Build it

```
cargo build --release
npm install
npm run build
```

`npm run build` does these things in order:

1. `scripts/write-version.mjs` writes `public/version.json` with the commit it is building.
2. `scripts/check-samples.mjs` checks the rules in `WRITING.md` that a script can see: line length, asserts, whole details, no comments, and a goal on every page.
3. `scripts/check-slides.mjs` checks that the Beatles Bench slides come from a deck that quotes the pinned bench.
4. `scripts/smoke.mjs` runs every example against the command built from this commit and compares what it printed with the saved output.
5. `astro build` writes `dist/`. It fails when a script has no caption or no saved output. The Settings page reads `../specification/settings.md`, and the build fails when that table's columns change.
6. `scripts/emit-md.mjs` writes a Markdown twin of every page and `dist/llms.txt`.
7. `pagefind --site dist` builds the search index from each page's `main`, without button labels such as "Copy". The Markdown twins, the redirects, the 404 page and the search page stay out.
8. `scripts/write-sitemap.mjs` writes `dist/sitemap.xml` with every page except the redirects, the 404 page and the search page.
9. `scripts/check-settings.mjs` fails the build when the Settings page and `../specification/settings.md` disagree. It also fails when a source file types a number after "default is", "defaults to" or "default of", and when a built page states a default the table does not hold.
10. `scripts/check-links.mjs` fails the build on a broken internal link, or a link to an anchor the page does not hold.
11. `scripts/check-cards.mjs` fails the build when a page lacks its social card.
12. `scripts/check-head.mjs` fails the build when a page lacks its canonical link, icons, manifest or theme colours, or when the sitemap, the search index, `robots.txt` or the home page's JSON-LD is wrong.

A page reads a setting's default, range or allowed values with `setting('Name')` from `src/lib/settings-table.mjs`: `.default`, `.number`, `.range`, `.bounds`, `.allowed`, `.note`, `.defaultOn('decide')` and `.surface('Configuration file')`. A name the table does not hold fails the build at that call.

`npm run dev` serves the site while you work. `npm run check` runs the sample check, the slide check, the smoke run, the settings check, the link check, the card check and the head check on the last build.

## Writing a page

Read `WRITING.md` first. It holds the page rules, the code rules, and how an example is added, recorded, and refreshed.

## The menu

Five entries: Install, Functions, How-tos, Learn, and Blog. Trust, Reference, and What it will not do sit in the footer and on the Learn page. Moved pages keep their old address through the redirects in `astro.config.mjs`.

## Where the examples come from

Every command and code sample on the site is a file under `examples/`. `src/data/samples.mjs` reads them for the pages, and `src/lib/remark-examples.mjs` puts them into the articles. The smoke run replays every script from a saved recording, with no key and no network. `WRITING.md` gives the layout, the skip list, and the refresh steps.

The names, the order, the captions, the one line for each function, and the option tables live in `src/data/catalog.mjs`.

## Where the Beatles Bench pages come from

The pages under `/learn/beatles-bench/` follow the talk "Analyzing the Beatles using Jev". `src/data/beatles.mjs` holds each page's words. The slides sit in `public/learn/beatles-bench/`. The scripts sit in `examples/beatles/`, and `examples/beatles/bench/` holds the files they read from the bench commit in `examples/beatles/bench-pin`. To copy those files again from a checkout at that commit:

```
BEATLES_BENCH=path/to/beatles-bench npm run pull-bench
```

The slides come from the talk's deck, which quotes one bench commit. The deck's own build renders and commits each `slide.png`. `src/data/slides.json` names the deck slide behind each image, the deck commit, the bench the deck quotes, and each image's SHA-256. To export them again from the deck's committed `slide.png` files:

```
DECK=path/to/deck npm run export-slides
```

The export stops unless the deck's `BENCH_AT` names the bench in `examples/beatles/bench-pin`. It trusts the deck's build to have rendered the slides after that pin moved. `scripts/check-slides.mjs` runs in the build. It fails when the recorded bench differs from `examples/beatles/bench-pin`, or when an image differs from its recorded SHA-256. Move the pin and the build fails until the slides are exported again.

## The Bash techniques

`/how-tos/bash/` teaches the shell forms: `if`, `case`, exit codes, bands, loops, pipes, `xargs`, and a CI gate. The shell recipes sit beside them. `TECHNIQUES` and `RECIPES` in `src/data/catalog.mjs` list the pages, and `examples/how-tos/bash/` holds their scripts.

## Drafts

A blog post in `src/articles/` with `draft: true` builds under `npm run dev`, or when `THINKTHEN_DRAFTS=1` is set for a build. A normal build leaves it out.

## The deploy proof

A docs deploy can fail while the old build keeps serving, over and over, with a green log. So every build writes the commit it came from to `/version.json`, and `.github/workflows/pages.yml` ends by polling `https://thinkthen.dev/version.json` for that exact commit. The deploy is not finished until the site serves it.

Check by hand:

```
curl -s https://thinkthen.dev/version.json
```

## Search

The search box in the header sends the words to `/search/`, where Pagefind's own interface shows the results. The index is plain files under `dist/pagefind/`, so search needs no server. It exists only after `npm run build`, so search does not work under `npm run dev`.

## The icons

`favicon.ico` (16, 32 and 48 pixels), `apple-touch-icon.png` (180), `icon-192.png` and `icon-512.png` sit in `public/`. `scripts/build-icons.mjs` renders them from `public/brand/thinkthen-mark-dark.svg` with resvg. Run it when the mark changes, and commit what it writes:

```
npm run icons
```

`site.webmanifest` names the two large icons. The SVG marks stay as the icon for any browser that reads SVG.

## The noindex switch

The site stays unlinked until launch, so every page carries `<meta name="robots" content="noindex">`. One constant turns it off:

```
src/data/catalog.mjs  ->  export const NOINDEX = true;
```

Set it to `false`, build, and the tag is gone from every page. `robots.txt` follows the same constant: it turns every crawler away while `NOINDEX` is true, and names the sitemap once it is false. The 404 and search pages keep `noindex` always.

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

Dark follows the system and is the default. The sun and moon button in the header overrides it, and the choice is kept for the visit.

## The language choice

The tabs on the home page and the cross-view on every function page share one choice, kept in `localStorage` under `tt-lang`. Pick Ruby once and every code block on every page reads Ruby, falling back to Bash where a surface has no sample. Plain JavaScript, no framework on the client.

## What is not here yet

- A pandas page, and a page for any serve mode.
- An installed-host smoke runner for library and database samples. `examples/SKIP` identifies the samples the CLI replay runner does not execute; each changed sample needs its own checked-package or source-example proof.
