# 0402 B: Uniform layout and shared documentation checks

Status: B source/implementation accepted at `383305ec9`, source `e15a772ef`. Source-preserving integration and B FINAL INTEGRATION SITE are complete; fresh integration receipt review and B landing remain pending. Ticket 0402 remains partial and in progress; C/D remain unimplemented. No helper participated.

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

## Source-preserving main integration and B FINAL INTEGRATION SITE

Fresh independent source review returned ACCEPT for exact candidate `383305ec90882a489297f5ded3b7a9a2525ab54c`, source `e15a772ef3a71981d07b89f875fe3cb545341290`, at `/tmp/thinkthen-0402b-source-review-cli-8cpifta0/review.md`. This integration rebases its two commits onto Windows A main `24212dffa80226a530e9a2f51161c436286c9ff3`. Frozen integration source is `c9ad8192c9fea990e4a3eb0355cd6007b63413e9`. Only the two expected team-record applications conflicted; main history and B updates are retained. No product conflict or source correction occurred. The final candidate adds records only and is reported separately.

The normalized complete B site patch before/after is byte-identical, SHA-256 `90e26df7b1c44713c119dc8a1607c533e0bc44e5607c41dc44b7525f3d121fe5`. Every tracked path outside B's accepted site inventory and its four records matches main byte for byte, including Windows source, scripts, fixtures, workflows and ratchets. Accepted B site paths match the accepted B source. No source growth: independently measured root Rust/C Rust/C fixture totals remain 117988/5487/1329. The actual generated result schema is `specification/result.schema.json`, SHA-256 `ebdbee26c5361912357cebb56b0a45de323fa644483d1e54c0eba41d11479c68`, unchanged. Main's runner-generated `site/examples/bindings-proof.json` remains byte-identical to main and its actual Windows replay artifact, SHA-256 `ebe8a4d56634caf65b0203a1a2c4baf1723bd3d9a5f987529ac87115a8da6092`. No proof row or hash was edited or regenerated.

All nine Windows Linux/canonical receipts still match their recorded hashes. Full-test/spec/C40/workflow113 execution inputs (engine, bindings, fixtures, scripts, workflows, contracts and dependency specifications) are byte-identical to accepted Windows integration `7a1b5dc385fbf5dc68f2ba2c0939502102b72762`. Retain its workspace 1408 passes/26 skips, library-only 143/four skips, external consumer 21/three skips, spec 64/one skip and 24 green demos/zero red, required lint, C40/shared corpus 53 passes/two exclusions, workflow113, and actual 308 language + 42 SQL replay. These are inherited proof on their original frozen source, not fresh execution here. Current strict proof validates all 350 rows against current source/input bytes. Routine smoke remains only the 143 CLI scripts. Native Windows proof remains pending.

All ten original B receipt hashes remain tied to original source `e15a772ef`; the original clean-clone lint is not relabeled as merged lint. The immutable original manifest and verification are retained in `target/0402b-integration-main/`.

| Original B receipt | SHA-256 |
| --- | --- |
| `target/0402b-build/site.log` | `50da78e07770cbf6329ae5730fc7062862a8947bd76e46f6271a21869d41bf7e` |
| `target/0402b-build/draft.log` | `de7fcfbcb25699a0741325328baf3cccc60ab621448252337094f58a37c087b9` |
| `target/0402b-build/proof.log` | `be6580e867fc04bb5c8a349334feb241474240a290e6c4e85bb909a2071ed75d` |
| `target/0402b-build/draft-source-plant.log` | `e578e6ef7d8cd16727472e01402cb7009310e49893c990b4764c4c790cc9f2d6` |
| `target/0402b-build/lint.log` | `1394a39aa5892b06610440c37be2087e1a8af55be02ebeb81bfca44f9c0d101f` |
| `target/0402b-build/final.log` | `1b02bc57685adbf06bacb2cc7888632defbb63eb57557a00c357bf091fa693a2` |
| `target/0402b-build/proofs.mjs` | `8015a884435f1e00821153e2cf2ee0f0669dbf29ad9e4bfff636e6863e345937` |
| `target/0402b-build/draft-source-plant.mjs` | `dee36c64f300d42ca1fd65262b54510358e012e0bd957a8fb17ae2adf206e79f` |
| `target/0402b-build/draft-blog.html` | `53b37fb4f07f296b803dde4af7aa7624846de68f712b0be38c3e7d3a3600d67a` |
| `target/0402b-build/draft-blog.md` | `2570afe3003382e7e71ef681f6d40cf94359e87890952a0094b88d709e586011` |

The root named B FINAL INTEGRATION SITE before execution. This checkpoint freshly runs offline policy, exact ratchets, complete required lint on frozen integration source, full site build and check, the actual 37-width browser sweep, redirect browser/card checks, strict zero-stale validation, and the retained independent retention/card activation/h4 omission/geometry controls. The owned proof runner changes only its protected-path comparison baseline from Docs A to the specified Windows main; all content/browser oracles and original runner bytes remain intact. The completed scope exits 0. Fresh checks pass 43 layout and 33 card cases, 128 nonstub pages at all 37 widths (4736 page views), 189 HTML routes/anchors, 61 redirect aliases, nine fragment cases, three direct-page controls and two no-JavaScript fallbacks. Independent retention passes 189 route/kind pairs, 771 headings, 881 sample/output blocks, 3984 destination/title checks and 114 unchanged Markdown twins. Six annotate h4 subsections remain in both actual outputs; the omission plant fails on `Flat fields` and restored controls pass. Six-card CSS `[4,2]` and hidden-child plants fail as intended, restored geometry passes all 37 widths, and pointer-description/visible keyboard focus/Enter card activation passes. Strict checks before and after report 350 matching saved proofs, zero stale pages. Retained draft output remains prior B evidence, not a freshly built draft here. Required lint includes 113 workflow cases, the 580-item inventory and four refused plants; Clippy and documentation deny warnings. Peak memory is 3506388992 bytes under MemoryMax=12884901888 and MemorySwapMax=1073741824; the scope is inactive.

The initial launcher stopped with exit 1 before any gate because clean HOME exposes pre-existing Python bytecode otherwise ignored in the usual environment. Its separate `checkpoint.log` is retained. The first retry then passed policy/exact ratchets but stopped lint at its child-environment self-test: its owned fixture TMPDIR was inside the checkout and Git enumeration hid planted files. The corrected launcher uses an owned external `mktemp` fixture root and preserves this failed `lint.log` and `checkpoint-retry.log`. No candidate source change was needed. New completed receipts use the separate `checkpoint2/` subdirectory. The bounded replacement requires no tracked/staged changes and admits only `.pyc` files in the eleven observed bytecode directories; it neither removes files nor changes global ignore policy.

Runs use owned `target/0402b-integration-main/` logs/configuration, clean HOME and Cargo config, cached Node 22.22.3 and Chromium, two offline Cargo jobs, an empty compiler wrapper, lane0's exclusive `/tmp/thinkthen-claude-0-heavy.lock` with matching HEAVY_LOCK/HELD, and retained shared toolchain/cache mutation locks. The accepted prior helper's local-fetch validator is adapted only to the owned scratch root; only its validated sibling `file://` negative fixture permits an offline exception, with no remote fetch. No initialization or credential file was read. Count-only checking reports 35 private-name patterns, zero path hits and zero text hits; names never enter records or receipts. No lint clone or permanent lane was added.

No new full test/spec/surfaces, canonical350, load/stress/timing, experiment/live/native/CI, tag, release, publication or download ran. Ticket 0402 remains partial and in progress: B source/implementation is accepted and integration receipts await fresh independent review; B is not landed. C/D remain unimplemented. Ian can overturn the inherited-proof decision and request another named checkpoint.

| Fresh frozen integration receipt | Exit | SHA-256 |
| --- | --- | --- |
| `target/0402b-integration-main/checkpoint2/policy.log` | 0 | `5815fff89140fc8210baca7994e1ff555d55bafcd952ade5cf623b7c2d9fda90` |
| `target/0402b-integration-main/checkpoint2/ratchets.log` | 0 | `bc6b86163b87c27a68ee630da4817234768ecb47254ced8af44ca1a3ca94fd33` |
| `target/0402b-integration-main/checkpoint2/lint.log` | 0 | `3ea8648ac82cd9a94a31b3c00eb4ce9deeb810a5d0deffb6455cbf55b4a40270` |
| `target/0402b-integration-main/checkpoint2/strict-before.log` | 0 | `d8a28d49070781af268c94e91f51150a4f8e3628d853a43cd75410db00fcbd30` |
| `target/0402b-integration-main/checkpoint2/site-build.log` | 0 | `6a612374aac7d32edb0e231fee235c07703678ace8dc29d207ec9213db92692c` |
| `target/0402b-integration-main/checkpoint2/site-check.log` | 0 | `090ce819eb4651a4d4889741959a6a599a2a55d398f522a978faf865c7da7cde` |
| `target/0402b-integration-main/checkpoint2/strict-after.log` | 0 | `d8a28d49070781af268c94e91f51150a4f8e3628d853a43cd75410db00fcbd30` |
| `target/0402b-integration-main/checkpoint2/retention-browser.log` | 0 | `541a22182e7d9797930a15b6e9ddbd3064af38c1d378f1b7bf3a28e3ac0e4a82` |
| `target/0402b-integration-main/checkpoint2/verify-before.log` | 0 | `b104d57edbd946c097b68d8139c3d92fb9fbe6f9e42b00ef91af45728bb599c4` |
| `target/0402b-integration-main/checkpoint2/verify-after.log` | 0 | `b104d57edbd946c097b68d8139c3d92fb9fbe6f9e42b00ef91af45728bb599c4` |
| `target/0402b-integration-main/checkpoint2/checkpoint.log` | 0 | `5f444ff9c16f72bc3d7b1fb09b27c98279f171bf6eca52faf7d45523148ab445` |

The complete source inventory, all 24 B and 36 Windows per-file digests, normalized before/after patch, nine inherited Windows hashes, ten original B hashes, failed launcher receipts, successful checkpoint runners and new receipts remain in owned `target/0402b-integration-main/`. `receipts.sha256` freezes this local handoff. The record-only candidate delta is separately inventoried for fresh receipt review. Older B/A logs and proof outputs are untouched.
