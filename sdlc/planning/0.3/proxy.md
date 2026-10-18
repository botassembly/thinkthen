---
date: 2026-10-11
status: design input for 0.3
source: Ian's dictation on 2026-10-11, gathered from the proxy sections of Ian's strategy and ideal-state notes, and the earlier proxy prototype's note on what the proxy does
---

# ThinkThen proxy

The proxy is a ThinkThen server you run on your own machine or your own servers. Code points at it with `THINKTHEN_BASE_URL`. 0.3 builds it beside the toolbox (`toolbox.md`). Evals are a separate design (`evals.md`).

Ian, 2026-10-13: the SDK already fronts any model. The proxy does the same on a server and adds one central choke point for controlling rollouts to production. Its store is the same database the SDK uses on its own (`data.md`), so nothing in the data strategy requires a running proxy.

## What is already ruled

- **Open source, one project.** MIT, in the ThinkThen repo, a second program under the same name, version and site. It shares the engine, so every function behaves the same (2026-10-02).
- **Another System One endpoint.** It speaks the System One wire shape, so every library, the command line and the SQL extensions reach it as one more backend (Ian, 2026-10-11).
- **Customers bring their own keys.** The proxy holds the vendor keys, rate limits and traces, and later the access rules.
- **The cache.** A repeated question costs nothing. Nothing is evicted, because the history becomes data. The SDK turns its own cache off in proxy mode.
- **The code owns its threshold.** The proxy may override it, so a business controls decisions and A/B tests in one place (2026-10-06).
- **Answer IDs and outcomes.** Every answer carries an ID. The proxy accepts what happened downstream, so questions improve from real outcomes.
- **Routing by question.** The proxy picks the backend, a group, or a small model tuned for one tool. It needs one model per tool type, routing on the server, and CPU support.
- **Retries live in the proxy.** It records retries, failures and latency by backend and model.
- **Data stays put.** Each customer's data lives in its own schema, database or SQLite file. Nothing is pooled.

## Two doors, not three modes

`run` is not a mode. It is the eleventh function, so the proxy has two doors.

1. **The System One door.** A plain pass-through. A caller sends a System One request, and the proxy forwards it to the right backend, with the shared cache and traces.
2. **The function door.** The ten functions run on the proxy. `run` stays in the client, which sends each function step here. An SDK pointed at a ThinkThen proxy sends the function call, and the proxy does the batching, caching, thresholds and backend calls.

### Why keep the System One door

The function door is better for ThinkThen callers. It sees the whole call: a `recognize` is one call, not dozens of decomposed requests. So curation, thresholds and optimization work at the level where the intent lives. Ian's point, and it holds.

The System One door still earns its place for callers that are not ThinkThen:

- **Other people's code.** DSPy's TypeSafe integration, pg-jev, jevgrep, and any script written against a vendor's API already speak System One. Pointed at the proxy, they get the shared cache, one place for keys, and traces without adopting ThinkThen. This is the OpenRouter play: own the control point even for code we did not write.
- **Older ThinkThen.** 0.1 and 0.2 SDKs reach any address through `THINKTHEN_BASE_URL` with System One requests. They keep working against a 0.3 proxy.
- **Debugging.** A raw request shows exactly what a model does, without function logic in the way.

The door is cheap, because the proxy already speaks System One to its backends. Curation through this door is weaker, because the proxy sees requests, not function calls. That is the honest trade-off.

### The backend type picks the door (Ian, 2026-10-11)

0.2 already gives each engine one provider API type. The ThinkThen proxy becomes a third type, so no detection and no extra setting are needed.

| Backend type | Who speaks it | What the SDK does |
| --- | --- | --- |
| System One | TypeSafe's Jev and nearly every other provider | Runs the function itself and sends System One requests |
| OpenAI Decisions | OpenAI only | Runs the function itself and sends Decisions requests |
| ThinkThen | A ThinkThen proxy | Sends the ten function calls. The proxy runs them |

A ThinkThen proxy also accepts System One requests, so other people's code can use it as a System One backend. A ThinkThen SDK configured with the ThinkThen type always uses the function door. A named backend in the configuration file carries its type, like any other backend.

### Tools through the proxy (Ian, 2026-10-11)

The client runs `run`. It reads the tool, runs shell steps on the caller's machine, and sends each function step through the function door, tagged with the tool's name, hash and step number. The proxy runs only the ten functions and never runs a shell command. It never stores or serves tool definitions: tools are local only (Ian, 2026-10-12). It may hold saved functions, with their wording and thresholds, that local tools call. Per-tool routing, outcomes and overrides key on the tool hash. Earlier text that had the proxy run whole tools is replaced.

### What runs where

All of this lives in the Rust core, shared by the SDK and the proxy. No language binding dispatches anything. The 0.3 refactor separates three layers:

| Layer | Where it runs | What it does |
| --- | --- | --- |
| Client | Always in the caller's process | Reads files and records, applies `--pointer` so only the pointed text leaves, reassembles records, prints results |
| Engine | In the caller, or in the proxy through the function door | Function logic, decomposition such as recognize's steps, batching, cache, threshold reading, retries, backend adapters |
| Proxy only | The proxy | Shared cache across callers, central overrides, routing per tool, outcome records, traces |

The proxy returns probabilities with each answer. The code's own threshold still applies unless the proxy overrides it.

### Streaming records through the function door

Options for `filter`, `rank` and other record runs:

1. **Chunked requests.** The client sends records in chunks, a few hundred at a time, with several chunks in flight, and prints answers as chunks return. It is plain HTTP and works from every language, because the Rust core does it.
2. **One streaming connection.** Newline-delimited JSON flows both ways over HTTP/2. It has the lowest latency, but full-duplex HTTP is fragile through proxies and load balancers.
3. **WebSocket or gRPC.** It is fast, but it adds a dependency and an operations burden for one gain.

Ruled: chunked requests. The proxy merges chunks from all callers into full backend batches, up to 64 rows for Jev, with a short wait cap. `rank` sends scores in chunks and sorts on the client. `find` and `relate` need every record in one call, so they send the whole set at once. The extra hop costs milliseconds against 60 to 500 ms per model answer. The shared cache and central batching should make most runs faster.

## 0.3 scope (Ian, 2026-10-11)

- **In:** both doors, the shared cache, central thresholds and overrides, routing per tool, and answer IDs with outcome records. The command line and configuration files drive everything.
- **The store:** SQLite for now.
- **No access control in 0.3.** The proxy starts as a remote cache that accepts every caller. The design must leave room for keys, access rules and budgets later.
- **Out:** the user interface (the studio), the hosted service, and the enterprise edition. Datasets, curation, benchmarks, optimization and training belong to `evals.md`.
- **Backends:** every System One provider, plus OpenAI Decisions, the one known backend with a different shape. Marketing tests the models after 0.2 ships. Each one that works joins 0.2 marketing, and each one that fails becomes a 0.3 bug.
- **Both modes work:** every function runs without the proxy. The proxy adds sharing and control. It never adds a tool behavior that a local run cannot reproduce.
- **The prototype.** The earlier proxy prototype stays for reference for now and is archived later. Its designs are not carried forward.

## What the proxy adds over single-player mode

| Feature | Single-player | Through the proxy |
| --- | --- | --- |
| Cache | Local folder, one user | Shared by every caller |
| Batching and concurrency | Each caller's own | Central, across callers |
| Functions | Run in the SDK | Run in the proxy through the function door |
| Tools | Run in the SDK from a local folder | Still run in the SDK from a local folder; function steps go to the proxy |
| Threshold | The code's | The code's, or the proxy's override |
| Model choice | The caller's setting | Routed per tool |
| Answer IDs | In each result | Plus outcome records and traces |

## Design inputs already written

- Ian's notes on the data model and on what is fixed and what is tunable: names for the parts of a call, and what may change before it becomes a new question.
- Ian's notes on decision knowledge: storing and reusing decisions across models.

## Ruled, 2026-10-11

- The proxy keeps the System One door for other people's code.
- Record runs use chunked requests through the function door.
- The client runs tools, which live only on the caller's machine; the proxy runs only functions and never serves tools (Ian, 2026-10-12).
