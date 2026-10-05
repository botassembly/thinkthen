# 0421: Add llama.cpp and MLX local backends

Status: ready. Fresh ticket review accepted; build follows the Windows corrections.
Milestone: 0.2

## Outcome

`--backend llamacpp` and `--backend mlx` select local System One servers without a key. Existing `ollama` keeps its address, model and text rendering. CLI, Rust, C, bindings and SQL resolve the new names through the existing engine. `thinkthen backends check` explains the compatibility of the ten functions without comparing model quality. One page, “Run a model locally”, shows the three supported server setups. One approved M5 check per runtime either passes or identifies an upstream runtime failure.

## Evidence

- Starts from: tickets 0339, 0377 and 0416; ADRs 0114 and 0115; the approved 2026-10-05 plan. Experiment 0030, final report dated 2026-10-04, observed six llama.cpp paths passing the four-request check on native revision `a7b94df2c616bc1f62a73b964b4a71cb0dcc488e`. Its MLX diagnosis on 2026-10-04 found that Strands score criteria require strings, with 2–10 levels; rich criteria returned HTTP 422 while string criteria returned HTTP 200. Its Ollama 0.35.1 findings were loader and decision-encoder failures. These observations justify adapter and setup work. They do not rank models.
- Keeps: unnamed and hosted request bytes; Ollama request bytes, `nimble` default and `http://localhost:11434/v1` base; backend selection precedence; configured entries overriding built-ins; existing explicit model and address overrides; loopback-only clear text; no Authorization header when the key is absent on loopback; missing-key refusal elsewhere; built-in credential host guards; existing failed-answer and exit-code contracts; cache and recording identity following encoded question bytes; replay and all text, record and column calls.
- Changes: add two rows to the built-in table, both posting `systemone` beneath a `/v1` base. llama.cpp uses `Authored`; MLX uses the existing `Text` rendering. Existing `Text` already turns null and empty score descriptions into their level names, so the rich score probe becomes an all-string array. No additional score dialect is needed. Parameterize workaround warnings so MLX never says it is Ollama, while the existing Ollama sentence remains unchanged. Extend the check report with ten function rows reporting actual minimal function-path execution separately from rich wire probes. Update the relevant contract, settings, backend examples and local setup page together.
- Proof: outside-in loopback tests send production requests through CLI and one shared C consumer. The llama.cpp mimic accepts authored criteria. The MLX mimic enforces string score criteria and returns the retained reply shape. Pin the rich score body, both warning identities, no Authorization headers and the existing non-loopback no-send refusal using request counts. Retain the four existing probes, add the minimal actual function-path checks, and assert the new function rows on success, partial score failure and early connection failure. Verify differing encoded criteria cannot reuse a cached answer. Run focused tests during the slices, then full tests, lint, spec and affected surfaces at landing. Run one real local-server check per runtime on M5 after the setup is reviewed; retain observed version/model and result in the ticket's short landing record.
- Defers: image support; installing, downloading or launching models from ThinkThen; generic MLX chat-server compatibility; model rankings and accuracy intake; new profiles or configuration fields; changing model-name cache refresh rules; provider/API-version and modality outcomes beyond 0.2; exhaustive real function execution on every model. No proof runner, receipt system or extra checker is added.

## Runtime choices and prerequisites

Ollama remains `http://localhost:11434/v1`, `nimble`, `OLLAMA_API_KEY`, `Text`. llama.cpp's usual server base is `http://localhost:8080/v1`; the built-in should read `LLAMACPP_API_KEY` under the existing missing-key and host-guard rules. Use a supported decision model on the native `/v1/systemone` build. A chat-compatible llama.cpp server or older release that lacks that route is insufficient. Native experiment successes included Kev4 and Clef27; choose a small successful model for the setup, rather than one based on its accuracy score. Bind the example to loopback explicitly and set model context and physical batch within its metadata and full prompt requirement.

The MLX setup is specifically `strands-decider serve CHECKPOINT --device mlx`, source revision `75c9fd32e664954cdc18481434018aa507eee8fb`, using the public `StrandsAgents/strands-decider-2B-hobson-v19` checkpoint and its required base model. It serves System One; plain `mlx_lm.server` does not establish support. The observed server alias was `strands-decider-2B-hobson-v19`. Use `MLX_API_KEY` for the optional key under the same rules.

Primary-source follow-up confirms the Strands CLI defaults to 127.0.0.1:8000 and a checkpoint-basename model name. Use http://localhost:8000/v1 and strands-decider-2B-hobson-v19 for the MLX builtin. Its pinned CLI source is https://github.com/strands-labs/strands-decider/blob/75c9fd32e664954cdc18481434018aa507eee8fb/src/strands_decider/cli.py. llama.cpp defaults to 127.0.0.1:8080; its default model id is the model-file path, while --alias selects an API id. Set an explicit documented alias in the setup, and do not claim a universal runtime default model name.

Use built-in llama.cpp model `local` and document `--alias local` in its server setup. This is the supported setup alias, not an upstream universal default; explicit `--model` overrides remain available. The saved experiment ports 11440/11441 are alternate experiment ports and do not set the built-in addresses.

## Check coverage

Retain the existing four rich primitive/mixed probes and their failure contracts. Extend this existing command with minimal generic inputs exercising all ten public function paths, including recognize span planning and relate endpoint planning. Report per-function answered, incompatible or unchecked when earlier failures prevent a safe run. Assert request/response/decoder compatibility and source-independent local invariants; do not require guessed labels or measure accuracy. Use existing request and timeout controls, with a documented bound derived from the actual staged inputs rather than an arbitrary estimated cap. The command sends no paid call through gates.

Rank and annotate coverage identifies the tested question kinds; a small successful probe does not promise every question set or all model behavior. A recognizable no-span response or empty relation set can be a valid answer. Never manufacture a span or edge merely to pass. Preserve rich score refusal evidence even if a simple score works. Existing warnings and critical counts describe the actual failures. This is an extension of backends check, not a new checker or benchmark.

## Runtime debt

Keep the existing Ollama issue as its one owning runtime debt issue. Add loader/architecture and encoder limits there as distinct findings with pinned primary references: `https://github.com/ollama/ollama/blob/v0.35.1/decision/systemone.go` and `https://github.com/ollama/ollama/blob/v0.35.1/server/routes.go`; retain object-criteria report `https://github.com/ollama/ollama/issues/18718`.

Create one MLX debt issue for the Strands rich-criteria schema mismatch, with primary reference `https://github.com/strands-labs/strands-decider/blob/75c9fd32e664954cdc18481434018aa507eee8fb/src/strands_decider/schema.py`. Track the text workaround, 2–10 score levels and unproved structured instructions there. Remove MLX text rendering when the server accepts the authored criteria. Do not invent an upstream issue number.

Create one llama.cpp debt issue for route/build/model prerequisites and unsupported decision encodings, citing `https://github.com/ggml-org/llama.cpp/blob/a7b94df2c616bc1f62a73b964b4a71cb0dcc488e/tools/server/README.md`. Separate a model returning 501 because it lacks this decision route from an adapter failure. Treat model/context/batch bounds as supported setup limits, rather than accuracy findings.

## Small build slices

1. Confirm pinned runtime defaults, add the two built-in rows and MLX warning identity, and land the loopback request/secrecy regressions with contract updates. Amend ADR 0115's Ollama-only text scope under the new approval.
2. Add minimal actual ten-function compatibility smokes and rows to the existing check report, without a new validation command. Preserve partial-failure and exit behavior.
3. Publish the local setup page with explicit runnable server commands and supported public models. Perform the three approved M5 checks through the existing manual workflow. Name actual runtime defects in their owning debt issues; then run the integrated landing gates and record the outcome once.
