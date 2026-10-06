# ThinkThen local MCP

Ticket 0455 is **WIP**. The private Rust protocol/admission implementation and
this named stdio consumer exist. `thinkthen mcp` is deliberately unexposed until
the shared native complete execution and result/2 serializers are adopted.
`check.sh` fails against the current command; protocol fixtures are not parity.
There is no published additional package or running daemon.

After the completed 0.2 command is published, install the ordinary command:

```sh
cargo install thinkthen --version 0.2.0 --locked
```

A client launches the command directly:

```json
{"command":"thinkthen","args":["mcp"]}
```

Startup will retain the admitted CLI spellings for backend/URL, model, timeout,
retry and process admission limits, cache/no-cache, record and strict replay.
Keys come only from the approved environment variables, never a tool argument.
The server owns one `EngineBuilder::from_env()` engine with its default cache.
Do not select `shared_host()`, invoke a shell or start a command per tool call.

`client.py` is a dependency-free consumer with explicit `decide`, `choose`,
`tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`
methods. It initializes one subprocess, checks matching JSON-RPC IDs and
structured/text equivalence, retains false/null and partial failures, and
raises `ToolError` with the native error/facts object on a failed call.
`pending_id` and `cancel(id)` let another thread cancel a held call; cancellation
wakes the caller with `CancelledError`, and the session can continue. Closing
cancels through EOF and waits for this owned subprocess alone.

Pass inline literal text, including `@` prefixes, or the existing ordinary JSON
question/set/plan grammar as `question`. Use `question_file` for an explicit
ordinary file. They are exclusive. Choose/tag/score descriptions and ordering
use the shared JSON question grammar. Evidence, original typed records and an
explicit `source` are exclusive. Source paths use the native line/window/file
reader; locations are separate carriers. Explicit `images` are ordered file
attachments (duplicates survive), optionally beside text evidence. Only
`decide`, `choose` and `score` accept images. Image folders use source media
image and whole-file units. No method creates question files or acts on answers.

Run the independent client fixtures with `python3 libraries/mcp/test_client.py`.
The existing surface consumer is `sh libraries/mcp/check.sh LOOPBACK_PORT`.
`installed.py ABSOLUTE_BINARY` performs initialization, ping, the ten-tool
catalog and an actual recorded decide call through an installed executable.
`conformance.py LOOPBACK_PORT ABSOLUTE_BINARY` makes actual named calls for ten
representative shared behavior cases, including false/null, record ordering,
full-set selection, annotations, spans and edges. It checks native complete
results and emits only those executed behavior cells. Remaining required cells
stay missing in the shared parity runner. No fixture test emits parity.

## Exact integration left for native adoption

1. Native lane0 owns the complete execution route and all result/2 serializers,
   observations, attempt/call facts, cache/replay origin, answer IDs and shared
   input composition. Add a concrete `runtime::Executor` over **one** resolved
   public Engine. Return the native complete-call object in `NativeReply.object`
   using `NativeObject::new(&native_complete_call)`, and the native failure
   decision in `failed`. Raw native serialization preserves authored map order;
   records likewise retain raw JSON through native composition. Its `output_schema()` must be
   the settled complete-call schema, including safe errors. The adapter wraps
   that object without projecting or inventing any native field. Admission
   errors use the existing safe native error shape with no invented facts.
2. `Invocation::question`, `source`, `attachments` and `controls` already use
   public native loaders/readers/validation. Full dispatch must select the
   native named methods and apply all admitted per-call model, reading, field,
   context/options pointers, batch, top and files-only controls. Fix the
   relative deadline once with `controls(token)?.started()` before preparation,
   then reuse it through the complete native call. Reuse shared
   source composition for all ten functions. Do not collect streaming sources
   into an adapter-owned scheduler or use JSON compatibility dispatch. Native
   helpers are still needed for saved decide/score rank preparation (including
   rejection of authored cuts), applicable find question-file preparation,
   record field/context/candidate composition and complete located aggregates.
   The current private preparation handles literal rank/find and ordinary
   atomic/set/recognize/relate loaders; it does not claim those missing paths.
3. **Native-owned `src/core/surface.rs` integration** (the file is not yet on
   this branch base): add `Mcp` to the closed
   Surface enum, map only exact `"mcp"` in its parser, return `"mcp"` from its
   spelling/serialization, and add the exact valid/unknown-token cases to the
   existing surface tests. The contract token and parity row are supplied here.
   Stamp MCP call/request identity through the same typed native setting as
   other surfaces before any request. Every actual send must use
   `User-Agent: thinkthen/<compiled-engine-semver> (mcp)` and the existing
   `X-ThinkThen-Call-Id` / `X-ThinkThen-Request-Id` values. Reject `MCP`,
   `mcp ` and other unlisted tokens locally; do not infer surface from URL.
   Request bodies and cache/answer identities remain native. MCP adds no
   identity generator or hash.
4. Once the above is complete, add `Command::Mcp` and applicable startup options
   in `cli/args/command.rs`, and dispatch in `cli/mod.rs` before ordinary judging
   environment/reader/interrupt setup. The module is already `cli`-gated in
   `lib.rs`. On Unix obtain unbuffered owned handles with `input::pipes()`;
   use `BufReader::new(input::PollInput::new(input_file, stop))` as the `serve`
   reader factory and `input::PollOutput::new(output_file, stop)` as its writer
   factory. Do not use a global buffered stdout handle. Neither protocol input nor output can block shutdown.
   `serve` requires stoppable reads and joins both
   reader and dispatch on EOF/output failure. Windows needs an existing safe
   interruptible pipe reader; do not substitute an unjoinable blocking thread.
   All decoding/loading belongs on dispatch, and native iteration must check
   the token between inputs. Preserve native joined in-flight attempt behavior.
5. Move `libraries/mcp` from planned to landed only after actual installed and
   surface checks pass. Extend the consumer to the remaining 204-case parity
   inventory, full output-schema/type checks, complete source/image forms,
   secrecy, counted zero sends, error/cancellation and cache/replay cases after
   native shapes settle. The present ten-case consumer and protocol tests do
   not qualify those cells. Connect `installed.py` to `release-smoke`'s
   `command_check` using the unpacked executable path; check loopback count
   before/after strict replay with its existing backend counter.
6. Add the bounded paired native/MCP timing case only after complete dispatch,
   through existing `test-stress --run`. No latency has been measured or claimed.
   Root runs policy, one fresh whole High review, full tests/lint and affected
   spec/surface/install checks on its landing candidate. Keep the release
   rehearsal and Ian's publication approval. Write the one record at landing.

Ian can overturn the local MCP contract and measurement plan. This branch is
not a main landing, release or support claim.
