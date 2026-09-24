---
flow: build
priority: 91
opens: conformance crates/thinkthen/tests crates/thinkthen/tests/fixtures/recognize-225 specification/find.md sdlc/planning
---

# 0091: Merge the branch conformance cases

Status: draft, design not reviewed. Owner: Claude.

## Outcome and authority

Fold the portable cases from the `surfaces` branch's `conformance/conformance.json` into main's `conformance/cases.json`, so 0085 and every surface run one file. Queue item 6 of `sdlc/planning/one-line-plan-2026-09-24.md`. The case-by-case map is section 4 of `sdlc/planning/surfaces-port-guide.md` (branch `planning/surfaces-port-guide`). The ADR 0017 shared-case amendment governs format and provenance. Ian can overturn any word ruling below.

## Work

- Re-encode per port guide section 4.1. Add the success kinds main lacks for `decide_many`, `recognize`, and `relate`, and nothing more.
- Add branch cases 06, 19, 80, 23, 76, 79, 82, 83, 84, and 68. Case 68 keeps scalar-value offsets past an accent and an emoji. The TypeScript ticket converts them to UTF-16 units (Q13).
- Add one-question `annotate` cases for `choose`, `score`, and `tag` whose values equal their scalar cases, the ruled bulk form of ticket 0095 (G4).
- Refresh main's recognize fixture digests. Map `MISC` to `other` for C05, C15, and C22. C06, C11, C13, C14, and C16 stay known divergences in tests.
- Relate cases 69–71 and the nine recognize cases with relations enter as `synthetic_contract` exchanges built from main's planner request bytes. No branch probability or digest carries over. A captured re-record needs a live run under Ian's authorization and is deferred past 0.1; the shaped exchanges prove the shapes.
- Case 17 asserts counter differences around a call, since counters have no reset (Q11). Cases 18 and 27 become main's `cancel_token` and `expired_deadline` injections.

## Rulings this ticket applies (Q16)

- The cases say `unsure`, per ADR 0017 section 6 item 4. The specification grammar keeps "unresolved".
- `find` with no selection has value `null`, as `specification/find.md` says. `none` stays the answer's pick word. Fix main's case 19 `bare: "none"` and the case that says kind `choice` where `result.md` says `find`.
- A question given as JSON text that breaks a rule is `usage`. The same question loaded from a named file is `local`, as `question-file.md` rules. Cases 08, 09, 10, and 22 test JSON text as `usage`, and one new case tests `load` as `local`.

## Left to surface tickets

Branch cases 17 (reset form), 81 (SQL NULL), and the `jobs` field on 05, 19, and 80 are binding tests. The branch skip table is not ported. Each surface runner reports a skipped case as not run (R5-32). Each surface ticket adds a mid-batch cancel arm for case 18 or proves it in its own tests.

## Acceptance

- The existing pure-core conformance test and the command runner pass on the grown file with no key and no network.
- A schema check refuses a case with an unknown success kind, a `captured` case without a stable recording path, or a header or credential in an exchange.
- `git diff --check` passes. Test changes stay under 400 nonblank Rust lines.

## Dependencies

After 0088 lands the public relate shape and `sdlc/issues/2026-09-24-a-target-side-choice-asks-the-reversed-relation.md` closes. Before 0085.

## Complexity

Contract 2; state and timing 0; reach 2; proof 2; cost of error 2; total 8. Final level: 2. Wrong expected values would mislead every surface.

## Review

- Design review: pending.
- Code review: pending.
