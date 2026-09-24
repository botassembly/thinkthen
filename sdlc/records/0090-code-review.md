REJECT

Review of ticket 0090 at b4353c23 (branch ticket/0090-tuned-for on main ccc947d5). Fresh read-only Claude session. Scratch copies built under /tmp/claude-1000/0090-review with CARGO_TARGET_DIR there, one build at a time, THINKTHEN_API_KEY and THINKTHEN_BASE_URL unset, no live or paid call. Scratch deleted.

## What was checked, by command

- Full revert of production, conformance, site, and spec files to main while keeping the new tests: 6 backend tests fail (a_profile_mismatch..., replay_warns_once..., the three warnings.rs counts, contract_pages...) and the inline cli::profile test fails. At HEAD all pass.
- Key-only revert (serde rename back to "calibrated", sentence kept new): a_profile_mismatch_warns_once_and_reaches_detailed_metadata fails on the key alone.
- Per-page revert: main's result.md, reference.astro, or backend-profiles.json each alone fails contract_pages_name_the_tuned_for_key_and_never_the_old_one. The old conformance JSON also fails shared_profile_cases... on case "different".
- Full cargo test at HEAD: library 275 passed 2 ignored, backend 352 passed, matching the record. fmt clean, clippy -D warnings clean, git diff --check clean.
- Ratchet: ratchet.mjs measures 43754 at main and 43776 at HEAD. Net nonblank Rust delta is +22, all in the new guard test. Production net 0. Earned.
- No old key is emitted: ProfileWarning is the only serializer and has only tuned_for. git grep of the whole repo finds no "calibrated" key outside the guard test's refused strings.
- Warning sentence: code (cli/profile.rs:70), all five test pins, and the ADR 0032 amendment agree on "threshold tuned for profile X is running under profile Y". Help, specification, and site do not quote the sentence, so nothing disagrees.
- Digest tests untouched and passing. question-file key stays profile.

## Finding (blocking, record only)

F1. sdlc/records/0090-build-tuned-for.md:104-110 claims to list each remaining calibrat* use, as the ticket's decision at sdlc/tickets/0090-rename-calibrated-to-tuned-for.md:22 requires. A whole-repo grep finds uses the list omits:
- specification/annotate.md:32 "the backend profile used to calibrate the set's thresholds" (verb form, same concept as the saved profile).
- site/src/data/catalog.mjs:41 and :258 "calibration profile" (site copy).
- crates/thinkthen/tests/question_file/relate.rs:53 "calibration identity".
- specification/roadmap.md:91 "limits-and-calibration file".
- specification/roadmap.md:61, demos/25-check-the-judge/README.md:60 and :100, transforms/ (real calibration, keep; name them under the transform line).
Smallest fix: add these lines to the record's lists with their reasons. No code change.

## Follow-ups (not blocking this ticket)

- FU1. sdlc/planning/surfaces-port-guide.md:267 is now stale. It says result.md "still says calibrated" and that 0082 renames it. Update when that guide is next touched, or in this record's follow-up list.
- FU2. specification/annotate.md:32 "used to calibrate" is the verb Ian's ruling rejects in spirit. Put it first in the later wording decision.
- FU3. crates/thinkthen/src/cli/conformance_tests/profile_cases.rs:39 MismatchCase has no #[serde(deny_unknown_fields)]. An old "calibrated" key reads as None and only the "different" case catches it. Adding the attribute makes each case fail loudly.
