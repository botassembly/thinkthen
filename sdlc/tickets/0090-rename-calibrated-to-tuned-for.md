---
flow: build
priority: 90
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests conformance specification site/src/pages/reference.astro sdlc/planning/adr sdlc/ratchet.json sdlc/records
---

# 0090: Rename the profile-warning key `calibrated` to `tuned_for`

Status: design accepted 2026-09-24 after re-review. Owner: Claude.

## Design and decisions

Ian ruled on 2026-09-23 that `meta.profile_warning.calibrated` becomes `tuned_for`. The saved name identifies the backend profile under which a person tuned the threshold. It does not prove statistical calibration, so `calibrated` overclaims. This ticket lands the tool and specification half of item 44 in `sdlc/issues/2026-09-22-command-wording-and-help-fixes-for-0-1.md`. It splits from ticket 0082 after that ticket's design review, findings F3 through F7 in `sdlc/records/0082-design-review.md` on branch `ticket/0082-close-command-contract`.

Decisions, each of which Ian can overturn:

- The ruling renames only the result key. The question-file key stays `profile`. `sdlc/planning/quality-plan.md` says a question file's threshold carries `tuned_for`, and that means the result key.
- The result object becomes exactly `{"tuned_for":NAME,"running":NAME}`. New results emit only `tuned_for`. No compatibility reader exists, because ThinkThen has not released 0.1.
- The standard-error warning in `crates/thinkthen/src/cli/profile.rs` line 70 follows the same ruling. `threshold calibrated for profile X is running under profile Y` becomes `threshold tuned for profile X is running under profile Y`.
- The conformance case key in `conformance/backend-profiles.json` becomes `tuned_for`. Each surface ticket in queue item 10 inherits the key from that file.
- Doc comments that say `calibrated under` in `core/question_set.rs` and `core/question_file/resolve.rs` say `tuned under`. Internal Rust names that mean this field (`calibrated`, `calibrated_profile`) become `tuned_for` names.
- The noun `calibration` in `calibration name`, `calibration profile`, and `calibration identity` falls outside the ruling. This ticket leaves it unchanged. The landing record lists each remaining use for a later wording decision. The `calibration` transform measures real calibration and keeps its name.
- ADR 0032 defined the old key. It gets a dated amendment naming the new key and Ian's ruling.

Stop rule: this ticket changes exactly one result key, `meta.profile_warning.calibrated`. Stop and re-score if the work changes any other result key or shape, request bytes, digests, or exit codes.

Review route: a fresh read-only Claude session reviews this design and the final diff. The rename changes a public result, so Codex (Astra) reviews it as the other vendor under `sdlc/planning/one-line-plan-2026-09-24.md`.

## Closure of item 44

Ian ruled "the key is `tuned_for`, everywhere in one commit". Item 44 closes when the rename lands in the tool, the specification, all nine surfaces, and the site copy. One commit in this ticket renames the key in the tool, the specification, the conformance cases, and the site reference page at `site/src/pages/reference.astro` line 76, because all of them live on main. The surfaces are the only part that lands later, because they are not on main. Each surface ticket in queue item 10 inherits `tuned_for` from the conformance file. The issue stays open until the last surface lands. The landing record states which part landed and names Claude, the queue owner, as owner of the surfaces part.

## Scope

Allowed:

- `crates/thinkthen/src/core/result/profile_warning.rs`: the field and its serialized key.
- `crates/thinkthen/src/core/question_set.rs` and `crates/thinkthen/src/core/question_file/resolve.rs`: the `calibrated under` doc comments.
- `crates/thinkthen/src/cli/profile.rs`: the field, the accessor, the warning sentence, and its inline test at line 130.
- `crates/thinkthen/src/cli/asking.rs`: the accessor call.
- `crates/thinkthen/src/cli/conformance_tests/profile_cases.rs` and `conformance/backend-profiles.json`: the case key.
- `crates/thinkthen/tests/backend/profile.rs` and `crates/thinkthen/tests/backend/profile/warnings.rs`: the exact pins.
- `specification/result.md` line 94 and any other specification line that names the key.
- `site/src/pages/reference.astro` line 76: the `profile_warning` field string only.
- `sdlc/planning/adr/0032-explicit-backend-profiles-carry-limits-and-calibration.md`: one dated amendment.
- `sdlc/ratchet.json` and the landing record.

Excluded: the rest of the site, the deck, the nine surfaces, the `surfaces` branch, the noun `calibration` in help and specification prose, the `calibration` transform, command help, `sdlc/scripts/lint`, request bytes, digests, recording and cache formats, exit codes, dependencies, live calls, and paid calls. Historical tickets, records, issues, and probes stay unchanged.

## Deterministic acceptance

- Red first: change the exact test in `tests/backend/profile.rs` to expect `meta.profile_warning` equal to exactly `{"tuned_for":"old","running":"new"}` and the standard-error line `thinkthen: warning: threshold tuned for profile old is running under profile new\n`. Watch it fail on the old key and the old sentence. Make it pass.
- The inline test in `cli/profile.rs` and all four `matches(...)` counts pin `warning: threshold tuned for` and still count the warning exactly once per run: `tests/backend/profile.rs` line 441 and `tests/backend/profile/warnings.rs` lines 35, 71, and 108.
- The conformance cases in `conformance/backend-profiles.json` use `tuned_for` and `profile_cases.rs` reads it. All four cases keep their `warning` outcomes.
- A fixed-string test in `tests/backend/profile.rs` reads `specification/result.md`, `conformance/backend-profiles.json`, and `site/src/pages/reference.astro` and finds no `"calibrated"` key and no `calibrated` field name. It finds `{"tuned_for":NAME,"running":NAME}` exactly once in `result.md` and exactly once in `reference.astro`. A planted old line makes it fail.
- `git grep -n calibrated -- crates specification conformance spec demos site README.md` returns only the lines the record lists as historical or out of scope, each with its reason. The list includes `let calibrated` in the calibration-identity test at `core/digest.rs` line 302 and the fixture name `calibrated-question.json`. Neither names the field.
- Question and question-set digests, request bytes, and recordings are unchanged. The existing digest tests pass without edits.
- Run `sdlc/scripts/install`, `sdlc/scripts/lint`, `sdlc/scripts/test`, and `sdlc/scripts/spec` in sequence with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, then `git diff --check`. No live or paid command runs.

## Budgets

Production Rust changes may touch at most six files and add at most 10 net nonblank lines. Most edits are renames. Rust tests may touch at most four files and add at most 60 nonblank lines. Specification, conformance, site, and ADR prose may touch at most five files. Add no dependency. The ratchet increase equals the measured Rust increase, and the record says why it earns its lines.

## Dependencies and order

Queue order: 0089, 0082, 0083, 0090. None of them builds in parallel with another. Work starts from main containing the landing records for 0089, 0082, and 0083.

This ticket runs after 0083 because their files overlap. Both change Rust lines, so both re-measure `sdlc/ratchet.json`, where the ceiling equals the measured total. It runs after 0082 because both edit `specification/result.md`, and because 0082's help baseline and vocabulary check should not move under it.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 1; total 5. Final level: 2. Claude owns the queue and builds this ticket. Re-score and stop if the rename reaches beyond the files above.
