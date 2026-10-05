Proposed public destination: `sdlc/planning/0402-c-glossary-design.md`. Returned here only; no file was written.

# 0402 slice C: Glossary and confidence

Status: frozen design for fresh Medium review against candidate `6eba7e3179a26140b0c09de63fddb6d45464ad23`.

A has the supplied source ACCEPT. Its actual 350 checkpoint remains running. B’s complete design has the supplied ACCEPT and SHA-256 `c8492286ed49d9062f4a77bb34cc95a318a4380f540847914e4e6cc4ac3aa38c`; B is not implemented. C implementation waits for its predecessors.

This design covers ticket 0402 outcomes 8 and 9 and the specified C wording changes. Everything described as implementation or proof below is future work.

## Scope and preservation

C defines the work-counting terms, explains the existing numbers, documents TypeSafe’s published confidence formulas, and distinguishes backend confidence from audit calibration.

It changes no function, command, flag, setting, request, reading rule, cache identity, result schema, numeric calculation, or failure behavior.

Preserve:

- A’s ten function homes, complete references, opening examples, goal lines, tool placement, Learn placement, and canonical internal links.
- All 61 redirect aliases: 42 historical aliases and 19 moved addresses. Preserve their direct destinations, fragment behavior, encoded and empty fragments, default anchors, noindex, metadata, and fallback links.
- The six retained annotate h4 headings, Markdown twins, `llms.txt`, and `llms-full.txt`.
- All 350 binding proof paths and hashes, all 143 discovered example scripts, `examples/REPLAY`, sample bytes, saved outputs, and exit codes.
- The page-owned three-row blind-spots recording, its `"john"`/exit 0 and `null`/exit 3 results, and the admitted open-book revision `a6a6be71`.
- Public installation wording at 0.1.2.
- B’s approved grid, blog, social-card, stylesheet, parser extraction, duplicate-paragraph, and writing-rule plans. C changes only content inside the trust page; it does not implement those plans.
- Existing parser, secrecy, cancellation, cache-miss, invalid-input, and conflict regressions.

Extract remains dropped. Choose-none wording remains pending its experiment. The “Rules propose, the model confirms” recipe still requires experiment 4’s comparison with recognize alone. C neither finalizes those texts nor treats strict receipt selection as that comparison.

No recipes, new tools, provider price claims, invoice claims, experiments, paid calls, native proof, release work, or publication belong to C.

## Evidence and authority

The local contract and code determine runtime statements:

| Subject | Admitted evidence |
| --- | --- |
| Public values and probabilities | `crates/thinkthen/src/public/results.rs` |
| Calls and attempt counts | `crates/thinkthen/src/public/results/call.rs`, `specification/result.md` |
| Typed score descriptions | `crates/thinkthen/src/public/builders.rs::ScoreBuilder::level` |
| Probability validation | `crates/thinkthen/src/core/probability.rs` |
| Distribution totals, leading options, ties, score arithmetic | `crates/thinkthen/src/core/answer.rs` |
| Current-rule reading of stored answers | `crates/thinkthen/src/public/asking.rs::Decided::judgment` |
| Reported confidence preservation | `crates/thinkthen/src/core/adapters/systemone/response.rs`, its response tests |
| Audit calibration pairs | `crates/thinkthen/src/core/measure/pairs.rs`, `measure/answer.rs` |
| Metadata digest | `crates/thinkthen/src/core/digest.rs` |
| Stored-answer identity | `crates/thinkthen/src/core/pack.rs::QuestionKey` |
| CLI contracts | `specification/{decide,choose,score,threshold,result,backends,audit}.md` |
| Cross-surface limits | 0405 report, matrix, and cell records; tickets 0406, 0407, 0408, 0411, and 0414 |

The admitted confidence message supplies the published formulas and this source link:

`https://docs.typesafe.ai/confidence#how-confidence-is-calculated`

The local TypeSafe confidence capture explains the distribution-summary meaning but predates the formula section. C attributes the formulas to the admitted published-formula record; it does not claim a fresh website verification.

No clinical, genomic, or literature research is required.

## File and route inventory

| File | Exact responsibility |
| --- | --- |
| `CONTRIBUTING.md` | Add the glossary and stable anchors |
| `specification/README.md` | Link the glossary beside the existing terminology guidance |
| `specification/backends.md` | Replace the unpublished-formula/live-sweep wording; document preservation and the saved discrepancy |
| `specification/audit.md` | Replace ambiguous calibration prose while retaining output members |
| `specification/result.md` | Define and link request identities and counts; qualify details and rereading claims |
| `site/src/pages/trust.astro` | Correct the number definitions, add confidence and audit distinctions, and give exact band boundaries |
| `crates/thinkthen/tests/public_batches/numbers_0402.rs` | Bounded regression through exported Rust APIs |
| `crates/thinkthen/tests/public_batches.rs` | Register that regression module |
| `spec/result.md` | Add a focused assertion over the existing replayed choice result |
| `sdlc/ratchet.json` | Match actual measured test-source growth |
| `sdlc/planning/0402-c-glossary-design.md` | Reviewed design |
| `sdlc/records/0402-c-glossary-build.md` | Future source, proof, failures, corrections, and limits |
| `sdlc/tickets/0402-docs-tell-one-story.md` | C status and evidence |
| Glossary issue | Close only after C’s reviewed implementation and required proof |

No route is added, renamed, removed, or redirected.

The affected site route is `/trust/`. Its generated `trust.md` and corresponding `llms-full.txt` content change through the existing exporter. `/learn/answers/`, `/learn/recording/`, and all function pages retain their existing bodies and addresses.

No catalog, stylesheet, layout, dependency, lockfile, schema, recording, or binding implementation changes are planned.

## Glossary content

Add `## Calls, requests and decisions` after “The four names” in `CONTRIBUTING.md`. Use explicit anchors `call`, `request`, and `decision`.

Exact definitions:

| Term | Definition |
| --- | --- |
| **call** | One use of a ThinkThen function on any surface. |
| **request** | One send to a model. |
| **decision** | One question answered about one piece of evidence. |

Follow with:

> One call can answer several decisions. A request can carry several questions. A retry sends another request. A cache hit or replay can answer a decision without sending a request.

Use this bounded counting example:

> One `choose` call over two records can answer two decisions in one packed request. Replaying the saved answers sends zero requests. If the original request is retried once before succeeding, two requests were sent.

Do not promise that every function maps one record to one backend question. Tag expansion, annotations, recognition stages, relation questions, packing, partial replies, and failures have their own accounting.

Add:

> `meta.requests` keeps its existing name for compatibility. It lists the stored-answer question keys behind a result. It does not count sends. `meta.requests_sent` counts the result’s attributed transport attempts, including retries. Run or call facts give the corresponding total.

`specification/README.md` links this section once. `specification/result.md` links the terms in its metadata explanation. The trust page links the glossary once, near its discussion of answers and failures.

Keep “The four names” and its existing anchor unchanged.

## Public API and result contract

No public API or result member changes.

Retain:

- `Answer::{Yes, No, Unsure}`.
- `Judgment::{Decision, Choice, Score, Tags}`.
- `Probabilities::YesNo { yes }` and `Probabilities::Named`.
- `Details::value()`, `probabilities()`, `nearest()`, `confidence()`, `requests()`, `requests_sent()`, and `to_json()`.
- `Facts::requests_sent()` as live attempts, including retries and failed attempts.
- `meta.requests`, `meta.requests_sent`, and `calibration.by_bin[].confidence`.

In the result metadata table, replace the `requests` explanation with:

> The stored-answer question keys behind this result, in answer order. These are request digests in the existing public vocabulary, not a list of transport sends. Packing and retries do not make their count equal to `requests_sent`. See the glossary and the question-store contract.

Keep the precise existing attempt attribution rules in `requests_sent`; add the glossary link without simplifying away batch shares, split-parent attribution, or shared in-flight sends.

Replace the blanket saved-run rereading promise with:

> Stored wire answers can be read under another supported reading rule without another send when the required questions and stages are already stored. A bare result is not a complete standalone store for every function. Strict replay refuses a missing question locally; ordinary cache mode may send it.

Qualify “A detailed result keeps everything” in its introduction:

> The following paragraphs describe the CLI details and native observation shapes named here. Complete aggregate details are not available through every generic C JSON and SQL route.

Link the 0405 audit and 0408 ownership from the specification’s limitations text. Do not rename the existing heading or disrupt incoming anchors.

## Backend confidence content

Replace “What the adapter keeps” with the existing preservation contract followed by three bounded explanations.

**Choice**

> TypeSafe publishes choice confidence as `(top − 1/n) / (1 − 1/n)`, where `n` is the number of options and `top` is the highest reported option probability. It rescales the winning probability relative to an even distribution.

For the existing three-option example, `top = 0.94` gives `confidence = 0.91`.

For fixed `n > 1`, a published-formula confidence cut `c` corresponds to the probability cut:

```text
top ≥ 1/n + c × (1 − 1/n)
```

State:

> For one fixed question, the published choice formula makes confidence and top-probability cuts equivalent after converting the cut. ThinkThen still cuts on the top probability and still treats an exact top tie as not sure. No live sweep is needed to establish that algebraic equivalence.

Limit this statement to the published formula. It does not assert that every reported vendor field matches it, or that one confidence cut means the same probability across different option counts.

**Score**

> TypeSafe publishes score confidence as one minus the probability-weighted distance from the most likely level, divided by the corresponding distance for an even distribution, floored at zero.

For levels indexed `0 … K−1` and a most likely level at `m`:

```text
distance         = Σ pᵢ × |i − m|
uniform_distance = Σ (1/K) × |i − m|
confidence       = max(0, 1 − distance / uniform_distance)
```

Explain that this summarizes spread around a leading level. It does not replace the weighted score position.

Do not invent TypeSafe’s tie-selection convention or its handling of rounded distribution totals. ThinkThen does not reconstruct confidence, so neither missing detail blocks C. Record both as limits on independent formula reproduction.

**Preservation and discrepancy**

> ThinkThen keeps the backend’s confidence field as received when it is valid. It does not compute or correct that field, and it never uses it as a threshold. A yes/no answer carries no confidence field.

Name `specification/fixtures/systemone/score-disruption.response.json` explicitly:

- Probabilities: `0`, `0.13`, `0.87`.
- Published-formula distance: `0.13`.
- Uniform distance from level 2: `1`.
- Published-formula result: `0.87`.
- Saved reported confidence: `0.79`.
- ThinkThen’s weighted position: `1.87`.
- The wire’s supplied `score: 1.86` does not replace that computation.

The discrepancy remains documented and tested. C neither repairs the fixture nor reports it externally.

Link the published formula page once from this section.

## Audit wording

Change “Calibration pairs” to:

> Each pair uses the probability of the answer given as run, under the question’s own rule, and the credit that answer earned against the key. Changing audit’s `--threshold` does not move these pairs.

Rename the table’s prose column from “Confidence” to “Probability used”. Retain the exact pairing rules:

| Answer as run | Probability used | Credit |
| --- | --- | --- |
| Yes/no says yes | `p` | 1 if keyed yes, otherwise 0 |
| Yes/no says no | `1 − p` | 1 if keyed no, otherwise 0 |
| Yes/no is not sure | `max(p, 1 − p)` | 0 |
| Choose or find | Highest option probability | Outcome credit, including the existing tie-share rule |

Explain that “probability of the answer given” is the general term; an unresolved yes/no answer uses the documented side-probability convention rather than a probability of `null`.

In “Calibration” and “Curve”, replace ambiguous prose references to confidence with “pair probability”. Keep:

- The ten bins and their boundaries.
- The formula, bootstrap method, interval, seed, and note.
- Every output member and value.
- Failed/unlabeled exclusions.
- Score/rank exclusions and pooled-tag limitations.

Add:

> `calibration.by_bin[].confidence` retains its name for compatibility. It is the bin’s mean pair probability. It is not the backend’s `answer.confidence` field.

Calibration error describes agreement between reported probabilities and labels in the measured set. It does not certify a future answer or establish population accuracy.

## Trust page content

Retain the page’s goal, title, description, samples, audit links, outcomes, and B’s planned `.grid` container.

Replace the universal lede with:

> The model reports probabilities behind its answers. You set supported reading rules and count the misses on your own labeled records.

This avoids claiming that every bare result exposes probabilities.

Use these definitions in “The words for the numbers”:

| Word | Planned text |
| --- | --- |
| probability | A number reported by the model for the question it was asked. A yes/no probability is the reported probability of yes. It is not a measured rate of correct answers. |
| probabilities | The reported numbers behind an answer. Choose and score carry a distribution across options or levels; accepted totals may differ slightly from 1. Tag labels have independent yes probabilities and need not add to 1. Availability depends on the details route. |
| threshold | A reading rule supplied by the caller. Its quantity depends on the function: for example, yes probability, winning option probability, or printed name strength. |
| band | Two boundaries on decide’s yes probability. Below the low boundary is no; at or above the high boundary is yes; the middle is not sure. |
| position | The number score prints: the probability-weighted average of zero-based level positions, divided by the accepted probability total. |
| strength | The printed number on a recognized name, computed from kind and span probabilities and rounded to four decimals. It is not itself a probability. |
| confidence | The backend’s own summary of how spread out the probabilities are. ThinkThen keeps it as reported and never cuts on it. A yes/no answer has none. |

Follow the table with:

> Audit also prints `calibration.by_bin[].confidence`. That compatibility name means the bin’s mean pair probability, not the backend’s confidence field.

> A probability of 0.9, a score of 1.6, and a confidence of 0.9 describe different quantities. None means that this outcome will be correct on 90% of future cases. Check agreement, coverage, and calibration on labeled records that represent the job.

Link detailed formulas to `specification/backends.md#what-the-adapter-keeps`; do not duplicate that explanation on the site.

### Exact band explanation

The ticket’s requested strict-distance sentence misses the inclusive low boundary. Use:

> Under `0.3:0.7`, decide is not sure exactly when `0.3 ≤ p < 0.7`. The interior has `|2p − 1| < 0.4`; the low boundary is also not sure, while the high boundary is yes. An uneven band such as `0.2:0.9` sets different bars for yes and no.

Add a small illustrative boundary table with a stable `band-boundaries` ID:

| Yes probability | `0.3:0.7` | `0.2:0.9` |
| --- | --- | --- |
| 0.2 | no | not sure |
| 0.3 | not sure | not sure |
| 0.5 | not sure | not sure |
| 0.7 | yes | not sure |
| 0.9 | yes | yes |

Label these probabilities illustrative. Preserve the threshold contract rather than changing it to fit the requested sentence.

### Score and abstention explanation

Use a short example:

> Three levels with probabilities 0.05, 0.30, and 0.65 give position 1.6. The most likely level is the highest one, but 1.6 is an average position. Score has no command threshold or not-sure answer.

For abstention:

> Decide returns `null` inside a band. Choose returns `null` below a supplied cut or at an exact top tie. A sole choose winner can be returned without a cut. These are valid unresolved answers. A failed question remains a failure.

Do not equate an explicit option named `none` with `null`. Do not add choose-none instructions or measured rates in C.

## Five layers and current limitations

The five layers are a proposed benchmarking model, not five independently stored product fingerprints:

1. Contract.
2. Item.
3. Wording.
4. Context.
5. Model and reading.

C does not publish the draft’s blanket independence or free-rereading claims.

The design/build record retains these distinctions:

- `question_sha256` currently combines contract, wording/descriptions/order, threshold, and optional calibration profile.
- Stored-answer identity separately includes adapter, resolved address, model, state, and wire question.
- A changed metadata digest does not itself establish a cache miss.
- A supported cut can change the read value without changing the stored wire answer.
- Score level order is part of its ordinal contract.
- Relation `single`/`either` changes questions; it is not merely a free cut.
- Recognition’s lower entity cut can expose relation stages that were never asked. Strict replay then refuses locally; ordinary cache mode may send missing stages.
- Batch packing belongs in experiment setup evidence, but it is not a new fingerprint layer implemented by C.

Retain gap ownership:

| Gap | Documentation limit until its proof lands |
| --- | --- |
| 0406 | Do not promise CLI saved criteria/score-rank parity across Rust, C, SQL, and foreign wrappers |
| 0407 | Shared context does not imply integrated per-record context |
| 0408 | Generic C JSON/SQL aggregate carriers do not universally preserve all probabilities and stages |
| 0411 | Engine/CLI rereading evidence does not prove changed-rule no-send behavior on every binding |
| 0414 | Annotate, find, recognize, and relate retain their separate-context refusals |
| Existing description gaps | Typed score descriptions and textual relation reads already work; rich relation objects, inline score-description CLI support, and DuckDB kind-description parity remain distinct issues |

No new universal function/surface matrix or capability claim is introduced.

## Retained measurements

C adds no performance rate.

Existing Beatles examples remain attributed to their named question, catalog, model, build, and cohort. Their sample counts remain sample counts. They establish neither population accuracy nor confidence in an individual outcome.

Retain experiment-pending wording and reviewed evidence limits. Do not infer a protective threshold from recipe results that established none. Do not claim new prices, billed spend, model identity, candidate coverage, or generalization.

## Planned proof

Use outside-in behavior and actual exported content. Do not land sentence-matching lint or tests that recompute production expectations from the same implementation.

### Exported Rust regression

Add `numbers_0402.rs` under the existing public-batches test harness. Use the existing canned loopback listener, serial guard, dynamic question loading, and exported engine/details APIs.

One table-driven regression covers:

- The committed score response, supplied directly as canned reply bytes.
- Full probabilities retained as `0`, `0.13`, `0.87`.
- `Judgment::Score(1.87)`.
- `nearest()` naming the highest level.
- `confidence() == Some(0.79)`.
- Serialized details retaining confidence `0.79` and value `1.87`.
- Exactly one counted request.
- No use of wire `score: 1.86` as the public value.
- Choice reads independent of reported confidence.
- A top tie producing an unresolved choice while detailed information remains available.

Expected numbers are hand-established contract values. The test never calculates its expected confidence with production code.

Reuse existing CLI choice, threshold, distribution-total, public-details, split-attribution, and audit golden tests for their established scenarios. Do not duplicate their complete suites.

### Executable specification

Extend `spec/result.md` over its already replayed choice row:

- Assert the winning probability and supplied threshold determine `value`.
- Assert the saved backend confidence is retained.
- Assert question-key count and attempts are distinct fields.

Keep its existing 22-row and ten-contract-row checks unless the actual source inventory changes for another authorized reason. This addition creates no example script or recording.

### Scenario and failure evidence

| Claim | Positive proof | Independent failure plant |
| --- | --- | --- |
| Confidence preserved | Public fixture produces `0.79` | Scratch adapter substitutes formula result `0.87`; regression must fail on confidence |
| Weighted score | Public value is `1.87` | Scratch reading uses wire score `1.86`; regression must fail on value |
| Confidence never drives choose cut | Winning probability clears its cut despite a differing reported confidence | Scratch reader cuts on confidence; retained choice regression fails |
| Band boundaries | Existing CLI threshold cases plus trust table above | Scratch low-boundary comparison changes inclusion; boundary assertion fails |
| Audit distinction | Existing goldens and pairing cases remain byte/value compatible | Scratch calibration uses backend confidence; relevant pair/bin assertion fails |
| Keys differ from sends | Existing split-details regression: three attempts, one key per row | Scratch result substitutes attempt count for key inventory; assertion fails |
| No second send | Existing counted CLI/public cache/replay cases, with exact scope | Disable lookup in scratch; listener count or strict-replay result fails |
| Exported documentation | Inspect actual `/trust/` HTML, `trust.md`, and its llms-full section | Remove only the numeric boundary table from a scratch export; numeric-row comparison fails |
| Glossary navigation | Built trust link reaches the glossary’s declared anchor; specification links resolve locally | Remove the target anchor in scratch; anchor validation fails |
| A/B preservation | Existing alias, heading, link, layout, card, sample, and proof checks | Retain their established independent plants; do not claim C reran them during design |

Each plant runs only in an owned scratch copy during future proof. Pin its intended assertion marker and exit status, then restore and pass the positive control. A generic nonzero result is insufficient.

The rendered-content proof compares numeric scenario rows and working links, not author prose. It checks the actual exporter output, including entity decoding and table retention. Any temporary proof script remains scaffolding and is deleted after its receipt is recorded.

Do not mutate A’s running checkpoint outputs, saved recordings, common live ledger, or shared installations.

## Checks, caps, and sequencing

Planned sequence:

1. Fresh independent Medium design review.
2. Successful predecessor checkpoints and landings; coordinator assigns C’s lane.
3. Reconcile this frozen inventory with landed A/B without widening scope.
4. Implement documentation and bounded regressions.
5. Run focused public-number, existing relevant CLI/audit, executable-result, and rendered-content checks offline.
6. Run offline site build/check with all A/B checks retained in their prescribed order.
7. Validate saved binding-proof inputs with the existing strict checker. Report this as saved-input validation.
8. Run policy, appropriate lint, ratchet, and ticket checks before Rust source review.
9. Obtain fresh independent source review of the exact candidate and evidence.
10. Coordinator names C’s checkpoint before full `test`, `spec`, or `surfaces`, and decides required docs/canonical350 execution.

Required pre-review policy command:

```text
CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py
```

Every future shell uses `/bin/bash`, `login:false`, without initialization files. Gates use an explicit minimal environment, owned scratch locations, cached tools, fake fixture credentials, and loopback canned replies. Missing cached prerequisites stop proof; nothing is downloaded.

Use the existing exclusive heavy-lane lock, isolated outputs, shared mutation locks, MemoryMax 12 GiB, MemorySwapMax 1 GiB, and at most two Cargo jobs. The coordinator may reduce concurrency under pressure. Heavy C proof does not overlap predecessors without capacity authorization.

No production Rust growth is planned. The new regression file has a design cap of 120 nonblank lines; registration has a cap of two added nonblank lines. Every Rust source/test file remains below 500 nonblank lines.

Against A’s measured `117988`, planned net growth is at most 122 lines. The ratchet must equal the actual measured total, never the allowance. Record the actual increase and why this regression earns it: it protects vendor-field preservation and score/value separation through exported APIs. Exceeding the design allowance requires simplification or renewed review. Reconcile any intervening source growth separately.

No fresh build, test, policy, replay, canonical350, native, release, or publication success is claimed by this design.

## Decisions Ian can overturn

Ian can overturn retention of the two compatibility names, glossary wording, the documented confidence discrepancy instead of an external report, and the placement of the explanations.

The band boundary correction follows the settled contract. Changing that behavior would require separate contract authorization and proof.

**C is complete for fresh Medium review. Implementation waits for predecessors; this turn performed read-only inspection only.**