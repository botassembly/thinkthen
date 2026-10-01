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
5. `scripts/check-binding-proofs.mjs` checks each replayed library sample against its proof. See "The binding replay" below.
6. `astro build` writes `dist/`. It fails when a script has no caption or no saved output. The Settings page reads `../specification/settings.md`, and the build fails when that table's columns change.
7. `scripts/emit-md.mjs` writes a Markdown twin of every page and `dist/llms.txt`.
8. `scripts/check-code.mjs` fails the build when a built page shows a code block that `src/lib/code.mjs` did not draw, a code block with no colour, or colour in an output pane.
9. `scripts/check-words.mjs` fails the build when page prose uses a retired word, such as "unsure" or a status word. `WRITING.md`, "Pages", lists them.
10. `pagefind --site dist` builds the search index from each page's `main`, without button labels such as "Copy". The Markdown twins, the redirects, the 404 page and the search page stay out.
11. `scripts/write-sitemap.mjs` writes `dist/sitemap.xml` with every page except the redirects, the 404 page and the search page.
12. `scripts/check-settings.mjs` fails the build when the Settings page and `../specification/settings.md` disagree. It also fails when a source file types a number after "default is", "defaults to" or "default of", and when a built page states a default the table does not hold.
13. `scripts/check-links.mjs` fails the build on a broken internal link, or a link to an anchor the page does not hold.
14. `scripts/check-cards.mjs` fails the build when a page lacks its social card.
15. `scripts/check-head.mjs` fails the build when a page lacks its canonical link, icons, manifest or theme colours, or when the sitemap, the search index, `robots.txt` or the home page's JSON-LD is wrong.

The Backends pages read each built-in backend's name, address, key variables and model with `backend('name')` from `src/lib/backends-table.mjs`, which parses the "Named backends" table in `../specification/backends.md`. A name the table lacks fails the build, and `scripts/check-settings.mjs` fails when the built overview disagrees with the table. A live `thinkthen check` cannot replay, so `src/data/backend-checks.mjs` holds each live check a page states, with its date, build and record.

A page reads a setting's default, range or allowed values with `setting('Name')` from `src/lib/settings-table.mjs`: `.default`, `.number`, `.range`, `.bounds`, `.allowed`, `.note`, `.defaultOn('decide')` and `.surface('Configuration file')`. A name the table does not hold fails the build at that call.

`npm run dev` serves the site while you work. `npm run check` runs the sample check, the slide check, the smoke run, the binding proof check, the code check, the word check, the settings check, the link check, the card check and the head check on the last build.

## Writing a page

Read `WRITING.md` first. It holds the page rules, the code rules, and how an example is added, recorded, and refreshed.

## The menu

Five entries: Install, Functions, How-tos, Learn, and Blog. The Backends pages under `/install/backends/` share a side list, `BACKEND_PAGES` in `src/data/catalog.mjs`. Trust, Reference, and What it will not do sit in the footer and on the Learn page. Moved pages keep their old address through the redirects in `astro.config.mjs`.

## Where the examples come from

Every command and code sample on the site is a file under `examples/`. `src/data/samples.mjs` reads them for the pages, and `src/lib/remark-examples.mjs` puts them into the articles. The smoke run replays every script from a saved recording, with no key and no network. `WRITING.md` gives the layout, the skip list, and the refresh steps.

The names, the order, the captions, the one line for each function, and the option tables live in `src/data/catalog.mjs`.

## The binding replay

The CLI smoke run cannot run a library sample, because each needs its language's toolchain. `examples/REPLAY` lists the library samples that replay. `npm run smoke-bindings` runs `scripts/smoke-bindings.mjs`, which builds each sample's binding from this working tree and runs the sample. The sample runs in a fresh folder with a copy of its page's `files/`. `THINKTHEN_CACHE` names a fresh copy of `recordings/thinkthen.jsonl`, and no key or address is set, so a missed answer fails with no request sent. The runner refuses to start when the shell holds any `THINKTHEN_` variable or a named backend's key.

The runner finds the recorded answers each sample read, and writes `examples/bindings-proof.json`. Each entry holds the hashes of the sample, its saved output, its `files/`, and each answer it read. It also holds a tree hash of the binding's folder and of each Cargo workspace member it builds on, and the toolchain versions.

`scripts/check-binding-proofs.mjs` runs in every build with Node and git alone. It fails when a listed sample has no entry, or a sample, its saved output, a file it reads, or an answer it read changed after its proof. A change in a binding or the engine only warns, and names each page to prove again, so another queue's commit never turns the site build red. The marketing lead reruns `npm run smoke-bindings` at each published checkpoint tag and commits the new proof file. The build cannot check that a run happened, so trust rests on review: the code reviewer reruns `npm run smoke-bindings` and diffs the proof file. The same check fails an install line that does not name the package its binding's metadata names. It also fails a release archive name that `sdlc/scripts/release-pack` does not make.

The replay needs Rust with the offline Cargo cache, a stable Python 3.12 or later with uv and maturin for the Python and pandas samples, and R 4.2 or later with dplyr for the R samples. The C++, Objective-C, COBOL and Ada samples need g++, gcc with Objective-C, GnuCOBOL and GNAT. They link the C library that `sdlc/scripts/installed.sh` lays out, and the runner compiles each sample once. A missing toolchain reports "not run" and keeps the old entry.

## Where the Beatles Bench pages come from

The pages under `/learn/beatles-bench/` follow the talk "Analyzing the Beatles using Jev". `src/data/beatles.mjs` holds each page's words. The slides sit in `public/learn/beatles-bench/`. The scripts sit in `examples/beatles/`, and `examples/beatles/bench/` holds the files they read from the bench commit in `examples/beatles/bench-pin`. To copy those files again from a checkout at that commit:

```
BEATLES_BENCH=path/to/beatles-bench npm run pull-bench
```

The slides come from the talk's deck, which quotes one bench commit. The deck's own build renders and commits each `slide.png`. `examples/beatles/deck-pin` names the deck commit the pages show. `src/data/slides.json` names the deck slide behind each image by name, the deck commit, the bench the deck quotes, and each image's SHA-256. An entry may name its own deck commit to keep a slide the pinned deck dropped. To export them again from the deck's committed `slide.png` files at the pinned commit:

```
DECK=path/to/deck npm run export-slides
```

The export reads the deck at `deck-pin`, never at the checkout's HEAD. It stops unless that commit's `BENCH_AT` names the bench in `examples/beatles/bench-pin`. It trusts the deck's build to have rendered the slides after that pin moved. `scripts/check-slides.mjs` runs in the build. It fails when the recorded deck differs from `deck-pin`, when the recorded bench differs from `bench-pin`, or when an image differs from its recorded SHA-256. Move either pin and the build fails until the slides are exported again.

The Pages build cannot read the private deck, so it cannot see a newer deck. With `DECK` set, `check-slides` also fails when the deck's `slides/`, `order.txt` or PDF in its working tree differ from `deck-pin`. Slides rendered and not yet committed count. The deck's own `build.sh` runs it that way.

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

Code panes add a code palette, after the deck's code panes. Shiki colours every block at build time with one theme, `src/lib/code-theme.mjs`, whose colours are the variables below. Output panes and plain-text files stay in ink. Contrast is measured on the card.

| Code role | Dark | Light |
| --- | --- | --- |
| keys, keywords, command names | `#2fd27a`, 8.34:1 | `#0b8046`, 5.01:1 |
| quoted strings | `#ffb000`, 9.00:1 | `#a56300`, 4.79:1 |
| numbers | `#ff7eb6`, 6.99:1 | `#b0306a`, 6.04:1 |
| true, false, null | `#7aa2f7`, 6.54:1 | `#3355cc`, 6.34:1 |
| comments, heredoc input, diff headers and ranges | `#8a9a90`, 5.58:1 | `#5d6b63`, 5.60:1 |

A diff's added and removed lines stay in ink, because green and red mean yes and no.

Dark follows the system and is the default. The sun and moon button in the header overrides it, and the choice is kept for the visit.

## The language choice

The tabs on the home page and the cross-view on every function page share one choice, kept in `localStorage` under `tt-lang`. Pick Ruby once and every code block on every page reads Ruby, falling back to Bash where a surface has no sample. Plain JavaScript, no framework on the client.

## What is not here yet

- A page for any serve mode.
- A replay for the function pages' library samples and the SQL samples. `examples/SKIP` lists them.
