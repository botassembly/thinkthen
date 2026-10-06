# Local MCP SDK

Status: **Settled** target for 0.2 by Ian's 2026-10-06 ruling and accepted
[ticket 0455](../sdlc/tickets/0455-mcp-ten-function-surface.md). Implementation
is WIP: the current command does not expose an incomplete server. Native
result/2 integration precedes installed support or parity claims.

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
exclusively complete JSON-RPC lines and at most 64 MiB per message. An output
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

Tool arguments select exactly one of `question` and `question_file`. Inline
strings are literal text even when they start with `@`. Structured questions
use ordinary native question/set/plan JSON grammar and preserve authored order
and descriptions. `question_file` names an ordinary existing file, never a
creation request. Evidence, records and source are exclusive. Source is
`{paths:[...],unit:"line"|"window"|"file",window?,media?}` and follows the
[native reader](files.md). Original value types and physical locations survive.
Image source requires file units. Ordered explicit `images` attachments may
accompany evidence, retain duplicates, and may not mix with records/source or
field pointers. Only decide/choose/score admit images; others refuse before
file content or transport. There is no network fetching or implicit discovery.

Applicable options retain native validation, readings, separate shared/record
context, candidate pointers, model selectors, batch, deadline/admission limits,
rank top, find none and source filter files-only semantics. Unknown application
arguments fail without echoing the payload. Complete input composition follows
native semantic owners rather than introducing a second reader or scheduler.

Successes return `structuredContent` holding the complete native call object
and one text content block encoding the equivalent JSON. Declared output
schemas come from the settled native complete serializers. False, null, empty
answers and partial member failures retain native meanings. Whole-call failures
use `isError:true` with safe native error/facts, including actual started
attempts; admission failures invent no started-call facts. No adapter changes
result/1 labels into result/2 or invents origins, IDs, timing, models or cache
hits. MCP supplies the closed `mcp` surface identity through native integration.

Installed consumer checks run through the shared surface/conformance ladder.
Missing complete execution remains failing/missing. Paired local overhead
measurement waits for complete dispatch and runs only through the explicit
stress entry point. Future proxy latency remains unmeasured.

Primary protocol contracts: [stdio](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports),
[lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle),
[tools](https://modelcontextprotocol.io/specification/2025-11-25/server/tools),
[cancellation](https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/cancellation).
