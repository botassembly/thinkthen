# 0402 B: Uniform layout and shared documentation checks

Status: B implementation and the root-authorized focused proof are complete on lane0, prepared for fresh independent source review. Ticket 0402 remains in progress; independent B source review, coordinator checkpoint and landing remain pending. No helper participated in this build.

## Accepted design and reconciliation

The [full frozen design](../planning/0402-b-docs-design.md) is published byte for byte at SHA-256 `c8492286ed49d9062f4a77bb34cc95a318a4380f540847914e4e6cc4ac3aa38c`. Its supplied fresh review returned ACCEPT. Its header describes the earlier frozen A candidate; implementation starts after A landed at `b7e5dd3e464117d38701617ca18c5fdb68f5fb2a`, with exact accepted source `53de60f6b`, on the clean assigned `ticket/0402-docs-tell-one-story` branch. No switch, rebase or landing occurred here.

The landed grid counts and all 21 repeated-paragraph families match the frozen inventory. Each exception now contains the complete literal paragraph, exact sorted routes, source and individual reason. No additional collision needed an exception or content move. B covers outcomes 4–7 and its portion of outcome 12 only. Experiment 0031 exists; B adds no recipe comparison, performance claim or experiment execution. C/D remain with their owners.

## Changes and retained behavior

The home page uses a ten-function table. Disallowed positive card counts become ordinary lists while retaining associations, order, titles, descriptions and destinations. Allowed cards use fixed one/two/three-column rules, and trust's four outcome boxes join that grid without changing their meaning or colors. The blog index uses dated title/summary entries; all three source posts gain only the accepted `line` field. Runtime validation includes excluded drafts.

The extracted bounded HTML tree reader serves Markdown export and the layout/card checkers. Rendering retains all existing heading and block handling. The layout checker rejects disallowed grid counts, invalid source summaries, missing or incorrect blog entries, exact paragraph collisions and stale or widened exceptions. The card checker enforces decoded metadata equality, addresses, duplicate/missing tags, both alt tags, retained same-site images and dimensions. The width checker measures actual visible rows and blocks external requests. Build/check retain every preexisting check's relative order, with the accepted fixture/check pairs added. Writing and build instructions describe the new rules and their limits.

One unplanned source correction was needed to meet the accepted contract: Astro emits the special 404 file at `/404.html` while `Base.astro` previously emitted `og:url` at `/404/`. Its URL now names the actual file. The page still has no canonical link and retains noindex. Base's image selection and all other metadata behavior remain. This is the only source addition to the planned file inventory; it does not add or change a route.

A's 61 aliases, fragment-preservation transform, complete reference, six annotate h4 subsections, Markdown twins, llms files, public 0.1.2 install claims, backend width corrections, article bodies, recordings and 350 sample/proof inputs remain. Rust, engine, binding, specification, examples, saved outputs, proof hashes, social images and dependency/lock files are unchanged. No Rust growth: 117988/117988.

## Failures and corrections

- The first layout fixture runner accidentally put a literal newline in its generated JavaScript string. Its own syntax failure was corrected; all 43 actual fixture cases then passed.
- Diff inspection caught a malformed closing tag introduced by the Bash list conversion. It was fixed before the successful builds and retention proof.
- The first actual site build failed only the new `/404.html: og:url-equality` check. The narrow Base correction above fixed it. Independent 404 success/wrong-address cases join the card fixtures.

## Focused evidence

Only the root-authorized focused B checks ran; no full test/specification/surfaces, canonical350, native, load/timing, live, release or deployment checkpoint ran.

- `check-layout.test.mjs`: 43 separate-process owned fixture cases pass. They exercise all allowed counts, 0/1/5/7/8/10 failures, nested content/comments, source line boundaries (including Unicode), excluded drafts, missing posts, date/title/summary/order failures, 24/25-word boundaries, inline code/links/entities, one-page repetition, excluded navigation/footer/pre content, and exact/unused/changed/duplicate/widened exception cases. Each negative pins exit 1 and its intended diagnostic; positives pin exit 0. Fixture expectations supply an independent empty or literal exception list through the real exported checker, not the production inventory.
- `check-cards.test.mjs`: 33 executable-checker cases pass, including separately changed Open Graph/Twitter title/description/address, removed/empty alt tags, duplicates, missing metadata, differing or off-site images, missing/short/malformed/wrong-size PNGs, decoded ampersands/quotes/apostrophes, search/404 without canonical links and redirect exemption.
- Actual offline normal and draft `npm run build` pass. Draft includes 129 nonstub pages and all three posts in the accepted order with the draft marker; normal restores 128 nonstub pages and two posts. The actual blog Markdown contains the summaries. Each build's CLI smoke replays 143 scripts; 365 non-CLI samples are deliberately skipped by that runner.
- Actual `npm run check` passes, including 128 nonstub pages at 37 widths (4736 page views), table/code containment and uniform visible grid rows. Redirect browser checks still pass 61 aliases, nine fragment cases, three direct-page controls and two no-JavaScript fallbacks. Link checks still cover 189 HTML routes and anchors.
- `check-binding-proofs.mjs --strict`: 350 saved samples match, zero stale pages. This is unchanged-input validation, not a fresh execution of those 350 samples. A's actual canonical replay remains A's evidence.
- Retention comparison against the copied landed A build passes all 189 route/kind pairs, 771 headings, IDs, 881 sample/output blocks and 3984 retained destinations/title checks. Complete function reference paragraphs/tables remain ordered and identical; 114 Markdown twins remain byte-identical. Protected Git source/sample/proof trees and all article bodies match A, with only the declared source `line` additions.
- In an owned scratch site, a six-card grid still passes structural checking after CSS alone changes to four columns. The actual width checker rejects `grid-rows` `[4,2]` with exit 1. A separately hidden child also fails. Restored six-card controls pass all 37 widths with exit 0.
- An owned scratch exporter omitting h4 rendering produces exit 1 and `missing preserved h4 heading: Flat fields` from A's independent oracle. Restoring the renderer and both real six-heading outputs produces exit 0.
- Actual browser activation from a retained card's description, its canonical destination, visible keyboard focus and Enter activation pass.
- An actual ordinary Astro build in an owned source copy refuses an excluded RAD draft with its `line` removed: exit 1 and `rad: post-line-missing`. The restored source builds with exit 0 and still excludes RAD. Its peak memory was 635248640 bytes.
- Offline policy passes, checking 254 resolved packages; ratchet passes at 117988/117988; ticket checks and diff whitespace checks pass.
- `sdlc/scripts/lint` passes on a clean local clone of immutable source `e15a772ef3a71981d07b89f875fe3cb545341290`, with isolated build outputs. It includes offline policy/dependency checks, formatting, Clippy and documentation with warnings denied, bounded workflow/package fixtures and the 580-item inventory with four refused plants. Peak memory was 2065940480 bytes. The clone cleanup first refused the lane-path plant, then removed only its own completed scratch clone.
- Count-only private-name checking covers 35 external patterns across tracked paths/text and reports zero matches. The names never enter the record or logs; lint itself uses the clean HOME and reports its optional private-name step skipped.

Normal/draft build and width-check peak memory was 2367877120 bytes. Retention/browser/omission proof peak was 429694976 bytes. Runs used user scopes with MemoryMax 12884901888 and MemorySwapMax 1073741824 bytes, lane0's exclusive `/tmp/thinkthen-claude-0-heavy.lock` and matching held environment, explicit cached Node 22.22.3/Chromium, clean owned HOME/Cargo configuration, offline Cargo with two jobs and an empty wrapper. Registry/advisory caches were reused without installation or download; no shared cache/toolchain lock was removed. Proof scripts and logs are in ignored `target/0402b-build/`.

## What the build taught us

The new equality check found a real distinction between Astro's source pathname and its emitted 404 file. Counting six cards cannot detect four-column CSS; measured browser rectangles do. Exact route-scoped exceptions preserve complete generated contracts while still catching accidental new copies. The independent h4 omission oracle proves parser extraction did not silently remove A's reference headings. Source validation must run before draft filtering, and visible blog entries need comparison with source summaries rather than card descriptions.

## Immutable review handoff

Production source is `e15a772ef3a71981d07b89f875fe3cb545341290`; its complete site tree is `d47633f26a637a718127504b00fa086623e7ea95`. The final normal build's version file names that source commit. Final `npm run build`, strict saved-proof validation, `npm run check` and the retention/browser/omission runner all pass on that exact source. Final peak memory was 2231582720 bytes. The handoff commit adds only `sdlc/` evidence/status; its site tree remains identical. The final candidate ID is the commit carrying this record, reported separately to avoid a self-reference.

Local receipts and reproducible focused runners remain under `target/0402b-build/`: `site.log` retains the original 404 failure; `draft.log` holds normal/draft success; `draft-source-plant.log` holds the real excluded-draft build failure/control; `proof.log` and `final.log` hold structural/browser/h4/content controls; `lint.log` holds the clean-clone result. `proofs.mjs` and `draft-source-plant.mjs` reproduce the owned plants. `baseline-dist/` preserves A's built comparison input, and `draft-blog.html`/`draft-blog.md` preserve the actual draft list. `receipts.sha256` freezes those named logs/runners and draft outputs. All successful user scopes report the required 12 GiB/1 GiB limits and lane0 exclusion.

Final receipt SHA-256: `1b02bc57685adbf06bacb2cc7888632defbb63eb57557a00c357bf091fa693a2`.

## Remaining limits and next owner

This rule catches exact paragraph duplication, not semantic overlap. Matching metadata does not certify words drawn in images. The 37-width sweep and fixed divisibility rule do not certify every browser or assistive technology. Saved-proof validation adds no new native/canonical replay, accuracy or calibration claim.

The coordinator must assign a fresh independent reviewer to the immutable B source/candidate and these receipts, then name any full checkpoint and landing. The builder will commit and push WIP only. Ian can overturn the table/list choices, blog lines, breakpoint and each literal exception while retaining ticket outcomes and A's content contract.
