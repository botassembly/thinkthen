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
2. `scripts/named-answers.test.mjs` runs named-answer fixtures, then `scripts/check-samples.mjs` checks the rules in `WRITING.md` that a script can see: line length, asserts, saved output for each SQL sample, whole details, no comments, and a goal on every page.
3. `scripts/check-slides.mjs` checks that the Beatles Bench slides come from a deck that quotes the pinned bench.
4. `scripts/smoke.mjs` runs every example against the command built from this commit and compares what it printed with the saved output.
5. `scripts/check-flags.mjs` compares each function's flag data in `src/data/catalog.mjs` and `src/data/flags.mjs` with `thinkthen <fn> --help`. It fails on a flag the help shows and the data lacks, and on one the data lists and the help lacks.
6. `scripts/check-binding-samples.mjs` checks replay coverage and install package/archive names. See "The binding replay" below.
7. `astro build` writes `dist/`. It fails when a script has no caption or no saved output. The Settings page reads `../specification/settings.md`, and the build fails when that table's columns change.
8. `scripts/preserve-redirect-fragments.mjs` preserves incoming fragments on all compatibility stubs.
9. `scripts/label-tables.mjs` copies each column heading into the cells of a table of three or more columns. On a phone or a tablet every table row stacks into a card, and the label names each cell. The same step marks each code span of 24 characters or fewer outside a code pane with class `short`, so a model name, a flag, a key name or a version never splits across lines.
10. `scripts/check-layout.test.mjs` and `scripts/check-layout.mjs` check grid counts and compare the blog list with its source posts.
11. `scripts/emit-md.mjs` writes a Markdown twin of every listed page, `dist/llms.txt`, and `dist/llms-full.txt`, which holds every twin in one file.
12. `scripts/check-markdown-headings.mjs` first checks the six annotate h4 subsections and llms-full. `scripts/check-code.mjs` fails the build when a built page shows a code block that `src/lib/code.mjs` did not draw, a code block with no colour, or colour in an output pane.
13. `scripts/check-words.test.mjs` runs fixtures, then `scripts/check-words.mjs` fails the build when page prose uses a retired word, such as "unsure" or a status word. `WRITING.md`, "Pages", lists them.
14. `pagefind --site dist` builds the search index from each page's `main`, without button labels such as "Copy". The Markdown twins, the redirects, the 404 page and the search page stay out.
15. `scripts/write-sitemap.mjs` writes `dist/sitemap.xml` with every page except the redirects, the 404 page and the search page.
16. `scripts/check-settings.mjs` fails the build when the Settings page and `../specification/settings.md` disagree. It also fails when a source file types a number after "default is", "defaults to" or "default of", and when a built page states a default the table does not hold.
17. `scripts/check-links.test.mjs` and `scripts/check-links.mjs` fail the build on a broken internal link, or a link to an anchor the page does not hold.
18. `scripts/check-redirects.mjs` proves direct fragment navigation and fixed fallbacks with cached Chromium.
19. `scripts/check-cards.test.mjs` and `scripts/check-cards.mjs` enforce matching decoded titles/descriptions, both alt tags, the Open Graph address and matching same-site 1200 by 630 PNGs.
20. `scripts/check-head.mjs` fails the build when a page lacks its canonical link, icons, manifest or theme colours, or when the sitemap, the search index, `robots.txt`, the llms files, or the JSON-LD on the home and About pages is wrong. It also fails when two listed pages share a title or a description.

The Backends pages read each built-in backend's name, address, key variables and model with `backend('name')` from `src/lib/backends-table.mjs`, which parses the "Named backends" table in `../specification/backends.md`. A name the table lacks fails the build, and `scripts/check-settings.mjs` fails when the built overview disagrees with the table. A live `thinkthen backends check` cannot replay, so `src/data/backend-checks.mjs` holds each live check a page states, with its date, build and record.

A page reads a setting's default, range or allowed values with `setting('Name')` from `src/lib/settings-table.mjs`: `.default`, `.number`, `.range`, `.bounds`, `.allowed`, `.note`, `.defaultOn('decide')` and `.surface('Configuration file')`. A name the table does not hold fails the build at that call.

`npm run dev` serves the site while you work. `npm run check` runs the sample check, the slide check, the smoke run, the binding sample check, the code check, the word check, the settings check, the link check, the layout fixtures/check, card fixtures/check, the head check and the width check on the last build. `scripts/check-widths.mjs` opens every built page in headless Chromium at every width from 320 to 1600 px in steps of 40, and at 375, 768, 820 and 1024 px. It fails when a page scrolls sideways, when a table or a code pane is wider than its column, or when visible direct card children form rows of different counts. External browser requests are blocked. It uses `playwright-core` and its matching Chromium. Get the browser once with `npx playwright-core install chromium`.

## Lighthouse

`npm run lighthouse` runs `scripts/lighthouse.mjs` on the last build. It serves `dist/` on a local port and audits twelve pages on Lighthouse's mobile and desktop presets: the home page, Install, the Python install page, the TypeSafe backend page, Functions, the `filter` function page, the `decide` function page, Learn, a Beatles Bench page, a blog post, the tutorial and the 404 page. It prints each run's performance, accessibility, best practices and SEO scores.

It fails when a run scores below 100 on accessibility or on SEO. While `NOINDEX` is true, every page fails the SEO audit "Page is blocked from indexing" by design, and the 404 page always does. The script lets that one audit fail on those pages and fails on any other SEO audit. A build with `NOINDEX` set to false scores 100 on SEO on every page but the 404 page (ticket 0051). Performance and best practices print and never fail the run, because local timings change with the machine's load.

`npm run check` leaves it out. The 24 runs take about three minutes, three times the rest of the check. The marketing lead runs it before each deploy. `npm run lighthouse -- https://thinkthen.dev` audits the live site instead. It uses the same Chromium as the width check, or the browser `CHROME_PATH` names.

## Writing a page

Read `WRITING.md` first. It holds the page rules, the code rules, and how an example is added, recorded, and refreshed.

## The menu

The menu links Install, Functions, How-tos, Learn, Guides and Blog. Recipes appears when its published catalog is available. The Backends pages under `/install/backends/` share a side list, `BACKEND_PAGES` in `src/data/catalog.mjs`. Trust and What it will not do sit in the footer and on the Learn page. Moved pages keep their old address through the redirects in `astro.config.mjs`.

## Guides

`/guides/` links useful Learn and Recipe pages and lists published lessons.
Copy `templates/guide.md` into `src/guides/` to write a lesson. `WRITING.md`
defines the metadata, actual video/article requirements and draft exclusions.
The lesson route reuses the existing Markdown renderer and base layout.
`node scripts/guides.test.mjs` renders independent fixtures in a temporary copy,
checks actual media links and replayed example presentation, and verifies draft
exclusion from the section list, search, sitemap and Markdown exports. The
fixtures never ship as lessons. This check runs in both build and check.

## Where the examples come from

Every command and code sample on the site is a file under `examples/`. `src/data/samples.mjs` reads them for the pages, and `src/lib/remark-examples.mjs` puts them into the articles. The smoke run replays every script from a saved recording, with no key and no network. `WRITING.md` gives the layout, the skip list, and the refresh steps.

The names, the order, the captions, the one line for each function, and the option tables live in `src/data/catalog.mjs`.

## The binding replay

The CLI smoke run cannot run a library sample, because each needs its language's toolchain. `examples/REPLAY` lists the library samples that replay. `npm run smoke-bindings` runs `scripts/smoke-bindings.mjs`, which builds each sample's binding from this working tree and runs the sample. The sample runs in a fresh folder with a copy of its page's `files/`. `THINKTHEN_CACHE` names a fresh copy of `recordings/thinkthen.jsonl`, and no key or address is set, so a missed answer fails with no request sent. The runner refuses to start when the shell holds any `THINKTHEN_` variable or a named backend's key. A sample passes when it exits 0. A sample that prints, such as a stream or a table, must print its `<sample>.out` byte for byte, and the page shows that file under the sample. A sample that prints with no `.out` fails. `--update` writes the file from the run.

A `REPLAY` line may end with `backend=NAME`, as in `install/ruby/backends.rb backend=liquid`. The runner then sets `THINKTHEN_BACKEND` to that name for the run. For `ollama` it also sets `THINKTHEN_BASE_URL` to the second port the Ollama recordings sit at. `BACKEND_ROUTES` in `src/data/catalog.mjs` holds each name and address. Each answer such a run reads must sit at that backend's address, so a sample that falls back to another backend fails. A backends sample on a line with no backend names every backend in its own code, as the Rust one does. Its own assertions check each call.

A function page's library sample sits at `functions/<fn>/<surface>.<ext>`. The runner takes its binding from the file name and runs it under the name `sample.<ext>`, so `polars.py` does not shadow the Polars package. A Go sample builds as `sample.go`, because `go mod init go` fails. A Swift or Zig sample builds in a copy of its install page's project files, under the name they give the first call.

A function page shows its C and Rust tabs as fragments with no `main`. The runner wraps each one, as a reader would. C keeps its `#include` lines on top, and the rest goes inside `int main(void)`. Rust goes inside `fn main() -> Result<(), Box<dyn std::error::Error>>` and builds with the Rust install page's `Cargo.toml`. A fragment that declares its own `main` fails the runner. A Java sample on a function page declares `void main()` with no class, and a Kotlin sample declares `fun main()`. The runner builds each one under the function's name in Pascal case, such as `QuestionFile.java`. A Scala sample names its `@main` for the function in camel case, such as `questionFile`. A C# sample runs as top-level statements in the C# install page's project. The `fragment` field of each surface in `src/data/catalog.mjs` holds the note `CrossView` shows under the C and Rust tabs, which names that `main`.

`npm run smoke-sql` runs `scripts/smoke-sql.mjs`, which replays the `REPLAY` lines that end in `.sql`. It builds each database extension from this working tree. It runs each sample in a fresh folder with the page's `files/`, the extension as the install page loads it, and a fresh fixture copy in a cache folder of mode 0700. A sample passes when it exits 0 and prints its `<sample>.sql.out` byte for byte. `--update` writes that file from the run. The function pages show it under "What SQLite printed", "What DuckDB printed" or "What PostgreSQL printed". SQLite runs on the pinned 3.50.0 CLI from `databases/sqlite/setup.sh`, in list mode. DuckDB builds with `databases/duckdb/cpp/build.sh` from the cached 1.5.5 source and runs as `duckdb -unsigned -list` on the pinned CLI. PostgreSQL packages with `databases/postgresql/pgrx-package-locked.sh` into a private copy of the cached 16.15 server. Each attempt starts that server on a socket with no TCP port, with `THINKTHEN_CACHE` in its environment. As the superuser it runs `CREATE EXTENSION thinkthen;`. It makes an ordinary role with the extension's documented grant. It sets that role's `thinkthen.file_directory` to the page's `files/` folder. The sample runs in `psql` as that role. The extension resolves a relative `@` name against the server's data folder, so the role cannot read a page's files. Three pages name a file: annotate reads `@form.json`, recognize reads `@names.json`, and the install page's first call reads `@refund.json`. Those three run as the superuser, with their `files/` copied into the data folder. Each attempt stops its server on exit or interrupt. An interrupt also removes the runner's temp folder.

The runners run each sample once against saved recordings and compare its printed output with the adjacent `.out` file. Samples that print nothing must exit successfully. Each run uses a fresh folder and strips keys and shell settings. A named-backend sample receives only recordings for that backend, so a fallback fails without sending a request.

`scripts/check-binding-samples.mjs` runs in every build with Node alone. It checks that every library or SQL sample has a `REPLAY` line and that each listed sample exists. SQL samples must have saved output. It also checks that registry install lines name the package in the binding metadata and that release archive names match `sdlc/scripts/release-pack`.

The replay needs Rust with the offline Cargo cache, a stable Python 3.12 or later with uv and maturin for the Python and pandas samples, and R 4.2 or later with dplyr for the R samples. The C++, Objective-C, COBOL and Ada samples need g++, gcc with Objective-C, GnuCOBOL and GNAT. The Java, Kotlin and Scala samples need JDK 21, kotlinc and scalac, the C# sample needs the .NET 8 SDK, the Go, Swift and Zig samples need Go 1.22 with pkg-config, Swift 6 and Zig 0.15.2, and the PHP and Dart samples need PHP 8.3 with FFI and the Dart 3.13.4 in the pinned Flutter. The C sample needs a C compiler and json-c, which builds with CMake into `~/.local`. These samples call the C library that `sdlc/scripts/installed.sh` lays out. The TypeScript sample needs Node 22. The Ruby sample needs the Ruby that `libraries/ruby/setup-ruby.sh` builds, and libclang. The Polars sample needs the Polars that `libraries/python/requirements-dev.txt` pins. The Rust sample builds as its own Cargo project, which takes the crate from this working tree through a Cargo patch. `src/data/build-lines.mjs` holds the lines that build and run each one. The install page shows those lines, and the runner runs the same lines once per sample, in a folder laid out as a reader's. A sample whose toolchain is missing reports "not run", and the run exits 1. A host that lacks some toolchains runs `npm run smoke-bindings -- --allow-missing`. That run prints how many samples did not run, and passes.

## The doc tests gate every release

Every example in the documentation is a test, and the tests run before any release of the site or the code (Ian, 2026-10-01). Whoever changes the engine, a binding or a sample runs the doc tests before the release:

```
npm run test-docs
```

The command builds `thinkthen` from this working tree and runs `npm run build`, `npm run smoke-bindings` and `npm run smoke-sql`. It unsets every `THINKTHEN_` variable and every key, token or secret first. It takes no options, so a missing toolchain fails the run. The Pages workflow builds the command and runs the site build, including shell replay and static sample checks. Release rehearsal retains the full binding and SQL replay.

## Where the Beatles Bench pages come from

The pages under `/learn/beatles-bench/` follow the talk "Analyzing the Beatles using Jev". `src/data/beatles.mjs` holds each page's words. The slides sit in `public/learn/beatles-bench/`. `src/pages/learn/beatles-bench/[name]-800.webp.js` writes an 800-pixel copy of each slide at build time, and a phone takes that copy. The scripts sit in `examples/beatles/`, and `examples/beatles/bench/` holds the files they read from the bench commit in `examples/beatles/bench-pin`. To copy those files again from a checkout at that commit:

```
BEATLES_BENCH=path/to/beatles-bench npm run pull-bench
```

The slides come from the talk's deck, which quotes one bench commit. The deck's own build renders and commits each `slide.png`. `examples/beatles/deck-pin` names the deck commit the pages show. `src/data/slides.json` names the deck slide behind each image by name, the deck commit, the bench the deck quotes, and each image's SHA-256. An entry may name its own deck commit to keep a slide the pinned deck dropped. To export them again from the deck's committed `slide.png` files at the pinned commit:

```
DECK=path/to/deck npm run export-slides
```

The export reads the deck at `deck-pin`, never at the checkout's HEAD. It stops unless that commit's `BENCH_AT` names the bench in `examples/beatles/bench-pin`. It trusts the deck's build to have rendered the slides after that pin moved. `scripts/check-slides.mjs` runs in the build. It fails when the recorded deck differs from `deck-pin`, when the recorded bench differs from `bench-pin`, or when an image differs from its recorded SHA-256. Move either pin and the build fails until the slides are exported again.

The export also copies the deck's PDF, `jev-thinkthen-beatles-bench.pdf`, from the same commit to `public/learn/beatles-bench/`. `slides.json` records its SHA-256 as `pdf`, and `check-slides` fails when the file differs. The section's main page links it.

A pin can move without `pull-bench` when the bench files the pages copy did not change. On 2026-10-01 `bench-pin` moved from af538978 to 97090765 with the deck. The files under `examples/beatles/bench/` are byte-identical at both commits. `pull-bench` would copy the bench's old recordings back over the converted `thinkthen.jsonl` copies, so it did not run.

The Pages build cannot read the private deck, so it cannot see a newer deck. With `DECK` set, `check-slides` also fails when the deck's `slides/`, `order.txt` or PDF in its working tree differ from `deck-pin`. Slides rendered and not yet committed count. The deck's own `build.sh` runs it that way.

## The Bash techniques

`/how-tos/bash/` teaches the shell forms: `if`, `case`, exit codes, bands, loops, pipes, `xargs`, and a CI gate. The shell recipes sit beside them. `TECHNIQUES` and `SHELL_JOBS` in `src/data/catalog.mjs` list the pages, and `examples/how-tos/bash/` holds their scripts.

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

The site is open to search engines. One constant controls it:

```
src/data/catalog.mjs  ->  export const NOINDEX = false;
```

Set it to `true`, build, and every page carries `<meta name="robots" content="noindex">`. `robots.txt` follows the same constant: it turns every crawler away while `NOINDEX` is true, and names the sitemap while it is false. The 404 and search pages keep `noindex` always.

## The footer and the About page

`src/components/SiteFooter.astro` draws the footer on every page: three columns of links, the exit-code legend, and a band with the mark, the GenomOncology logo and the license. The legend stays in the footer because coloured exit codes show on most pages. `src/data/about.mjs` holds the author's and the company's links and their schema.org nodes. The footer, `/about/` and the home page's JSON-LD read them from there. The GenomOncology logos in `public/brand/` are PNGs at twice their shown size, one for each theme, because the brand folder holds no true vector.

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

A function page groups its tabs into the rows `TAB_GROUPS` in `src/data/catalog.mjs` names, such as "JVM and .NET" and "Databases". A language's tab appears when `examples/functions/<fn>/<surface>.<ext>` exists, so a new sample needs no page edit. A function that a surface cannot run names that surface in its `cannot` field, with one sentence the tab shows. A chosen language a page lacks falls back to its family, such as Java for Kotlin, and then to Bash.

## What is not here yet

- A page for any serve mode.

The build transforms the 61 generated redirect stubs with `preserve-redirect-fragments.mjs` immediately after Astro. Incoming fragments override configured defaults, including an empty `#`. The fixed refresh lives inside `<noscript>` and the visible fallback remains. `check-links.test.mjs` plants redirect and anchor failures; `check-links.mjs` validates the independent compatibility inventory and requires canonical internal links. `check-redirects.mjs` uses cached Chromium with no downloads to prove direct navigation, fragment preservation and fixed JavaScript-disabled fallbacks. It intercepts only the exact extensionless aliases using generated bytes; this does not test hosting slash normalization. Both build and check run those checks.

`check-markdown-headings.mjs` preserves the six existing annotate subsections after nesting them at h4. Build checks the emitted canonical twin and `llms-full.txt`; check validates the saved build.


Recipes use `RECIPE_PAGES`; Bash examples remain `SHELL_JOBS`. Normal builds publish Rules, verify and cache SQL in the index, navigation, HTML, Markdown, llms exports and search. Link, navigation and transcript-search recipes remain later drafts. `THINKTHEN_DRAFTS=1` also emits visibly marked later drafts.

`check-recipes.mjs` checks source content and rendered evidence and visibility. Run the ordinary sample replay with `node scripts/smoke.mjs recipes/`; the SQL example uses the stock DuckDB CLI. Illustrative outputs establish local behavior, not provider quality. Every supports result in verify requires a person to review the original source before action.
