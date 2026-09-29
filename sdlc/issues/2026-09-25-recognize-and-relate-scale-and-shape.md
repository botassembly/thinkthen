# recognize and relate: scale and shape

Status: Open.

This issue merges seven files: `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, `2026-09-25-recognize-and-relate-scale-and-shape.md`, and one leftover from the deleted `closed/2026-09-23-relate-call-math-efficiency-and-real-jobs.md`. Each item was checked against main at `71ea0a84`, the two specifications, `sdlc/planning/relate-design.md`, and the tickets. They belong together because they share one engine path. The relation planner and the request splitter serve both commands. Every open item is about how that path grows with the input, or how its output and its fixed wording read to a user. Ian can overturn any fix below.

Ticket 0123 fixed items 1 and 2. Items 7 and 8 now have their current recognition guidance. Items 3 through 6 remain later features or separate proof.

## 1. relate sends a request over Jev's input token limit

Fixed by ticket 0123, landed 2026-09-25 from branch `ticket/0123-relate-fits-the-backend`.

## 2. Planning a full entity set encodes every request prefix

Fixed by ticket 0123, landed 2026-09-25 from branch `ticket/0123-relate-fits-the-backend`.

## 3. A both-ways edge prints a direction it does not have

Later feature.

What happens today. Ticket 0088 made each edge carry both names and kinds. An `--either` edge still prints `source` and `target`, normalized to input order (`specification/relate.md` line 44). The reader cannot tell a both-ways pair from a one-way edge without the rule.

What the ruling says. Ian's words, from his review of the relate examples on 2026-09-22: "relate source and target should be more obvious too. im worried that function isnt obvious or useful for real needs." The 2026-09-22 issue asked that a both-ways rule print a pair with no `source` and `target`, and that a one-way rule keep its direction and may print the sentence the rule implies from `reads`.

The fix. Give `--either` edges an unordered shape, for example `{"relation":"duplicates","pair":[{…},{…}],"probability":…}`. Keep the one-way shape. Update the specification, the Option A details, and every surface's edge type in one change.

Done when: an `--either` edge prints no `source` or `target` key on the command and on every surface.

## 4. relate takes one set, and a grown table re-asks every question

Later feature. The item 3 and item 4 asks of the 2026-09-23 issue merge here, because both need one input marked as new or as the other side.

What happens today. `relate` reads one complete entity set (`specification/relate.md` line 28). Ticket 0088 added the kind column, and cross-kind rules ask one choice per member of the larger side. That covers the cost half of the two-set job: payments and invoices in one file with a kind column no longer pay for same-side pairs. Two more things are missing.

- No form takes two inputs. The job "which invoice does this payment settle" still needs the user to merge the sets and add a kind.
- No form marks new rows. Every relate request state carries the complete entity array (`relate-design.md`, "Exact relation request state"). Adding one song changes the state of every request, so every question misses the cache and bills again. A new album also changes every song's choice options.

What the design says. `relate-design.md` defines only the complete-set form and says nothing about growth. The 2026-09-23 issue called new rows "the most likely thing a database user needs after the first run", and asked the manual to say that a new option bills every old question again.

The fix.

- Say now, in `specification/relate.md` and the database pages, that adding any entity re-bills the whole run.
- Design one form that marks a subset as new, from a second input or a column. It asks only questions that touch a new entity: a choice for each new cross-kind member, and yes/no pairs that hold a new entity within a kind. Build the request state so old questions keep their digests where the method allows. A two-set run is the same form with every row of one side marked new.
- Measure the bill for "ten new songs against 184 old ones" before and after.

Done when: a second run that adds ten entities sends only questions touching those ten, and the specification states what it re-bills.

## 5. Block before pairing when the set is large

Later feature. The 2026-09-23 item 4 (links to a known entity table) merges here.

What happens today. Same-kind yes/no pairs grow with N². At 255 entities an either rule asks 32,385 questions. Experiment 225 counted 5, 14, and 43 false edges at 10, 50, and 200 records. A cross-kind choice past 255 options falls back to yes/no pairs, which is costly at table scale. Nothing narrows candidates first.

What the design says. The deleted call-math issue recommended: "A `choose` or `tag` pass puts records in coarse groups, and pairs are asked only within a group." The 2026-09-23 issue asked for a cheap first cut (the `find` family or a plain text match) when mapping names to a table past the option ceiling. `relate-design.md` has no blocking step.

The fix. Add an optional blocking step to the shared planner. A key column, a text match, or one cheap question per entity puts entities in groups. Pairs and choices are asked only within a group. Dry run reports questions with and without blocking. Recognize keeps its current path, since one sentence holds few names.

Done when: a relate run with a blocking key asks only within-group questions, and dry run shows the saving.

## 6. The 255 refusal does not teach the pair math

Later. It is a local refusal that already works.

What happens today. More than 255 entities exits 2 with "relate takes at most 255 entities" (`crates/thinkthen/src/core/relate_file.rs:88`). Tests pin 255 and 256 in core (`relate_file/tests.rs`). No wave covers 254, 255, and 256 on every surface.

What the review says. Quality review item 8: the refusal names the limit, the pair arithmetic, and the narrowing move, and each surface tests 254, 255, and 256.

The fix. Word the refusal like "relate takes at most 255 entities; this set has N, and a same-kind rule would ask about N×(N-1)/2 pairs; split the set by kind or block it first". Add the three boundary cases to the shared conformance cases.

Done when: the 256 refusal names the count and the pair math on the command and every surface, and conformance pins 254, 255, and 256.

## 7. No page maps a recognize symptom to its dial

Fulfilled by the recognition-guidance Quick Fix. `specification/recognize.md` maps missing and excess names, a missing custom kind, and excess relation edges to the shipped cuts and description input. It states what each cannot recover and keeps measured quality numbers in the existing provenance paragraph. No obsolete span-gate number was reused. R8 still owes its separate batching cost and long-text measurement outcomes.

## 8. recognize discloses its current fixed wording

Fulfilled for the current ADR 0056 three-step design by the recognition-guidance Quick Fix. The historical news-document wording and ticket 0080 lineage-B numbers are superseded; `core/recognize/questions.rs::step_one_words` now uses fixed generic categories with no kinds, or caller kind names without descriptions. `specification/recognize.md` and compiled `recognize --help` both disclose that wording, the step-2 description boundary, and the absence of accuracy proof outside measured corpora. The existing compiled help assertion pins the disclosure on both surfaces. This does not establish new provider accuracy or change question digests.

## Already fixed

- 2026-09-22 ask 1, edges carry the records. Ticket 0088 prints both endpoint names and kinds on every edge (`specification/relate.md` line 41). Name and kind form a unique identity, since duplicates are refused.
- 2026-09-23 item 1, every surface takes a kind column. Ticket 0088 added `--kind-field` and the `fields.kind` key. The public Rust relate entity is a name and a kind (`crates/thinkthen/src/public/relate.rs`). The surfaces build on it, and the planner lives in the core.
- 2026-09-23 item 5, graph queries stay in the host. This is a standing rule with no work. `relate-design.md` adds no graph function.
- Call-math waste 5, two-set jobs pay for same-side pairs. The ticket 0081 hybrid planner asks cross-kind rules as one choice per member of the larger side. The stand-in engine that refused the kind field is gone.
- Call-math waste 2, nothing splits the request. Ticket 0079's splitter splits on profile limits. The no-profile gap is item 1.
- Quality review items 2 through 7 and 9 through 11 were closed before this merge, as the caller reported. They are not repeated here.

## CLI boundary progress, 2026-09-29

The CLI portion of item 6 landed from independently accepted `8e452e45b`: an oversized complete input reports its actual count, the 255 limit, clearly hypothetical unordered all-kind pair arithmetic and a split/narrow remedy. The existing live 256 refusal proves zero requests; focused 254/255 plans and exact 256/257 diagnostics pass. Shared core/public errors and the bounded public iterator are unchanged. Other host diagnostics and the per-surface boundary matrix remain open; SQL/DataFrame work remains held. See [the build record](../records/qf-relate-boundary-guidance.md).
