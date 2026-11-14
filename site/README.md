# Site

The documentation site, built with Astro and Starlight. It builds to `site/dist/`.

## What a clean machine needs

- Linux. `scripts/prepare-packages.sh` copies the native libraries by their Linux names.
- Node 22.22 or later (`package.json` `engines`) and npm.
- The Rust toolchain from `rust-toolchain.toml`. `examples/lang-rust` builds offline with the workspace's `Cargo.lock`, so it needs only the crates that step 1 of the build fetches.
- `jq`, `bash` and `mkfifo`. Many examples pipe through `jq`.
- Python 3 with pandas and Polars 1.31, for the pandas and Polars examples.
- The host tools listed in the header of `scripts/prepare-packages.sh`: Ruby with the ffi gem, Go, a C and C++ compiler, CMake, patchelf, pkg-config, sqlite3 with its headers, Maven, the .NET 10 SDK, and R 4.3 or later with the jsonlite, vctrs, rlang, cli and tibble packages (on Ubuntu, `r-base-core` and the matching `r-cran-*` packages), and PostgreSQL 18 with its server headers (on Ubuntu, `postgresql-18` and `postgresql-server-dev-18` from [apt.postgresql.org](https://wiki.postgresql.org/wiki/Apt)). The PostgreSQL examples start a private server in a temporary folder; a root build runs it as `nobody` through `runuser`.
- JDK 22.0.2, Swift 6.3 and Zig 0.17.0, unpacked under `~/.local/share/thinkthen-build-tools` (or `$THINKTHEN_BUILD_TOOLS`) as `jvm/jdk-22.0.2+9`, `swift/swift-6.3-RELEASE-ubuntu24.04` and `zig/zig-x86_64-linux-0.17.0`. They come from [Adoptium](https://adoptium.net/), [swift.org](https://www.swift.org/install/linux/) and [ziglang.org](https://ziglang.org/download/). The JVM packages build only with JDK 22, because newer compilers add warnings the build treats as errors.
- Network for `npm ci`, `cargo fetch`, `scripts/duckdb-prepare` and `scripts/prepare-packages.sh` (Maven and NuGet). The examples themselves need none.

## Build

1. Build the program and the native libraries from this commit, in the repository root: `cargo build --release`, then `cargo build --release -p thinkthen-ffi -p thinkthen-python -p thinkthen-node`.
2. Fetch the DuckDB release the repository pins, for the database examples: `sh scripts/duckdb-prepare` in the repository root.
3. Fetch every crate of the workspace lock, which the R package vendors: `cargo fetch --locked` in the repository root.
4. Stage the language packages and database extensions in `target/site-packages/`: `sh scripts/prepare-packages.sh` in `site/`. It takes a few minutes, most of it the R package's Rust build.
5. Install the site's packages: `npm ci` in `site/`.
6. Run every example on the test backend: `npm run examples`. It writes each run to `.example-runs/`, which git ignores, and changes nothing in `examples/`. The Zig example compiles the Zig standard library on its first run, so the first run takes about a minute longer.
7. Build and check the site: `npm run build`. The build changes no tracked file.

### Record the real answers

The [documentation capture decision](../sdlc/decisions/2026-11-02-docs-show-real-answers.md) authorizes 100 calls and 300 KB per run, paying once per unique question and message. The PM keeps a conservative combined ceiling of 99 calls and 300,000 request-and-response bytes across batches. Subtract earlier actual charges before a later paid batch, and set the configurable input guard within the remaining total allowance. Its default 10,000 encoded-byte limit is an optional stricter guard, not a separate documentation budget. Cache hits consume no paid allowance. A total-byte overflow can discard the current response; earlier completed entries remain reusable.

The outputs a page shows come from one run against the real backend, recorded once per release and again when an example changes. `node scripts/run-examples.mjs --capture ID ...` (through `scripts/source-job.sh capture ID ...` in a job) runs the examples you name, function cells and ordinary examples alike. Prepare compiled examples from the same source before capture. For a compiler-free capture host, set `SITE_RUNTIME_BUNDLE` to the validated copied package/program/runtime closure; that execution branch runs no build setup, compiler fallback or npm install. Capture and replay take only explicit example IDs. Each example talks to a local recorder that sends its requests to the real backend with `THINKTHEN_API_KEY` from the environment, saves only the request and response bodies in `examples/captured-responses.json` (or `SITE_CAPTURE_FILE`), and reuses a saved response rather than asking again. It stops at `SITE_CAPTURE_MAX_CALLS` (99) calls, `SITE_CAPTURE_MAX_INPUT_BYTES` (10000) outgoing request bytes or `SITE_CAPTURE_MAX_BYTES` (300000) total request and response bytes. `--replay` runs the same way from the saved responses with no network, and fails on a request it has no saved response for. An example's output counts as a real answer only when the recorder served at least one answer during that example's own run; an earlier example's requests don't count. An example that gets no answer from the recorder fails, so a function cell that writes its answers without asking the backend is refused. So is an example that answers from a stand-in server on the local machine, such as `backend-wire`, `error-reply` or `error-status`: it writes no output and keeps its `unverified` file. A function cell needs its function's command cell in the same run: the command cells run first, and every other surface's cell must give the same answers. An ordinary example needs only the recorder's answers, from capture or from replay. An example that passes gets `output.txt`, `stderr.txt`, `exit`, `ran` and `capture.json`, and loses its `unverified` file; a failed run keeps the saved files. For a backend example, `capture.json` names the `systemone` backend, the responses file and how many answers the example received. The examples listed in `exampleLocalCaptures` in `product.config.mjs`, including local record refusals and `version`, call no backend: each must make no request to the recorder, and its `capture.json` says `backend: local` with the date.

### Use a release candidate

`scripts/preview-job.sh /path/to/candidate prepare`, from the repository root, stages a qualified candidate's command, native libraries, language packages and database extensions in `target/` instead of steps 1 to 4, then installs the site's packages. `preview-job.sh /path/to/candidate build` builds the site from them and writes a site archive beside the candidate.

`npm run dev` serves the site while you write.

## How the site is made

- `examples/functions/<function>/` holds each function page's example: one `case.json` with the question and records, and one cell per surface in `src/data/function-surfaces.json`. A cell's `run.sh` writes its answers as JSON to `SITE_ANSWERS_FILE`, and the build checks that every runnable cell gives the command cell's answers. `npm run cells` checks the cells' files.
- `src/content/docs/` holds every page. Each page's front matter carries its brief: `title`, `description`, `goal`, `reader`, `kind` and `verified`.
- `examples/<id>/run.sh` is an example the build runs against `../target/release/thinkthen`. Language and database examples load the packages staged in `../target/site-packages/` by relative path. Pages include an example with `<Example id="..." />`. The files in an example's folder:
  - `show.<ext>`, when present, is what the page shows; `run.sh` runs it.
  - `output.txt`, `exit` and `ran` (the day of the run) hold the recorded run that the page shows. A capture run writes them; never edit them by hand.
  - `unverified` holds the reason, for readers, that the page shows no output yet. The page shows the code and the reason. Routine runs skip it; `npm run examples -- ID` still runs it when named.
  - `needs` lists example ids that must run before this one, one per line.
  - Every other file is an input the example reads. Each page that runs the example shows each input with `<ExampleFile id="..." file="..." />`, which shows an xlsx file as one table per worksheet, or links to a page that shows it. Generated answers and reference pages list the inputs themselves.
  - `build-only` lists input files that point the build at the packages staged from this commit, one `FILE: reason` per line, such as a `go.mod` with a `replace` line. A reader writes those files with the published package, so pages need not show them.
- `src/components/Header.astro` and `src/components/Footer.astro` override Starlight's top bar and page foot on every page. The top bar holds a copy icon left of search; the foot holds a larger copy button, the raw link to the page's generated Markdown twin, and the site footer. `src/components/CopyMarkdown.astro` draws both copy controls. Clipboard API handles HTTPS; the private HTTP preview uses the browser’s copy command and reports success or a useful error.
- `src/lib/markdown-components.mjs` gives each of this site's components its Markdown form for the Markdown twins and `llms-full.txt`. `scripts/pages.mjs` renders the shared components and fails on a component with no Markdown form.
- `src/data/site.json` holds each fact that appears on more than one page, such as the version and the install lines. Pages read it with `<Fact k="..." />` and `<InstallLine />`. Prose never types the version.
- `questions/` holds the question list. `npm run answers` writes only `answers_page: true` candidates whose routes are unclaimed by the sitemap and redirects, and writes an index only when its route is unclaimed. Eligibility is decided before reading example data. Existing canonical answer redirects are retained, so today it writes no answer pages. Eligible pages still require their real-output provenance and fields; the generator never deletes authored files or an output folder. An eligible answer page is verified on the day its example last ran.
- `scripts/gen-reference.mjs` writes the reference pages from the program's help, except at an address the site menu or `src/data/redirects.json` already claims. Those claim every page it makes today, so it writes none and still checks the error and exit-code data. Fix a wrong reference sentence in the help text, never in the page. A page's `verified` date changes when its text changes.
- `WRITING.md` holds the word list's reasons and the rules a check can't see. `src/data/words.json` holds the words the build rejects.
- `public/social-card.png` is the link-preview card of every page. After a brand change, run `NODE_PATH=$(npm root -g) npm run social-card` (it needs Playwright with Chromium) and commit the PNG.

## How the examples run

Each example runs as `bash -o pipefail run.sh`, so a failed command anywhere in a pipe fails it. An output that holds `jq: error` fails it too.

The examples of one build share a fresh environment: a temporary `HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME` and `TMPDIR`, `PATH` with the built program first, and `THINKTHEN_BACKEND=stub`. No variable ending in `_API_KEY`, `_TOKEN`, `_SECRET` or `_PASSWORD` reaches an example. From the shell only `HTTPS_PROXY`, `HTTP_PROXY`, `NO_PROXY` and `SSL_CERT_FILE` pass through, plus the Rust and Go settings in `exampleSetup` in `product.config.mjs` (`CARGO_HOME` and `RUSTUP_HOME`, or their defaults under your home folder). An example that runs longer than its limit, `exampleTimeoutMs` in `product.config.mjs`, is stopped with every process it started.

A few examples name the `systemone` or `openai` backend to show what happens before or around a request: `error-key-missing`, `error-url`, `backend-route`, `backend-wire`, `error-transport` and `error-tls`. None of them leaves the machine: each stops on a missing key or a refused address, or talks to a stand-in server on the local machine. Every other example runs on the stub.

Routine runs check each example's exit code against its `exit` file, never its output against `output.txt`, because the test backend's answers are not the recorded ones. The run stops when `src/data/site.json` `version` differs from the version that `thinkthen --version` prints. `SITE_BIN_DIR` points the runs at another build of the program.

## Checks

`npm run build` fails when:

- an example fails or exits with another code, a runnable function cell's answers differ from the command cell's, or the built program's version differs from `site.json`;
- the reference no longer matches the build;
- Astro warns;
- a link or an anchor breaks, or a page has no incoming link;
- two source files make the same page;
- front matter is incomplete;
- a rejected word appears in prose (quoted words and blockquotes are exempt), or prose, a title or a description types the version;
- the README's install lines differ from `site.json`;
- search omits an authored page, any of the ten function queries fails to rank its function guide first, or browser copy differs from the raw Markdown;
- a rendered page lacks copy controls or a Markdown twin, or full-text agent content differs from its twin;
- the agent files miss a page, or a Markdown twin or `llms-full.txt` holds a raw component tag outside code;
- a page runs an example and neither shows nor links to one of the example's input files;
- an old address in `src/data/old-addresses.txt`, or the `.md` twin of an old page address, no longer resolves, or a redirect in `src/data/redirects.json` lands on anything but a built page;
- `robots.txt`, `sitemap.xml`, `version.json`, the social card, or a page's `og:image` is missing.

## Landing checks

`npm run landing` checks each built page as a reader who lands on it from search or an agent link and has read nothing else. Run it after `npm run build`. It loads every page of `dist/` in Chromium at 1280 and 390 px wide, reads the Markdown twins and the page sources, and needs Playwright with Chromium (`npm root -g` is searched when the site's packages lack it).

It writes `landing-report.json` and `landing-shots/`, both ignored by git, and prints a summary. The report lists each page's route, source file, kind, family, and whether a generator writes it and which one. Each violation names its rule, the offending text, a suggested fix and its cause: the page, an example, a generator, a shared template in `src/components/` or `src/lib/`, or data a generator reads. A violation caused by a generator, template or example is fixed there once, never on one page. `landing-shots/` holds one page per family at 1280 and 390 px, light and dark, for a reviewer.

| Rule | Flags |
|---|---|
| L1 code-line-length | a line of a rendered code or output block longer than 80 characters |
| L2 one-line-json | an output line that is a JSON object or array longer than 80 characters |
| L3 first-paragraph-context | a first paragraph that does not name the product, or opens with a caveat from `words.json` `landing.caveatStarts` |
| L4 maintainer-text | a phrase from `words.json` `landing.maintainerPhrases` in the title, description, body or Markdown twin, outside code |
| L5 page-length | over 1,500 words outside code, or over 4,000 px tall at 1280 px |
| L6 near-duplicate | over 60% of a page's word 5-shingles also on one other page; pairs are grouped into families |
| L7 title-is-id | a title that is a bare identifier, such as `local.cancelled error` |
| L8 undefined-jargon | the first use of a term in `src/data/glossary.json` neither linked to its page nor defined |
| L9 overflow-390 | a page that scrolls sideways at 390 px, or a code block that scrolls inside its box |

The header of `scripts/check-landing.mjs` gives each rule's exact test. `product.config.mjs` `landing.generators` names the generator of each generated page, and `landing.limits` may change a limit. `src/data/landing-allow.json` lists justified exceptions, each with a rule, a route or glob, an optional `match` substring and a reason.

- `npm run landing -- --strict` exits 1 on any violation the allowlist does not cover.
- `npm run landing -- --worklist FILE` writes the violations grouped by the file where each fix goes.
- `npm run landing -- --pages REGEX` checks only the matching routes, with no duplicate check and no screenshots.

`npm run build` runs the strict landing checks as its last step, so a new violation fails the build.

## Publishing

Publish these pages, and merge this README's install lines into the default branch, only after ThinkThen 0.2.0 is published on every registry the install lines name. The lines pin 0.2.0, so before then they fail instead of installing an earlier version, and the release archive links answer 404. After publishing, run each install line in a clean container.

`dist/` holds every page, its Markdown twin, `llms.txt`, `llms-full.txt`, `robots.txt`, `sitemap-index.xml` and the same index at `sitemap.xml`, and `version.json` (the version, the source commit and branch, and the build time).

The build copies the repository's `install.sh` and `install.ps1` unchanged into `dist/`, preserving the documented installer addresses.

### Execute a copied runtime closure

Prepare packages and compiled example IDs on the build host with `source-job.sh stage` and `source-job.sh prepare-examples /absolute/prepared ID ...`, then use `source-job.sh bundle-runtimes /absolute/runtimes`. Copy the packages, prepared examples and runtime bundle to the same absolute owned paths on the execution host. Set `SITE_RUNTIME_BUNDLE=/absolute/runtimes` and run `source-job.sh capture ID ...` or `replay ID ...` through `pm job`. This branch verifies copied file hashes, links and source identities before loading the recorded runtime environment, and runs no compiler or package installer. Keys remain in the execution environment. Use a body cache outside the checkout with `SITE_CAPTURE_FILE` so capture does not change prepared source inputs.

`source-job.sh guide-check` is an offline source check: it builds the command and private loopback fixture once, runs the literal public Bash and service guides, checks publication copies and accounts for every registered guide. The existing shared source gate executes the other host guides.
