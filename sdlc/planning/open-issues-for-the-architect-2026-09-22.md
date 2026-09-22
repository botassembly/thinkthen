# Open issues for the architect, 2026-09-22

Status: a map. It authorizes nothing. A Sonnet reader classified every issue file whose status is not closed on 2026-09-22, and the product side checked the priorities. Ian can overturn any priority.

Cleaned up on 2026-09-22 at commit `2000932`: 22 done files were closed, 16 wording issues were merged into `2026-09-22-command-wording-and-help-fixes-for-0-1.md` (40 items, one ticket in lane A1), and 4 leftover lists were merged into `2026-09-22-small-leftovers-from-early-reviews.md` (22 items, low priority). What remains open is below.

Priority: P0 blocks 0.1. P1 lands in 0.1. P2 after 0.1. P3 reference or wording no user sees.

## P0: blocks 0.1 (2)

| Issue | Asks | Lane |
| --- | --- | --- |
| `2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything` | A resume under another address misses the cache and pays for every record again | A4 |
| `2026-09-20-launch-gaps-found-in-marketing-prep` | Four launch gaps: a key on first run, no installer, no exit-1 page, no owner for a second backend | A10 |

## P1: lands in 0.1

Build team:
- `2026-09-22-command-wording-and-help-fixes-for-0-1`: 40 items, one ticket, lane A1. Item 1 is the ruling on the words for the third and fourth outcomes. Item 26 holds a conflict on the 400 body that the ticket picks. Check each item against the binary first; ticket 0057 may have landed some.
- `2026-09-20-new-user-stumble-register`: stays open until launch.
- `2026-09-21-what-must-land-before-the-bindings-freeze` and `2026-09-21-what-a-procedure-runtime-asks-of-a-judgment`: shapes before the libraries copy them, lane A7.
- `2026-09-20-libraries-ruled-in-and-every-public-name-is-thinkthen`: two-crate naming still contradicts the ruling, lane A7.
- `2026-09-21-nothing-says-how-the-command-gets-installed`, `2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs`, `2026-09-20-what-the-launch-needs-from-the-build`: lane A10.
- `2026-09-20-accuracy-round-on-three-public-sets-and-a-speed-rerun`: a crash in `cost.jq` and a rate-limit question. No lane.

Marketing:
- `2026-09-21-the-public-examples-page-fails-as-printed`
- `2026-09-21-the-candidate-6-flow-goes-red-on-its-designed-outcome`

## Handed to the library team (Ian, 2026-09-22)

1. `2026-09-21-product-rulings-on-the-surfaces-adversarial-review`: ten rulings on their fix wave. Close what is done.
2. `2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it`: confirm each pick against the code and record any difference.
3. `2026-09-21-the-polars-shape-as-the-deck-shows-it`: mirror the plain-list form, no new names.
4. `2026-09-21-rusqlites-loadable-headers-stop-at-sqlite-3-34`: pick the workaround and record it. Any upstream report is Ian's call.
5. `2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api`: pick the workaround and record it.
6. `2026-09-20-a-process-that-forks-after-its-first-call-hangs`: the engine fix is lane A8; the library team owns the proof on each language.
7. `2026-09-21-jobs-opens-one-connection-per-in-flight-request`: one sentence on each surface that exposes `jobs`.

## Rulings owed to the architect (5)

| Issue | Question |
| --- | --- |
| `2026-09-21-quality-review-of-the-recognize-and-relate-designs` | ten usability points before A6 builds |
| `2026-09-21-recognize-is-the-ninth-function-and-the-deck-needs-one-real-output` | items to settle for recognize |
| `2026-09-21-candidates-for-a-tenth-function-relate-and-find-in` | `relate` is in. Rule `find-in` out or into the backlog |
| `2026-09-21-windows-over-long-text` | the cheapest answer is no new verb |
| `2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields` | approved; make the ticket after 0065 |

One ruling for Ian: `2026-09-21-one-state-per-request-caps-table-scale-classification`, whether a throughput claim needs a paid packing probe.

## P2: after 0.1

- Lane A4: `a-run-stopped-by-sigint-prints-no-stopped-at-line`
- Lane A5: `the-same-request-answers-differently-twice-measured`
- Lanes A11 and A12: `are-we-using-everything-the-service-offers`, `full-project-review-and-follow-up`, `second-coverage-pass-no-new-verb-and-six-soft-spots`, `use-cases-from-the-notes-that-no-page-teaches`, `how-to-candidates-where-one-answer-feeds-the-next`, `feedback-after-the-flagship-how-to-and-before-the-transforms`, `marketing-copy-breaks-the-approved-vocabulary`
- No lane: `packing-rows-into-one-request-measured` (a live-launcher bug wastes a paid run), `size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry`, `steering-on-the-version-one-completion-plan`, `what-a-tool-search-feature-asks-of-find-as-a-function`

## P3: reference

`2026-09-22-small-leftovers-from-early-reviews`, `printed-speed-and-cost-numbers-name-no-measuring-record`, `the-literal-lines`, `where-a-user-could-lose-trust-a-first-list`.
