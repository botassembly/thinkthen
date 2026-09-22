---
flow: build
priority: 35
opens: crates/thinkthen/src/cli/args.rs crates/thinkthen/src/cli/args/command.rs crates/thinkthen/tests/tag_edge.rs crates/thinkthen/tests/version.rs spec/version.md sdlc/ratchet.json sdlc/issues sdlc/planning
---

# 0066: Align the help introduction and example layout

Status: landed

## Outcome and authority

Root help introduces the tool with the approved semantic-commands wording. Tag and annotate help lead with their descriptions and put their existing examples after usage and options. This closes items 2 and 5 of `../issues/2026-09-22-command-wording-and-help-fixes-for-0-1.md`, not the entire issue. It serves the reliable command-help gap. Ian explicitly authorized creating bounded tickets within the reviewed engine plan on resumption. The queue's execution amendment permits this non-overlapping correction while 0065 keeps its owner.

## Current facts and scope

The coordinator ran root `--help`, `tag -h`, and `annotate -h` at `71c9bb8`. Root opens with `Put a decider model in the shell`; both verbs print two example lines before their descriptions. `cli/args.rs` holds the root doc-string, but its bare Clap `about` overrides that string with the package description. Remove that override so the requested doc-string supplies the help without changing package metadata. `cli/args/command.rs` supplies both `before_help` attributes. Update the two existing `spec/version.md` first-line assertions, including no-argument stderr help, to match the same new introduction. Preserve the existing no-argument exit/channel behavior. Use existing Clap help facilities, with no new formatter or dependency.

The root first line becomes exactly `Semantic commands for the shell: if, grep, and sort that understand meaning`. Both existing examples per affected verb remain unchanged and in their current order, beneath an `Examples:` heading after usage/options in short and long help. Preserve each verb's description, options, record-exit advice, and annotate's partial-failure advice. `specification/channels.md` sections Arguments and Exit code retain their existing meaning. This ticket requests only the root help doc-string edit; preserve all other comments.

Excluded: per-verb first-line wording, command-list reordering, outcome vocabulary, package descriptions, README/site edits, diagnostics, parser or runtime behavior, response-body recording, engine/core code, 0065, surfaces, dependencies, paid calls, publication, and any other item of the forty-item issue. The researcher flagged `--field` help, but the coordinator found the existing conditional document-mode wording already correct; it is not work for this ticket.

## Acceptance

- Compiled root `-h` and `--help` each begin with the exact new first line, exit 0, and leave stderr empty.
- Compiled `tag` and `annotate`, with both `-h` and `--help`, begin with their existing description; usage precedes `Examples:`, and each exact existing example appears once, in order, after that heading. The tests must fail on the current `before_help` layout, not merely find an example anywhere.
- Keep tag's described-label and positional-label checks, both verbs' visible options, all record-exit checks, and annotate's exit-6 advice. Do not pin unrelated whole-help bytes.
- Run focused `cargo test --locked --package thinkthen --test version --test tag_edge --test decide_edge` red then green. The coordinator runs the full four-rung ladder and `git diff --check` after independent code acceptance. The reviewer explicitly checks the public help and any exact ratchet adjustment; no policy is weakened.
- Mark only items 2 and 5 addressed in the owning issue and record the printed-output change there for marketing. Leave the remaining issue open. Record actual review/test results and update the queue without touching another team's worktree.

## Complexity

Contract 1; State/timing 0; Reach 1; Proof 1; Cost of error 0; Total 3. Minimum floor: none. Final level: 2. Reasons: an explicit public-help correction across three entry points, proved by compiled short/long help tests, with no runtime state change. Selected implementation: `swe2-implementer` (`swe-2-high`), Ian's approved bounded-worker experiment; independent design and code review use separate `sol-reviewer` sessions. Re-score and stop if runtime or unsettled vocabulary changes become necessary.

## Review

Independent Sol design review: ACCEPT, including re-review of the Clap override and executable-spec corrections. A fresh Sol code reviewer accepted the actual diff, public help, retained safety advice, and exact +74-line ratchet adjustment. SWE-2 observed the new layout and introduction tests fail before the fix and pass afterward. The coordinator ran all four gates and `git diff --check`: 547 Rust tests, doctests, replay checks, and nineteen green how-tos passed. Hosted gate run `35742562545` passed on `42039dd`, which was then fast-forwarded to main and pushed. `../records/0066-align-help-introduction-and-layout.md` records the evidence.
