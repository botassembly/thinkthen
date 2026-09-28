# The site's samples and pages after the surfaces land

Status: Open.

This issue merges seven older issues: `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`, and `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`. Each one asks for a change under `site/`. Most of them trace to one cause. The site's samples and badges were drawn from the deck before any library or database landed. Seven surfaces landed at `eb3fae21`: C 0094, Rust Polars 0120, TypeScript 0107, SQLite 0109, Ruby 0112, R 0108, and PostgreSQL 0111. Rust 0093 landed before them. Python 0105 and Python Polars 0106 landed on 2026-09-25. DuckDB 0110 and pandas 0122 have not landed. The site's samples must be regenerated against the landed surfaces once every surface is on main. Evidence below was read on main at `71ea0a84`.

## 1. Drop every status word from the site

### Ian's ruling

Ian, 2026-09-24: "We're not doing the marketing and not publishing the website until everything's done." Every command, library, and database extension ships together, and Polars too. No copy says one thing is done and another is not. That rules out preview, planned, beta, "at launch", "ships first", "not run yet", "not yet shipped", "after" a release, and any staged rollout.

Ian, 2026-09-24: "We can say the words 0.1. I'm not afraid of saying that that's the version when we publish. It's just that I hate all this hedging." "ThinkThen 0.1" can stay. A badge that gives Bash "ships first" and every other surface "comes with 0.1" cannot, because it tells one surface from another.

Ian's landing ruling of 2026-09-25 lets Claude land each surface when it is ready. It governs landing. It does not change the publishing ruling above.

### What the site shows today

Every line the 2026-09-24 grep listed is still on main. Examples:

- `site/src/data/catalog.mjs:303` gives Bash `status: 'ships first'`. Lines 319 to 412 give every other surface `status: 'planned'`, and Polars has `release: null`.
- `site/src/components/Code.astro:5` turns `drawn` into the badge "not run yet".
- `site/src/pages/install.astro:17` says "Bash ships first." Lines 26 and 42 print "comes with 0.1" or "planned".
- `site/src/pages/surfaces.astro:7` says "Bash ships first." Line 21 heads a section "Not here yet".
- `site/src/pages/[surface]/index.astro:15` to `:79` split each surface page on `shipped`.
- `site/src/pages/functions/[name]/index.astro:19` to `:62` mark recognize and relate as preview. Line 62 says "The Bash command works today."
- `site/src/components/CrossView.astro:37`, `:41`, and `:45` add "comes with 0.1" or "is planned".
- `site/src/pages/tutorial.astro:39` and `:40`, `site/src/pages/index.astro:21`, `:45`, and `:55`, `site/src/components/InstallLine.astro:4` and `:5`, `site/scripts/emit-md.mjs:210`, `site/src/styles/site.css:143` to `:145`, `site/src/data/examples.mjs:15`, `site/scripts/pull-examples.mjs:487`, `:498`, and `:501`, `site/README.md:43`, `:92`, and `:94`, and `site/src/articles/code-that-understands.md:173` to `:177` carry the same words.
- Ten example cells carry `"status": "planned"` and four carry `"status": "preview"`, for example `site/src/data/examples/recognize__shell.json:2`.
- `site/src/data/catalog.mjs:220` to `:240` say the command has no recognize and no relate. The command has both now. `crates/thinkthen/src/cli/recognize.rs` and `specification/recognize.md` hold them.

### What it should show

Every surface and every function reads as a finished part of ThinkThen 0.1. A page names a version only as the same plain number for every surface.

### The fix

Drop the `status` and `release` fields from `SURFACES` in `catalog.mjs`, and drop `preview` from recognize and relate. Remove the badge, caption, and gate lines above. Delete lines that exist only to stage something, such as "Not here yet" and "No sample yet". Keep `run` as the only cell status once item 5 lands, and make `pull-examples.mjs` fail on a missing cell in place of writing `planned` or `preview`. The marketing repository's `scripts/check-no-staging.py` patterns can become a site check.

Done when: no built page shows preview, planned, beta, ships first, not run yet, at launch, works today, comes with a release, or not shipped, and a site check fails on any of them.

## 2. Draw the C tab from the landed C library

### What the site shows today

The eleven `site/src/data/examples/*__c.json` files say `"status": "drawn"` and `"source": "deck surfaces.md"` or `"deck recognize-surfaces.md"`. They were written against the retired `surfaces` branch door. `rank__c.json` asks `{"decide": ..., "rank": true, "records": [...]}` and its comment promises places. The landed door's only rank spelling is `{"rank": ..., "records": [...]}`, and its reply is the records in order. The relate sample passes bare sentences and `{"either": ["contradicts"]}`. Each relate text is now one JSON record with `name` and `kind`, and the spec is a version-one relate file. The recognize sample passes `{"kinds": [...], "relations": {...}}`. The spec is now a version-one recognize file, and its entities carry `name`, `kind`, `start`, `end`, and `strength`.

### What it should show

One call per function that compiles and runs against `libraries/c`, with its recorded output.

### The fix

Pull the C tab from `libraries/c/examples/functions.c` and `functions.txt`, which the C check compiles and runs. `libraries/c/DESIGN.md` gives the envelope grammar.

Done when: every C cell comes from `libraries/c/examples/` and carries `"status": "run"`.

## 3. Draw the Rust tab from the landed Rust library

### What the site shows today

The eleven `site/src/data/examples/*__rust.json` files say `"status": "drawn"` and come from the deck. None compiles against the 0086 public API. `decide__rust.json` calls `tt.decide(ask, text)` with a string question. `decide` takes a `Question` or a `BandedQuestion`. Other cells use `Question::choose("…", &teams)?.build()?` with a slice of options. The public builder takes typed options from `choices!`, one `option` call each. `found.index`, `one.index`, `.value`, `Recognize::new().kinds([…])`, and `Relate::new().either(…)` do not exist.

### What it should show

One compiled program per function, with its recorded output.

### The fix

Pull the Rust tab from `libraries/rust/examples/<fn>.rs` and `<fn>.txt`. Each one is compiled and tested.

Done when: every Rust cell comes from `libraries/rust/examples/` and carries `"status": "run"`.

## 4. Show the landed PostgreSQL result columns

### What the site shows today

Ticket 0111 ported the PostgreSQL extension onto the public API and left `site/` untouched. `site/src/data/examples/recognize__postgresql.json` selects `n.text` from `thinkthen_recognize`, and it reads a `tickets` table the slide never draws. The other `*__postgresql.json` files show the old shapes too.

The landed shapes are these:

- `thinkthen_recognize` returns `(name, kind, start, end, strength)`. The `text` column is now `name`. It also takes a version-one spec, `thinkthen_recognize(body, '@names.json')`, beside the kinds array.
- `thinkthen_relations` returns `(relation, source_name, source_kind, target_name, target_kind, probability)`.
- `thinkthen_relate` returns `(relation, source, target, probability)`. The first column was `name`. It takes a version-one relate spec in place of the rules array.
- `thinkthen_usage()` returns four columns: `requests_sent`, `cache_answers`, `input_tokens`, and `output_tokens`.

### What it should show

Each PostgreSQL sample shows the landed columns and the rows the extension printed.

### The fix

Pull each PostgreSQL cell from `databases/postgresql/examples.json`, which `check.sh` runs on every surface rung.

Done when: every PostgreSQL cell comes from `databases/postgresql/examples.json` and carries `"status": "run"`.

## 5. Pull every other surface's samples from its run examples

### What the site shows today

`site/scripts/pull-examples.mjs` reads the deck's `surfaces.md` and `recognize-surfaces.md`. The site holds 98 cells marked `drawn`, and 80 of them come from `deck surfaces.md`. The Python, Polars, TypeScript, R, Ruby, SQLite, and DuckDB tabs were all drawn before their surface landed. The cells that carry `printed` output say "Recorded from the branch build" (`site/src/pages/[surface]/index.astro:52`, `site/src/components/CrossView.astro:42`). Experiment 231 recorded that output against the old branch through a key forwarder. `recognize__shell.json` and `relate__shell.json` have no Bash sample. `libraries/ruby/tests/slide_sample.rb` still copies the deck's drawn Ruby block verbatim. The site catalog has no pandas entry, and one Polars entry for both the Python and the Rust Polars layers.

### What it should show

Each tab shows code that a landed surface's check runs, and the output that run printed.

### The fix

Point `pull-examples.mjs` at each surface's run examples file in place of the deck:

- Python: `libraries/python/tests/examples.py`
- TypeScript: `libraries/typescript/examples.json` and `tests/examples.test.mjs`
- R: `libraries/r/examples/examples.json` and `examples.R`
- Ruby: `libraries/ruby/tests/examples.rb`
- SQLite: `databases/sqlite/examples.json`
- Polars, DuckDB, and pandas: the examples file each surface lands with
- Bash recognize and relate: the command's own examples

Record printed output from the landed build. Give pandas a catalog entry when 0122 lands. Decide in the site ticket whether Python Polars and Rust Polars share one tab. Replace Ruby's `slide_sample.rb` with a check that runs `examples.rb`, or drop it.

Done when: every cell on the site carries `"status": "run"`, names a landed examples file as its source, and the pull fails if a surface or function has no cell.

## 6. Add a Beatles Bench section

### What the site shows today

thinkthen.dev has no Beatles Bench page. A grep of `site/src` and `site/scripts` for "beatles" finds nothing. Ian asked on 2026-09-25 for one section that walks through the ten functions on one worked example each.

### What it should show

One page per function. Each page shows the example's picture, the command, its output, and how context fills that function's gap, or why no context helps. The section links to the repository.

### The fix

Beatles Bench ticket 0006 (github.com/botassembly/beatles-bench, `sdlc/tickets/0006-one-worked-example-per-function.md`) adds `examples/<NN>-<fn>/` for each function. Each folder holds a recorded run that replays with no key, the committed outputs, `slide.png`, and `README.md`, the article.

1. Add a Beatles Bench section that links to the repository.
2. Pull each page from the bench's `examples/*/README.md` and `slide.png`, the way `scripts/pull-examples.mjs` pulls the deck. Copy no text by hand.
3. Fail the build when a pulled page is missing or its example has no committed output.

The bench ticket lands first. Ian can overturn all three.

Done when: the built site has ten Beatles Bench pages pulled from the bench, and the build fails on a missing page or output.

## 7. Check the built site for a stray code tag after a table

### What the site shows today

An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag right after `</table>`. The rest of the page then renders in monospace. The smallest case is `<table><tr><td><code>{v}</code></td></tr></table>`, which builds to `</table><code> <p>After the table.</p>`. `site/src/pages/install.astro:5` to `:13` work around it by building rows with `.map()`, and every other table does the same. No check guards against a return. `site/package.json` runs only `scripts/check-links.mjs` after the build.

### What it should show

No built page has a start tag directly after `</table>`.

### The fix

Add a check over `dist/` that fails on `</table>` followed by a start tag. Run it beside the link check in `npm run build`.

Done when: the build fails on the smallest case above and passes on main.

## 8. Bring two reference lines up to Quick Fix qf-command-edges-and-prune

Quick Fix qf-command-edges-and-prune changed two rules that `site/src/pages/reference.astro` states. `sdlc/records/qf-command-edges-and-prune.md` records both.

- `site/src/pages/reference.astro:32` says `--timeout` "Covers one attempt, connect to last byte. 0 is a usage error." The command now takes a whole number from 1 to 86400, and anything else exits 2 with `--timeout takes a whole number of seconds from 1 to 86400`. `specification/backends.md` and the Timeout row of `specification/settings.md` state it.
- `site/src/pages/reference.astro:267` says prune refuses the alias passed to `--model`. Prune now also refuses a model that no reply in the folder names, so a typo deletes nothing. An empty folder accepts any name. `specification/recording.md`, "Pruning a cache", states it.

Done when: both lines match the specification.

## 9. Migrate Ruby samples with ticket 0234

Preparation against main `79244123` found direct bare-result consumers under `site/examples/functions/{decide,question-file,filter,rank,choose,score,annotate,recognize,relate}/`, `site/examples/install/ruby/first-call.rb`, and syntax expectations in `site/scripts/named-answers.test.mjs`. Ticket0234 landed at `b07e56b7` with the reviewed Ruby `Call` value, facts and details. Its library consumers are updated; the site work below remains. The marketing owner must update these consumers and generated copies against the final accepted package before public 0.1. A truth test on a wrapper can pass while its contained answer is false or nil. Preserve named answers and inspect `.value` for actual scalar, list, rank, annotate, recognition and relation assertions. Keep nil, false, empty collections and failed answers distinct. Reuse the existing offline examples and recordings, with explicit batch one where historical request bodies require it; do not spend new provider calls just to change accessors. Ticket 0234 migrates Ruby-owned examples and tests, while this existing site issue owns site edits. The C and Python wrapper migrations remain in their existing specific issues and can share the same marketing batch.

## 10. Migrate TypeScript samples with ticket 0236

High-reviewed design0236 at `5283cfe3` makes asking methods return `Call<T>` with the former result in `.value`, plus owned facts and details. Its implementation is in progress; migrate the site against the final accepted package. Preparation found ten direct consumers: `site/examples/install/typescript/first-call.ts` and `site/examples/functions/{decide,question-file,choose,tag,rank,find,annotate,recognize,relate}/typescript.ts`. Include generated copies and `site/scripts/check-samples.mjs` in the marketing owner's verification. Preserve null, false, empty arrays, typed failed fields and actual answers; a truth test of the wrapper is not an answer check. Reuse the existing offline sample route and explicit batch one where historical recordings require it. The library ticket owns its examples and tests only. This belongs with the existing C, Python and Ruby sample migrations before public0.1.

## 11. Migrate R samples with ticket 0237

High-reviewed design 0237 at `e24f2c29` returns `thinkthen_call` with the former answer in `$value`, plus facts and owned details. Its implementation is in progress. The marketing owner must migrate `site/examples/functions/{decide,question-file,filter,rank,find,tag,score,annotate,recognize,relate}/` R consumers and `site/examples/install/r/first-call.R` against the final accepted package. Include generated copies and `site/scripts/named-answers.test.mjs` in verification. Preserve false, NA, empty and failed results and inspect the contained answer. Reuse offline samples and explicit batch one for historical request bodies. Library-owned examples belong to 0237; site examples remain here before public 0.1.

## Order

Items 1, 2, 3, 4, 7, and 8 can go now. Item 5 can repoint the pull script now for every landed surface. Its final run waits for DuckDB 0110 and pandas 0122. Item 6 waits for Beatles Bench ticket 0006. Regenerate every sample once, after DuckDB and pandas land, before the site publishes.

## Already fixed

- The deck-side sample mismatches in the 2026-09-22 branch-API issue: Rust's `decide` chain, `Recognize::kinds`, the false key comment, undefined Rust, Ruby, Python, and TypeScript variables, the PostgreSQL `@form.json` spelling, the C free call and unchecked `rc`, the DuckDB subquery and `INSTALL` lines. Marketing commit `6394261` fixed them in the deck, and site `741816f` pulled them in.
- The stand-in engine that sent no key. No `standin/` directory remains on main. Every landed surface binds the engine through the 0086 public API.
- No live capture of `choose`, `score`, `tag`, and `annotate`. Experiment 231 captured each one, and every landed surface's check now runs its examples file against the loopback backend.
- The stale Python and TypeScript slide tests. `libraries/python/tests/slide_sample.py` and `libraries/typescript/tests/slide.test.mjs` no longer exist. `examples.py` and `examples.test.mjs` replaced them.
- PostgreSQL's missing both-ways relate spelling. Ticket 0111 made `thinkthen_relate` take a version-one relate spec in place of bare relation names.
- The monospace install page. `site/src/pages/install.astro` builds its rows with `.map()`. Item 7 keeps only the guard.
