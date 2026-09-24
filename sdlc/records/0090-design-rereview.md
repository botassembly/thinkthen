REJECT

Reviewer: fresh read-only Claude (Opus) session, 2026-09-24. Checked: ticket at ticket/0090-tuned-for (a1153d48), Ian's ruling as recorded on the 0082 branch (line 17, and commit 42b96a94), the ruling line in the merged issue (item 44, commit f14011e0) and in quality-plan.md, ADR 0032, and every `calibrat` hit on main in crates, specification, conformance, spec, demos, transforms, and site.

## Decisions checked

- The result object `{"tuned_for":NAME,"running":NAME}`, with no compatibility reader before 0.1, is sound. ADR 0032 lets Ian overturn "the metadata shape". Its own text already says the name "identifies the backend used to tune its threshold", so `tuned_for` matches the ADR's meaning.
- The profile.rs:70 wording "threshold tuned for profile X is running under profile Y" is sound. No specification page pins the old sentence. Its pins sit at profile.rs:130, tests/backend/profile.rs:333 and :441, and warnings.rs:35, :71, and :108.
- Renaming the `conformance/backend-profiles.json` key (lines 14-17) is sound. Surface tickets inherit it from main.
- A dated amendment to ADR 0032 is right. The repo CLAUDE.md prescribes amendments for accepted ADRs.
- Keeping the noun "calibration" (args.rs:93, find.rs:49, backend_profile.rs:9, question-file.md:90, result.md:38 and :107, the schema description, and the `calibration` transform) is consistent with the ruling. The ruling names the key only.
- The production budget of six files is exact: profile_warning.rs, question_set.rs, question_file/resolve.rs, cli/profile.rs, asking.rs, and conformance_tests/profile_cases.rs. It has no slack, but it holds. The test cap and the prose cap are realistic.

## Blocking finding

B1. The ticket contradicts the ruling for a file in this repository. The ruling reads "the key is `tuned_for`, everywhere in one commit". The ticket excludes the site and leaves `site/src/pages/reference.astro:76` showing `{"calibrated":NAME,"running":NAME}` "until then". That file lives in this repo on main, so the one commit can cover it. The surfaces cannot join the commit, because they are not on main. That split is unavoidable and correctly recorded. The site line needs no split. Smallest change: add `site/src/pages/reference.astro` (the line-76 field string only) to Scope and `opens`. Extend the fixed-string test or the `git grep` acceptance to cover `site/`. State that the surfaces half is the only part that lands later, because it is not on main.

## Follow-up (non-blocking)

- Acceptance line 51 names the three counts in warnings.rs. It omits the fourth count at tests/backend/profile.rs:441. Name all four.
- quality-plan.md says "a question file's threshold carries `tuned_for`". A reader could take that for the question file's `profile` key. Add one sentence saying the ruling renames only the result key, and the question-file key stays `profile`.
- The status line says "revised after design review", but no 0090 review exists. It was split from the 0082 review. Say "split from 0082's review; first review".
- `digest.rs:302` (`let calibrated`, a calibration-identity test) and the fixture name `calibrated-question.json` do not name the field. The record should list them as out of scope, as the grep acceptance already expects.
- The order after 0082 and 0083 is correct. 0089 also edits result.md and lands first.
