---
date: 2026-10-13
status: draft PRD
release: 0.5
source: Ian's rulings in [../proxy.md](../proxy.md) and [../README.md](../README.md), including 2026-10-13 on the proxy as one choke point for rollouts over the SDK's own database; PRD 00; Ian's ThinkThen notes (strategy, data-model, the later enterprise edition); ThinkThen 0.2's `specification/`. The 27 worked questions and sources are in [05-proxy-questions.md](05-proxy-questions.md)
---

# PRD 05: The proxy

The SDK already fronts any model. The proxy does the same on a server for every program at once, and adds one place to control what reaches production. It is `thinkthen proxy`: MIT, in the ThinkThen repo, sharing the engine and the database (PRD 01).

## Who and what's in it for them

The 0.3 customer is one builder running several programs: agents, scripts, a notebook, a deployed app. One script alone gains nothing, and the docs say so.

- **One rate limit for every process.** 0.2 paces each process alone, so four processes send four times the rate. Proof: four parallel runs, refusals counted direct and through the proxy.
- **Agents hold no vendor keys.** Keys live in the proxy's environment, and a spend cap bounds a looping agent. Proof: a demonstrated run with every vendor key unset in the agent's shell.
- **Change a decision once.** A threshold or model change reaches every caller with no code edit, tried on a share first, rolled back with one edit. Proof: a demonstrated rollout.
- **Other people's code gets the same.** Anything that speaks System One gains the store, the keys and the traces without adopting ThinkThen. Proof: a 0.2 SDK and one outside client running unchanged.

## Use cases

U42 one server for every caller, U43 overrides and routing, U44 other people's code, U45 retries and latency, U28 outcomes, U18 and U20 reuse and caps, U46 small models, routed and not run.

## The design

**Starting it.** `thinkthen proxy` listens on `127.0.0.1` at one fixed default port, opens the same default database as the SDK, and reads one rules file. `--listen`, `--database` and `--config` change these. `thinkthen proxy --check` validates the rules, reaches each backend and exits. The Docker image runs the same executable.

**Reaching it.** A built-in named backend, `proxy`, points at the default address with the ThinkThen type. `--backend proxy` is the whole client change, on every surface. The engine checks the proxy's version once and turns local reuse off.

**Two doors.** The function door takes one route per action, `/v1/decide` through `/v1/relate`, plus PRD 00's data jobs. Records travel in chunks, and a call keeps one policy version from its first chunk. `run` stays in the client, which tags each function step with the tool's id, hash and step. The System One door, `/v1/systemone`, is where a 0.2 SDK already posts. It derives the question id from contract and wording, and treats the `model` field as a route name when one matches. It never overrides a threshold, because its callers read probabilities themselves.

**Rules.** One file, watched and reloaded; a broken file is refused and the running version stays. A rule matches a tool id and step, a question id, or a route name. It may set a threshold per model, a route, and a candidate on a share of traffic: `shadow` asks both and serves the current answer, `serve` answers that share with the candidate. A hash of rule, question and evidence picks the arm, so a record never changes sides. Every loaded version is stored, and every answer names it. Compare is `eval compare` across arms; promote and roll back are edits under git. 0.3 overrides thresholds and routes only. Wording stays in the local tool.

**Reporting.** Each result fills 0.2's reserved `proxy` fields, the answer id, the code's and the effective threshold, and adds the policy version, rule, arm and backend. Probabilities always return, so a local rerun at the effective threshold reproduces the answer. A route to a model with no threshold stated for it warns in every result.

**Keys, limits, failure.** Keys come from the proxy's environment by 0.2's `key_env` names; a caller's key is never forwarded. Each backend takes a rate, an in-flight cap and a daily spend cap. Past a bounded queue the proxy answers 429 with `Retry-After`, and callers take turns by claimed name. Vendor retries live only in the proxy. Batches merge across callers only when route, context and function match, never past a caller's batch size, with a short wait cap. A proxy that is down fails the call with the command to go direct, never a silent fallback.

**No access control, safely.** Loopback by default. A wider listen is typed, warned in `status` and every response, and refused without spend caps. A foreign `Host` or browser `Origin` is refused, closing the rebinding hole Ollama had.

**Room to grow.** Seven internal seams with open defaults (identity, authorization, audit, usage, quota, secrets, retention) let hosted run one proxy per customer on the same code.

## What must be true

| Risk | What retires it |
| --- | --- |
| Chunked streaming is slower than direct | The README's `filter` and `rank` prototype |
| The hop costs too much | Added latency on loopback, 50th and 99th percentiles |
| Merged batches shift answers or gain nothing | 1,000 single-question callers: throughput, cost, probability spread |
| A web page reaches a local proxy | Rebinding and foreign-origin tests |
| Proxy and local runs fight over one SQLite file | Concurrent writes, waits counted |
| Rollouts confuse without a screen | A fresh agent runs shadow, compare, promote and roll back from help |

## Out of 0.3

Sign-in, roles and per-caller budgets. Screens and the hosted service. MCP on the proxy. Saved functions and wording overrides. Answering from right answers. Models run inside the proxy. Automatic routing. An OpenTelemetry exporter, first after 0.3. PostgreSQL.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 25, 27, 28 and 29.
