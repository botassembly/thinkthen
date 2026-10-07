# Local MCP SDK

Status: **Settled** target for 0.2 by Ian's 2026-10-06 ruling and accepted
[ticket 0455](../sdlc/tickets/0455-mcp-ten-function-surface.md). Development execution now exposes the ten tools through native complete calls.
Native typed source serialization and dynamic choose saved-model/on preparation
are integrated. Fresh whole MCP review, landing gates, complete parity, timing
and final Windows qualification remain pending.

An installed `thinkthen mcp` is a local stdio subprocess over one resolved Rust
engine. Its default persistent cache, endpoint, effective key and provider API
type follow the ordinary settings contract. Startup exposes only applicable
existing CLI engine settings; tool arguments never select a key or route.
There is no HTTP server, background installation or proxy business policy.

The supported protocol is MCP 2025-11-25. Initialization accepts the client's
requested version and responds with that version when supported, otherwise
with 2025-11-25. Only the tools capability is advertised. The client then sends
`notifications/initialized`. Ping is permitted during initialization. Tools
wait for initialized; duplicate initialize is invalid. Unsupported resources,
prompts, administration, runs/audit, transforms and task methods are refused.
The tool catalog contains exactly decide, choose, tag, score, filter, rank,
find, annotate, recognize and relate, in that order.

Input consists of UTF-8 newline-delimited JSON-RPC objects, without batch arrays.
IDs are strings or integers and are preserved without coercion. Null IDs are
invalid. Each incoming line including its ending is at most 16 MiB; an oversized
or unterminated frame closes the session without unbounded draining. One
bounded four-message protocol inbox and at most one active tool dispatch keep
the reader responsive. Excess active calls receive a safe busy error; inbox
overflow closes and cancels the session without an unbounded reply backlog.
The reader never writes output, and stdio pipe output must be stoppable. Output is
exclusively complete JSON-RPC lines and at most 192 MiB per message, including the newline. Both equivalent result representations count toward this bound and increase peak memory. Oversized output closes without a partial JSON line; work may already have sent requests. An output
failure cancels the session and joins only its owned reader/dispatch work.

Cancellation references an active tools/call ID with the same original type.
Malformed, unknown and already completed cancellations are ignored, and their
reason is never echoed or logged. Client cancellation suppresses the tool
response and fires the existing native cancel token; already-sent native
attempts finish and join under the native contract. EOF likewise cancels active
work. Stoppable input reads are required, including a partial frame when stdout
breaks; a permanently blocked reader is not a completed implementation.
Windows stdio requires pipe handles. Own duplicates without changing or closing
the process's inherited handles; interrupt only the registered MCP read/write
and join its cancellation watcher. Flush must not wait for a peer to drain.

Tool arguments select exactly one of `question`, `question_file`, `question_name` and `question_reference`. Inline
strings are literal text even when they start with `@`. Structured questions
use ordinary native question/set/plan JSON grammar and preserve authored order
and descriptions. `question_file` names an ordinary existing file, never a
creation request. `question_name` explicitly selects a native named question.
`question_reference` explicitly invokes native @ reference resolution with working-directory path precedence. Evidence, records, source and inputs are exclusive. Source is
`{paths:[...],unit:"line"|"window"|"file",window?,media?}` and follows the
[native reader](files.md). Original value types and physical locations survive.
Image source requires file units. Ordered explicit `images` attachments may
accompany evidence, retain duplicates, and may not mix with records/source or
field pointers. Only decide/choose/score admit images; others refuse before
file content or transport. There is no network fetching or implicit discovery.

`inputs` is a finite array of explicit native descriptors. Each descriptor supplies exactly one `text`, `json` (including JSON null), or `source`, or image-only `images`. Its source uses whole-file text reading and must yield exactly one item. Optional `context` and replacement choose `options` stay separate from evidence. Explicit descriptor context/options conflict with their corresponding projection pointers. Absent context uses shared fallback; empty text suppresses it; null or a mismatched declared type refuses natively. Existing arbitrary `records` objects retain their original meaning.

Descriptor `images` preserves order and duplicates. An entry is either a path string with detected media or `{path,media}` declaring `image/png` or `image/jpeg`; native pixel admission checks the declaration against the actual bytes. Images may accompany selected text/JSON originals, including field projections; image-only originals have no projection pointers. Other seven functions refuse before content reads. Finite descriptor arrays admit every item before execution. Large captions whose escaped JSON exceeds the incoming frame use explicit native text files; decoded native and vendor body limits still apply.

Applicable options retain native validation, readings, separate shared/record
context, candidate pointers, model selectors, batch, deadline/admission limits,
rank top, find none and source filter files-only semantics. Supplied `options.proxy` is reserved and always refuses through native admission. Unknown application
arguments fail without echoing the payload. Complete input composition follows
native semantic owners rather than introducing a second reader or scheduler.

Successes return `structuredContent` holding the complete native call object
and one text content block encoding the equivalent JSON. Declared output
schemas come from the settled native complete serializers. False, null, empty
answers and partial member failures retain native meanings. Incremental reader failures retain ordered native `completed` results beside their terminal safe error and final facts. Whole-call failures
use `isError:true` with safe native error/facts, including actual started
attempts; admission failures invent no started-call facts. No adapter changes
result/1 labels into result/2 or invents origins, IDs, timing, models or cache
hits. MCP supplies the closed `mcp` surface identity through native integration.

Installed consumer checks run through the shared surface/conformance ladder.
Only actual executed cells qualify support; remaining shared cells stay missing.
Paired local overhead is registered under the explicit stress entry point;
measurement remains pending. Future proxy latency remains unmeasured.

Primary protocol contracts: [stdio](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports),
[lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle),
[tools](https://modelcontextprotocol.io/specification/2025-11-25/server/tools),
[cancellation](https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/cancellation).
