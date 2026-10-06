# 0455: Expose the ten functions through a local MCP server

Status: ready. Fresh ticket review accepted the local MCP contract; implementation remains open.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review.
Milestone: 0.2
Owner: builder.

## Outcome

An installed ThinkThen command starts a local MCP server. Agents call the same ten functions with ordinary question files, explicit file or folder inputs, complete typed results and the existing settings and cache. MCP appears in the executed parity table.

## Evidence

- Starts from: Ian's 2026-10-06 ruling in shared message `2026-10-06-pm-add-an-mcp-server-to-0-2-with-the-ten-functions-and-question-files.md`, asks 1–5. Main `5974abe80` supplies native/CLI hosted images. 0432 supplies the shared case inventory; 0442–0445/0450 settle complete results and one-route identity/cache behavior. Existing native readers and question loaders remain authoritative.
- Keeps: All ten function names and semantics, authored question order/descriptions, six error kinds, unsuccessful-member results, cancellation, request limits, secrecy, count-only usage, one endpoint/key/API type and per-question cache/replay. No core I/O.
- Changes: Add `thinkthen mcp` as a local stdio transport over the existing Rust engine. Add named tools, a public consumer in the existing surface ladder, install/start documentation and the MCP parity column. Supersede the service-declining row in `ten-use-cases.md` with Ian's current ruling. Measure local adapter overhead and state that future proxy latency is unmeasured.
- Proof: Installed subprocess initialization and actual tool calls exercise the shared cases through each named method. Compare independent expected results, full typed schemas, request bodies and counted sends. Cover file/folder locations and order, question-file validation, image support/refusals, cache and zero-send replay, false/null successes, errors, secrecy, cancellation and EOF cleanup. Run policy before fresh code review, full tests/lint on the landing commit and affected spec/surface/package checks.
- Defers: HTTP serving, background installation, audit/runs/cache-administration tools, prompts/resources, question-writing tools, business routing and proxy implementation or latency claims.

## Contract and ownership

Expose exactly decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. `question_file` explicitly loads an ordinary file; inline strings, including `@` prefixes, stay text. The agent writes question files through its own tools. Validate mutually exclusive inline/file questions and inline records/file sources before work. Reuse applicable CLI reading, context, field, model, batch, record/replay and admission options; route credentials are startup settings, never tool arguments. Preserve per-record context/options and full-set semantics under their owning contracts.

Source paths reuse the native reader and its framing, limits, ordering, repeated inputs and locations. A folder yields separate items. Explicit ordered attachments allow admitted image decide/choose/score. The other seven functions refuse images under 0447's ruling. Paths never silently become model evidence. File access runs with the launching user's authority; no network fetch or question creation is added.

Start with MCP 2025-11-25 stdio initialization/negotiation, ping, tools/list, tools/call and cancellation. Keep stdout exclusively JSON-RPC. Bound incoming messages and dispatch; a reader remains able to cancel active work. EOF or broken output cancels and joins owned work. Use existing serde and threads; no new dependency is planned. Match supported protocol negotiation, input/output schemas, structured content and safe error mapping to the official MCP specification at implementation.

Use one engine resolved by existing settings. Preserve the default persistent cache; do not use a host helper that disables it. Never shell out per call or build a second reader, cache or scheduler. Serialize the settled native complete result/2 types, including observations/facts and partial failures. Add validated `mcp` surface identity with 0443; no future proxy protocol is inferred from a hostname.

Primary files: `src/mcp/`, CLI command/dispatch registration, `libraries/mcp/` consumer, shared conformance/surface inventory, `specification/mcp.md` and install/agent guidance. Shared native helpers and surface identity stay with their owner until settled. MCP can implement its transport independently; complete execution depends on 0443/0444/0445/0450 and relevant input/image semantics, not C or other binding families.

## Added-latency check

Use the existing explicit stress/timing entry point for a bounded comparison of direct Rust calls and MCP calls against identical saved/loopback answers. Sample atomic, record and whole-set paths, separate cold startup from warm calls, and report payload sizes, request counts, median, exploratory p95 and maximum. Keep the same engine/settings/cache state for paired measurements. This measures local MCP overhead; proxy network/policy overhead waits for an actual proxy. Add no broad benchmark, retained timing receipts or unmeasured speed claim.
