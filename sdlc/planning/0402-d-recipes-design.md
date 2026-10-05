# 0402 D: Recipes section and repeatable question-file harness

Proposed destination: `sdlc/planning/0402-d-recipes-design.md`

Status: complete corrected proposed design for SHA-256 freezing and fresh independent review. This artifact has not been saved. Implementation waits for C to land and fresh independent acceptance of this complete design.

## Contract and starting point

Ticket `0402-docs-tell-one-story.md` owns this slice. The specification remains the product contract. D adds recipe documentation, publication rules and repeatability proof; it changes no function, flag, setting, engine behavior or binding contract.

The inspected main commit is `cf457ff989f21e626d33f0bfeb3de55297eebee2`. C’s exact accepted candidate is `4f9dce58207c066c607d0e9ab51f39ad96017cd7`, with implementation source `dbc6bf72d999122b4c2f520c2034f90ebf09b177`. Its independent High source review returned ACCEPT. C’s root checkpoint is active. D must not mutate its candidate, lane, locks, outputs, main or public files, interfere with its checkpoint, or treat pending canonical proof as completed.

This correction read all 582 lines of the supplied design, whose SHA-256 is `58b6516f11011d2a8084dca38cf3d8bba083935802c9b127729702eff86ee406`, and all three P2 findings. That review returned findings, not acceptance. Its acceptance of the measured tables and qualifications does not constitute acceptance of this corrected artifact.

The accepted experiment-planning record was read from immutable commit `ea615c0e6a3412bb4716ad97b704a3f698be30cb`, at `sdlc/records/2026-10-05-reviewed-experiment-planning.md`. Its accepted 0031 disposition resolves the Rules recipe’s comparative measurement wait for the frozen cohort and tested configuration only. It does not resolve publication proof.

The evidence inspection used repository instructions, README, specification, Rust standards, current planning and ticket outcomes, A/B build records, C’s source-review receipt, relevant source, the admitted 0031 final aggregate report and completed aggregate record, and the supplied aggregate-review receipt. It imported no raw benchmark inputs, gold rows, clinical or biological material, sealed records or external datasets.

Only read-only inspection occurred. No helpers, check reruns, builds, downloads, network access, credentials, model calls, live/stress/native/CI execution, tags, release work or publication occurred. Every shell invocation used `/bin/bash` with `login:false`. No subagents were used.

## Retained behavior and scope

D implements outcome 10 and the recipe portion of outcome 12. It preserves C’s implementation of outcomes 8 and 9. Outcome 11’s none-wording measurement remains a separate dependency.

Retain:

- The ten functions. Extract stays dropped; D adds no eleventh function or annotate mode.
- C’s glossary: a call is one use of a function; a request is one send to a model; a decision is one question answered about one piece of evidence.
- `meta.requests` as stored-answer question keys, and `meta.requests_sent` as attributed send attempts.
- C’s confidence definitions, the vendor field as received, the audit compatibility field and its distinct meaning, and cuts on probability.
- A’s compatibility inventory of 61 addresses, fragment preservation, complete function references, six annotate h4 subsections, Markdown twins and llms exports.
- B’s permitted grid counts, newest-first blog list, social-card equality rules, source validation and 21 literal, individually justified duplicate-paragraph exceptions.
- Existing 350 sample/proof paths and saved inputs. Assess proof freshness against the final source; historical receipts are not fresh canonical execution.
- Public 0.1.2 installation claims.
- Every existing build check in its existing relative order.
- Parser, secrecy, cancellation, cache-miss, invalid-input and conflict regressions.

No production Rust, C, SQL extension, binding, dependency, lockfile, workflow, installer, release or public distribution change is planned.

Candidate discovery belongs to the recipe’s caller program and remains outside pure core. ThinkThen receives validated questions and evidence and returns judgments. It never executes a selected label, command or free-text instruction.

## Complete recipe scope and issue dispositions

D may land infrastructure with all six recipe pages hidden as drafts. Every required slug has an explicit owner, evidence wait and publication disposition.

| Recipe slug | Owning issue | Owner | Evidence and remaining wait | D disposition |
| --- | --- | --- | --- | --- |
| `rules-propose-model-confirms` | `sdlc/issues/2026-10-03-draft-function-extract.md` | Queue owner; ticket 0402 owns page and harness | Accepted 0031 resolves experiment 4’s comparison wait only for its frozen cohort and tested `choose` configuration; public sample, recording, audit, provenance and fresh publication proof remain pending | Draft; eligible for a later publication candidate after proof |
| `link-records` | `sdlc/issues/2026-10-03-draft-function-link.md` | Queue owner | Experiment 0006’s earlier tested linking evidence; completed 0026 wording comparison returned no answers; none-wording improvement remains unknown; compatible sample and publication artifacts remain pending | Draft; retain human review of none decisions |
| `verify-a-claim` | `sdlc/issues/2026-10-03-draft-function-verify.md` | Queue owner | Experiment 0007 and reviewed citation-support limits; verify and qualify share this recipe; constructed-claim limits, source review, no validated protective cut, compatible sample and publication artifacts remain | Draft; qualification coverage awaits its own admitted evidence within this page |
| `navigate-many-documents` | `sdlc/issues/2026-10-03-draft-function-navigate.md` | Queue owner | Experiment 0010’s admitted numbers, method and compatible retained artifacts; caller-owned manifest writing and fallback policy | Draft; no navigate command, library function or `find --tree` promise |
| `search-transcripts` | New `sdlc/issues/2026-10-05-recipe-search-transcripts.md` | Queue owner | Ticket 0401, experiment 422 and reviewed held-out evidence; final source reconciliation, compatible replay and publication artifacts | Draft; preserve unequal reading lengths and overlap limits |
| `ask-your-cache-with-duckdb` | New `sdlc/issues/2026-10-05-recipe-ask-your-cache-with-duckdb.md` | Queue owner | Experiment 0019’s admitted numbers and compatible retained artifacts; reconcile experiment’s DuckDB v1.5.5 with the product’s separately owned supported-build work | Draft; no new extension build, version-support or efficacy claim |

Verify and qualify remain under the ticket’s shared `verify-a-claim` recipe. D creates no separate `qualify` slug or issue. A split requires a separately recorded and reviewed changed disposition.

In the D implementation commit:

1. Relabel the existing link, verify and navigate issues, retaining their filenames and open status.
2. Give each a `Recipe: ...` title, `Kind: recipe`, `Milestone: 0.2`, queue owner, job, experiment wait, exact slug and measured-publication closure condition.
3. Preserve the verify issue’s qualification scope within `verify-a-claim`.
4. Update the already relabeled extract issue to `Kind: recipe`, add its exact slug, and record the scoped 0031 superseding disposition. Preserve extract’s dropped-function history and separate 0413 ownership.
5. Create exactly the two new recipe issues named above, with open draft status, `Kind: recipe`, `Milestone: 0.2`, queue owner, job, dependencies, slug and publication closure condition.
6. Update `sdlc/planning/milestones.md` in that same commit: list all six recipe issues, replace stale link/verify/navigate function-draft descriptions, keep verify/qualify shared, and distinguish Rules measurement acceptance from pending publication proof.
7. Update ticket 0402 and the D build record accurately in that same whole change.

The ticket explicitly requires recipe issue metadata; no unrelated issue-format redesign is included.

Infrastructure landing closes none of these issues. Each stays open until its measured page publishes. No issue creation, relabeling or milestone mutation occurs during this design correction.

## Recipes section and draft visibility

Rename the shell catalog export `RECIPES` to `SHELL_JOBS`. Update every importer and change the Bash section heading to “Shell jobs.” Preserve every existing `/how-tos/bash/<slug>/` route, item, association, description and example.

Declare a separate `RECIPE_PAGES` catalog in `catalog.mjs`. Keep recipe selection and metadata validation in `recipes.mjs`.

Each entry contains:

- Slug, title, goal and one-sentence job.
- Functions used.
- `draft`.
- Publication evidence status.
- Example directory.
- Owning issue.
- Owning source record and immutable source commit.
- Structured measured entries.
- Structured fixture assertions, separate from measured entries.
- Structured page body used by the numerical-prose mechanism.

Do not identify recipes by a reused shell-job group name.

Follow the existing post visibility convention: drafts are available during development or when `THINKTHEN_DRAFTS=1`. Rendered draft detail pages carry `data-draft`. An index that includes draft entries also carries `data-draft`.

Normal builds exclude draft details and draft entries from navigation, footer, search, sitemap, Markdown twins and llms exports. Validate draft source metadata even when its page is excluded.

The public Recipes menu and footer entry appear only when at least one recipe is published. An explicit all-draft preview exposes its index by direct address; it does not add a global Recipes menu or footer entry merely because preview was enabled.

### Exact index generation and redirect transition

An unconditional `site/src/pages/recipes/index.astro` would itself register a static route. Hiding its body is insufficient to exclude that route.

Use `site/src/pages/recipes/[...index].astro` as the section-index generator:

- `getStaticPaths()` returns `[]` when the index is disabled.
- When enabled, it returns exactly the empty rest-parameter path that generates `/recipes/`.
- It generates no other rest paths.
- `site/src/pages/recipes/[slug].astro` separately generates eligible detail paths.

This is the concrete implementation of the ticket’s section-index intent. Record the filename adjustment in the accepted design and build record; do not leave a competing unconditional `index.astro`.

Use one small environment-neutral selection function for configuration and page generation. It accepts the recipe catalog and an explicit draft-preview boolean. Astro supplies development or `THINKTHEN_DRAFTS=1`; offline scripts use the declared build mode.

The index is enabled exactly when there is a published recipe or explicit draft preview. `astro.config.mjs` declares `/recipes` as a Bash redirect exactly when that index is disabled.

| Build state | `/recipes/` output | Recipe details | Menu/footer |
| --- | --- | --- | --- |
| Normal, all recipes draft | Existing fragment-preserving Bash redirect | None | No Recipes entry |
| Explicit draft preview, all recipes draft | Static index marked `data-draft`; no redirect | Marked draft details | No global Recipes entry |
| Normal, at least one published recipe | Static index listing published recipes; no redirect | Published details only | Recipes entry |
| Explicit draft preview with published recipes | Static index including marked draft entries; no redirect | Published and marked draft details | Recipes entry |

An explicit preview replaces the redirect only in its owned preview output. Normal all-draft output continues to retain the alias until a publication slice changes catalog disposition.

Reject a missing route, both route outputs, an unexpected route mode, a redirect in an index-enabled build, or an index in a normal all-draft build.

### Every redirect consumer

Retain the independent historical compatibility inventory in `redirect-contract.mjs`; do not derive expected aliases from Astro’s redirects.

Add narrowly scoped `/recipes` state handling there:

- Redirect state requires the exact Bash destination, existing stub shape and fragment preserver.
- Index state requires canonical `/recipes/` content with one main, no refresh or redirect preserver, and the appropriate draft/public marker.
- Every other alias retains its exact fixed destination and validation.
- Expected mode comes from the declared catalog/build state, not from accepting whichever output happens to exist.

Apply that contract to all existing consumers:

- `astro.config.mjs`.
- `redirect-contract.mjs`.
- `preserve-redirect-fragments.mjs`.
- `check-links.mjs`.
- `check-links.test.mjs`.
- `check-redirects.mjs`.

`preserve-redirect-fragments.mjs` must validate the complete expected input before editing generated stubs. In index state it validates `/recipes/` as content and excludes only that address from stub transformation. It must neither demand a recipe stub nor inject a preserver into the recipe index.

`check-links.mjs` keeps rejecting content links to redirect stubs. Its compatibility pass accepts `/recipes/` only in the declared state.

`check-redirects.mjs` keeps actual cached-Chromium checks. In redirect state it exercises the recipe alias and fragments; in index state it exercises the direct recipe index. Report actual inventory counts.

The frozen inventory contains 61 addresses in total, including `/recipes`. When `/recipes` becomes content, the other 60 remain redirect aliases. Preserve all 61 compatibility addresses and every existing fragment contract; do not invent a 62nd alias or silently shrink coverage.

Keep the redirect state’s incoming-fragment override, encoded fragments, empty suffixes, fixed no-JavaScript fallback, and existing hosting-normalization qualification. Use existing Bash anchors verified independently in the future proof. An index does not promise that historical Bash fragments now identify recipe-index sections.

Use lists or tables for the recipe index. Add no disallowed grid count.

## Page shape

Every recipe uses this sequence:

1. The job and the evidence boundary.
2. The question file.
3. A small labeled sample.
4. The call and named answer.
5. The audit command and retained output.
6. Interpretation, abstention and failure handling.
7. The measured table and commit-pinned sources.

Use existing code-rendering components and example loading conventions. Preserve goal lines, sample width rules, named-answer rules and the prohibition on comments in displayed code.

Link to canonical function, recording, audit, threshold and glossary explanations rather than copying their paragraphs. Give fixture notices short recipe-specific wording; add no blanket duplicate-paragraph exception.

Use established plain language. Explain “calls that did not return an answer” in prose; machine outputs may retain contract vocabulary.

A fixture table must say that its responses are controlled. It must never appear under a heading or carrier presenting it as measured provider performance.

Waiting draft bodies may state their jobs, boundaries and missing evidence. Do not manufacture complete examples, placeholder audit outputs or empty recordings to satisfy the page shape.

The join how-to folds into `link-records` only when that recipe publishes. The transcript how-to folds into `search-transcripts` only when that recipe publishes. Their redirects and content retention belong to those publication slices, not infrastructure landing.

## Evidence admitted for Rules

Use the accepted owning record as the public provenance home:

`../records/2026-10-05-reviewed-experiment-planning.md`

The publication candidate must pin its exact committed version. The inspected source version is `ea615c0e6a3412bb4716ad97b704a3f698be30cb`; an integration commit may be used only after verifying that it preserves the admitted record.

Public page source links point to ThinkThen’s owning SDLC record at an immutable commit. New public text contains no source workspace path, private repository locator, raw-data path or operator home path.

Preserve the following real-result tables. Fractions avoid introducing rounded values requiring another derivation.

### Full population and conservative counts

All 145 documents reached terminal outcomes on 858 fields. Terminal completion includes unresolved outcomes.

| Arm | Resolved fields | TP | FP | FN | Precision | Conservative recall |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| Frozen rules | 858/858 | 533 | 6876 | 42 | 533/7409 | 533/575 |
| Confirmation using `choose` | 857/858 | 507 | 124 | 68 | 507/631 | 507/575 |
| Frozen custom-role recognize | 850/858 | 17 | 424 | 558 | 17/441 | 17/575 |

Unresolved present fields remain in conservative recall denominators. They are not explicit-none answers.

### Fully matched comparison

The matched cohort contains 849 fields on 144 documents. Every arm uses the same matched fields.

| Arm | TP | FP | FN | Precision | Recall |
| --- | ---: | ---: | ---: | --- | --- |
| Frozen rules | 528 | 6786 | 41 | 528/7314 | 528/569 |
| Confirmation using `choose` | 503 | 123 | 66 | 503/626 | 503/569 |
| Frozen custom-role recognize | 16 | 423 | 553 | 16/439 | 16/569 |

On this cohort, confirmation improves precision over rules while losing recall. It exceeds this recognize configuration on aggregate precision and recall.

Keep these qualifications beside the tables:

- Rules used frozen regexes, a broad 9919-word dictionary and fixed candidate caps. This was not a best-possible rules baseline.
- Confirmation could not recover 42/575 present values absent from candidates.
- The tested confirmation arm used `choose`. A caller could implement a decide-based pattern, but equivalent measured performance was not established.
- All 283 absent fields were receipts. Confirmation correctly abstained on 211; recognize on 277, with three unresolved faults; rules on zero.
- The separate name control contained five cases: recognize recovered 3/5, confirmation 0/5.
- The date control contained eight cases: recognize recovered 0/8, confirmation 5/8.
- Recognize used one frozen custom-role configuration. These results establish no generic recognize failure.
- Source-location accuracy, preparation timing, local-model comparison and provider invoice were not measured.

Keep candidate misses, local absence, model none, not-sure answers and faults distinct.

Keep earlier 0027 results separate. Append a scoped superseding disposition to the historical recipe-evidence record; preserve its earlier stopped status as history.

If Rules cost is shown, use only admitted accounting: 0031 new known usage-derived charge `$0.182553`, conservative exposure `$0.203782`, and separately retained historical 0027 charge `$0.068566848`. These are not an independently verified invoice or a price promise for the public fixture.

Both genuine fault holds remain: `$0.000380` and `$0.021019`. The latter includes `$0.000170` known partial usage; do not add it again. The corrected false logical-versus-sent excess hold does not erase either genuine fault. Logical reused shares do not multiply provider consumption.

No raw benchmark inputs, gold rows, sealed TEST rows, licensed corpus extracts or new external datasets enter D.

## Current API decisions

### CLI

The supported recipe path is:

```text
thinkthen choose @question.json --jsonl \
  --field /text --options /options --batch 1 \
  --replay recording --input candidates.jsonl
```

`--options` requires JSONL. Each nonempty pool contains its candidates plus a last explicit `none` option. One discovered candidate therefore yields two valid options.

The question file contains wording, the selected model and any declared reading rule. Per-record options belong to the item and come from `/options`; do not insert a competing fixed option list.

Use a closed supported question-file object, such as:

```json
{
  "choose": "Which candidate is the stated total?",
  "model": "recipe-fixture-v1"
}
```

For a standalone fixed-options example, use the supported `options` array or ordered description map. Invent no recipe-specific question-file keys.

Shared context requires record mode. A singleton JSONL invocation may use `--context FILE`; a scalar document call cannot. Per-item context may use separate eligible singleton record calls. Promise no integrated record/context-pair batch.

### Rust

Use the existing public API:

- `Question::load` or `Question::from_json`.
- Dynamic labels through `Question::choose_labels`.
- Existing descriptions and builders.
- `Engine::details` / `details_with` for a full answer carrier.
- Eligible many-call context through `CallOptions::context`.

The public loader refuses nonempty `on` selection because library evidence is supplied whole. Keep CLI projection at the edge; do not claim a CLI question file with `on` loads unchanged through Rust.

A caller may construct one question per item and loop scalar calls with different candidate lists. Integrated many-record candidate options remain pending under 0413. Existing fixed-question `choose_many` is not that capability.

### SQL

SQL scalar calls may receive independently constructed question JSON and candidate descriptions per row. The caller may execute separate scalar calls.

Promise no keyed integrated many-record candidate collection. Do not treat scalar output as a full probability carrier when the function returns only a label.

0405’s context and answer-carrier gaps, changed-rule proof gaps and host exclusions remain separate. D creates no new native samples or fresh native proof.

### Audit

Audit grades saved result rows, not recording files alone. It reads IDs from `input`, with `/id` as the default.

Use detailed CLI rows for the internal audit carrier. Keep the displayed simple call free of `--details`; the internal proof runner collects detailed receipts without broadening the site’s displayed-example exception.

Attach the original immutable input object to scalar receipts when necessary. Preserve question, answer, threshold and metadata unchanged. Record that addition in provenance; manufacture no distribution and convert no failure into an answer.

Variable candidate sets require item-aware grading. Repeated labels such as `c001` do not establish cross-item probability comparability. Use per-case audit output and independently checked task totals. Aggregate grouping requires compatible option semantics.

Audit’s exclusion of failed or unlabeled rows is not an end-to-end success rate. The harness separately accounts for every input case.

## Candidate producer and ZERO / ONE / MANY handling

Finish and validate the producer before invoking ThinkThen. A failed producer must never leave partial output that is silently judged.

Validate:

- Exit status.
- JSON syntax and duplicate members.
- Required fields and types.
- Nonempty evidence.
- Unique case IDs.
- Candidate count and bounded sizes.
- Printable unique labels.
- Stable ordering.
- Descriptions and exact source locations.
- Reserved final `none` option after adaptation.

Use structured subprocess arguments, not shell interpolation or command substitution.

| Discovered candidates | Behavior |
| --- | --- |
| ZERO | Emit a local absence result. Invoke no asking function. Keep discovery absence separate from model abstention. |
| ONE | Ask `choose` over the candidate and explicit `none`; one candidate is not automatically correct. |
| MANY | Ask `choose` over validated ordered candidates and explicit `none`. |

A candidate label identifies a retained source value. Selecting it copies that exact value; no model-generated replacement text is accepted.

The fixture may use a small money-pattern finder written with the Python standard library. It is illustrative, not a reproduction of 0031’s dictionary, caps or benchmark implementation.

If a future recipe uses `find`, preserve 0419’s separate contract: handle zero and one outside find; invoke `find --none` only for 2–254 validated candidates within 16 MiB. Rules uses `choose`, so its ONE branch remains a confirmation call.

## Public sample and independent labels

Use handwritten generic synthetic purchase notes. They contain no patient, private customer, biological, clinical, lyric or licensed-source input.

Create six ordinary sample cases and two reading controls:

| ID | Purpose | Expected handling |
| --- | --- | --- |
| R01 | No proposed amount | Local none; zero requests |
| R02 | One candidate is the stated total | Select `c001` |
| R03 | One candidate is unrelated to a total | Select `none` |
| R04 | Multiple candidates; one stated total | Select its stable candidate ID |
| R05 | Multiple amounts; no stated total | Select `none` |
| R06 | Present target deliberately missed by discovery | Record candidate miss; retain explicit abstention separately |
| N01 | Exact tied top probabilities | Not sure |
| N02 | Winner below the declared positive cut | Not sure |

The ordinary sample has six end-to-end outcomes, five asking calls and one local absence outcome. With both reading controls, fixture preparation has seven asking calls.

Author labels before running the producer. An independent expectation file records exact text, expected candidate values and order, target presence, target value and permitted final status.

Keep two keys:

- `key.jsonl`: valid candidate-choice labels for audit.
- `labels.jsonl`: task truth, including target presence, discovery misses and local absence.

For R06, the choice key may say `none` because no offered option is correct. Task truth still records a present value missed by discovery. Agreement with the choice key never turns that miss into successful extraction.

Option labels are deterministic ASCII IDs, ordered by retained occurrence and documented deduplication. Descriptions carry exact candidate text and its role in the source. `none` stays last and means no offered candidate is the requested value.

Keep duplicate-occurrence and location policy explicit. Exact-value agreement does not establish source-location accuracy.

## Recording eligibility and controlled-fixture fallback

First assess whether an admitted public generic recording matches the complete request identity. Reuse only when wording, evidence, options, descriptions, ordering, context, requested model and address match.

No compatible Rules recording was established by this inspection. Do not relabel an unrelated recording as a Rules judgment.

The proposed fallback is an **owned loopback saved recording**, explicitly identified as a controlled fixture. Its canned replies prove repeatability, parsing, reading rules, audit wiring and no-send replay. They prove no provider quality, model capability or benchmark result.

Future preparation uses an owned loopback listener and the real CLI recording path. The listener checks literal expected request bodies independently of the producer and returns declared canned replies.

Use a distinctive requested and returned fixture model identifier. Record both fields as emitted. Never present that identifier as a real provider model.

Record into a new owned scratch folder. Convert only that scratch recording with existing `thinkthen cache convert DIR`. Commit resulting `thinkthen.jsonl`, not `thinkthen.sqlite`.

Use the admitted question-store format produced by the program. Do not hand-author request digests or label synthetic replies as historical live-provider answers.

This fallback is an explicit narrow replacement for the ticket’s experiment-cache sample route when compatible retained answers cannot safely be reused. Fresh design acceptance must cover that disposition. It preserves publication proof while separating repeatability from measured efficacy.

No paid call is needed or authorized. This design does not execute even the proposed loopback preparation.

## Provenance and fingerprints

Each recipe has one provenance manifest containing:

- Evidence class: authentic reused recording or controlled loopback fixture.
- Public owning-record path and immutable commit.
- Data authorship and synthetic/public classification.
- Question-file hash.
- Input, label and key hashes.
- Candidate-producer source hash.
- Exact option order and description carrier.
- Context hash or explicit absence.
- Requested model and actual returned model.
- Recording hash and stored question keys.
- Program source commit and executable hash.
- Command arguments and reading rules.
- Saved result, audit and harness receipt hashes.
- Exact population, completed, failed, not-sure, none, local-none and candidate-miss counts.
- Receipt source fingerprints and execution stage.

Question-file identity and recording identity differ. A threshold changes the question-file digest, while supported reading changes may reuse stored probabilities. Wording, model, evidence, descriptions, options or shared state can change the wire question and miss strict replay.

Record both identities. Add no product fingerprint schema.

No manifest contains credentials, headers, private source paths or unscreened environment values.

## Enforceable numerical prose and evidence rendering

Explicit evidence references alone do not reject an unreferenced literal claim. Recipe bodies therefore use a closed structured content model rendered by the shared recipe template.

Ordinary text nodes hold prose, headings, captions and link labels. Evidence nodes reference validated measured entries or validated fixture assertions. Artifact nodes reference inspected example files. Raw HTML, arbitrary Astro expressions and free-form Markdown bodies are not admitted as recipe content.

Apply these rules to every recipe, including hidden drafts:

1. Ordinary prose text contains no literal decimal digit. Inspect decoded text character by character, including Unicode decimal digits; this is a scoped content rule, not a broad regex attempting to classify arbitrary numbers.
2. A numerical claim is represented only by an evidence-reference node. Authors cannot supply a replacement value, arbitrary formatted number or numerical child text.
3. The renderer resolves the reference and emits its canonical value, denominator, measure meaning and source or fixture citation.
4. Titles, goals, jobs, headings, captions and link labels follow the same ordinary-text rule.
5. URLs, hashes and artifact identifiers occupy typed fields. They cannot be used as free-form prose escapes.
6. Displayed code, sample data, filenames and saved machine outputs come from exact declared artifacts through existing renderers. They do not become numerical evidence merely because they contain digits.
7. Numbered page steps are generated by the template, not authored numerical claims.
8. Source-record identifiers or technical versions needed in prose use validated typed reference nodes, with fixed rendering; they cannot carry arbitrary claims.

This rule applies to recipe content, not unrelated site pages, shared footer years or product documentation. Human review still checks semantic claims, including unsupported quantities written as words. The automated contract specifically closes the literal-number escape.

Measured carriers require:

- Stable evidence ID.
- Exact canonical value or fraction.
- Population or sample size and relevant denominator.
- Cohort.
- Backend/model, or explicit not-applicable/unknown qualification.
- Measure definition.
- Resolved/unresolved accounting where relevant.
- Public owning-record source pinned to a full immutable commit.
- Evidence class `measured`.

Fixture carriers require:

- Distinct fixture ID and class.
- Exact saved artifact and hash.
- Explicit selector for the asserted value, or a declared deterministic derivation checked against independent expectations.
- Fixture denominator and scope.
- Visible controlled-fixture qualification where applicable.
- No measured-provider label.

A value merely occurring somewhere in `.out` is insufficient. A fixture number cannot be placed in the measured carrier.

Render evidence with ordinary visible text and visible links inside the recipe page. HTML `data-*` markers identify carrier IDs for validation but are not the only place evidence lives. The existing Markdown inline renderer preserves visible span text and links.

Rendered validation checks:

- Every HTML carrier resolves to the expected class, exact value, denominator, qualification and citation.
- All recipe narrative text outside validated carriers and exact artifact blocks remains free of literal digits.
- Recipe Markdown retains numerical prose, full measured and fixture tables, qualifications and citations.
- `llms-full.txt` retains the complete corresponding page body and evidence.
- `llms.txt` lists exactly eligible twins and preserves any evidence-bearing blurb without dropping its qualification or citation.
- Normal exports contain no draft recipe or draft index.
- Preview exports retain draft identity visibly; `data-draft` alone does not survive Markdown export, so include a short visible draft notice.

Do not add a new duplicate-paragraph exception for that notice.

### Independent unsupported-number negative and supported control

Author a complete otherwise-valid checker fixture independently of the renderer and producer. Add exactly one ordinary prose sentence:

`The method is 97.3% accurate.`

That number is absent from its measured entries and saved outputs. Require checker exit **1** and exactly one diagnostic line:

`recipe unsupported numerical prose: fixture-number: body[0]`

The test pins the entire diagnostic line and its single occurrence, with no unrelated rejection. Restoring the valid body pins exit **0**.

A separate supported control references an independently authored measured entry for `503/626`, with its matched-cohort definition, denominator and immutable source. Require exit **0** and literal expected value, qualification and citation in HTML, Markdown and the full llms page block.

A separate fixture-number control references a saved fixture assertion and displays its controlled-fixture qualification. It passes as fixture evidence; changing only its class to measured fails with the evidence-class diagnostic.

Plant a literal directly in rendered HTML outside a carrier as well. Rendered validation must reject it even if source validation previously passed. Missing citations, altered values and dropped qualifications are independent rendering negatives.

## Concrete file inventory

| File or group | Planned change |
| --- | --- |
| `site/src/data/catalog.mjs` | Rename shell export to `SHELL_JOBS`; declare all six `RECIPE_PAGES` entries |
| `site/src/pages/how-tos/bash/index.astro` | Read `SHELL_JOBS`; retain existing list and routes |
| Every other existing `RECIPES` importer | Mechanical rename; preserve behavior |
| `site/src/data/recipes.mjs` | Environment-neutral selection, closed metadata/body validation and evidence resolution |
| `site/src/pages/recipes/[...index].astro` | Conditional static index through empty-or-singleton `getStaticPaths()` |
| `site/src/pages/recipes/[slug].astro` | Shared structured recipe renderer and selected detail paths |
| `site/src/layouts/Base.astro` | Conditional Recipes menu entry |
| `site/src/components/SiteFooter.astro` | Conditional Recipes footer entry |
| `site/astro.config.mjs` | Declare recipe redirect exactly when index generation is disabled |
| `site/scripts/redirect-contract.mjs` | Preserve historical inventory; narrowly validate recipe redirect/index states |
| `site/scripts/preserve-redirect-fragments.mjs` | Validate all expected routes; transform only expected stubs |
| `site/scripts/check-links.mjs` | Declared recipe-state compatibility and draft-leak enforcement |
| `site/scripts/check-links.test.mjs` | Independent route, alias, fragment and draft-leak plants |
| `site/scripts/check-redirects.mjs` | Actual-browser proof of both recipe route states; retain other contracts |
| `site/scripts/check-recipes.mjs` | Source/artifact, numerical-prose and rendered-publication validation |
| `site/scripts/check-recipes.test.mjs` | Independent positive, negative and rendering fixtures |
| `site/scripts/recipe-proof.py` | Bounded external CLI/provenance runner |
| `site/scripts/emit-md.mjs` | Change only if evidence-export proof finds a real loss; preserve existing conversion behavior |
| `site/src/lib/listed-pages.mjs`, sitemap/search/head consumers | Inspect and preserve selection behavior; change only for a demonstrated recipe visibility gap |
| `site/scripts/smoke.mjs` | Preserve existing execution; explicitly use `/bin/bash` for shell children; admit complete recipe samples only |
| `site/package.json` | Wire focused recipe source and rendered checks without reordering existing checks |
| `site/WRITING.md` | Full recipe shape, evidence classes, carrier rule and publication requirements |
| `site/README.md` | Updated section, route-state and build map |
| `sdlc/planning/0402-d-recipes-design.md` | Complete accepted, hash-frozen design |
| `sdlc/records/0402-d-recipes-build.md` | Exact implementation, candidate and proof record |
| `sdlc/records/2026-10-04-recipe-evidence-limits.md` | Append scoped 0031 superseding disposition; retain history |
| `sdlc/tickets/0402-docs-tell-one-story.md` | Accurate D scope, status, filename adjustment and evidence |
| Three existing link/verify/navigate issue files | Required recipe relabels, waits and slugs |
| Existing extract issue | Recipe metadata and scoped 0031 disposition; preserve DROP |
| `sdlc/issues/2026-10-05-recipe-search-transcripts.md` | Create required open recipe issue |
| `sdlc/issues/2026-10-05-recipe-ask-your-cache-with-duckdb.md` | Create required open recipe issue |
| `sdlc/planning/milestones.md` | Same-commit six-recipe inventory and qualified waits |

For each recipe slug reserve this artifact shape:

```text
site/examples/recipes/<slug>/
  1-ask.sh
  1-ask.out
  2-audit.sh
  2-audit.out
  files/
    question.json
    cases.jsonl
    key.jsonl
    labels.jsonl
    expected.json
    provenance.json
    measured.json
    results.jsonl
    audit.jsonl
    audit-cases.jsonl
    harness.json
    recording/thinkthen.jsonl
```

Rules additionally has `files/candidates.py`. Its runtime `candidates.jsonl` is created only in the smoke/proof scratch copy.

Create no placeholder outputs or empty recordings for waiting recipes. Their catalog entries and draft bodies may exist. Publication validation reports missing evidence, while ordinary infrastructure validation accepts an explicitly waiting draft with an accurate missing-evidence disposition.

The proof runner uses Python’s standard library. Site validation uses existing Node modules and the existing HTML parser. No new dependency is justified.

## Commands and proposed harness flags

Existing product interfaces:

```text
thinkthen choose @question.json --jsonl \
  --field /text --options /options --batch 1 \
  --replay recording --input candidates.jsonl

thinkthen audit results.jsonl key.jsonl --cases
```

Retain aggregate audit output only where grouping is valid. Do not use `audit --write` to tune the fixture or rewrite the source question.

Proposed external proof interface:

```text
python3 site/scripts/recipe-proof.py \
  --recipe rules-propose-model-confirms \
  --bin <reviewed-command> \
  --artifacts <owned-output-directory> \
  --mode replay
```

A separate `--mode prepare-fixture` is limited to future owned loopback preparation. It creates no provider route and accepts no credential option.

Proposed checker interfaces:

```text
node site/scripts/check-recipes.test.mjs
node site/scripts/check-recipes.mjs
node site/scripts/check-recipes.mjs --dist <owned-dist>
```

These are harness interfaces, not new ThinkThen commands. Their implementation remains subject to fresh review.

The displayed `1-ask.sh` invokes `@question.json` and names the selected answer before using it. `2-audit.sh` audits retained rows and names its output. Detailed receipt collection remains internal to `recipe-proof.py`.

Future shells use `/bin/bash` without login semantics. Subprocesses use argument arrays. Capture expected nonzero status explicitly and restore strict handling immediately. Do not use `jq //` where false or null has distinct meaning.

## Proof stages and negative plants

All stages below are future implementation proof. None was run during this correction.

### Stage 1: independent source and artifact validation

Validate all six catalog entries, including excluded drafts. Validate their owning issues, waits and shared verify/qualify disposition.

Require every publication artifact for entries marked publishable. Explicitly waiting drafts remain hidden and cannot pass publication eligibility merely by passing metadata checks.

Render measured tables from structured admitted entries. Preserve exact values, denominators, cohorts, backend/model qualifications, definitions and commit-pinned sources.

| Independent plant | Required rejection |
| --- | --- |
| Missing question, key, recording, audit or provenance on a publishable recipe | Named missing-artifact failure |
| Missing required slug or owning issue | Recipe-scope failure |
| Separate qualify slug without reviewed changed disposition | Recipe-disposition failure |
| Missing required issue creation/relabel or milestone entry | Recipe-scope failure |
| Unpinned source URL | Source-pin failure |
| Fixture rate placed in measured carrier | Evidence-class failure |
| Missing denominator or unresolved count | Measurement-definition failure |
| Unreferenced ordinary literal-number sentence | Exact single unsupported-numerical-prose diagnostic |
| Evidence reference with author-supplied replacement value | Carrier-shape failure |
| Changed case/key/recording bytes | Hash mismatch |
| Private home path or private source locator in new text | Public-provenance failure |
| Published entry with incomplete proof | Publication-evidence failure |

Each negative pins exit **1** and its intended diagnostic. Restored controls pin exit **0**. The literal-number plant additionally pins exactly one complete diagnostic line.

### Stage 2: producer and CLI behavior

Use literal expected cases and request bodies, not expectations generated by the producer under test.

Plant nonzero producer exit with apparently valid partial output, malformed JSON, duplicate members, missing fields, wrong types, duplicate IDs, duplicate labels, control-character labels and excessive pools. Each stops before an asking call. Count listener requests and pin zero.

Prove ZERO, ONE and MANY independently. One candidate must still allow `none`.

Plant a wrong copied value, swapped description and reversed option order. Compare with independent expectations.

Pin:

- Ordinary chosen `none`: label, exit **0**.
- Exact tied top: null, scalar exit **3**.
- Winner below cut: null, scalar exit **3**.
- Completed record run containing null: exit **0**.
- Unsupported question-file field or malformed question file: local exit **5**.
- Missing candidate pointer or invalid candidate list: input/usage exit **2**.
- Missing replay answer: exit **5**.
- Malformed recording or conflicting store files: exit **5**.
- Missing/malformed audit input or key: exact local failure under the audit contract.

The fixture’s singleton JSONL reading controls obey record-run exit semantics. Separate scalar controls establish scalar exit **3**; do not mislabel JSONL exit **0** as a scalar answer exit.

Failures are neither none nor not-sure answers.

### Stage 3: actual record, replay and cache proof

Controlled preparation uses seven singleton record asking invocations with retries disabled. Count exactly seven listener requests. Then convert the owned scratch recording.

Strict replay produces five ordinary asking rows and two reading-control rows with zero listener requests. R01 remains local and invokes no command.

Independently reconcile all eight case statuses, five ordinary audit rows, two reading controls and one local absence case. Candidate misses remain in task totals.

A separately owned cache round proves:

- Initial misses send exactly the declared fixture requests.
- The identical second run sends zero.
- A supported cut change rereads retained distributions with zero sends.
- Wording/model/options/description/evidence changes fail strict replay rather than falling through to a backend.

Use the listener as the zero-send oracle. `--plan`, `meta.cached` and reported request totals are supporting observations.

Bound preparation and the separate cache-fill round independently to seven listener requests each; record their distinct counters and cumulative ceiling of fourteen synthetic sends. Replay, rereading and replay-miss plants permit zero sends. Do not describe the preparation’s seven-request ceiling as covering a second cache-fill round.

Verify committed source-fixture hashes before and after replay. Reject mutation of committed artifacts.

### Stage 4: audit truth and coverage

Run the real audit against detailed saved rows and independent keys.

Plant wrong choice, duplicate result identity, missing key, wrong key type and deliberately omitted result row. Detect the intended condition and preserve the full input denominator.

Audit may exclude failed or unlabeled answers from measures. The harness still reports them in end-to-end counts and detects smaller retained sets that falsely improve a rate.

Tiny fixture calibration, suggested thresholds and perfect controlled agreement are not model-quality evidence.

### Stage 5: rendered site and publication boundary

Prove all four route/visibility states using owned normal and explicit-draft outputs and independent publication fixtures. A planted published fixture is test input, not a catalog publication or efficacy claim.

Normal all-draft proof requires:

- No recipe detail pages, recipe index content or Recipes menu/footer entry.
- `/recipes/` remains the exact Bash redirect.
- Fragment transformer, link checker and browser checker all validate it.
- Recipe incoming fragments retain existing serialization and fallback behavior.
- No recipe twins, sitemap/search entries or llms listings.

Explicit all-draft preview proof requires:

- `/recipes/` is exactly one static index with `data-draft`; no recipe redirect stub.
- Draft detail pages carry `data-draft`.
- Preview Markdown and llms bodies retain visible draft notices.
- No global Recipes menu/footer entry solely because drafts are previewed.
- Existing published site pages retain their normal visibility.

Normal published-selection proof requires:

- Index and navigation/footer include the independently declared published recipe only.
- Draft details and entries remain excluded.
- Published recipe has exactly one Markdown twin and the expected sitemap/search/llms visibility.
- `/recipes/` is canonical content, not a stub.
- No new actual recipe publication follows from the plant.

Published-plus-draft preview proof requires the published entry and explicitly marked draft entries without changing their dispositions.

In every state preserve all 61 compatibility addresses, all other fixed aliases, the original nine fragment cases, direct-page checks and Chromium fallback contracts. Add recipe-specific redirect-fragment and direct-index cases; report their actual counts separately.

Independent plants include:

- Draft page leaked into normal output.
- Draft catalog link leaked into normal navigation or footer.
- Published recipe omitted from index.
- Missing `/recipes/` output.
- Both redirect and index generated.
- Index emitted in normal all-draft state.
- Recipe stub retained in explicit preview or published state.
- Fragment preserver missing from normal recipe redirect.
- Fragment transformer attempting to rewrite recipe-index content.
- Any other alias omitted, retargeted or stripped of its preserver.
- Missing recipe Markdown twin.
- Measured or fixture value altered during rendering.
- Missing source, denominator, evidence class or qualification.
- Unsupported literal inserted outside a rendered carrier.
- Evidence lost in Markdown or `llms-full.txt`.
- Draft leaked into normal llms, sitemap or search.
- Duplicate long paragraph requiring a new unapproved exception.

Use literal independent rendering expectations for numerical controls. Do not derive expected Markdown by calling the renderer under test.

Retain A/B omission plants, six annotate h4 checks and existing content retention. Do not widen the 21 duplicate exceptions.

## Experiment eligibility and remaining waits

| Recipe or claim | D disposition |
| --- | --- |
| `rules-propose-model-confirms` | Scoped 0031 measurement accepted; draft publication proof pending |
| `link-records` | Draft; completed no-answer wording stop retained; improvement unknown |
| `verify-a-claim`, including qualify | Shared draft; constructed-claim limits and source review retained; qualification evidence remains separately scoped within the page |
| `navigate-many-documents` | Draft; waits on admitted 0010 numbers and compatible artifacts |
| `search-transcripts` | Draft; waits on 0401 reconciliation and publication proof; unequal reading lengths and overlap limits retained |
| `ask-your-cache-with-duckdb` | Draft; waits on admitted 0019 numbers and compatible artifacts |
| Extract function or annotate mode | Dropped |
| Choose none-wording improvement | UNKNOWN; separate measurement wait |
| Replace-regex-with-meaning post | Unsupported |
| Quality-qualified local-model winner | None established by 0030 |

Verify’s retained citation-support observation is 20/20 supported claims and 0/40 contradicted or unsupported claims, with frozen verdict score 57/60. These are constructed claims clustered within 20 excerpts from five RFCs, not independent observed citations. One weekday label has a defensible contrary reading. No protective cut was established; source review remains required. These limits do not establish qualification efficacy.

Transcript held-out evidence remains 13/29 ranges in 460 actual lines for narrow searches with context, 15/29 in 535 lines for broad search, and 13/29 in 270 lines for the same narrow ranks without context. Unequal reading lengths prevent an equal-length comparison; any-line overlap does not prove complete context. Earlier 14/15 in 570 lines did not generalize.

Earlier strict-selection evidence remains separate from 0031: choose 644/800, including 177/283 absent fields; discovery 483/517, leaving 34 unreachable present fields; generated annotate disagreed on 37 receipt labels. Different encodings remain confounded. Exact reduction to annotate is unsupported.

### 0030: qualified NONE

No completed path reaches the prespecified 45/60 bar.

| Model | Hard score | Function passes |
| --- | ---: | ---: |
| Laya | 14 | 4 |
| Kev4 | 17 | 5 |
| Strands | 20 | 4 |
| Clef27 | 28 | 10 |
| ClefFlash9 | 22 | 6 |
| Kev9 | 29 | 7 |
| Kev0.8 | 24 | 2 |

Kev9’s one-answer lead establishes no qualified winner. Clef27 has the most function passes. Timing remains directional; RSS has sampling limits; historical results lack original weight hashes. D derives no speed, memory-tier or quality recommendation.

Preserve route-specific loader and decision-encoding incompatibilities, Strands’ original score-check failure and its separately reviewed untouched-case continuation. Native llama.cpp evidence is text-only and establishes no image capability.

F10’s frozen zero threshold was rejected before model dispatch. Record an experiment-build incompatibility with no model wire. Exact executable/version/source identity is insufficient to establish a product defect; a later source pin does not retroactively identify that binary.

### 0032: terminal evidence, efficacy unanswered

Banking TUNE stopped at record 3851: 18 HTTP exchanges sent 3850 questions and retained 3850 valid answers; 37 questions were unsent. BGL never ran.

No threshold, combined-method score, TEST score or mistake example exists. All 16210 TEST records remain sealed. Perfect regex training fits supply no held-out replacement evidence. The replace-regex post stays unsupported.

The non-guaranteed plan estimate was used as a hard admission limit. Saved-byte arithmetic is consistent with a conditional ten-token rounding difference, but the refused body and source-to-binary identity remain unknown. Exact native admission cause remains unproved.

0032 does not answer the choose none-wording question. Keep current cells terminal. Commission no diagnostic, retry, efficacy pass or reveal.

### Admitted accounting

| Evidence | Usage-derived amount | Conservative exposure |
| --- | ---: | --- |
| 0031 new work | $0.182553 | $0.203782 |
| Historical 0027, retained separately | $0.068566848 | 0031 combined conservative total $0.272348848 |
| 0030 | $0 local model spend | Eight-question manual hold is accounting, not a dollar invoice |
| 0032 | $0.015998934 | Full $0.292177704 reservation, including known usage |
| Reported whole wave | $0.535958896 | $0.934418388 |

The 0032 old `$0.425252856` estimate and `$1.873360440` reservation are planning figures, not actual charges. None of these amounts is an independently verified invoice. They create no budget or authority for D.

## Runtime, cost and resource limits

Replay mode permits zero outbound sends. Use strict replay and explicit zero process request and estimated-input admission totals where supported, retaining the actual listener oracle.

Controlled preparation uses only an owned loopback server, explicit fixture model and address, no key, no backend from ambient configuration, retries zero and a per-invocation request cap of one.

Bound each synthetic asking stage to eight cases, seven asking invocations and at most two offered candidates plus none for MANY cases. Enforce the declared stage listener ceilings independently. Set estimated-input limits above inspected synthetic request bodies. These are admission limits, not dollar or output-token ceilings.

Use a minimal explicit environment, cached offline tools and owned configuration, cache, usage and output directories. Missing cached prerequisites stop proof. Install and download nothing.

No new live authority exists. Do not read, copy or alter the live ledger.

Future heavy work retains the 12 GiB memory and 1 GiB swap envelope, lane locks, isolated outputs and shared toolchain/cache mutation exclusion. Use two Cargo jobs if a later coordinator checkpoint invokes Cargo.

Scripts delete only directories created during that same run. Plant the lane directory as a cleanup target and require refusal before the first cleanup execution. Do not delete another lane’s outputs, stop its processes or change its locks.

Timing and load claims remain outside D. Any later timing run requires the separate `test-stress --run` path and authority.

## Review, checkpoint and landing sequence

1. Leave C’s exact active root checkpoint untouched.
2. Freeze and SHA-256 hash this complete corrected artifact.
3. Obtain fresh independent review of that exact complete artifact before saving or building.
4. Let C finish and land under its existing owner and proof obligations.
5. Reconcile the accepted D design against landed C and the immutable admitted planning record. Material changes require a new complete hash and review.
6. Save the accepted design through the coordinator’s authorized workflow.
7. Claim only the coordinator-assigned lane under `worktrees.md`; do not mutate main or another lane.
8. Implement infrastructure, all six hidden draft entries, required issue dispositions and the bounded Rules fixture.
9. Make the three relabels, two issue creations, Rules metadata update and milestone update in the same whole D implementation commit.
10. Run focused source/artifact checks, independent omission plants, owned loopback preparation and replay/cache/audit proof as authorized for implementation.
11. Run the offline normal and explicit-draft site build/check proof.
12. Obtain fresh independent source review of the exact candidate and complete proof record.
13. ROOT names a checkpoint before full `test`, `spec` or `surfaces` and owns required canonical350 proof.
14. Land and push the whole reviewed slice with accurate ticket and build-record status, without agent attribution. The lander frees the lane under existing rules.

Run `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` before any Rust code review. No Rust change is planned. Do not increase the ratchet for documentation or JavaScript. Reconcile the measured total after C lands and explain any later Rust growth.

This design claims no new canonical350, native, full-gate, CI, release, tag or publication receipt. Infrastructure landing may retain every recipe as a draft.

## Remaining blockers

D saving/building waits on exact-artifact hashing, fresh independent design acceptance, C landing and source reconciliation.

Rules publication additionally waits on reviewed synthetic labels, compatible retained or controlled recording, actual replay/audit/cache proof, public-safe commit-pinned provenance, full rendered-evidence validation and the coordinator’s required checkpoint.

The other five recipe publications wait on their own admitted evidence and compatible artifacts. Verify/qualify remains shared. None-wording remains unknown. 0413 and other 0405 audit follow-ups remain product work with separate contracts and reviews.

Infrastructure can land without those product changes and without publishing an unsupported recipe.

## What Ian can overturn

Ian can overturn recipe slugs, catalog order, index presentation, synthetic wording, illustrative candidate finder, controlled-fixture fallback and separation of infrastructure landing from individual publication.

A split of verify and qualify requires a recorded reviewed changed disposition. Changing wording or options requires matching fixture proof. Changing a measured claim requires admitted evidence. Route-generation or carrier changes require equivalent independent omission and rendering proof.

Any decision to publish remains separate from this complete proposed design. No choice here authorizes paid calls, raw or sealed data imports, native execution, CI, release work or publication.