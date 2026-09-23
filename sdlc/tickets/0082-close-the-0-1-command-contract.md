---
flow: build
priority: 82
opens: README.md crates/thinkthen/src/cli crates/thinkthen/tests specification spec demos sdlc/scripts sdlc/ratchet.json
---

# 0082: Close the 0.1 command contract

Status: ready

## Outcome and authority

Re-audit all 45 items in `sdlc/issues/2026-09-22-command-wording-and-help-fixes-for-0-1.md` against the integrated command after tickets 0080 and 0081 land. Exercise root help and short and long help for all ten functions: `decide`, `filter`, `rank`, `choose`, `find`, `score`, `tag`, `annotate`, `recognize`, and `relate`. Change only mismatches that still survive in command wording, help, specification, diagnostics, vocabulary, or result documentation. Do not repeat a fix that the integrated tree already carries.

The issue's approved vocabulary and the later clarification in `sdlc/issues/2026-09-21-the-help-first-lines-and-the-public-words.md` govern outcome words. Public teaching copy says **not sure** and **broken**. The specification may introduce `unresolved` once as the formal contract/schema term and then use it where exact contract language needs it. `failed` remains valid for one question that a backend could not answer inside an otherwise usable `annotate` result; it does not name the whole broken outcome. This ticket makes that bounded reconciliation. Ian can overturn these words before implementation.

Ian ruled on 2026-09-23 that item 44 uses `meta.profile_warning.tuned_for`. The existing `calibrated` key overclaims what the profile records: the saved name identifies the backend profile under which a person tuned the threshold, but it does not prove statistical calibration. `tuned_for` states the measured meaning. The implementation changes the public result object, tests, examples, and readers before 0.1 and receives a second-agent public-surface review. It closes item 44 rather than deferring it.

## Required audit and disposition

The implementer starts from the merge base that contains the landed records for 0080 and 0081. Before editing product text or code, capture root help plus `-h` and `--help` for every function, inspect every source location named by the merged issue, and write the observed disposition into the implementation record. A prior ticket or an accepted design is not proof. The integrated binary or owning file is proof.

The expected disposition below is the ticket's bounded design. If the post-merge audit contradicts one row, correct that row in the record and fix only a surviving in-scope mismatch. Stop for design review if a contradiction requires new behavior, a new surface, or another outward decision.

| Item | Disposition | Required evidence or work |
| --- | --- | --- |
| 1 | fixed here | Reconcile public **not sure** and **broken** with the formal `unresolved` term under the rule above. Pin the four-outcome teaching sentence. |
| 2 | already fixed | Ticket 0066 root introduction remains exact. |
| 3 | fixed here | Preserve the eight accepted introductions and pin `recognize` as `Find every named entity in a text.` and `relate` as `Find ruled relationships among named, kinded records.` |
| 4 | fixed here | Pin root order as `status`; `decide`, `filter`, `rank`, `choose`, `find`, `score`, `tag`, `annotate`, `recognize`, `relate`; `cache`; generated `help`. |
| 5 | already fixed | Ticket 0066's description, usage, options, examples layout remains intact. |
| 6 | fixed here | Complete the approved fixed-word sweep in built help, README, and green how-tos, with contextual allowances stated below. |
| 7 | already fixed | Ticket 0057 names the `0.5` default wherever the option exists. |
| 8 | already fixed | Ticket 0057 names the default width of 4 in long help. |
| 9 | already fixed | Ticket 0057 and ADR 0031 put `--url` in short help and keep other advanced options in long help. |
| 10 | already fixed | Ticket 0057 gives `find --none` empty output and exit 3. |
| 11 | already fixed | Ticket 0057 refuses zero timeout locally. |
| 12 | fixed here | Every record-capable command, including `recognize`, carries the exact accepted record-run exit sentence. `relate` reads one whole entity set and carries its whole-set exit contract instead. |
| 13 | already fixed | Ticket 0058 names both no and not sure in the `set -e` warning. |
| 14 | already fixed | `rank` help describes its printed order rather than input order. |
| 15 | already fixed | `filter` and `rank` no longer describe a nonexistent unframed field mode. |
| 16 | already fixed | `--input` directory refusal names the option and emits no framed-record stop line. |
| 17 | already fixed | Invalid framed bytes say that the record is not valid UTF-8. |
| 18 | already fixed | Ticket 0058 omits recording counts when no recording feature was named. |
| 19 | already fixed | System One and recording JSON refusals name the object that is not valid. |
| 20 | already fixed | Ticket 0058 uses singular `field` for one CSV field. |
| 21 | already fixed | Ticket 0058 gives a fixed action for a recording path that is a regular file. |
| 22 | already fixed | Ticket 0058 documents the question-name rule and points the refusal at it. |
| 23 | already fixed | Ticket 0058 names empty or blank evidence directly. |
| 24 | already fixed | Ticket 0058 names the unknown single-question-file key. |
| 25 | already fixed | Ticket 0058 names the missing `questions` wrapper first. |
| 26 | already fixed | Ticket 0058 chose a fixed tool-owned status-400 phrase and suppresses hostile bodies. |
| 27 | stale | Ticket 0058's secrecy decision forbids exposing backend refusal bodies. Recording one would retain untrusted evidence-bearing text without a user-visible contract. |
| 28 | already fixed | Ticket 0058 maps structured transport failures to fixed tool-owned guidance. |
| 29 | already fixed | Ticket 0058 prints tolerance `0.01`. |
| 30 | already fixed | Ticket 0058 applies one-document width validation to dry-run before a plan or send. |
| 31 | already fixed | Ticket 0057 aligns exit 6 and partial question failure. |
| 32 | already fixed | Ticket 0057 distinguishes an empty document from an empty framed stream. |
| 33 | already fixed | Ticket 0057 distinguishes empty line/JSONL streams from headerless CSV/TSV. |
| 34 | already fixed | Ticket 0057 makes an exact `choose` tie unresolved with or without a cut. |
| 35 | already fixed | Ticket 0057 repairs the detailed `rank` and `tag` examples. |
| 36 | already fixed | `result.md` says `filter`'s `value` is the cut's boolean. |
| 37 | already fixed | A `rank` plan omits an inapplicable threshold source and `from` names only settings the function takes. |
| 38 | already fixed | `result.md` says `question.verb` names the question kind; `filter` and `rank` therefore report `decide`. |
| 39 | already fixed | The how-to index no longer carries the stale built-command list. |
| 40 | already fixed | The root README no longer claims that its examples use an unlanded grammar. |
| 41 | already fixed | The landed 0080/0081 work must leave the specification index and result-kind count at ten functions and the current result kinds. |
| 42 | already fixed | Live-call text promises robustness or reports measured variation; only replay promises the same recorded answer. |
| 43 | fixed here | Remove `decider model` and `decision model` from current public command, README, specification, and how-to prose. Use `System One model` when naming the product type and `the model` otherwise. |
| 44 | fixed here | Rename the profile-warning field from `calibrated` to `tuned_for` in every owning schema, example, reader, and exact test. New results emit only `tuned_for`; no compatibility reader is required because ThinkThen has not released 0.1. |
| 45 | fixed here | Make `annotate` begin exactly `Answer a saved set of questions about every record.` everywhere its command introduction is owned in this repository. |

## Scope

Allowed work is narrowly limited to surviving rows 1, 3, 4, 6, 12, 43, 44, and 45 plus their exact tests and mechanical vocabulary check. Edit existing command help, root README text, specification text, green how-tos, and result wording only where one of those rows requires it. A diagnostic may change only if the post-merge audit proves that an item classified already fixed still reproduces and the repair changes wording or context without changing behavior. Any contradiction outside the authorized rows stops implementation for a reviewed ticket amendment or a separate bounded ticket; the implementer may not silently widen scope. Update the exact source ratchet only for necessary Rust tests or help text. The implementation record carries the final 45-row disposition and cites the test or file that proves every `already fixed` and `stale` row.

The vocabulary check runs from the existing ladder. It reads the built root help and short and long help for all ten functions, plus README and green how-to prose. It rejects `decider model`, `decision model`, `rating`, and unsanctioned uses of `judgment`; `document` where a new user means text; `row` where the contract means record; and `label` where `choose` means option. It rejects `unresolved` in first-use teaching copy but permits the one formal-definition bridge and exact specification/schema uses. It permits `label` for `tag`, CSV `header row`, `failed` for an individual failed question, and `probability` for a backend-reported probability. If Ian keeps `calibrated`, the checker permits that literal field only where the ruling requires it. If he chooses `tuned_for`, current public examples and emit-path tests must use `tuned_for`; `calibrated` remains permitted only in ADR/history and explicitly required compatibility fixtures. The check prints the file, line, word, and violated rule for every hit and includes planted self-tests for one banned and one sanctioned use.

Excluded: new commands, options, functions, dynamic option sources, threshold comparison, recognition or relation policy, planner changes, APIs, language or database surfaces, the `surfaces` branch, packages, installers, workflows, publication, site or deck edits, dependencies, request bytes, cache or recording formats beyond Ian's item-44 ruling, unrelated result shape changes, exit-code changes, live calls, and paid calls. Do not edit the merged issue merely to declare victory; the landing record supplies the observed closure evidence.

## Deterministic acceptance

- The baseline artifact contains 21 help captures: root help and both help modes for each of the ten functions. A table names the command, capture, exit code, and exact first sentence. The post-change artifact has the same 21 members. A byte diff is empty except for lines authorized by rows 1, 3, 4, 6, 12, 43, and 45 and unavoidable 0080/0081 landing content already present in the baseline.
- One exact inventory assertion pins root order as `status`; the ten functions in the order named above; `cache`; generated `help`. It finds each function once and no eleventh judgment function. Exact short and long help assertions pin `recognize` and `relate` to the introductions above while preserving their beta warning, cut description, and cost disclosure.
- Every record-capable command carries the exact sentence `A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers.` exactly once in long help. `relate` carries no record-run sentence and instead pins its whole-set exit wording from ticket 0081.
- Exact compiled assertions pin `annotate -h` and `annotate --help` to `Answer a saved set of questions about every record.` as their first sentence. The root row uses the same sentence. Its usage, options, result-shape paragraph, examples, record-exit sentence, and exit-6 sentence remain present exactly once.
- Exact assertions pin the four-outcome teaching copy to yes, no, not sure, and broken. Specification text that uses `unresolved` identifies it as the formal name for not sure. No public teaching sentence uses `error` as the fourth outcome or `failed` for the whole run.
- The vocabulary checker scans built help for all ten functions and source prose in README and every green how-to. Its unsanctioned-hit count is exactly zero. Its self-test exits nonzero for planted `decider model` and accepts planted `tag label`, CSV `header row`, individual `failed question`, and the formal `not sure (unresolved)` bridge.
- Fixed-string checks find zero public-prose uses of `decider model` or `decision model` in README, specification, green how-tos, and built help. Historical tickets, records, issues, probes, recordings, fixtures, and measured quotations remain unchanged.
- A machine-readable 45-row audit in the implementation record contains every integer 1 through 45 exactly once and only one of `already fixed`, `stale`, or `fixed here`. It names the proving command/test/file for every row. Row 44 names Ian's ruling and the exact compatibility behavior it required.
- Every changed local refusal is tested at its exact exit code and complete standard-error sentence. A counted loopback listener observes exactly zero requests for each such case. A dry run alone is not accepted as no-send proof. If the audit finds no surviving diagnostic mismatch, no diagnostic source changes.
- Focused help, vocabulary, specification, and changed diagnostic tests fail for the stated old text before the fix and pass afterward. Run `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` sequentially with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, followed by `git diff --check`. No live or paid command runs.

## Budgets

Production Rust changes may touch at most four files and add at most 80 nonblank lines. Rust tests and test helpers may touch at most four files and add at most 220 nonblank lines. The vocabulary enforcement may add at most 140 nonblank lines across existing scripts and fixtures. Public prose may touch at most twelve files and add no more than 120 net nonblank lines; prefer replacement and deletion. Add no dependency and keep every Rust file below the existing 500-line ceiling. The ratchet increase must equal the measured Rust increase and the record must name why each added block earns its lines and where duplication was removed first.

Stop and re-score if the audit needs more than these budgets, changes a result key or shape, alters request or runtime behavior, reaches outside this repository, or finds another outward decision.

## Dependencies

Ticket 0080, designed at `5908f3551cacc85484a3edb7ba286c08d9079051`, must land first with `recognize`, its help, specification, result shape, and tests. Ticket 0081, designed at `ca7cf7442a5d208d0f6f538716c83568ccf5bba2`, must then land with `relate`, its help, specification, result shape, and tests. This ticket starts its audit only from main containing both landing records. Ian's 2026-09-23 item-44 ruling selects `tuned_for`. Tickets 0057, 0058, 0066, 0067, and 0087 supply the accepted prior fixes. ADR 0032 owns the old field and this ticket records its replacement before 0.1.

## Complexity

Contract 2; state/timing 0; reach 1; proof 2; cost of error 1; total 6. Final level: 2. Luna Extra High owns implementation design, case analysis, code, and remediation under the three-ticket trial. Independent Sol High sessions recheck this amended ticket after Ian's item-44 ruling and review the final diff. Re-score and stop if work crosses any exclusion, changes runtime semantics, or starts item 44 without Ian's ruling.
