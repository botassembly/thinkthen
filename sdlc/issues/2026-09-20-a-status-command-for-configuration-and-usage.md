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
