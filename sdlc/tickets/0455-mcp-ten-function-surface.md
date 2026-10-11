# 0455: Expose the ten functions through a local MCP server

Status: in progress.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review.
Milestone: 0.2
Owner: builder, lane0 `ticket/0455-rank-members-completion`.
Risk: High. Local protocol dispatch and cancellation must preserve secrets, spend limits and started-failure behavior; broken output and EOF must cancel and join owned work.

Independent WIP now includes Windows owned pipe I/O with an operation-scoped
thread registry and joined cancellation watcher, strict native grammar/loading
regressions, and an expanded public named consumer. The High risk includes the
exact Windows-only `src/mcp/input/windows/ffi.rs` cancellation leaf and the
existing `windows-sys` package's required `Win32_System_IO` feature. Existing
unsafe leaves and their guards remain unchanged. Windows build/runtime proof
is still pending on the existing runner; Linux checks do not qualify it.
Installed execution uses one native Engine and complete result/2 calls/errors.
Native text admission correction fbee4ea7c and image fixtures b79ef2fc3 are merged.
All 33 MCP Rust tests, nine client fixtures, installed replay and 25 public
consumer cases pass, along with focused Clippy, policy and the source ratchet.
Nine of twelve installed behavior groups pass, including ordered images, native
object contexts, named/file questions, explicit controls, default cache and
refresh, strict no-key replay, secrecy, cancellation and EOF. Three retained
installed regressions expose native gaps: atomic
complete record JSON drops retained physical source locations; find needs the
settled located original serializer; dynamic choose needs a saved-model
override and authored non-root `on` preparation exposed. Do not invent fields or add a host parser.
The additional installed regressions remain required and unqualified until those
preparation helpers and native located serializers are adopted. Root owns fresh whole High review and full landing gates;
final real Windows execution and local timing remain pending. No complete parity,
main landing or publication is claimed.

Reviews: revision fe9bf662f0234e8177eecd2a0f0adea96b36e5d0, accept

## Outcome

An installed ThinkThen command starts a local MCP server. Agents call the same ten functions with ordinary question files, explicit file or folder inputs, complete typed results and the existing settings and cache. MCP appears in the executed parity table.

## Evidence

- Starts from: Ian's 2026-10-06 ruling in shared message `2026-10-06-pm-add-an-mcp-server-to-0-2-with-the-ten-functions-and-question-files.md`, asks 1–5. Main `5974abe80` supplies native/CLI hosted images. 0432 supplies the shared case inventory; 0442–0445/0450 settle complete results and one-route identity/cache behavior. Existing native readers and question loaders remain authoritative.
- Keeps: All ten function names and semantics, authored question order/descriptions, six error kinds, unsuccessful-member results, cancellation, request limits, secrecy, count-only usage, one endpoint/key/API type and per-question cache/replay. No core I/O.
- Changes: Add `thinkthen mcp` as a local stdio transport over the existing Rust engine. Add named tools, a public consumer in the existing surface ladder, install/start documentation and the MCP parity column. Supersede the service-declining row in `ten-use-cases.md` with Ian's current ruling. Measure local adapter overhead and state that future proxy latency is unmeasured. Ownership: `crates/thinkthen/src/mcp/**` `libraries/mcp/**`.
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


## Complete-input amendment, 2026-10-07

Astra's read-only code and fixture review found concrete gaps within the existing consistency outcome. Add an explicit `inputs` array of native record descriptors without reinterpreting existing arbitrary `records`. Descriptors retain original text or JSON, optional whole-file text sources, separate context, replacement options and ordered image paths with optional declared media. Reuse existing native composition, admission and readers. Keep `inputs` exclusive with the existing top-level input forms and reject conflicting projections before sends. Finite inputs validate before execution; the existing lazy route retains completed prefixes.

The same 0455 owner may make the bounded shared serialization prerequisite because no other active lane owns those serializers. Complete record wrappers expose their actual original zero-based index; complete find exposes its selected index and ordered original candidates with supplied locations. Ancillary images remain present beside retained originals. Use existing typed snapshots and views; preserve values, identities, facts and ordinary failure envelopes. The incremental tool route retains native completed observations alongside its terminal safe error and final facts instead of discarding the prefix through `Batch::into_call`. Update the generated schema and result contract additively. Standalone scalar results invent no record index.

Keep incoming messages bounded at 16 MiB. Explicit file-backed captions cover admitted decoded inputs whose escaped JSON would exceed that frame. The outgoing frame and bundled client cap become 192 MiB, including the newline, so required large two-record image results can retain both current equivalent representations. Native and vendor admission limits stay in force. Document increased peak memory and that an oversized output terminates without a partial JSON line; do not claim zero sends for output failure.

Risk: High for public result correctness, memory bounds and cancellation. One fresh whole-change review covers these shared prerequisites and MCP adoption. Existing required cases prove declared media mismatch, candidate order, separate contexts, physical locations, original ordinals, completed prefixes and image limits. No scheduler, parser, host cache, HTTP fetching, additional tool or verification framework is added. Ian can overturn this additive API choice. Astra's design assessment ran no builds or tests and does not establish parity.

## PM amendment, 2026-10-11

0455 closes on its Linux checks, under the rule that no ticket waits on Windows. MCP passes 258 of 258 installed cases. 0543 carries the Windows execution of the pipe cancellation code and the bounded MCP timing. Land this ticket now.

