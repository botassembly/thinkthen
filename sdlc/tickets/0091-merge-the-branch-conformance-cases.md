---
flow: build
priority: 91
opens: conformance crates/thinkthen/src/cli/conformance_tests crates/thinkthen/tests/fixtures/recognize-225 crates/thinkthen/tests/backend/recognize.rs specification/question-file.md sdlc/planning
---

# 0091: Merge the branch conformance cases

Status: design accepted 2026-09-24 after re-review. Owner: Claude.

## Outcome and authority

Fold the portable cases from the `surfaces` branch's `conformance/conformance.json` into main's `conformance/cases.json`, so 0085 and every surface run one file. Queue item 6 of `sdlc/planning/one-line-plan-2026-09-24.md`. Section 4 of `sdlc/planning/surfaces-port-guide.md` maps the cases. The ADR 0017 shared-case amendment governs format and provenance. Ian can overturn any word ruling below.

## Cases

- New cases take IDs from 26 upward and record their branch ID in `provenance`. Branch IDs collide with main's.
- Add branch cases 06, 19, 80, 23, 76, 79, 82, 83, and 84 after re-encoding per port guide section 4.1.
- Add one-question `annotate` cases for `choose`, `score`, and `tag` whose values equal their scalar cases. They follow ADR 0017 section 6 item 8 (bulk is the same verbs over the host's container). Ticket 0095's ruling on the bulk form is pending review, so these cases carry a pending mark until 0095 is accepted. The mark sits in `provenance` only. The cases still run and must pass, so no skip table returns.
- Recognize stays in main's fixture folder `crates/thinkthen/tests/fixtures/recognize-225`, whose 40 replay files pass today. Branch cases map branch to main (`other` to `MISC`), and main's ten counted divergences stay as they are. `cases.json` gains only branch case 68 (offsets past an accent and an emoji; TypeScript converts them to UTF-16, Q13) and the relation cases below.
- Relate: one same-kind case (branch 69) and one cross-kind case (branch 71), plus the nine recognize cases with relations. All enter as `synthetic_contract` exchanges built from main's planner request bytes. Entities gain concrete kinds, since the branch method used `*` over bare text. Expected edges follow from the stated synthetic probabilities, and 36-C09 splits "Karst and Vellum" under main's rule. Branch case 70 adds only bulk and is cut. A captured re-record needs a live run under Ian's authorization and is deferred past 0.1.
- Case 17 asserts counter differences around a call, since counters have no reset (Q11). It runs in the command runner with a temporary cache folder, not in the pure-core test.
- Branch case 27 duplicates main's `24-deadline-fault` and is dropped. Branch case 18 (a mid-batch cancel that keeps finished results) stays with the surfaces.

## Runner arms

The command runner gains three success arms (`decide_many`, `recognize`, `relate`) and one `question_form: text|file` field for the usage and local pair, and one counter step for case 17. Nothing else.

## Rulings this ticket applies (Q16)

- The cases say `unsure`, per ADR 0017 section 6 item 4. The specification grammar keeps "unresolved". Main's case IDs already agree.
- `find` with no selection is `null` in `operation`, as `specification/find.md` says. Case 19 is correct and stays unedited: `answers[]` holds the wire-level choice decode, where `bare: "none"` and `kind: "choice"` are right. `conformance/README.md` gains one sentence saying so.
- A question given as JSON text that breaks a rule is `usage`. The same question loaded from a named file is `local`. `specification/question-file.md` gains the JSON-text sentence, since the specification is the contract. Branch cases 08 and 10 form the pair. Cases 09 and 22 are verb mismatches, which are `usage` either way, and add nothing to the split.

## Left to surface tickets

Branch case 18, case 81 (SQL NULL), and the `jobs` field on 05, 19, and 80 are binding tests. The branch skip table is not ported. Each surface runner reports a skipped case as not run (R5-32).

## Acceptance

- The pure-core conformance test and the command runner pass on the grown file with no key and no network.
- A schema check refuses an unknown success kind, a `captured` case without a stable recording path, and a header or credential in an exchange. Each refusal has a planted bad case that turns its test red.
- Test and runner changes stay under 650 nonblank Rust lines. `git diff --check` passes.

## Shared files

`conformance/README.md` with 0090, `cli/conformance_tests/runner.rs` with 0076, and `specification/` with 0082. Whichever lands second rebases.

## Dependencies

After 0090. 0088 landed on main at `71841025`, and the target-side relation issue is closed by 0088. Before 0085 and 0092.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code.

## Complexity

Contract 2; state and timing 0; reach 2; proof 2; cost of error 2; total 8. Final level: 2. Wrong expected values would mislead every surface.

## Review

- Design review: the 2026-09-24 review (`sdlc/records/2026-09-24-spine-review-controls.md`) found six mechanical gaps and one needless edit to case 19. All applied. The re-review (`sdlc/records/2026-09-24-rereview-near.md`) accepted it with one edit, applied.
- Code review: `sdlc/records/0091-code-review.md` found two blocking problems at `df303345`. Both are fixed at `906c39f2`. A re-review is pending.
