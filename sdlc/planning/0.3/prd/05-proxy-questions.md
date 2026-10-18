---
date: 2026-10-13
status: draft PRD
source: Ian's rulings in [../proxy.md](../proxy.md) (2026-10-02 to 2026-10-13) and [../README.md](../README.md); PRD 00 and its questions; [../data.md](../data.md), [../toolbox.md](../toolbox.md), [../mcp.md](../mcp.md), [../evals.md](../evals.md); Ian's ThinkThen notes (strategy, data model, fixed and tunable, the later enterprise edition) and the ThinkThen ideal state; ThinkThen 0.2's `specification/` (sdk-boundary, backends, cache, records, result) and `thinkthen --help`; the thinkthen issues on the proxy service and OpenTelemetry traces; the earlier proxy prototype, for lessons only
---

# PRD 05 working file: the proxy's hard questions

This file backs [05-proxy.md](05-proxy.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the four that need his product direction. The PM can carry the rest into tickets, and Ian can overturn any of them. Claims about 0.2 cite the specification page in [`specification/`](../../../../specification/).

## Why run it at all

**Q1. What does a single builder gain from a proxy on their own machine, when the SDK already has the database?**
The honest answer for one script is nothing, and the docs should say so. The gain starts when more than one program asks questions, which is the normal case for a builder with agents. Four gains hold on one machine, and each has a test:
- **One rate limit for every process.** 0.2 paces requests inside one process only: "separate processes each get the full rate, so N processes can send N times it" ([backends.md](../../../../specification/backends.md#named-backends)). Three agents, a notebook and a DuckDB session hit the vendor's limit together. The thinkthen issue on the proxy service names this as its first gain. Proof: four parallel `filter` runs against one paced backend, counting refused requests direct and through the proxy.
- **Agents run with no vendor keys.** The key lives in the proxy's environment, so an agent's sandbox, shell history and logs never hold it, and a spend cap on the proxy bounds a looping agent. Proof: a demonstrated run with every vendor key unset in the agent's environment.
- **Small callers share full batches.** An agent asking one `decide` at a time, or a SQL row function, sends one question per request. The proxy packs questions from many callers into one request. Proof: throughput and cost of 1,000 single-question callers, direct and through the proxy.
- **One place to change a decision.** A threshold or a model change reaches every script, notebook and deployed app at once, with no code edit, and the change is recorded. Proof: a demonstrated rollout from the command line (Q8).
Recommendation: the docs lead with "run the proxy when more than one program asks questions", and the first gain marketed is the one with the cleanest number.

**Q2. Does the proxy run the same database as the SDK, and the same file?**
Ian ruled the proxy uses the same store as the SDK (`data.md`). Options: a separate default file for the proxy, or the same default file. The same file means a builder who starts a proxy keeps every answer already paid for, and local runs and proxy runs land in one place for `answers` and `eval compare`. SQLite allows several processes on one file in WAL mode, with one writer at a time. Recommendation: the proxy opens the same default database as the SDK (PRD 01), and `--database PATH` points elsewhere. An experiment measures write contention between the proxy and local runs on one file. If it hurts, the proxy gets its own default file and `answers import` moves rows.

## Doors and identity

**Q3. How does a caller reach each door?**
0.2 builds the posting address from a base plus a path suffix, `systemone` by default ([backends.md](../../../../specification/backends.md#named-backends)). A 0.2 SDK with `THINKTHEN_BASE_URL` set to the proxy therefore posts to `BASE/systemone` and must keep working. Recommendation: the System One door answers at `/v1/systemone`, the shape any System One client already posts. The function door answers at one route per action, named as PRD 00 names actions: `/v1/decide` through `/v1/relate` for the ten functions, and `/v1/answers/list` or `/v1/curate/outcome` for the data jobs. `run` has no route, because the client runs tools and sends each function step to its function's route. A ThinkThen-type backend always uses the function door; any other type sees a plain System One server.

**Q4. How does the System One door identify a question it did not create?**
A raw System One request carries a model name, the state and the questions, and nothing else. Options: treat every request as anonymous (cache and traces only); derive an id from the request; ask callers to add a header. Derivation is free and PRD 00 Q15 already defines it: the question id comes from the contract and the wording when no tool names it. A header costs other people's code a change, which defeats the door's purpose, but it helps those willing. Recommendation: the door derives the question id from contract and wording, accepts an optional `ThinkThen-Question` header, and treats the `model` field as a route name when it matches one in the proxy's configuration, the way OpenRouter and LiteLLM treat model aliases. It never applies a threshold override, because this door returns probabilities and the caller reads them. The result of a System One call is the System One reply, plus response headers naming the answer ids and the route; vendor-shaped bodies gain no fields.

**Q5. What keys do rules match on: the tool's hash, the tool's id, or the question?**
`proxy.md` says per-tool routing and overrides key on the tool hash. PRD 00 Q17 makes the id the identity, because the hash changes on every edit, including a typo fix. A rule keyed on the hash would silently stop matching after any edit, and the old setup would quietly return. Recommendation: a rule matches the tool id and step, or the question id for calls with no tool, or the route name on the System One door. A rule may pin a hash. A pinned rule that has matched nothing since a new hash appeared is reported by `proxy --check` and in the log. Answers still record the hash. This amends `proxy.md`.

**Q6. Does the caller's named model mean anything through the proxy?**
The SDK boundary says a future proxy mode "must refuse question-level model selection because the proxy owns that choice" ([sdk-boundary.md](../../../../specification/sdk-boundary.md#proxy-separation)). Refusal breaks the promise that the same code runs locally and through a proxy: a script with `--model jev-1.13.0` would fail the day it points at a proxy. The 2026-10-06 ruling gives the right pattern: the code owns, the proxy may override. Recommendation: a caller's model, from a flag, a tool or a question file, is a request. The proxy honors it unless a rule routes the call elsewhere, and the result names the model that answered and the rule that chose it. The ADR amendment PRD 00 Q14 calls for carries this line.

## Rollouts with no screen

**Q7. Where do rules live: a configuration file, commands, or both?**
PRD 00 Q25 keeps administration local and puts overrides and routing in the proxy's configuration file. Commands that edit a file someone also edits by hand fight each other. A file can sit in git, so a rollout gets review, history and a one-line revert for free. Recommendation: one rules file is the only source. The proxy watches it, loads a valid version at once, and keeps the running version when a new one is broken, with the reason in the log. Every loaded version is stored in the database with its text, its digest and the time, so each answer names the policy version that served it and old rules are recoverable without git. `thinkthen proxy --check` validates a file, reaches each backend and prints which tools each rule would touch, without starting a server.

**Q8. What does a rollout look like: share of traffic, compare, promote, roll back?**
`data-model.md` already states the shape: try a new setup on a share of traffic, compare it, promote it, roll it back with one switch. Feature-flag tools split by hashing a stable key into buckets, so raising a share only adds buckets and nobody flips back and forth (LaunchDarkly). Decisions differ from page views in one useful way: they are cheap and they can be paired. Running the candidate beside the current setup on the same evidence gives a direct comparison with no answer key, which the prototype found works, and it feeds the data strategy of curating where two models disagree (`data.md`). Recommendation: a rule may name a candidate with a share and a mode. `shadow` asks the candidate too and serves the current answer; `serve` answers that share with the candidate. The bucket is a hash of the rule name, the question id and the evidence, so one record always lands in the same arm, and the cache stays coherent. Compare is `eval compare` over the two arms in the store (PRD 00). Promote and roll back are edits to the rules file. Shadow spend counts against the backend's spend cap.

**Q9. What happens to a run that spans a rules change?**
A `filter` over a million records sends many chunks. A reload in the middle would answer half the run under each policy, and the run's numbers would mean nothing. Recommendation: a call pins the policy version at its first chunk, keyed by its call id, and keeps it to the end. New calls take the new version.

**Q10. Which overrides does 0.3 allow?**
0.2 already reserves three override kinds in the result: none, threshold, and decision ([result.md](../../../../specification/result.md#reserved-proxy-types)). `proxy.md` also lets the proxy hold saved functions with wording. A wording override changes what is asked, so it needs an answer key to justify it, and the tool's wording already lives in a local file under git. A decision override answers from a settled right answer, which PRD 00 puts out of 0.3. Recommendation: 0.3 activates threshold overrides and routes. Decision overrides wait for answering from right answers. Wording stays in the local tool, and the proxy holds no saved functions in 0.3.

**Q11. A rule moves a tool to another model. Whose threshold applies?**
A threshold only means something for one model (`data-model.md`). If a route moves a tool from Jev to d1 and the code's 0.7 was tuned on Jev, applying 0.7 to d1 silently miscalibrates every answer. PRD 00 Q36 lets a tool carry a cut per model. Recommendation: the client sends the tool's cuts per model. The proxy applies, in order: the rule's threshold for the routed model, the tool's cut for that model, then the code's cut with a `threshold_model_mismatch` warning in each result, like 0.2's calibration profile warning. `proxy --check` warns about any rule that reroutes without a threshold.

**Q12. How does the result report what the proxy did, so a local run and a proxy run compare?**
Principle 6 says the proxy never adds a behavior a local run cannot reproduce. Reproduction needs the facts. 0.2 reserves `proxy:{decision_id, override}` with the code's threshold and the effective threshold. Recommendation: activate that reservation and add the policy version, the rule name, the arm and the backend name. The probabilities are always returned, so a local rerun with `--threshold` set to the effective value gives the same answer from the same probabilities. `eval compare` can line up a local run and a proxy run by question and evidence.

## Running it safely with no access control

**Q13. Where does the proxy listen by default, and how does a user open it wider?**
The proxy holds vendor keys and spends money, and with no access control anyone who reaches it can spend. Redis learned this in public: instances listening on every interface without a password were found and wiped, and Redis added a protected mode that answers only loopback when no password is set. Ollama listens on loopback by default. Recommendation: listen on `127.0.0.1` by default. A wider address must be typed (`--listen 0.0.0.0:PORT`), and then the proxy prints a warning at start, repeats it in `status` and in every response header, and refuses to start if any backend has no spend cap. The Docker image listens wider inside the container, so its documented command publishes the port on `127.0.0.1` only.

**Q14. Can a web page in the builder's browser reach a proxy on loopback?**
Yes, through DNS rebinding. Ollama's CVE-2024-28224 let any web page a user visited reach the local API and use its models, until Ollama checked the host. A local proxy holding paid keys is a richer target. Recommendation: the proxy answers only requests whose `Host` names a loopback address or a host listed in its configuration, sends no cross-origin headers, and refuses any request carrying a browser `Origin` it does not list. A test proves the refusal.

**Q15. Should a proxy opened to a network require one shared token? (Ian)**
Ian ruled no access control in 0.3. A single optional token, set in the proxy's environment and sent by clients as their key, is not users or roles, but it is a form of access control. Without it, the only safe wide deployment is a private network the builder trusts, such as a VPN. With it, a builder can run the proxy on a small server for their deployed app at little risk. The token also uses the slot later sign-in needs: the client's `Authorization` header to the proxy. Recommendation: allow one optional token, required whenever the proxy listens beyond loopback, and say plainly that it is not sign-in. The cost is small: one setting, one check, one refusal message.

**Q16. Who holds the vendor keys: the proxy, or callers who send them through?**
Ian ruled customers bring their own keys and the proxy holds them. Code written against a vendor sends that vendor's key in `Authorization`. Forwarding it would let callers bypass the proxy's caps and would mix several owners' answers in one cache. Recommendation: the proxy reads keys only from its own environment, by the same `key_env` names 0.2 uses, and its configuration file never holds a key, as in 0.2 ([backends.md](../../../../specification/backends.md#named-backends)). A caller's `Authorization` header is never forwarded, logged or stored. In 0.3 it is ignored, or checked against the shared token if Ian allows Q15. A backend with no key on the proxy fails with a message naming the variable to set on the proxy.

## Throughput and failure

**Q17. How does batching across callers work without changing answers?**
0.2 fills a request with records up to 96,000 bytes, a record never counts as evidence for another, and a threshold saved in a question file was tuned at that file's batch size ([records.md](../../../../specification/records.md)). Ian ruled that batching adds noise users measure, and that each decision records its batch size (2026-10-06). A caller who chose `--batch 1` asked for its records to travel alone. Recommendation: the proxy merges questions only when they share a route, a shared context and a function, and never past any caller's batch size, so `--batch 1` stays alone. It waits for a fuller request at most a short cap, the way Triton's `max_queue_delay_microseconds` bounds its dynamic batcher, and sends at once when a request fills. Each answer records the batch size it travelled in. In a later hosted service, no request ever holds two customers' questions. An experiment sets the default wait against measured model latency.

**Q18. How does the proxy apply backpressure and limits per backend?**
Options: queue without limit, refuse when busy, or queue with a bound. An unbounded queue turns a slow vendor into memory exhaustion and long silent waits. Recommendation: each backend in the proxy's configuration takes 0.2's `requests_per_minute`, a cap on requests in flight, and a daily spend cap from 0.2's caller prices. The proxy queues within a bound and answers past it with status 429 and `Retry-After`, which the client honors within its own deadline. Callers are served in turn by the name they claim (PRD 00 Q39), so one million-record `filter` cannot starve an agent's single `decide`. Retries to vendors live in the proxy only. The client retries the proxy only when it could not connect or was told to wait, so retries never multiply.

**Q19. What happens when the proxy is down?**
Options: fail; fall back to a direct backend; make fallback a setting. Fallback needs vendor keys on the client, which undoes the keys gain. It also skips the rules, so a rollback would silently stop applying. ADR 0119 already refuses provider fallback in the SDK: "a failed call cannot discover or switch to another provider" ([sdk-boundary.md](../../../../specification/sdk-boundary.md#one-configured-route)). Recommendation: never fall back. The call fails with the exit code of an unreachable backend and a message naming the proxy address and the command to go direct (`--backend typesafe`). A retried chunk never pays twice, because answers already stored are served from the store.

**Q20. How do a client and a proxy of different versions meet?**
The SDK boundary notes that no proxy protocol yet establishes capabilities. A 0.3 client against a 0.4 proxy, or the reverse, must fail clearly or work. Recommendation: the protocol carries its version in the path (`/v1/`). A ThinkThen-type engine asks the proxy's status once when it is built and refuses an incompatible proxy with a message naming both versions. Additive fields never break a reader, as with result/2.

## Shape, deployment and later

**Q21. One executable or two? (Ian)**
Ian ruled the proxy "a second program" under the same name, version and site. PRD 00 Q8 left packaging to this PRD. Two executables mean a second install path in Homebrew, the download script and every package. One executable means `thinkthen proxy` works wherever `thinkthen` is installed, and the Docker image is that executable with a different first argument. The libraries in 24 languages must not carry a web server. Recommendation: the proxy is its own crate in the ThinkThen repo, built only into the command-line executable, and started by `thinkthen proxy`. The libraries link the engine alone. This reads "second program" as a separate part of the code, not a separate download.

**Q22. Does the proxy also accept OpenAI Decisions requests from other people's code? (Ian)**
Outbound, the proxy speaks OpenAI Decisions to OpenAI, as 0.2 does. Inbound is the open choice. Code written with OpenAI's client can set its base address to the proxy. A same-shape pass-through (Decisions in, Decisions out to OpenAI) is cheap and gives that code the cache, keys and traces. Translation (Decisions in, Jev or d1 out) is the stronger pitch, "point your OpenAI code at ThinkThen and switch models with one line", but it needs a reverse adapter, and two shapes that differ in names and options may not map one to one. Recommendation: in 0.3, a Decisions door that passes through to OpenAI only. Translation to System One backends follows if the Awesome ThinkThen tests show the shapes map cleanly.

**Q23. Does the proxy run small models itself? (Ian)**
Ian allows the docs to promise small models on the proxy, and whether models such as EmbeddingGemma or GLiNER can answer System One questions is open. Running inference inside the proxy adds model files, CPU and GPU builds and a second performance story to one small server. Routing to a small model served by llama.cpp, MLX or a thin System One wrapper costs nothing new, because the proxy routes to any System One backend. Recommendation: in 0.3 the proxy routes to small models and runs none. An experiment wraps one small non-chat model in a System One shim and tests it on the ten functions. Running models in the proxy waits for that result.

**Q24. What does the operator watch?**
U45 asks for retries, failures and latency by backend and model. The store already records each request (PRD 01), so `answers stats` through the proxy can show them with no extra system. The 2026-09-30 ruling keeps telemetry out of the SDK and puts traces in the proxy, opt-in, with no keys or text in spans. Recommendation: in 0.3, the store holds every call and request with latency, attempts and failures; the proxy writes one JSON log line per call with no text and no keys; `GET /v1/status` reports health, version, store, policy version and exposure. An OpenTelemetry exporter is the first addition after 0.3, using the published generative AI conventions for its field names.

**Q25. How does the hosted service later reuse this proxy without a fork?**
Ian's notes on the later enterprise edition recommend open extension points with working open defaults, paid code in a private repo, and one proxy for hosted and on premises. The cheapest moment to cut those seams is before the code exists. Recommendation: the proxy calls seven internal interfaces, each with a 0.3 default: identity (the name the caller claims), authorization (allow all), audit and usage (tables in the store), quota (per-backend rate and spend caps), secrets (the environment) and retention (keep everything). They stay internal in 0.3, so nothing public must stay stable yet. One proxy serves one store, and the hosted service first runs one proxy per customer, which matches "nothing is pooled".

**Q26. Does the proxy serve MCP?**
The strategy says the proxy serves the functions "over HTTP and MCP". `mcp.md` makes MCP a local server that sends its calls to a proxy when one is configured, so paths are read on the client and no server reads an agent's disk. Recommendation: no MCP on the proxy in 0.3. A remote MCP endpoint belongs with sign-in and the hosted service.

**Q27. What may the docs promise now?**
Marketing covers 0.2 until 0.3 ships, and every claim needs a measured number or a demonstrated behavior. Recommendation: the docs may state the direction: an open-source proxy under MIT in the same project, the same functions, a shared store, your own keys, one place to change thresholds and models, small models later. They may not promise access control, a hosted service, cross-caller speedups or latency numbers until the experiments below report.

## Experiments that retire the risks

1. **Streaming through the function door** (already in the README): a large file through `filter` and `rank`, direct and through a local proxy, with time and cost.
2. **Overhead:** added latency per call on loopback, with and without a stored answer, at the 50th and 99th percentiles. The strategy names ThinkThen's own overhead as the number to publish.
3. **Rate limits across processes:** four processes, one paced backend, refusals counted direct and through the proxy.
4. **Batching across callers:** 1,000 single-question callers; throughput, cost and the spread of probabilities against batch size.
5. **Safety:** a loopback proxy refuses a rebinding page and a foreign `Origin`; a wide listen without spend caps refuses to start.
6. **Shared file:** proxy and local runs writing one SQLite file at once, with waits counted.
7. **Rollout:** a shadow rule on a recorded tool, `eval compare` across the arms, promote, roll back, and every answer naming its policy version.

## Sources

- Redis security and protected mode: https://redis.io/docs/latest/operate/oss_and_stack/management/security/ and https://www.antirez.com/news/96
- Ollama DNS rebinding, CVE-2024-28224: https://www.nccgroup.com/research/technical-advisory-ollama-dns-rebinding-attack-cve-2024-28224/
- LaunchDarkly percentage rollouts and bucketing: https://launchdarkly.com/docs/home/releases/percentage-rollouts
- Triton dynamic batching and its queue delay: https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/batcher.html
- LiteLLM routing, fallbacks and model aliases: https://docs.litellm.ai/docs/routing
- OpenTelemetry generative AI conventions: https://opentelemetry.io/docs/specs/semconv/gen-ai/gen-ai-spans and https://opentelemetry.io/docs/specs/semconv/gen-ai/gen-ai-metrics/
- HTTP 429 and `Retry-After`: RFC 6585 section 4 and RFC 9110 section 10.2.3
- ThinkThen 0.2: `specification/sdk-boundary.md`, `backends.md`, `cache.md`, `records.md`, `result.md`; `sdlc/issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md`; `sdlc/issues/2026-09-30-opentelemetry-traces-after-0-1.md`
- The earlier proxy prototype's lessons (a two-model comparison works with no answer key; each answer stores the threshold it used)
