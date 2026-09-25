# What must land before the bindings freeze

Status: Closed on 2026-09-25 after a check against main. The contract (0084) and public API (0086) landed; the bindings landed at eb3fae21. Earlier status: Open

Written 2026-09-21 by the product side for Ian's plan with the build team. Nine surfaces will copy the core's shapes. A shape changed after that is nine changes and a broken caller. This page lists only the changes of that kind, each with its source. Everything else in `sdlc/issues/` can land later without breaking a caller.

| # | The change | Why it cannot wait | Source |
| --- | --- | --- | --- |
| 1 | `meta.request`, the request digest, on every function's result | One field now, nine changes later | `2026-09-21-what-a-procedure-runtime-asks-of-a-judgment.md`, item 1 |
| 2 | A marker for a question that failed inside a request that otherwise succeeded. Never `null` | `null` means "not sure". Callers will branch on it, and then it cannot be reclaimed. A caller measured about 1 live reply in 15 refused. `recognize` and `relate` send many questions per request | The same page, item 5 and the reply |
| 3 | A local size check before a request leaves, from a backend profile that states the limits | A caller measured stage outputs from 40 bytes to 1.4 MB. The top of that range is about five times the vendor's whole-request limit. Today that request is sent, refused at status 400, and the reason is hidden. The check decides an exit code, 2 in place of 4, and exit codes are shape | `2026-09-21-a-refused-request-hides-the-backends-reason.md`, `2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md` |
| 4 | The record comes back with its answer | It changes what five functions print and return | `2026-09-21-two-function-flows-lose-the-record-between-stages.md` |
| 5 | The cache on by default in the XDG folders with the 100 MB limit | It changes the default behavior of every surface | ADR 0017, Ian's rulings |
| 6 | One name for the number on a relation | Two functions and nine surfaces print it | `sdlc/planning/relate-design.md` |
| 7 | `recognize` and `relate` in the core, with results of no fixed size | The C door is designed once. Both need a returned string and one free function | `sdlc/planning/recognize-design.md`, `sdlc/planning/relate-design.md` |
| 8 | A threshold does not carry between backends: a per-backend threshold or a loud warning | It decides whether a question file names its backend, and the question file is shared by all nine | `2026-09-21-a-second-backend-tried-through-the-systemone-adapter.md` |

## Three rulings that are free to take now, for a build that can wait

From the spend ledger discussion, agreed by the caller's team and the product side: a ledger row holds counts and never a text. The ceiling stays off unless the user sets one. The count lives in the engine layer, so a library and a database extension inherit it.

## What can wait

The ledger itself, a cut inside a `score` question file, the request count under `--dry-run` (additive), the status command, `find --in`, and the reliability findings from the quality experiment. The reliability findings matter before databases run the engine at width, and none of them changes a shape.

## What Ian can overturn

All of it.
