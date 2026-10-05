Proposed public destination: `sdlc/planning/0402-b-docs-design.md`. This design is returned in chat only; no file was written.

# 0402 slice B: Layout and shared documentation checks

Status: read-only design against frozen candidate `6eba7e3179a26140b0c09de63fddb6d45464ad23`, inspected on clean lane0. Slice A has the supplied fresh Medium ACCEPT. A has not landed, and its coordinator checkpoint remains a dependency. B implementation waits for A’s successful checkpoint and landing.

This design covers ticket 0402 outcomes 4–7 and the B portion of outcome 12: uniform card grids, the blog list, matching social metadata, duplicate-paragraph checks, and writing/build instructions. All execution and verification described below is future work.

## Scope and retained behavior

B changes presentation and documentation checks. It changes no command, function, flag, setting, backend, recording, probability calculation, or result field.

Retain:

- The ten functions: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize`, and `relate`. Question-file remains a saved question, not an eleventh function.
- A’s canonical function reference bodies, opening examples, goal lines, section IDs, tools under Functions, and answers/question sets/recording under Learn.
- All 61 compatibility aliases: 42 historical aliases and 19 moved addresses. Retain incoming fragments, configured default fragments, encoded suffixes, empty `#`, fixed noscript fallback, canonical metadata, noindex, and visible fallback links.
- Markdown twins, `llms.txt`, `llms-full.txt`, and all six independently checked annotate h4 subsections.
- All example and proof paths, sample bytes, saved outputs and exit codes, `examples/REPLAY`, binding proof entries and hashes.
- The page-owned three-row blind-spots recording and its exact sample results: `"john"` with exit 0, and `null` with exit 3. Retain the admitted open-book revision `a6a6be71`.
- Public installation version 0.1.2. Main’s development version does not change public install claims.
- A’s backend width corrections in `install/backends/index.astro`, `openrouter.astro`, and `perplexity.astro`, including local column widths and long inline-command wrapping. Preserve short code spans and existing table/code containment checks.
- Existing draft visibility, search, navigation, language persistence, clipboard behavior, social image selection, theme colors, and outcome meanings.

Extract remains dropped. “Rules propose, the model confirms” remains pending experiment 4’s comparison with recognize alone. The completed strict-selection pass does not satisfy that comparison.

B introduces no glossary/confidence work from C, recipe infrastructure or “When nothing fits” work from D, provider prices, dollar-bill assumptions, experiments, live calls, workflow changes, publishing, or dependencies. Existing measured statements retain their sources and limits; B adds no accuracy, population, model, or calibration claims.

## Complete grid inventory and disposition

Count direct element children of each built `.grid`; whitespace and comments are not cards. Allowed counts are exactly 2, 3, 4, 6, and 9. Every other collection uses a semantic list or table without `.grid` or `.tile`.

The inspected sources and existing built output give this inventory:

| Source / route | Existing collection | B disposition |
| --- | --- | --- |
| `src/pages/index.astro`, `/` | Ten functions | Two-column table: Function and What it does. Preserve all ten canonical links, order, and `fn.line`. |
| Same | Six featured how-tos | Retain six-card grid and all existing entries. |
| `src/pages/functions/index.astro`, `/functions/` | Function table, tools and across-function lists | Retain unchanged. A already removed its ten-card grid. |
| `src/pages/functions/[name]/index.astro` | Jobs that use the function | Retain allowed grids; use a list for every other positive count. Details below. |
| `src/pages/how-tos/index.astro`, `/how-tos/` | Business teams: 5 | List preserving title, reader, function names, order, and destination. |
| Same | Data science: 4 | Retain grid. |
| Same | Ops/Security: 1 | List. |
| Same | Combined Bash techniques and shell recipes: 16 | List preserving title, label, order, and destination. |
| `src/pages/how-tos/bash/index.astro`, `/how-tos/bash/` | Techniques: 11; shell recipes: 5 | Two lists under the existing headings, preserving the existing introductory sentence for every entry. |
| `src/pages/how-tos/[slug]/index.astro` | Functions used by the how-to | Retain allowed grids; use a list for other positive counts. Details below. |
| `src/pages/learn/index.astro`, `/learn/` | Six introductory tiles | Retain grid. Retain separate talk and Questions and answers lists. |
| `src/pages/trust.astro`, `/trust/` | Four `.outcomes` boxes | Change container to `.grid`; retain four `.outcome` children, colors, labels, exit codes, and printed-value text. |
| `src/pages/blog/index.astro`, `/blog/` | Two published post tiles | Replace with newest-first ordered list. Draft build includes RAD in the same list. |

Function job counts at the frozen candidate:

- Retain grids: choose 2, filter 3, find 2.
- Convert to lists: decide 5; annotate, rank, score, and tag 1 each.
- Recognize, relate, and question-file have no job collection; do not add empty containers.

How-to function counts:

- Retain two-card grids on triage, survey answers, grouped alerts, expenses, scanned packets, and transcript search.
- Retain the three-card grid on inbound leads.
- Convert the one-function collections on joining tables, screening studies, and screening posts to lists.

Templates choose grid versus list from their actual collection length using the fixed allowed-count set. Do not truncate collections, pad them, split them arbitrarily, or alter catalog associations to satisfy layout.

Use ordinary `<ul><li>` lists. Each item has one canonical title link followed by its existing description. A list item does not retain tile styling. The home function table uses existing responsive table behavior.

## Stylesheet and rendered card behavior

In `src/styles/site.css`:

1. Keep `.grid` as the sole general content-card grid.
2. Below `46rem`, use one column.
3. At and above `46rem`, use two columns for counts 2 and 4, and three columns for counts 3, 6, and 9.
4. Use `minmax(0, 1fr)` tracks. Remove the six-track trailing-row redistribution, only-child accommodation, and all ten-card rules.
5. Remove `.facts`, `.fact`, `.fact b`, `.fact span`, and `.outcomes`. Retain all `.outcome` rules.
6. Preserve `.tile` colors, full-card anchor behavior, hover treatment, and the existing global keyboard focus outline.

The count rule and fixed column counts establish uniform row cardinality at intermediate widths as well as sampled widths. Do not use auto-fit columns for `.grid`.

The rule does not prohibit CSS grid used for navigation, footer columns, videos, tabs, or structural layout. Those are not general content-card collections and must not be renamed `.grid`.

Retained link cards remain one full-card `<a class="tile">` with their current href and text. Do not introduce nested interactive controls or click handlers. Outcome boxes remain noninteractive `<div>` elements.

Proof distinguishes two claims:

- Structural proof establishes allowed counts and the fixed divisibility rule.
- Browser proof measures actual rows and containment at the existing 37 viewport widths. It does not claim exhaustive browser testing at every possible width.

## Blog content and source contract

Retain `/blog/` and all three existing post routes:

- `/blog/jev-is-trained-to-decide/`
- `/blog/introducing-thinkthen/`
- `/blog/rad/`, draft only

Add a `line` front-matter field to all three Markdown sources. Proposed exact text:

| Article | `line` |
| --- | --- |
| `jev-is-trained-to-decide.md` | `Why a model trained for decisions interests programmers, and what remains a bet.` |
| `introducing-thinkthen.md` | `Ask a bounded question about text and use the typed answer in your code.` |
| `rad.md` | `A Beatles Bench example compares answers from memory with answers given a song catalog.` |

Each is nonempty and under 100 characters. These lines summarize page contents without adding performance claims.

`blurb` remains the post description and social description. `line` supplies only the index summary. Preserve titles, dates, authors, goals, body content, cards, links, video behavior, and the index’s existing agent-drafting/editorial note.

Render:

```html
<ol class="blog-posts">
  <li>
    <time datetime="YYYY-MM-DD">YYYY-MM-DD</time>
    <a href="/blog/post-slug/">Post title</a>
    <span>One summary line.</span>
  </li>
</ol>
```

The summary may wrap naturally on a phone. Do not force nowrap, truncate, or hide it. Use small local CSS for spacing and muted dates/summary text; no card grid.

Sort by descending ISO date, then ascending slug for deterministic equal-date ordering. Preserve the existing draft-title indication. Normal builds show Jev, then Introducing ThinkThen; draft builds append RAD.

Add `src/lib/post-line.mjs` with `postLineProblems(post)`, returning stable problem codes for a missing/nonstring/blank line, embedded newline, or more than 100 Unicode code points. `posts.mjs` validates all slug-bearing imported articles before draft filtering, so an invalid draft also fails an ordinary build.

For source checking, require `line` to be a single-line, double-quoted JSON-compatible string in Markdown front matter. This bounded new-field convention needs no YAML dependency. `check-layout.mjs` reads that field from every article source, passes its decoded value to the same validator, and compares visible index entries with source slug/date/title/line information. Astro remains responsible for parsing the complete front matter.

This catches a missing source line, a template that renders `blurb` instead of `line`, an omitted published post, and incorrect ordering. Draft visibility continues to follow `SHOW_DRAFTS`.

## Social-card contract

Retain `Base.astro`, `cards.mjs`, all image assets, and their existing selection behavior. Base already emits matching title/description values and both image-alt tags; the missing work is enforcement.

Extend `scripts/check-cards.mjs` to use `builtPages()` and decoded head values. For every nonstub page, require:

- Exactly one nonempty title and meta description.
- `og:title` equals the document title.
- `og:description` equals the meta description.
- Nonempty `og:url`.
- Nonempty `og:image:alt` and `twitter:image:alt`.
- Existing `summary_large_image`, identical image URLs, same-site absolute URLs, local PNG existence, and 1200 × 630 dimensions.

For listed pages, require `og:url` to equal the canonical URL. Existing `check-head.mjs` independently requires that canonical URL to equal `SITE + route`.

For unlisted `/404.html` and `/search/`, retain the deliberate absence of a canonical link. Require `og:url === SITE + route`; existing head checks continue proving noindex, no Markdown twin, and no search-index mark. Do not add a canonical link to satisfy the card checker.

Also validate the Twitter title and description against the document values, since Base already emits them. Compare decoded values, including ampersands, quotes, numeric entities, and apostrophes. Reject duplicate required tags.

Redirect stubs remain exempt from cards. Their publication shape stays governed by A’s independent redirect checks.

Expose `cardProblems(page, dist)` for bounded fixture use; keep the executable entry point accepting an optional dist directory, consistent with existing checker conventions. Fail with exit 1 and route-qualified problem codes. Short or malformed image files must produce a diagnostic rather than an uncaught buffer error.

Matching metadata does not prove that text drawn inside a slide image matches the HTML title. Retain existing slide provenance checks for that separate surface.

## Duplicate explanations and exceptions

`check-layout.mjs` compares complete paragraphs inside `main` after `label-tables.mjs`.

Normalize decoded text by collapsing whitespace. Preserve case and punctuation. Include inline code and link text; exclude scripts, styles, buttons, and paragraphs inside preformatted blocks. Treat `<br>` as whitespace. Do not compare navigation, footer, head metadata, Markdown twins, or redirect stubs.

A normalized paragraph of at least 25 whitespace-delimited words appearing on two distinct page routes fails. Repetition within one page alone does not trigger this rule.

This proves exact paragraph duplication, not semantic similarity, repeated sentences below the threshold, or paraphrased explanations. Review still enforces one home per idea.

Use an `ALLOWED` array in `check-layout.mjs`. Each entry contains:

```js
{ text, routes, source, reason }
```

`text` is the complete normalized literal paragraph; `routes` is the exact sorted route set. A prefix, wildcard, whole-template exemption, or dynamically generated expectation is insufficient. Reject duplicate entries, unused entries, changed text, and an occurrence on an unlisted route.

The frozen candidate’s repeated paragraph families requiring explicit disposition are:

| Family | Existing source | Exact route scope |
| --- | --- | --- |
| Quantity tested by each cut | Function template | annotate, choose, decide, filter, recognize, relate, tag |
| Framing exclusivity and empty/invalid input | Function template | All ten functions |
| Repeated inputs and positional-file rules | Function template | annotate, choose, decide, filter, rank, score, tag |
| Window rules | Function template | Same seven |
| Detailed position provenance | Function template | Same seven |
| CSV/TSV header and cell contract | Function template | All ten except find |
| Compact output and details | Function template | All ten |
| Record-mode exit semantics | Function template | All ten |
| Backend/key-variable selection | Function template | All ten |
| Backend/address precedence | Function template | All ten |
| Keyless loopback behavior | Function template | All ten |
| Hosted request/evidence limits | Function template | All ten |
| Jobs paragraph with document-mode restriction | Function template | choose, decide, recognize, score, tag |
| Jobs paragraph without that restriction | Function template | filter, rank |
| Invalid cut forms for cut-only functions | Function template | choose, filter, recognize, relate, tag |
| Multiple-document envelopes and failures | Function template | choose, decide, score, tag |
| Default cut boundary paragraph | Function template | decide, filter, recognize, relate, tag |
| OpenAI announcement note | `catalog.mjs`’s `OPENAI_ANNOUNCED`, rendered by install template | ada, c, cobol, cpp, csharp, dart, go, java, kotlin, objective-c, php, python, r, ruby, rust, scala, shell, swift, typescript, zig install pages |
| Setting precedence | `specification/settings.md`, rendered by settings/configuration pages | `/install/settings/`, `/install/configuration/` |
| JVM settings constructor explanation | `catalog.mjs`, rendered by install template | `/install/kotlin/`, `/install/scala/` |
| Frame-engine settings explanation | `catalog.mjs`, rendered by install template | `/install/pandas/`, `/install/polars/` |

These are narrow exceptions for existing contract/reference material or shared installation instructions. They preserve A’s complete bodies and existing source ownership. Record each reason individually; do not characterize a whole page as exempt.

At implementation, copy each complete paragraph from the frozen built/source content into the reviewed literal inventory. The table above defines its permitted scope. The designer’s read-only inspection is not a passing receipt from the future checker. Any additional collision found by that checker must be reviewed and either replaced by a link to its existing home or admitted with an equally specific exception.

Do not create a new Learn page or move A’s reference paragraphs solely to eliminate these sanctioned generated repetitions.

## Checker implementation and file inventory

Reuse `src/lib/listed-pages.mjs` for built-page enumeration and classification.

Extract the existing HTML tree parser from `scripts/emit-md.mjs` into `src/lib/html.mjs`, retaining its existing entity decoding and tree shape. Export `parseHtml`, `findElement`, `textContent`, and `hasClass`. The exporter imports the parser without changing its block or inline rendering. The layout/card checkers traverse this tree instead of counting closing tags with regular expressions.

Keep this helper bounded to the site’s generated HTML. It is not a general browser parser. Independent browser and Markdown checks remain its external oracles.

Planned source inventory:

| File | Change |
| --- | --- |
| `site/src/styles/site.css` | Uniform grid columns; remove obsolete rules; bounded blog-list styling |
| `site/src/pages/index.astro` | Ten-function table |
| `site/src/pages/functions/[name]/index.astro` | Conditional jobs list/grid only |
| `site/src/pages/how-tos/index.astro` | Conditional group list/grid; Bash list |
| `site/src/pages/how-tos/bash/index.astro` | Two semantic lists |
| `site/src/pages/how-tos/[slug]/index.astro` | Conditional function list/grid |
| `site/src/pages/trust.astro` | Four outcomes use `.grid` |
| `site/src/pages/blog/index.astro` | Date/title/line ordered list |
| `site/src/data/posts.mjs` | Validate every source post; retain draft filtering |
| All three `site/src/articles/*.md` | Add only `line` |
| `site/src/lib/post-line.mjs` | Bounded source-value validation |
| `site/src/lib/html.mjs` | Extract existing HTML parser |
| `site/scripts/emit-md.mjs` | Import extracted parser; retain rendering |
| `site/scripts/check-layout.mjs` | Grid, blog source/output, and paragraph checks |
| `site/scripts/check-layout.test.mjs` | Outside-in fixture cases |
| `site/scripts/check-cards.mjs` | Metadata equality and alt enforcement |
| `site/scripts/check-cards.test.mjs` | Outside-in card fixtures |
| `site/scripts/check-widths.mjs` | Add actual row-cardinality measurement |
| `site/package.json` | Wire new checks without removing/reordering existing checks |
| `site/WRITING.md` | Layout, Links, One home per idea, stronger Social cards |
| `site/README.md` | Accurate build/check steps and new failure rules |
| `sdlc/planning/0402-b-docs-design.md` | Reviewed design |
| `sdlc/records/0402-b-layout-build.md` | Future implementation/proof record |
| `sdlc/tickets/0402-docs-tell-one-story.md` | B evidence and status |

No route is added, renamed, removed, or redirected in B. No lockfile, dependency declaration, workflow, Rust source, specification contract, sample, proof input, or social-image edit is planned.

`check-layout.mjs` accepts optional dist and article-directory arguments for owned fixtures. Expose `layoutProblems(pages, articles, allowed)` returning route-qualified problem codes. Fixture expectations must be independent of the production allowed-count set and ALLOWED inventory.

Add the layout fixture/check pair immediately after `label-tables.mjs` in build, before Markdown export. Add the same pair to check before Markdown validation. Add card fixture execution immediately before the existing card checker in both scripts. Preserve every preexisting check’s relative order.

## Planned proof and independent failure plants

All plants use scratch trees created by the future proof runner. They never mutate the candidate, A’s checkpoint output, or a shared installation. Each negative pins exit 1 and its intended problem code; its restored positive control pins exit 0.

| Claim | Planned proof and plants |
| --- | --- |
| Allowed card counts | `check-layout.test.mjs` invokes the real checker over HTML fixtures. Counts 2, 3, 4, 6, 9 pass; 0, 1, 5, 7, 8, 10 fail with `grid-count`. Include nested content and comments so only direct element children count. |
| Blog list | A grid on `/blog/` fails `blog-grid`; missing, blank, 101-character, or multiline source line fails its named line code. A 100-character line passes. Reversed dated entries fail `blog-order`. Missing post and wrong rendered summary fail independently. |
| Draft source coverage | An invalid line in an excluded draft still fails. Normal build excludes RAD; draft build includes RAD in date order with its existing draft marker. |
| Duplicate paragraphs | The same 25-word paragraph on two routes fails `duplicate-paragraph`; 24 words pass. Inline links/code and equivalent entities cannot evade comparison. Navigation/footer copies pass. Exact scoped ALLOWED entry passes; a third route fails; unused or changed exception fails. |
| Social title/description/address | Separate plants change only `og:title`, only `og:description`, and only `og:url`, respectively. Each fails the corresponding equality code. |
| Card alt/image retention | Independently remove each alt tag, diverge image URLs, and provide missing, short, or wrong-size PNGs. Pin diagnostics. Valid entity-encoded metadata passes. Unlisted pages without canonicals pass; an incorrect unlisted `og:url` fails. |
| Actual uniform rows | Extend `check-widths.mjs` to group visible direct-card rectangles by top coordinate with a one-pixel tolerance, reporting route, width, grid index, and row counts. Reject hidden/zero-sized card children. Retain all existing overflow checks. |
| Geometry oracle catches CSS errors | In a separate scratch copy of the built site, keep an allowed six-card grid and change only its CSS to four columns. HTML layout check still passes; real width checker fails `grid-rows` with `[4,2]`. Restore and pass. |
| Link-card behavior | Rendered proof checks a retained card’s canonical destination, pointer activation from its description area, keyboard focus and Enter activation. Check converted lists retain every source destination and label. |
| Content retention | Compare before/after inventories of function sections, IDs, sample blocks, saved outputs, function/how-to associations, and blog bodies. Allow only the declared list/table markup and new index lines. |
| Markdown parser extraction | Run A’s six-heading check on actual annotate Markdown and llms-full. Independently omit h4 handling in a scratch exporter and require the existing named failure. Verify new table/list/blog content appears in its twins. |

Run the unchanged width sweep across every nonstub page at its existing 37 widths, including 320–1600 in steps of 40 and 375, 768, 820, and 1024. Retain table and preformatted-pane measurements. No screenshot snapshots, CSS-source-string assertions, or exact pixel-spacing tests are needed.

The future build record must name failures and corrections, report actual counts, and distinguish saved-proof validation from fresh execution.

## Checkpoint, capacity, and landing dependencies

Execution order:

1. Coordinator completes A’s named canonical checkpoint and lands A.
2. Coordinator assigns B implementation on the claimed lane. Confirm clean state and reconcile this frozen inventory with landed A; do not switch or rebase during A’s active checkpoint.
3. Obtain fresh independent design review before implementation.
4. Run the new focused fixture checks and omission plants, then offline `npm run build` and `npm run check`.
5. Run `node scripts/check-binding-proofs.mjs --strict` to establish unchanged saved inputs. This alone is not fresh canonical350 evidence.
6. Complete required policy/lint/ratchet and ticket checks. Run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` before source review.
7. Obtain fresh independent source review of the exact B candidate and proof record.
8. Coordinator names B’s checkpoint before full `test`, `spec`, or `surfaces`, and owns any required `npm run test-docs` or fresh canonical350 replay. Reuse A receipts only where source and inputs remain valid.

Use existing cached Node 22.22.3, matching cached Chromium, offline dependencies, explicit tool paths, minimal environments, and owned output/configuration directories. Missing prerequisites stop proof; do not download them.

Use lane0’s exclusive heavy lock and existing resource envelope: MemoryMax 12 GiB, MemorySwapMax 1 GiB, and two Cargo jobs where Cargo is invoked. Preserve shared toolchain/cache mutation locks. No heavy B work overlaps A’s checkpoint without coordinator capacity authorization.

The existing width checker’s six tabs remain the default; the coordinator may reduce concurrency under pressure. Record peak memory and actual execution evidence. A’s recorded memory use is prior capacity evidence, not a B result.

No Rust source growth is planned. The 500-nonblank-line Rust file cap still applies, and the ratchet remains `117988/117988`; do not raise it for JavaScript or documentation. Any changed measured Rust total indicates scope drift or an intervening landing requiring reconciliation.

## Remaining limits and overturnable choices

B’s exact-paragraph rule cannot prove that every idea has one semantic home. Metadata equality cannot certify image wording, provider availability, calibration, accuracy, or generalization. Finite browser widths complement the CSS/count invariant; they do not certify every browser or assistive technology.

Recipe publication, glossary/confidence wording, help gaps, doc-test checkpoint/release routing, and future experiment comparisons remain with their existing owners. B authorizes no new execution of those experiments.

Ian can overturn the home function table, list choices for disallowed counts, proposed blog summary lines, two/three-column breakpoint, and individual duplicate-paragraph exceptions. Such changes must still satisfy the authorized ticket outcomes and preserve A’s compatibility/content contract.

B is fully specified for fresh review. Implementation waits for A’s checkpoint and landing; this turn performed only read-only inspection.