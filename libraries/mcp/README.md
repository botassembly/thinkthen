# ThinkThen for MCP clients

Give an agent the ten ThinkThen functions as tools over local stdio. The server uses the same engine, settings and answer cache as the command line. Use Jev, TypeSafe's System One model, OpenAI or a local model through the command's [backend settings](https://thinkthen.dev/models/). Your agent gets typed answers, saved-answer reuse and offline replay without implementing provider calls itself.

## Launch the server

Install the [ThinkThen command](https://thinkthen.dev/start/) and put `thinkthen` on `PATH`. These examples use ThinkThen 0.2 or newer. Check that `thinkthen mcp --help` is available, then add this launch entry to your MCP client:

```json
{"command":"thinkthen","args":["mcp"]}
```

Startup accepts the command's backend, URL, model, profile, timeout, retry, concurrency, request limits, cache, record and replay settings. Run `thinkthen mcp --help` to see their spellings. Set the key in `THINKTHEN_API_KEY` or the selected backend's own key variable, as described in [Models](https://thinkthen.dev/models/). A tool cannot change the backend or key. The server never invokes a shell or starts a process per call.

The included [`client.py`](client.py) needs only Python's standard library. It provides a method for each judging function and owns one subprocess. It checks response IDs and the equivalent structured and text results.

## Try a saved answer

The Python client is included in this repository. From the repository root, run this example with `PYTHONPATH=libraries/mcp`. It uses the refund demo's saved answer and needs no key or network:

```python
from client import Client
from pathlib import Path
with Client.launch(("thinkthen", "mcp", "--replay",
                    "demos/01-refund-gate/recording", "--no-cache")) as client:
    call = client.decide(question="Does the customer ask for money back?",
                         evidence=Path("demos/01-refund-gate/message.txt").read_text())
    print(call["value"]["value"])  # True
```

## Discover tools and supply inputs

`tools/list` provides compact input guidance for all ten tools. It preserves caller field names and native choice values, omits output schemas, and leaves full validation to native admission. Run `thinkthen mcp --help` for the full grammar links. Discovery types describe writable fields; they do not replace native checks for exclusive selectors, reader controls or authored questions.

Use inline literal text, including `@` prefixes, or the ordinary native JSON question/set/plan grammar as `question`. `question_file` explicitly loads an ordinary existing file. `question_name` loads a validated name from the native config questions folder. `question_reference` invokes explicit native @ resolution with working-directory precedence. These four selectors are exclusive. The agent writes its own question files; the server creates none. Authored question names, wording versions, declarations, descriptions and ordering remain native.

Evidence, original JSON `records`, explicit `source` and finite `inputs` are exclusive. A `source` object takes ordered `paths`, optional `unit`, `window`, `media` and `framing` fields. The default unit is `line`; `window` requires window units, and image media requires file units. It uses the native reader, with folder order and repeated paths preserved. See [source composition](../../specification/mcp.md). No path silently becomes evidence. Explicit `images` are ordered PNG/JPEG attachments, including duplicates, optionally beside text evidence. Only decide/choose/score admit images. Image source requires file units. Seven other functions refuse images before reading files or sending.

Explicit `inputs` descriptors carry `text`, original `json`, or a whole-file text `source`, optional separate `context`, replacement choose `options`, and ordered `images`. An image-only descriptor can omit the original. Images accept path strings or `{path,media}` declarations validated by native decoding. Field pointers can select a text/JSON original with ancillary images; image-only sources refuse pointers. Explicit context/options conflict with their corresponding pointers. The finite array is admitted before sends; arbitrary objects in existing `records` keep their settled meaning.

The [MCP specification](../../specification/mcp.md) defines frame, attachment and decoded-input bounds. Use an explicit text source file for captions that would exceed the incoming frame after JSON escaping. Both structured and text representations count toward the outgoing frame bound and increase peak memory. Output overflow closes the session without a partial JSON line; requests may already have been sent.

Per-call `options` admits model, threshold, shared context, field or ordered field list, per-item context/candidate pointers, batch (integer or `max`), rank top, find none, filter files-only, deadline, attempt capture and send limits. The deadline starts before preparation and remains shared through execution. Literal evidence never becomes parsed JSON to satisfy an authored pointer or declaration. Structural selection requires explicit records or native source composition.

`options.cancelled` is a boolean initial native call cancellation state, defaulting to false. True cancels the call's native token before admission and returns the native cancelled error with zero sends. Notification cancellation continues to cancel active requests and retire their owned work.

## Read answers and failures

A successful tool response has `structuredContent` containing the complete native call object and one text block containing the same JSON. With the included Python client, `call["value"]["value"]` reads a single decide answer; `call["facts"]` describes the call. Keep `false`, `null` and empty answers as answers. They do not mean the tool failed. See the [result contract](../../specification/result.md) for each function's complete result.

A failed tool response sets `isError: true` and retains the native error and any actual call facts. The Python client raises `ToolError` with that object. Incremental failures also retain ordered `completed` results; treat them as a completed prefix, not a finished dataset. The server preserves native source locations, identities, models, cache hits and per-record results.

Recognition accepts `options.context_field` and explicit descriptor context through the native record reader. Empty text clears shared context. Every selected record context is checked before recognition sends; each stage retains that record's context. See [per-record recognition context](../../specification/recognize.md#per-record-context).

## Close or cancel a call

The Python client's `pending_id` and `cancel(id)` let another thread cancel active work. Cancellation wakes the caller with `CancelledError`; the session can continue after the started native attempt joins. Wait for required responses before closing the client: closing sends EOF, cancels active work and waits for the owned subprocess. Cancellation cannot retract requests already sent. Standard output contains only JSON-RPC; diagnostics use standard error.

See the [MCP contract](../../specification/mcp.md) for question grammar, input composition, protocol lifecycle and cancellation, and the [agent skill](../../skills/thinkthen/SKILL.md) for function selection and bounded requests.
