# Open issues for the architect, 2026-09-22

Status: a map. It authorizes nothing. A Sonnet reader classified every issue file whose status is not closed on 2026-09-22, and the product side checked the priorities. Ian can overturn any priority.

84 files are open by their status line. 23 of them are done in fact and only need a closing status line. Ten need a ruling from the architect or Ian. The rest are build work, and most of it is already in a lane of `build-queue-2026-09-21.md`.

Priority: P0 blocks 0.1. P1 lands in 0.1. P2 after 0.1. P3 reference or wording no user sees.

## First job: close what is done (23 files)

Add a closing status line to each. No code.

- `2026-09-18-two-lint-rules-do-not-hold-as-written` (fixed)
- `2026-09-20-feedback-to-the-experiment-team-after-both-harvests`
- `2026-09-20-live-probe-findings-packing-tagging-status-and-cost` (folded into the plan)
- `2026-09-20-what-the-two-experiments-ask-of-the-engine-and-the-order-to-build-it`
- `2026-09-21-a-last-job-for-the-recognize-team-a-harvest-package` (delivered)
- `2026-09-21-a-release-quality-experiment-is-open`
- `2026-09-21-a-second-backend-tried-through-the-systemone-adapter`
- `2026-09-21-feedback-to-the-quality-team-after-wave-one`
- `2026-09-21-filter-details-must-still-filter` (landed in 0056)
- `2026-09-21-four-small-closings-for-the-recognize-team`
- `2026-09-21-is-there-an-eleventh-function-a-sweep-of-the-three-primitives`
- `2026-09-21-one-rule-for-every-number-the-tool-prints` (ruled)
- `2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it` (passed)
- `2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief`
- `2026-09-21-the-disk-cache-is-never-on-unless-the-user-names-a-folder` (superseded by 0062)
- `2026-09-21-the-low-edge-of-a-band-excludes-the-value-that-reaches-it` (landed in 0056)
- `2026-09-21-the-product-sides-reply-to-the-build-teams-response`
- `2026-09-21-the-recognize-brief-for-the-experiment-team`
- `2026-09-21-two-function-flows-lose-the-record-between-stages` (closed by 0060)
- `2026-09-21-update-for-the-library-team-recognize-and-relate`
- `2026-09-20-database-extensions-ruled-in-as-a-fast-follow` (ruling recorded)
- `2026-09-20-the-first-release-is-0-1-on-every-surface` (ruling recorded)
- `2026-09-21-triage-of-the-open-issues-by-layer` (replaced by this file)

## P0: blocks 0.1 (2)

| Issue | Asks | Lane |
| --- | --- | --- |
| `2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything` | A resume under another address misses the cache and pays for every record again | A4 |
| `2026-09-20-launch-gaps-found-in-marketing-prep` | Four launch gaps: a key on first run, no installer, no exit-1 page, no owner for a second backend | A10 |

## P1: lands in 0.1 (24)

Command wording and help. Lane A1 groups these. One ticket can take them all:
- `2026-09-19-hands-on-test-pass-two`: ten wording mismatches on filter and rank
- `2026-09-21-a-refused-request-hides-the-backends-reason`
- `2026-09-21-exit-6-and-partial-failure-are-told-two-ways`: two spec pages contradict each other
- `2026-09-21-four-spec-sentences-promise-what-the-binary-refuses`
- `2026-09-21-question-file-refusals-name-the-wrong-thing`
- `2026-09-21-several-messages-cannot-be-parsed-by-a-stranger`
- `2026-09-21-the-empty-evidence-refusal-names-the-rule-backwards`
- `2026-09-21-statuses-400-and-500-carry-no-phrase`
- `2026-09-21-record-mode-always-exits-0-and-the-help-never-says-so`
- `2026-09-21-the-help-hides-the-defaults-a-user-assumes-wrong`
- `2026-09-21-the-help-first-lines-and-the-public-words` (no lane)
- `2026-09-20-new-user-stumble-register`: eighteen stumbles, stays open until launch

Shapes before the bindings freeze. Lane A7:
- `2026-09-21-what-must-land-before-the-bindings-freeze`: eight shape changes
- `2026-09-21-what-a-procedure-runtime-asks-of-a-judgment`: request digest and failure marker (no lane; feeds A7)
- `2026-09-20-libraries-ruled-in-and-every-public-name-is-thinkthen`: two-crate naming still contradicts the ruling

Release. Lane A10:
- `2026-09-21-nothing-says-how-the-command-gets-installed`: ruled, Homebrew plus a script
- `2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs`: the checklist
- `2026-09-20-what-the-launch-needs-from-the-build`: marketing's list

Engine. Lane A8:
- `2026-09-20-a-process-that-forks-after-its-first-call-hangs`

Measurement, no lane:
- `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun`: one crash in `cost.jq` and a rate-limit question

Library team, lane B:
- `2026-09-21-product-rulings-on-the-surfaces-adversarial-review`: ten rulings
- `2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api`: needs Ian's pick of a workaround

Marketing, no build work:
- `2026-09-21-the-public-examples-page-fails-as-printed`
- `2026-09-21-the-candidate-6-flow-goes-red-on-its-designed-outcome`

## Rulings owed (10)

| Issue | Question | Who |
| --- | --- | --- |
| `2026-09-21-the-third-and-fourth-outcomes-have-two-names` | "not sure" and "an error" on the deck against "unresolved" and "broken" on the pages. Pick one pair. The product side recommends the deck's words | architect |
| `2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it` | feeds the ADR 0017 rewrite | architect |
| `2026-09-21-quality-review-of-the-recognize-and-relate-designs` | ten usability points before A6 builds | architect |
| `2026-09-21-recognize-is-the-ninth-function-and-the-deck-needs-one-real-output` | items to settle for recognize | architect |
| `2026-09-21-candidates-for-a-tenth-function-relate-and-find-in` | `relate` is ruled in. `find-in` is not in the queue; rule it out or into backlog | architect |
| `2026-09-21-windows-over-long-text` | does long text need its own verb; the cheapest answer is no | architect |
| `2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields` | approved; make the ticket after 0065 | architect |
| `2026-09-21-rusqlites-loadable-headers-stop-at-sqlite-3-34` | push upstream, keep a workaround, or both | Ian |
| `2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api` | which workaround | Ian |
| `2026-09-21-one-state-per-request-caps-table-scale-classification` | whether a throughput claim needs a paid packing probe | Ian |

## P2: after 0.1 (19)

- Lane A1 wording: `the-probability-total-refusal-prints-float-noise`, `transport-failure-messages-paste-the-http-clients-own-words`, `the-shipped-help-breaks-the-approved-fixed-words`
- Lane A4: `a-run-stopped-by-sigint-prints-no-stopped-at-line`
- Lane A5: `the-same-request-answers-differently-twice-measured`
- Lane A8: `jobs-opens-one-connection-per-in-flight-request`
- Lane A11 and A12: `are-we-using-everything-the-service-offers`, `full-project-review-and-follow-up`, `second-coverage-pass-no-new-verb-and-six-soft-spots`, `use-cases-from-the-notes-that-no-page-teaches`, `how-to-candidates-where-one-answer-feeds-the-next`, `feedback-after-the-flagship-how-to-and-before-the-transforms`, `marketing-copy-breaks-the-approved-vocabulary`
- Lane B: `the-polars-shape-as-the-deck-shows-it`
- No lane: `small-leftovers-from-the-security-ticket` (control-character check), `packing-rows-into-one-request-measured` (a live-launcher bug wastes a paid run), `size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry`, `steering-on-the-version-one-completion-plan`, `what-a-tool-search-feature-asks-of-find-as-a-function`

## P3: reference and small cleanups (6)

`review-leftovers-from-the-core-tickets`, `review-leftovers-from-ticket-0006`, `review-leftovers-from-ticket-0010`, `the-set-e-warning-names-only-no`, `printed-speed-and-cost-numbers-name-no-measuring-record`, `the-literal-lines`, `where-a-user-could-lose-trust-a-first-list`.

## Duplicates to merge

The five wording issues under P1 overlap each other and belong in one ticket. The three review-leftover files belong in one. `the-help-first-lines-and-the-public-words` and `the-shipped-help-breaks-the-approved-fixed-words` ask the same thing.
