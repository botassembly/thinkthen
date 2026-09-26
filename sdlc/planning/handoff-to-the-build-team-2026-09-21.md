# Handoff to the build team, 2026-09-21

Written by the product side for Ian to give the build team when ticket 0052 is done. It bundles one day of rulings, two new functions, a caller's review, and the first quality wave. It authorizes nothing. Ian's plan with the build team turns it into tickets. Every line names its page, and the page holds the detail.

## 1. What changed today, in six lines

1. Ian accepted ADR 0017. The cache is on by default in the XDG folders with a 100 MB limit. One repo, laid out as `libraries/<language>/` and `databases/<engine>/`. Linux and macOS first.
2. There are ten functions. Ian ruled `recognize` and `relate` in, to be built together.
3. The question file is the one special form. It is not counted as a function.
4. A second backend answered the tool's exact request bytes with no change to the tool. A threshold did not carry between the two models.
5. A team that calls the tool from a program reviewed it and found shape gaps that a person typing commands does not meet.
6. The first quality wave ran 345 checks. The tool held where it matters, and the wave filed 21 findings.

## 2. The order the product side suggests

| Step | What | Why here |
| --- | --- | --- |
| A | Finish ticket 0052, the shared conformance cases | It is the contract every surface is tested against |
| B | The shape changes in section 4 | Nine surfaces will copy these shapes. Later is nine changes each |
| C | The four-step engine merge of ADR 0017 | Everything after it waits on it |
| D | The C door, designed once for results of no fixed size | `recognize` and `relate` need a returned string and one free function |
| E | `recognize` and `relate` in the core | Section 3 |
| F | The money and promise findings from the quality wave | Section 5, the blocker and the majors |
| G | The release pass: help in the public words, the manual, installation, the how-tos | Slice 13 of `plan.md` |

B may run beside C where a change sits in the pure core. The build team knows the merge and the product side does not. The order is theirs to change.

## 3. The two new functions

| Page | Holds |
| --- | --- |
| `sdlc/planning/recognize-design.md` | The command, the kinds, the relation rule, the output object, size and cost duties, the call on all nine surfaces, the database join rule |
| `sdlc/planning/relate-design.md` | The command, how it asks by choices over legal pairs, the edge output, the first real result, the call on all nine surfaces |
| `experiments/RECOGNIZE-PRODUCT-SPEC.md` | The method, final: three passes, each a pick-one question. The confidence formula, the dials, the measured limits |
| `experiments/THINKTHEN-RECOGNIZE-MASTER-REPORT.md` | The record of sixteen experiments, every number confirmed on a full corpus or marked |
| `experiments/222-recognize-demo/`, `225-relate-demo/`, `226-graph-demo/` | Real outputs with recordings, ready to become fixtures |
| the marketing repository's `decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md` | The acceptance test for the libraries and extensions: every call written out |

Four things the design pages rule that the method page does not: the output carries the user's kind word and never a code, a relation rule says `from`, `to`, and `either`, word positions show only under `--details`, and `--dry-run` prints the request count and the pair count. One engine path asks the pairs for both functions.

The method page names two fixes the core needs first: the tolerance on a probability total, and stating shared instructions once per request in place of once per question. The second one decides the public price of `recognize`.

Open for the build team: how a long text is cut into pieces, the question count one request may hold, one name for the number on a relation, and the record limit for `relate`.

## 4. The shape changes that land before the bindings freeze

The full table with sources is `sdlc/issues/closed/2026-09-21-what-must-land-before-the-bindings-freeze.md`. Ian and the caller's team agreed these.

1. `meta.request`, the request digest, on every function's result. `annotate` has it and the single functions do not.
2. A marker for a question that failed inside a request that otherwise succeeded. It is never `null`, because `null` means "not sure". A caller measured about 1 live reply in 15 refused.
3. A local size check before a request leaves, from a backend profile that states the limits. A caller measured inputs up to 1.4 MB. Today that is sent, refused at status 400, and the reason is hidden.
4. The record comes back with its answer.
5. The cache on by default, with its limit, its prune, `--no-cache`, the config file, and `thinkthen status`. The quality wave rates its absence the one blocker.
6. One name for the number on a relation.
7. Results of no fixed size through the C door.
8. A threshold per backend, or a loud warning that thresholds do not carry.

Three rulings taken now for a build that waits: a spend ledger row holds counts and never a text, the ceiling stays off unless the user sets one, and the count lives in the engine layer.

## 5. The first quality wave

The one page is `experiments/218-thinkthen-release-qa/wave1/FOR-IAN.md`. The proposed gates are `sdlc/planning/quality-plan.md`. Each finding is one issue file dated 2026-09-21 with a reproduction.

| Severity | Finding (issue file, without the date) |
| --- | --- |
| Blocker | `the-ruled-cache-surface-is-missing-from-the-command` |
| Major | `a-cache-resume-under-a-different-address-silently-re-bills-everything` |
| Major | `a-retried-send-is-invisible-to-the-user` |
| Major | `the-public-examples-page-fails-as-printed` |
| Major | `the-candidate-6-flow-goes-red-on-its-designed-outcome` (the product side fixes the deck. The tool question is whether `find --none` should end a pipe cleanly) |
| Major | `the-third-and-fourth-outcomes-have-two-names` |
| Minor | `a-cache-write-hitting-a-size-limit-kills-the-process` |
| Minor | `a-closed-connection-still-costs-the-whole-timeout` |
| Minor | `a-run-stopped-by-sigint-prints-no-stopped-at-line` |
| Minor | `a-write-failure-after-a-good-exchange-discards-the-paid-answer` |
| Minor | `lock-files-stay-after-their-entries-land` |
| Minor | `question-file-refusals-name-the-wrong-thing` |
| Minor | `the-empty-evidence-refusal-names-the-rule-backwards` |
| Minor | `the-probability-total-refusal-prints-float-noise` |
| Minor | `the-set-e-warning-names-only-no` |
| Minor | `the-shipped-help-breaks-the-approved-fixed-words` |
| Minor | `transport-failure-messages-paste-the-http-clients-own-words` |
| Minor | `the-public-word-for-the-judged-thing-needs-one-ruling` (Ian rules: "evidence" or "text") |
| Surprise | `jobs-opens-one-connection-per-in-flight-request` |
| Product side's to fix | `marketing-copy-breaks-the-approved-vocabulary`, `printed-speed-and-cost-numbers-name-no-measuring-record` |

Earlier the same day, from the limit probes and the second backend: `a-refused-request-hides-the-backends-reason`, `a-second-backend-tried-through-the-systemone-adapter`, and `size-cost-and-other-backends-what-the-manual-and-the-tests-must-carry`.

## 6. The backlog: agreed as good, not now

| Item | Page |
| --- | --- |
| The spend ledger and its ceiling | `what-a-procedure-runtime-asks-of-a-judgment`, item 2 |
| A cut inside a `score` question file | The same page, item 4 |
| The request count and every record's request under `--dry-run` for the eight functions | The same page, item 3. It is required for `recognize` and `relate` from the start |
| `find --in FILE`, the affordable join | `candidates-for-a-tenth-function-relate-and-find-in` |
| `filter` with a band and a choice of pile | `two-function-flows-lose-the-record-between-stages` |
| A `recognize` that takes names the user already found | `candidates-for-a-tenth-function-relate-and-find-in` |
| The `chat-logprobs` adapter and the subprocess adapter | ADR 0004. No second adapter was needed to reach a second backend |
| More open models | `experiments/220-thinkthen-second-backend/README.md` |
| The how-tos that chain functions, and the incident map with `recognize` and `relate` | `how-to-candidates-where-one-answer-feeds-the-next` |
| Windows | Ian's ruling: Linux and macOS first |

## 7. What the other teams hold ready

- **The surfaces rehearsal.** Worktree `thinkthen-surfaces` holds a contract, a stand-in engine, six libraries, three database extensions, and a checker. The real engine replaces the stand-in with one changed dependency. Its brief for the two new functions is `sdlc/issues/closed/2026-09-21-the-recognize-brief-for-the-experiment-team.md`.
- **The library team.** Its update is `sdlc/issues/closed/2026-09-21-update-for-the-library-team-recognize-and-relate.md`.
- **Polars.** Ian ruled that Python's data frame is Polars and that all scaling runs in Rust. The plan is `sdlc/planning/polars-plan.md`. What a user types is `sdlc/issues/2026-09-21-the-polars-shape-as-the-deck-shows-it.md`. It asks the engine for nothing new. It rides the same bulk call the lists use.
- **The quality team.** Wave 2 starts when the rebuilt libraries pass their own cases.

## 8. What Ian can overturn

All of it. The order in section 2 is a suggestion. Sections 3 and 4 carry his rulings.
