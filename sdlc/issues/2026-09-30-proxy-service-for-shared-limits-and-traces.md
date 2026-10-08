# The proxy: a second program that serves the functions

Status: open. Filed 2026-09-30 on Ian's request. Rewritten 2026-10-03 from Ian's product decisions of 2026-10-02, sent in the docs message "Proxy, terms and function drafts decided" and its addenda. Not a ticket. Owner: the queue owner.
Kind: idea
When: after the 0.1 release, once the decision store experiment reports
Milestone: 0.3

## Ian's decisions of 2026-10-02

1. The proxy is part of ThinkThen. It is a second program in this repository, with the same name, version, site and license (MIT). It shares the engine. One repository answers Ian's worries about code overlap, brand dilution, versioning and coordination across repositories.
2. It serves the functions over HTTP and MCP, never the bare model endpoint.
3. It also speaks the model's wire, so every SDK and every ThinkThen surface reaches it as one more backend. The client names a backend, a model or a group by name, and the proxy holds the vendor keys.
4. It keeps every decision in a real database, and nothing is evicted.
5. It supports every batch size, from 1 up to the model's context window. Batching adds noise, and users measure it with their own evaluations.
6. It knows and tracks each provider's and model's context window.
7. Retries move into the proxy. It records retries, failures and latency by backend and model.

The command keeps no `serve` mode. Record mode through a `coproc` still serves a loop from one process.

## What it solves

- Rate limits across processes and machines. Today the requests-a-minute pacer covers one process, so separate command runs, bench cases and PostgreSQL connections add their rates together. The proxy sees every request and can pace them all.
- One place to export traces, where the OpenTelemetry idea (`2026-09-30-opentelemetry-traces-after-0-1.md`) fits best.
- One place to hold the keys, so clients need none.
- An MCP face, so an agent calls the functions directly.

## Limits a design keeps

- It is optional. The command and libraries keep working with no proxy.
- Key secrecy, the recording rule and "sends only to the address the user names" apply to the proxy as well.
- `thinkthen` judges and never acts. The proxy serves judgments and runs no commands.

## Before a ticket

- The experiments repository's experiment 0014 prototypes the decision store in SQLite and writes a risk register. It lists the fields the store needs that ThinkThen's results do not record today.
- `specification/roadmap.md` declines "a `serve` command or a daemon" and says "A service is a different program". That row gains a pointer to this issue in the first change outside `sdlc/`. `sdlc/planning/ten-use-cases.md` and `sdlc/planning/libraries/command.md` already point here.
- The glossary issue `2026-10-03-glossary-call-request-decision.md` defines call, request and decision, which the proxy's records count.

A lighter step for one machine stays possible: a lock file in the cache folder that every process on the machine shares for pacing. Ticket 0308 deferred that limit across processes.
