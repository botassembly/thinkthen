---
flow: build
priority: 35
opens: crates/thinkthen/src/cli/args/command.rs crates/thinkthen/tests/version.rs crates/thinkthen/tests/tag_edge.rs crates/thinkthen/tests/decide_edge.rs spec/decide.md sdlc/ratchet.json sdlc/issues sdlc/planning
---

# 0067: Teach command operations in help

Status: landed

## Outcome and authority

The eight judgment commands begin with the approved description of their text operation, and root help lists them in the teaching order. This closes items 3 and 4 of `../issues/closed/2026-09-22-command-wording-and-help-fixes-for-0-1.md`. Ian authorized creating bounded reviewed tickets within the engine plan. Ticket 0066 is landed; this correction is independent of 0065 and the library team's work. It serves the command-help usability gap without changing answers or options.

## Current facts and decisions

The compiled help checked during 0066 still begins several descriptions with printing or exit-code mechanics. The current declaration order is decide, choose, tag, score, filter, rank, find, annotate. The owning source issue `2026-09-21-the-help-first-lines-and-the-public-words.md` gives these replacement first sentences and order:

| Command | First sentence |
| --- | --- |
| decide | Answer one yes or no question about a text. |
| filter | Keep the records where the answer is yes. |
| rank | Sort records by how likely the answer is yes. |
| choose | Pick one option from your list. |
| find | Pick the one line or record that best answers a question. |
| score | Place a text on a scale you name. |
| tag | Name every label that fits. |
| annotate | Fill out a question set for every record. |

Keep status before these eight and cache after them. Reorder the existing enum variants, including their help blocks, rather than adding a separate command-order registry. Parsing and command behavior remain unchanged.

Safety advice outranks the source issue's cosmetic suggestion to move cautions out of short help. Keep decide's existing record-exit sentences in its first paragraph. Keep find's exact `Every unit leaves together and sees every other unit` disclosure in its first paragraph and its existing bounds/none guidance in long help. Keep rank's per-record, local-sort, no-pairwise-comparison caution in the first paragraph and its `most likely yes first`, tie, no-tournament, and all-input guidance in long help. Preserve every verb's long-help record-exit advice and annotate's exit-6 advice. Move displaced result-shape statements to the long description where necessary, rather than deleting the tag JSON-array or annotate JSON-object information. Keep 0066's examples unchanged and after options. Ian can overturn the cosmetic wording/order; this ticket changes no settled exit or disclosure contract.

## Scope and acceptance

- Edit only the eight help introductions, necessary retention of displaced help details, and enum declaration order in `cli/args/command.rs`. Help doc-comment edits are requested; add no unrelated comments.
- One table-driven compiled test checks the exact opening sentences under `-h` and `--help` (allow only Clap's existing final-period removal), and root help's eight command rows appear exactly once in the stated order. Preserve status/cache and normal help/version availability. The new test must fail against the current binary before implementation.
- Update existing decide short-help, tag/annotate description, and executable `spec/decide.md` expectations to the same approved text. Retain existing option, record-exit, find disclosure/bounds, rank-order, and annotate partial-failure tests unchanged except their old introduction text. Do not snapshot whole help screens.
- Focused checks: `cargo test --locked --package thinkthen --test version --test tag_edge --test decide_edge --test find_edge --test choose_and_score_edge`, and `mustmatch test spec/decide.md` with the built binary on PATH. Coordinator runs the full ladder and diff checks after independent code acceptance.
- Mark only items 3 and 4 addressed, keep the forty-item issue open, and record the output change for marketing. Update the measured ratchet only with justification and explicit independent review.

Excluded: other vocabulary or diagnostic rewrites, outcome naming, other forty-list items, package/site/library changes, parser/runtime/engine/core behavior, result or recording shapes, new dependencies, paid calls, publication, and any change to 0065 or surfaces. Stop if preserving existing safety advice requires a new public behavior decision.

## Complexity

Contract 1; State/timing 0; Reach 1; Proof 1; Cost of error 1; Total 4. Minimum floor: none. Final level: 2. Reasons: multiple public help entries with safety sentences to preserve, but no execution-state change. Selected implementation: `swe2-implementer` (`swe-2-high`), Ian's bounded-worker experiment; independent design and code review use separate `sol-reviewer` sessions.

## Review

Independent Sol design and fresh code reviews: ACCEPT. They confirmed the explicit summaries/order, retained safety and result-shape advice, bounded scope, level-2 routing, and exact +69-line ratchet. SWE-2 observed sixteen old-introduction failures and the old root-order failure before the fix; forty-seven focused tests then passed. The coordinator ran all four gates and `git diff --check`: 548 Rust tests, doctests, replay checks, and nineteen how-tos passed. Hosted gate run `35745327707` passed on `be078b7`, which was fast-forwarded to main and pushed after Ian resolved a full filesystem. The matching record holds the evidence.
