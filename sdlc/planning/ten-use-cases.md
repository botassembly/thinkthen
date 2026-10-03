# Ten use cases, and what each asks of the tool

Written 2026-09-19. Ian brought a list of ten uses of the decider model that was going around, and he asked whether any of them would blindside the design. A second agent read the vendor's pages, every command page, and the wire code, and wrote one example for each. The steering agent judged the ideas that came out. Ian can overturn any verdict here.

## Two facts from the code

- Record mode writes and flushes one line per record and reads its input lazily. A Bash `coproc` can therefore keep one process alive and ask it one question per line.
- The binary built a new connection for every request. Every record paid a new handshake. Ticket 0013 now shares one connection pool across the run.

## The ten, with the command that fits

| # | Use case | Fit | Command | The honest limit |
| --- | --- | --- | --- | --- |
| 1 | Real-time loops, ten decisions a second | None | | Each decision waits on a network round trip to a model, and a shell tool adds a process start to each one. This use belongs to a library |
| 2 | Picking the next action in a multi-step run | Good when the agent is a shell script | `choose`, with `--options POINTER` when the candidates change per step | Most agents are written in Python or TypeScript, and they want a library call |
| 3 | Gating a tool call as safe or unsafe | Strong | `decide` with a band, and the exit code is the gate | The same limit as 2 when the caller is a program |
| 4 | Routing a request to a cheap or a strong model | Good | `choose easy hard`, or `score` on difficulty | The vendor routes on `confidence`, and the tool cuts on the winning probability. ADR 0014 item 2 holds that question open |
| 5 | Goal and stuck checks in a loop | Strong | `tail -n 20 run.log \| thinkthen decide ...` | None. `tail` is the context window |
| 6 | Context compaction by keep or delete | Strong | `filter --jsonl --field /text` | Needs `--jobs` and shared connections to be quick |
| 7 | Picking a skill or a tool from a long list | Good | `choose`, or `find` over a list of skills | The vendor's recipe chains two calls. Here that is two commands |
| 8 | Guardrails on outputs and traces | Strong | `annotate` with a question set of hazards and one severity `score`, in one request | Per-turn use inside a program wants a library |
| 9 | Ticket and email triage at volume | Home ground | `annotate` and a policy transform | Needs `--jobs` and shared connections |
| 10 | Reranking retrieved passages | Home ground | `rank` with the query inside the record, and `--top N` | One request per passage is the vendor's own method. It needs `--jobs` |

## The three that could blindside the design

1. **Real-time loops.** The tool's unit is a run over a file. The README says plainly that this use is out of reach.
2. **Use inside an agent written in another language.** Records, recordings, transforms, and exit codes buy nothing there. The real ask is a library over the same pure core. That is a second product, and it goes on the roadmap for after version one.
3. **Volume.** Triage and reranking are the home ground, and the binary ran one record at a time with a new connection each. Ticket 0013 closes this gap, and nothing about volume is claimed before it lands.

## Verdicts on the ideas

| Idea | Verdict | Reason |
| --- | --- | --- |
| Share one connection pool across records | Build, in ticket 0013 | It adds no surface, and `--jobs` means little without it |
| `--jobs N` | Build, in ticket 0013 | Already specified |
| Honor `Retry-After` on a 429 | Build | The vendor asks for it, and parallel requests make a 429 likelier |
| A `serve` command or a daemon | Decline | A process that waits for work is a service. Record mode through a `coproc` already serves a loop. A how-to shows it. Ian ruled on 2026-10-02 that the service is a second program in this repository, the proxy, which serves the functions over HTTP and MCP; the command still has no `serve` (`../issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md`) |
| Several questions in one call with no file | Document only | `annotate` with a question set is the home for several questions, and `thinkthen annotate <(jq -n '...')` needs no file on disk |
| Many yes/no questions in, a list of tags out | Document only | A question set and a small `tags` transform do it |
| More than 255 options | Document only | A Bash loop of `choose` walks a tree of groups. The vendor also reports weaker picks above about 240 options, and `choose.md` says so |
| A `--context` option for recent history | Decline | `tail`, `jq`, and several `--field` pointers already build the evidence |
| The time a call took, inside `--details` | Decline | A clock value in a row breaks byte-for-byte replay. `time` answers the question |
| A written rule for a gate when the service is down | Document | `channels.md` says it in one sentence: word the question so that yes permits the action, and treat every exit code other than 0 as a refusal |
| A cut on `confidence` | Open | ADR 0014 item 2 waits for an eval with more errors |
| `find --top N` | Measure first | The test on long documents comes first |
| Descriptions for options on the command line | Build | Ian ruled that every structural option has a home on the command line. The interface audit gives the spelling |
| A library over the core | After version one | It is the honest answer to use cases 1, 2, 7, and 8 inside a program |

## New how-tos

Five of the strongest fits had no page. `documentation-plan.md` gains them: gate a tool call and fail closed, screen a turn for several hazards in one request, compact a transcript by keep or delete, route a request to a cheap or a strong model, and serve a loop from one long-lived process.
