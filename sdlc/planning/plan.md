# Plan

Updated 2026-09-18. The reasons behind each slice are in `design-study.md`, section 6. This file holds the order and the state.

| Slice | Delivers | State |
| --- | --- | --- |
| 0 | The workspace, both crates, the lint tables, the ratchet, the ladder, `thinkn --version`, one executable spec | In progress |
| 1 | `thinkn decide if` over text, the result shape, the pass mark, `--status`, exit codes, `--plan` | Waits on nothing. Every option in the study includes it |
| 2 | `thinkn decide which` with inline options and `--from` | Waits on question 6 for the file form of `--from` |
| 3 | `--record` and `--replay` over one content-addressed directory | Ready after slice 1 |
| 4 | JSONL and lines framing, `--on`, `--id`, `where`, bounded parallel requests, request cap, rate limit, token ledger | Ready after slice 3 |
| 5 | `thinkn decide run FILE` | Waits on question 6 |
| 6 | `thinkn eval FILE --cases FILE` | Ready after slice 5 |
| 7 | `rank` and `how` | Ready after slice 4 |
| 8 | `--as` enrichment for JSONL, then CSV and TSV | Ready after slice 5 |

## Rules for the plan

- A slice becomes one or more tickets. Ticket numbers come from the shared counter.
- Slice 1 also writes the wire-format specification and its fixtures, because the first request it sends defines them.
- Nothing parked in the study enters this table without a real use named beside it.
- The first live call to a paid backend needs Ian's authorization and a token cap. The experiment cap he granted on 2026-09-18 covered the experiment harness only.

## Levers left open

- Ian's rulings on the ten questions in `design-study.md`, section 9.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
- The public release: license, changelog, `cargo deny`, and a CI workflow. `rust-standards.md` lists them.
