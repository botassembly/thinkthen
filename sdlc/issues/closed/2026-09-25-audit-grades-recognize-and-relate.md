# audit grades recognize and relate

Status: Closed 2026-09-26 by ticket 0135. Part of Ian's 0.1 ruling in `2026-09-25-audit-is-complete-for-0-1.md`, section 1. Split out of ticket 0125 on the design review of 2026-09-25. It needs its own 0.1 ticket.

## What is missing

Ticket 0125 teaches audit `tag`, `score`, `find`, and `rank`, plus `--optimize`, `steady`, and `--write`. It refuses a `recognize` or `relate` answer with `thinkthen: audit: results line N holds an answer audit cannot grade; ...`. Ian's ruling asks audit to grade every function type in 0.1. These two remain.

| Function | What a key holds | What audit grades | Bar it tunes |
| --- | --- | --- | --- |
| `recognize` | the names, with their kinds and places | precision, recall, and F1 over names | the strength cut, `threshold` in a recognize file |
| `relate` | the edges | precision, recall, and F1 over edges | the probability cut, `threshold` in a relate file |

Neither has true "no" answers, so accuracy has no meaning. The ticket decides what `--optimize accuracy`, the default, does for them.

## Open design points

1. **How names match.** Exact text and kind, with places ignored and each name matched at most once. Or exact places. Or overlapping places. Workspace experiment 214 graded names by exact boundary and kind, and it counted 93 partial-overlap predictions and 37 split or truncated names as errors. Text-only matching hides a boundary error that repeats elsewhere in the text. The 0125 design review raised this as its finding 7.
2. **Rows cut at the run's cut.** A saved `recognize` row holds only names whose `strength` reached the run's cut, and a `relate` row holds only edges at or above its cut. audit cannot see a candidate below that cut. Either candidate cuts start at the run's cut, and the page tells users to run with a low cut. Or audit rebuilds candidates from `answer.tokens` (recognize) and `answer.questions` (relate), which reproduces the tokenizer and the planner's acceptance rule outside the command.
3. **Relation edges.** A `relate` edge matches on relation name and both endpoints. An `either` relation matches in both orders. `recognize` also emits beta relation edges under `relation_threshold`. Decide whether audit grades them and whether `--write` ever touches `relation_threshold`.
4. **Record ids.** `relate --details` prints no `input`, so a line needs an id. 0125 gives a `find` line its line number. `relate` could do the same.
5. **The key.** A list of `{name, kind}` for `recognize`, with `start` and `end` if places count. A list of `{relation, source, target}` with `{name, kind}` endpoints for `relate`.
6. **Output.** 0125 maps yes/no counts onto rows. Matches, extra items, and missed items could fill `true_yes`, `false_yes`, and `false_no`, with `true_no`, agreement, and the interval null.

## Done when

`thinkthen audit` grades saved `recognize --details` and `relate --details` rows against a key, reports precision, recall, and F1, suggests a cut under `--optimize`, shows `steady`, and writes `threshold` through `--write` under 0125's digest and ReAnchor rules. A test replays the committed recordings of demos 44 and 45 and grades them.
