# 0139: Build the batching ADR

Status: built; checks run once after the merge of `origin/main` `b2d03a6f`; ready for review. Owner: Claude.

Branch `ticket/0139-batching-adr`, in `worktrees/thinkthen-0139`. B0 writes documents only, so the coordinator had it built in its source-only worktree, outside the lanes. The ticket is `sdlc/tickets/0139-batching-adr.md`. The coordinator accepted it on 2026-09-26 after two fresh read-only reviews. Ian can overturn every decision the ticket lists.

## Result

- `sdlc/planning/adr/0048-records-batch-into-full-requests.md` records Ian's batching rulings in 13 numbered items, with the amendment table, the item-to-ticket table, what recognize R0 may rely on, and 19 points Ian can overturn.
- ADRs 0007, 0008, 0010, 0032 and 0040 keep their text. Each named line carries `(Amended by ADR 0048, below.)`, and each ADR ends with one dated amendment section.
- `specification/roadmap.md` drops the held rows for `--context FILE` and packing, and says ADR 0048 brought them in.
- `records.md`, `result.md`, `channels.md`, `question-file.md` and `backends.md` keep today's sentences. Each new rule stands beside its sentence as `Not built yet, by ADR 0048 item N: …`. Their status lines name ADR 0048.
- No code, test, fixture, schema, help text or ratchet changed.

## Checks

| Check | Result |
| --- | --- |
| `sdlc/scripts/lint` | exit 0 |
| `sdlc/scripts/tickets` | 0 evidence failures |
| `contract_pages_name_the_tuned_for_key_and_never_the_old_one` | 1 passed |
| `grep -rn "Not built yet, by ADR 0048" specification` | 18 lines, one for each marked passage in the ticket's amendment table |
| The `spec/` grep for seven amended sentences | no match, exit 1 |
| Files changed | only files the ticket's `opens` names. The schema is unchanged |

## Plants

Each ran once, turned its guard red, and was restored. Restored files were touched, and a final run passed.

| Plant | Result |
| --- | --- |
| (a) A second `{"tuned_for":NAME,"running":NAME}` in the `batch_warning` row | contract test RED, exit 101 at `profile.rs:503` |
| (b) The row's key written as `"calibrated"` | contract test RED, exit 101 at `profile.rs:503` |
| (c) The `- Defers:` item dropped from the ticket | `tickets` RED, exit 1: "the Evidence section lacks a `- Defers:` item with text" |

## Budgets

- ADR 0048: 82 nonblank lines of 170.
- Amended ADRs: 3, 4, 3, 3 and 4 added lines, 17 of 50.
- Specification pages: 24 added lines of 45.
- No stop rule was crossed.
