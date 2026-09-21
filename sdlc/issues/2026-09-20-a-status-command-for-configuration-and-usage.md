# A status command for configuration and usage

Status: Open

Asked by Ian on 2026-09-20. He ran the repository's own spend wrapper and liked what it printed:

```
$ sdlc/scripts/live --status
authority_id 5c95a2e0-...
status active
limit_tokens 476000000
charged_tokens 429118
pending none
```

His words: it would be cool to have some configuration, status, and usage statistics available from the command-line tool. The tool depends on the service provider for some of it. Where the provider has it, the tool should expose it.

## What the provider offers today

Checked against the vendor documentation snapshot on 2026-09-20. Every response carries `usage` with `input_tokens` and `output_tokens`. The documentation states rate limits in tokens per second and requests per minute, and a request over either limit returns 429. The snapshot shows no endpoint for account usage, quota, or remaining balance. A model list exists as a documentation page. The roadmap already holds a `models` listing.

So the provider can tell the tool what one request used. Everything across requests is the tool's own bookkeeping today.

## What a status command could print

1. **The resolved configuration.** The address and where it came from (`--url`, `THINKTHEN_BASE_URL`, or the default). The model and where it came from. Whether `THINKTHEN_API_KEY` is set. The key itself is never printed, hashed, or shortened.
2. **Local usage.** Requests, input tokens, output tokens, and cost at a stated price, for today and in total. The numbers come from the `usage` object on each response.
3. **The limit and what is left,** when the user set one.
4. **Provider usage,** only when the backend offers an endpoint for it. The line is absent otherwise. The command never guesses.
5. **`--json`** for scripts.

## What has to be decided

- **Where local usage lives.** A recording already holds every exchange, so one run's usage is a `jq` sum today, and the `cost` transform does it. Usage across runs needs a ledger file outside any one recording. The roadmap holds "history across runs" and "a request cap" because both need a store and a cross-process budget. `sdlc/scripts/live` already implements exactly that for this repository's own development. The question is whether that mechanism moves into the product.
- **Whether a cap belongs in the same change.** The ideal state says a ledger counts requests, tokens, and cost, and caps stop a runaway stream before it spends real money. A status line that shows a limit implies something enforces it.
- **The name.** `status` reads well and is not a judgment verb. The flat-verb ruling covers the verbs that judge. A reporting command sits beside `help`.
- **Whether it opens a connection.** Printing configuration and the local ledger needs no network. A reachability check costs a request. `--dry-run` already proves a request can be built without sending it.

## Constraints

- No credential or value derived from one appears in the output, the ledger, or an error.
- The command changes nothing. A `config set` stays held.
- The ledger format is plain rows, so the existing transforms read it.

## A recommendation, 2026-09-21

Ian asked on 2026-09-21 what the status and configuration commands are, and whether a user controls a spending cap. He added that he may not care about a cap. The marketing side's answer, for the builder to weigh:

**No ledger and no budget across runs in 0.1.** It needs a store, a lock across processes, and a file the user never named. The price argues against the weight: 3,000 short records cost 3.6 US cents, and a wrong file of 100,000 lines costs a little over a dollar. The account-level control is the vendor's own. Item 1 of `2026-09-21-where-a-user-could-lose-trust-a-first-list.md` ranked this too high for the command.

**One stateless limit, as an engine setting.** `--max-requests N` refuses a run before its first request when the input holds more than `N` records, and the default is no limit. It keeps no state and writes nothing. It earns its place in the databases, where one `WHERE` over a hundred million rows is a bill of a thousand dollars. `--dry-run` prints the number of requests the run would make.

**The whole command tree when the plan is done:**

| Command | Does | Network |
| --- | --- | --- |
| The eight verbs | Judge | Yes, unless `--replay` or `--dry-run` |
| `thinkthen status` | Prints the version, the address and where it came from, the model and where it came from, whether the key is set, and the cache folder with its entry count and size when `THINKTHEN_CACHE` names one. `--json` for scripts | Never |
| `thinkthen cache prune DIR` | Removes entries by age, size, or answering model | Never |
| `thinkthen help` | Help | Never |

No `config` command exists, and nothing is set through the tool. Three variables are the whole configuration: `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `THINKTHEN_CACHE`. Ian removed the configuration file on 2026-09-19, and `specification/roadmap.md` holds the shape to return to.

**Usage comes from two places and no store.** One run's usage is the `usage` object on every `--details` row, and `transforms/cost/cost.jq` sums it. A library process reads the engine's `usage()` counters: requests, cache answers, and tokens since the process began. `status` prints no usage, because the command keeps none between runs.

