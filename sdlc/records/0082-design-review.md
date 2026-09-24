REJECT

Reviewer: fresh read-only Claude (Opus) session, 2026-09-24. Checked: ticket at origin/ticket/0082-close-command-contract (42b96a94), origin/main at 989d5b0c (0080, 0081, and 0088 records all on main; 0088 branch is an ancestor of main), 0089 ticket at origin/ticket/0089-no-transport-resend, the merged 45-item issue, and main's `cli/args/command.rs`, `args.rs`, `args/relate.rs`, `cli/profile.rs`, `core/result/profile_warning.rs`, `specification/relate.md`, `specification/result.md`.

## 1. Correctness against current main

Still live on main (the ticket is right to fix these): row 1 (`decide` help says "3 for unresolved"), row 12 for `recognize` (no record-run sentence), row 43 (`score` help: "Measurement of the first decider model"; 12 hits across crates/spec/demos/README), row 44 (`calibrated` in `profile_warning.rs` and `result.md`), row 45 (`annotate` says "Fill out a question set for every record."). Row 4 order already holds on main (status, ten functions, cache, help); only the pin is missing.

Findings:

F1. Row 3 contradicts landed 0088 wording. Main says `recognize`: "Find every name in a text and assign one of the given kinds." and `relate`: "Find named relations across one complete entity set." Both passed the 0088 Claude and Codex reviews. The ticket pins different sentences ("Find every named entity in a text.", "Find ruled relationships among named, kinded records.") that appear nowhere else. Smallest change: mark row 3 "already fixed by 0080/0088" and pin main's two sentences. If the author wants new sentences, say why and add a marketing output-change note.

F2. Row 12 for `relate` pins "its whole-set exit wording from ticket 0081". No such help text exists. `relate` help carries no exit sentence at all. The contract lives only in `specification/relate.md:62` and came from 0088, not 0081. Smallest change: give the exact help sentence to add, drawn from relate.md:62 (for example, "A run with some failed relation questions prints the complete result and exits 6; with none answered it exits 4."), and add it to the allowed diff for row 12.

F3. The stop rule contradicts row 44. Budgets say "Stop and re-score if the audit ... changes a result key or shape". Row 44 changes a result key. Smallest change: exempt row 44 by name, or split it out (see section 3).

F4. Row 44 leaves the stderr warning open. `cli/profile.rs:70` prints "threshold calibrated for profile X is running under profile Y". The ruling's reasoning (calibrated overclaims) applies to it too. The ticket must say whether it becomes "threshold tuned for profile X ..." (then profile.rs and its exact test at line 130 join the file list) or stays (then the vocabulary checker needs a sanction for it). Also delete the dead "If Ian keeps `calibrated`" branch from Scope.

F5. Row 44 closure scope conflicts with the issue. Issue item 44 says it closes "when the rename lands in the tool, the specification, all nine surfaces, and the site copy together". Under the 2026-09-24 plan the surfaces port later (queue item 10). Smallest change: say 0082 closes the tool and specification half; each surface ticket inherits `tuned_for`; the record names the site-copy owner.

F6. Row 44 has no exact acceptance line. Add: a red-first binary test that pins `meta.profile_warning` as exactly `{"tuned_for":"old","running":"new"}` and a check that finds no `"calibrated"` key in emitted results, `result.md`, or examples.

F7. Frontmatter `opens` omits `crates/thinkthen/src/core` (row 44 edits `core/result/profile_warning.rs`, and the `core/question_set.rs` and `question_file/resolve.rs` doc comments say "calibrated under").

F8. Dependencies and baseline are stale. The text names 0080 and 0081 as the relate source and SHAs from 2026-09-21. `relate`'s public command came from 0088. Queue item 2 (0089) lands first and rewrites `--max-retries` long help in `cli/args.rs` and `cli/args/find.rs`. Smallest change: the baseline is main containing the 0088 and 0089 landing records. Without that, the "21 captures, byte diff empty except authorized rows" check fails on 0089's lines.

F9. The owner line is stale. "Luna Extra High ... Sol High ... three-ticket trial" predates the one-line plan. Name Claude as the owner and give the review route the plan uses.

## 2. Acceptance

The red-first rule and exact pins are good for rows 1, 3, 4, 12, 43, and 45, and for the vocabulary self-test. Gaps: F2 (no sentence to pin for relate) and F6 (no pin for row 44).

## 3. Scope and budget

Four production Rust files is tight. The help work touches `command.rs`, `args.rs`, and `args/find.rs`. Row 44 adds `core/result/profile_warning.rs`, and `cli/profile.rs` makes five if F4 changes the warning. Recommendation: split row 44 into its own small ticket. Its write set is `core/result/profile_warning.rs`, `cli/profile.rs`, `specification/result.md`, and the `tests/backend/profile*` tests. It has its own public-surface review, it resolves F3, and it leaves 0082 as pure wording. If it stays in, raise the production file cap to five.

## 4. Order and shared files

0082 and 0083 cannot run in parallel. Shared files: `crates/thinkthen/src/cli/args/command.rs` (0082 rewrites help; 0083 adds the `Transform` variant), the root-help inventory test (0082 pins the order ending `cache, help`; 0083 inserts `transform`), `sdlc/ratchet.json` (the ceiling equals the measured total, so either landing invalidates the other), and `sdlc/scripts/lint` (the vocabulary check and the transform byte check). Keep the queue order: 0089, 0082, 0083. 0082 also shares `cli/args.rs`, `cli/args/find.rs`, `specification/result.md`, and the ratchet with 0089.
