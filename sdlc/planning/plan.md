# Plan

Updated 2026-09-18, after Ian's rulings in ADR 0003. The whole command line is planned. The `decide` family comes first. Every family gets a document under `specification/` before any ticket.

## The `decide` family

| Slice | Delivers | State |
| --- | --- | --- |
| 0 | The workspace, both crates, the lint tables, the ratchet, the ladder, `thinkthen --version`, one executable spec | Done |
| 1 | `thinkthen decide if` over text: the types, the pass mark, the result, the `systemone` adapter, `--status`, exit codes, `--plan` | Done. Tickets 0001, 0002, 0003 landed. No live judgment yet, because the vendor account has no credits |
| 2 | `--record` and `--replay` over one content-addressed directory. Moved up by ADR 0005, because demos replay recordings | Done. Ticket 0004 landed. Demo 01 turns green once it is recorded live with `demos/01-refund-gate/record.sh` |
| 3 | `thinkthen decide which` with inline options and `--from` | Drafted in `specification/decide.md` |
| 4 | A configuration file with named backend profiles, and `thinkthen backend check` | Needs a specification document |
| 5 | The `chat-logprobs` adapter, proven against a local server | Needs its section in `specification/backends.md` |
| 6 | Record framing for JSONL and lines, `--on`, `--id`, `where`, bounded parallel requests with order kept, a request cap, a rate limit, a token ledger | Needs its section |
| 7 | `thinkthen decide run FILE` over a saved question file | Waits on question 6 of the study |
| 8 | `thinkthen eval FILE --cases FILE` | After slice 7 |
| 9 | `rank` and `how` | After slice 6 |
| 10 | `--as` enrichment for JSONL, then CSV and TSV | After slice 7 |
| 11 | `match` and `segment` | Needs its section |

## Demos

ADR 0005 makes demos the driver. `demos/` holds small real shell jobs as executable pages. A demo starts red, argues for design choices, and turns green when its verbs exist and its recording is made. The `spec` rung runs the green ones. The demo set is being drafted now.

## The other families

In order: the utility families `eval`, `record`, `backend`, `config`. Then `patch`, `reduce`, `fold`, `resolve`, and `derive`. `specification/README.md` gives one line for each. The order can change when Ian rules.

## Rules for the plan

- A slice becomes one or more tickets under `sdlc/tickets/`.
- Code follows the specification. A ticket cites the sections it builds.
- Opus agents build, one ticket per worktree. A second agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency.

## Live testing budget

Ian granted more budget for paid calls on 2026-09-18. The cap for this repository is 20,000,000 input tokens. Spent so far: 0. Every live call adds its tokens here. The first live calls on 2026-09-19 returned a billing error, because the vendor account has no credits, and they spent nothing.

## Levers left open

- Ian's rulings on questions 6, 7, and 10 of `design-study.md`, and on the public release.
- Ian's read of `specification/`. Sections marked Draft wait for it.
- Registration with Factory 2, and the five `sdlc/project/` scripts that come with it.
