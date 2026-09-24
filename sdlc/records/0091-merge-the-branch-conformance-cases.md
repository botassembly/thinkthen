# 0091: Merge the branch conformance cases

Status: built on `ticket/0091-conformance-union`. The code review (`0091-code-review.md`) found two blocking problems, and both are fixed below. Not merged.

## Result

`conformance/cases.json` grows from 27 to 54 cases. Every new case records its branch case under `provenance.branch`. The three one-question `annotate` cases have no branch case and carry `provenance.pending` for ticket 0095. The pure-core test and the command runner pass on the grown file with no key and no network.

The runner gains the three success arms, the `question_form` field, and the counter step the ticket names. `crates/thinkthen/src/cli/conformance_tests/command.rs` holds the command-side arms. The schema now refuses unknown case and success keys.

## Case by case

| Main case | Branch case | Encoding |
| --- | --- | --- |
| 26-filter-empty-list | 06 | `filter` with no exchange and `indexes: []`. The branch `jobs` field is left to the surfaces |
| 27-decide-many | 19 | `decide_many`, one exchange per record. The branch `jobs` field is left to the surfaces |
| 28-decide-many-repeated-texts | 80 | `decide_many` over ten alternating records |
| 29-usage-json-text | 08 | threshold 90, `question_form: text`, valid evidence, `usage` |
| 30-local-question-file | 10 | blank question, `question_form: file`, valid evidence, `local` |
| 31-usage-rank-blank-question | 23 | blank `rank` question, `question_form: text`, two records under `--lines`, `usage` |
| 32-score-equal-distribution | 76 | score 0.99, `level: weak` |
| 33-tag-threshold-excludes | 79 | cut 0.6 keeps one label |
| 34-annotate-repeated-texts | 82 | three records, one exchange each |
| 35-annotate-score-repeated-texts | 83 | three records, one exchange each |
| 36-annotate-two-columns | 84 | one record, two questions |
| 37-annotate-choose-one | none | values of `06-choose-billing`, pending 0095 |
| 38-annotate-score-one | none | values of `11-score-middle`, pending 0095 |
| 39-annotate-tag-one | none | values of `09-tag-two`, pending 0095 |
| 40-decide-counters | 17 | two calls, counter differences 1 request and 1 cache answer |
| 41-offsets-past-an-accent-and-an-emoji | 68 | `recognize`, one kind, start 10 and end 20 |
| 42 to 50 `-recognize-Cnn-relations` | 28, 29, 39, 40, 41, 45, 51, 60, 67 | `recognize` with relations |
| 51-same-kind-alerts | 69 | `relate`, kind `alert`, yes/no pairs |
| 52-cross-kind-staff | 71 | `relate`, `person` to `organization`, one choice per person |
| main's 19-find-none | 25 | already covered. The README now says why `bare` is `"none"` and `selected` is `null` |

The 31 recognize cases without a relation stay in main's fixture `crates/thinkthen/tests/fixtures/recognize-225`.

Dropped as the ticket says: 27 duplicates `24-deadline-fault`, 70 adds only bulk, and 18 and 81 stay with the surfaces. Cases 09 and 22 add nothing to the text and file split. `36-C09` has no relation, so it stays in the recognize fixture, which already splits "Karst and Vellum".

## How the recognize and relate exchanges were built

A throwaway loopback stand-in answered the real command's requests. It gave every token of a stated branch name the name's strength as its `IN` probability and 1.0 for the name's kind. Other tokens got 0.02. A relation question got the stated branch probability for a stated edge and zero otherwise. `none` took the rest, and an unstated yes/no pair got 0.1. The command ran under `--record` with a dummy key against that loopback address. The exchanges copy the recorded request bytes verbatim and the recorded responses. Before any case was written, the generator checked the command's names and edges against the branch expectation, mapped to main's shape. All twelve matched. The request digests were recomputed for the canonical address. `question_sha256` comes from the command's own `--details`.

Recognize questions use described kinds (`person`, `organization`, `place`), since a version-one file requires descriptions. Relate entities keep the branch record texts as names and gain the concrete kinds listed above.

## Red, then green

Each new case ran alone beside main's 27 through the unchanged `origin/main` sources:

- 26: the core said "has no successful exchange".
- 27 and 28: the core said "has success kind `decide_many`". The old runner passed them, because it never read the kind.
- 29 to 31: the core said "has no injection", and the runner stopped at "fault injection".
- 34 and 35: the core said "has wrong requests", because the old annotate path listed every exchange's request on each answer.
- 41 to 52: the core said "unknown verb `recognize`" or "unknown verb `relate`". The runner could not plan them.
- 32, 33, 36 to 39, and 40 passed on the old sources. Case 40 passed because the old schema ignored `counters`.

Each case with digests then ran alone on the new sources with its production digests blanked. Every one failed on its metadata or its request list and passed once the digests were restored. The branch digests of 32, 33, 27, and 28 equal main's, which confirms the two canonical forms agree.

The counter step went red with the cache folder removed: the second call sent again, and the loopback listener refused it (`Transport(Refused)`). Each acceptance refusal went red when its check was removed. With privacy off, ported mutation 3 (a `headers` key in an exchange) passed. With the kind check off, ported mutation 0 (`decide_several`) passed. With the captured-path check off, ported mutation 1 (a `captured` exchange with no path) passed.

## Code review fixes

1. The text-form arm passed for the wrong reason. It sent empty input, so `decide` and `rank` exited 2 whatever the question said. Cases 29 to 31 now carry valid `evidence`, `rank` runs under `--lines`, and a fault with a `question_form` must carry evidence. The runner pins each exact diagnostic: `thinkthen: --threshold: a single cut is above zero and at most one` for 29, `thinkthen: the question file's \`decide\`: a question is text, not white space` for 30, and `thinkthen: a question is text, not white space` for 31. Red: with 29's threshold set to 0.9 the runner failed on exit 4 against 2. With 31's question made non-blank it failed the same assertion. The core also failed both with "breaks no rule as usage".
2. Nothing guarded the unknown-key refusal. Ported mutations now plant `"surprise": 1` on a case and `"counterz": 1` in a `success` object. With `deny_unknown_fields` removed from `Case`, ported mutation 0 passed. With it removed from `Success`, ported mutation 1 passed.

Non-blocking notes applied: the provenance rule parses the leading case number, and a drop guard removes each case folder when the case fails. The stand-in limits stay as the review states them. A captured re-record is the only fix for them.

## Decisions Ian can overturn

- The command runner has no JSON-text door, so `question_form: text` passes the question's members as typed command-line values. The port guide equates the two.
- `annotate` over several records reads exchange `n` as group `n mod G` of record `n div G`. Each answer lists only its own record's requests. The existing two-group case keeps its meaning. This generalizes the annotate arm and adds no fourth arm.
- The runner checks recognize and relate only through the command. The pure-core test checks their shape, provenance, and one `result` answer, because their assembly lives in the command.
- Case 17's counters are read from the persisted totals folder, because the process counters have no getter and no reset.
- The branch 08 and 10 pair becomes 29 (text, `usage`) and 30 (file, `local`). The ticket's split made branch 10 `local`.

## Budget

Test and runner Rust: 524 nonblank lines added gross and 81 removed, measured with `git diff --unified=0 origin/main...HEAD -- 'crates/**/*.rs'`. About 70 of the added lines are the mutation test moved out of `conformance_tests.rs` to keep it under 500 lines. The ticket cap is 650. The ratchet rose to 44,340 on `origin/main` at `ce0e3d6f`. The runner arms needed new command-side code, and the shared helpers `asked`, `same_json`, and `record_requests` are reused, not copied. No production Rust changed.

## Gates

The branch was rebased onto `origin/main` at `ce0e3d6f`. At `906c39f2`, with `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, `THINKTHEN_URL`, and `THINKTHEN_CACHE` unset, one rung at a time, the one-minute load was 1.98 at the start. `install` exit 0. `lint` exit 0, `ratchet: crates 44340/44340`, `pages: 1 coming, 21 green`. `test` exit 0, 730 passed, 0 failed, 2 ignored across 13 result lines. `spec` exit 0, `demos: 21 green, 0 red`. `git diff --check origin/main...HEAD` passed. Only this record, the copied review, and the ticket's review line changed after that run.

No paid or live call ran. The generator and the counter step used loopback listeners only.
