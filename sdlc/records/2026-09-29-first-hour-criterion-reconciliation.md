# First-hour criteria: current evidence and queue correction

Source pin: clean `ticket/qf-first-hour-reconciliation` from main `cde1bdca`, 2026-09-29. This checks only [new-user stumbles](../issues/2026-09-20-new-user-stumble-register.md) and the [docs/how-tos issue](../issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md). The earlier [36-row readiness record](2026-09-29-other-remainder-readiness.md) calls docs pages 8/9 absent. That is stale: current `site/src/data/catalog.mjs` registers `long-lived-loop` at 702 and `judge-paragraphs` at 708; the actual scripts live under `site/examples/how-tos/bash/`. Its old `src/data/examples` path is not the current catalog. No site or issue file is changed here.

## New-user rows, counted once

| Row | Current evidence and disposition |
| --- | --- |
| 4 | **Unfulfilled, 0.1 message work.** `cli/asking/plan.rs::print_plan` writes the JSON plan only. `specification/decide.md` explains question and evidence in prose, but the requested plain words **above** dry-run JSON are absent. Keep this bounded command/help criterion; an existing dry-run page and compiled message case are the smallest later proof. |
| 8 | **Fulfilled by accepted `28fc9e4b`.** Two runnable commands precede the long-help explanation; `spec/decide.md` and the compiled-help case pin their order. Root closes the issue row; no second help pass. |
| 9 | **Explicit later page work.** Current `find` and `choose` still have the 255-candidate limit; no present catalog recipe demonstrates a cheap upstream first cut for an over-limit list. This overlaps docs page 11, which the issue explicitly places after 0.1. One eventual recipe should close both criteria. |
| 10 | **Fulfilled by accepted 0241 (`db7e2418`).** `site/examples/how-tos/bash/judge-paragraphs/1-paragraphs.sh` uses `awk -v RS=` then `jq -Rc` and `filter --field /text`; saved output has two complaint paragraphs. Catalog text names a sentence splitter and code parser. The 0241 build record reports replayed CLI smoke for this page. Same proof closes docs page 9. |
| 11 | **Fulfilled by existing `tag`.** [Tag](../../specification/tag.md) returns a label array, including empty success; [demo 39](../../demos/39-screen-a-message/README.md) pins `["urgent"]`. No extra `jq` recipe is needed for this alternative criterion. |
| 12 | **Fulfilled by accepted 0241.** `site/examples/how-tos/bash/agent-tool-guard/files/guard.txt` uses `case` on 0/1/3 and denies every other exit, including 2; its saved output is `allow`, `ask`, `deny`. The [0241 build](0241-site-and-sample-build.md) records the official `PreToolUse` mapping check and replayed example. This is the host-hook page requested, not proof of arbitrary host integration. |
| 13 | **Fulfilled by accepted 0241.** `site/examples/how-tos/bash/long-lived-loop/1-loop.sh` opens one `choose` coprocess, feeds three JSONL steps, reads one answer before the next, and closes its input; saved output has three action labels. It sets `--batch 1`: `core/batch.rs` closes at one record, while `cli/asking/batched.rs::Former::next` otherwise waits for a content cut, 50 ms pause or end of input. The [records contract](../../specification/records.md) explains the interactive choice. This avoids the producer waiting on an answer while the consumer waits for another input; it is no general no-hang or live-backend latency guarantee. Same proof closes docs page 8. |
| 14 | **Fulfilled by current refusals page and accepted 0241 site work.** `site/src/pages/refusals.astro` says screen, image and audio become text first. Same page closes docs page 6, whose neighbor list also names writing, redaction, clustering, sampling and extraction/graph tools. |
| 16 | **Fulfilled by accepted result guide `f6e22242`.** It compares five current `answer.kind` shapes before the long `--details` examples, especially singular `yes_no.probability` versus `choice.probabilities`. Root owns this row closure. This commit is not an ancestor of this branch's `cde1bdca` pin, so root should use the landed integration commit when editing status. |
| 17 | **Documentation fulfilled; optional telemetry remains a separate later idea.** [Records §jobs](../../specification/records.md#jobs) names default width 4, the vendor's 1,200/minute guidance, experiment 206's 1,267/1,319/1,272 at width 4 and the five width-3 checks. This cites old measurements; none is remeasured here. The proposed stderr measured-rate line is not an accepted required option and does not keep this page criterion open. Preventive pacing is tracked separately by register 30. |
| 18 | **Partly fulfilled; current onboarding fact is external.** [README](../../README.md) has a keyless, network-free `--replay` first run. The issue's marketing note says the 2026-09-15 waitlist announcement does not establish today's wait. Keep the current wait time/registry onboarding question with marketing through launch; do not publish the old one-day observation as current fact. |

Rows 7 and the bad-record recipe (docs 10) were already closed; they are not counted again. The new-user register deliberately remains open until launch. Among these named rows, only row 4 is an unheld first-hour command fix; row 18 needs a current external fact, and row 9 is later documentation.

## Docs/how-tos pages

| Page | Current disposition |
| --- | --- |
| 6, 8, 9 | **Fulfilled.** The refusals page, coprocess page and paragraph page above meet the literal criteria. The accepted [0241 code review](0241-code-review.md) and [build record](0241-site-and-sample-build.md) provide replay/build evidence; file presence or a catalog link alone is not the proof. In particular the build reports the four practical pages among 95 replayed CLI examples, not package/runtime qualification. |
| 11 | **Explicit after 0.1, unfulfilled.** First cut before an over-255 `find`/`relate` job; same eventual recipe as new-user row 9. |
| 12 | **Explicit after 0.1, unfulfilled.** Existing `lint-prose-for-hedging` numbers lines of one `draft.txt`, but does not search a directory or print `file:line` paths. |
| 13 | **Explicit after 0.1, unfulfilled.** No recorded trace page combines `find --none` for the failing step with `choose` for its error kind. The current `group-alerts-into-incidents` uses `find` for a different job. |
| 14 | **Explicit after 0.1, unfulfilled.** `code-open-ended-survey-answers` runs `score` then `tag`; it does not group `annotate` answers and ask a second `decide` question. |
| 15 | **Explicit after 0.1, partly fulfilled.** [Records §jobs](../../specification/records.md#jobs) explains ordered waiting, but no current `tail -f` live-log how-to or explicit no-state/no-delivery promise meets the page criterion. The old replayed line-timing observation is not a live-backend guarantee. |
| 16 | **Held SQL/DataFrame and later.** Recognition and relation demos exist separately, but no four-function incident-map page ends in SQL. Keep the named ADR 0105 hold; no SQL source was read here. |
| 17 | **Explicit after 0.1, unfulfilled.** There is no repository `examples/` directory or River Run replay example. This asks for a larger standalone example, not another first-hour shell recipe. |
| 18 | **Later measurement dependency, unfulfilled.** [Profiles README](../../profiles/README.md) says it ships no claimed backend limits; `conformance/backend-profiles.json` has synthetic local byte-edge cases, not the requested measured backend edge. Do not infer a byte ceiling from the old token bracket. |

Root can update the new-user rows above and change the docs issue's stale 6/8/9 gap text, retaining pages 11–18 under their stated later/held/evidence routes. In the work-plan table, the docs umbrella should no longer be queued as a **first-priority build-ready 0.1 page**: its first-hour pages are done, while the remaining pages are later, held or measurement dependent. Keep the umbrella open; neither umbrella is eighteen independent issues. The new-user row remains a bounded 0.1 message task because of row 4, with row 18's external launch fact separate. These are root's issue/status/plan edits, not edits made by this lane.

No tests, builds, network, provider or site commands ran. The next useful proof, if root doubts the accepted page evidence, is the existing offline 0241 replay fixture for the *specific* script and its saved `.out`, not a full site or provider rerun. This record only cites that accepted proof.

## Review and application

Fresh independent Medium review accepted candidate `b23f1a18`. The coordinator applied the exact row and page corrections to the two issues. Both umbrellas stay open; this closes subcriteria and changes no whole-item count. Row4 proceeds to bounded design0274.

## What the build taught us

The stale queue came from searching an old site data path and treating the umbrella's historical gap prose as current status. The actual catalog, scripts, saved output and 0241 acceptance settle the loop and paragraph criteria without new writing. A first-hour doc item can be complete while its umbrella stays open for explicitly later pages and launch intake. The source pin predates the accepted result guide's integration, so root must apply that separate closure against its landed commit.
