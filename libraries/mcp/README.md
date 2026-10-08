# ThinkThen local MCP

The development command exposes `thinkthen mcp` over local stdio with exactly
`decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`,
`recognize` and `relate`. It uses one environment-resolved native Engine and its
default persistent cache. No additional package, daemon or publication is made.
The reviewed local MCP implementation and installed 251-case parity passed before
landing. Bounded timing, final platform qualification and release rehearsal remain
required. Public installation remains 0.1.2 and does not provide this 0.2 command.

After 0.2 is published, install the ordinary command:

```sh
cargo install thinkthen --version 0.2.0 --locked
```

A client launches the command directly:

```json
{"command":"thinkthen","args":["mcp"]}
```

Startup accepts the existing backend/URL, model, profile, timeout, retry, jobs,
request/admission limits, cache/no-cache/refresh-cache, record and strict replay settings.
Keys come only from the approved environment variables. A tool cannot change
the route or key. The server neither invokes a shell nor starts a process per
call. Count-only usage follows the native engine.

`client.py` is a dependency-free consumer with ten explicit methods. It owns one
subprocess, checks matching JSON-RPC IDs and structured/text equivalence, retains
false/null and partial failures, and raises `ToolError` with the native complete
error/facts object. `pending_id` and `cancel(id)` let another thread cancel held
work. Cancellation wakes the caller with `CancelledError`; the session can
continue after the native attempt joins. Closing sends EOF and waits for the
owned subprocess. Stdout contains only JSON-RPC.

For a reviewed development checkout, run with `PYTHONPATH=libraries/mcp` from
the repository root. This no-key example uses the demo's exact saved answer:

```python
from client import Client
from pathlib import Path
with Client.launch(("thinkthen", "mcp", "--replay",
                    "demos/01-refund-gate/recording", "--no-cache")) as client:
    call = client.decide(question="Does the customer ask for money back?",
                         evidence=Path("demos/01-refund-gate/message.txt").read_text())
    print(call["value"]["value"])
```

Use inline literal text, including `@` prefixes, or the ordinary native JSON
question/set/plan grammar as `question`. `question_file` explicitly loads an
ordinary existing file. `question_name` loads a validated name from the native
config questions folder. `question_reference` invokes explicit native @ resolution with working-directory precedence. These four selectors are exclusive. The agent writes
its own question files; the server creates none. Authored question names,
wording versions, declarations, descriptions and ordering remain native.

Evidence, original JSON `records`, explicit `source` and finite `inputs` are exclusive. Source
is `{paths:[...],unit:"line"|"window"|"file",window?,media?}`; it uses the native
reader and composition, with folder order and repeated paths preserved. No path
silently becomes evidence. Explicit `images` are ordered PNG/JPEG attachments,
including duplicates, optionally beside text evidence. Only decide/choose/score
admit images. Image source requires file units. Seven other functions refuse
images before reading files or sending.

Explicit `inputs` descriptors carry `text`, original `json`, or a whole-file text `source`, optional separate `context`, replacement choose `options`, and ordered `images`. An image-only descriptor can omit the original. Images accept path strings or `{path,media}` declarations validated by native decoding. Field pointers can select a text/JSON original with ancillary images; image-only sources refuse pointers. Explicit context/options conflict with their corresponding pointers. The finite array is admitted before sends; arbitrary objects in existing `records` keep their settled meaning.

Incoming frames remain 16 MiB including newline. File-backed captions use the native reader when JSON escaping would exceed this limit. Outgoing frames, including newline and both equivalent complete representations, are bounded at 192 MiB in the server and client. This increases peak memory. Native decoded-input and vendor limits still apply; output overflow closes without a partial line after any work already performed.

Per-call `options` admits model, threshold, shared context, field or ordered
field list, per-item context/candidate pointers, batch (integer or `max`), rank top, find none,
filter files-only, deadline, attempt capture and send limits. The deadline starts
before preparation and remains shared through execution. Literal evidence never
becomes parsed JSON to satisfy an authored pointer or declaration. Structural
selection requires explicit records or native source composition.

`options.cancelled` is a boolean initial native call cancellation state, defaulting
to false. True cancels the call's native token before admission and returns the
native cancelled error with zero sends. Notification cancellation continues to
cancel active requests and retire their owned work.

Successes return unchanged native complete call envelopes and the packaged native
output schema. Safe errors use `Error::complete()`, including joined started-call
facts. Incremental failures additionally retain actual ordered `completed` results. Admission failures invent no call facts. No adapter rewrites schemas,
origins, IDs, timing, models or cache hits. Native calls stamp the closed `mcp`
surface and compiled-engine User-Agent, call ID and request ID.

Run the independent client fixtures with `python3 libraries/mcp/test_client.py`.
`installed.py ABSOLUTE_BINARY` initializes, pings, reads the ten-tool catalog and
makes an actual recorded decide call. Release smoke runs it against the unpacked
executable and checks its existing backend counter for zero sends.
`conformance.py LOOPBACK_PORT ABSOLUTE_BINARY` runs the entire required MCP
inventory, currently 251 entries, through named public methods. Ordered rank-set
members retain their actual native child results, authors, identities, probabilities
and metadata. Call token totals come from native call facts. It reuses the
canonical fixture projection and independent assertions, isolates request counts
and body capture per case, and fails for missing native fields or unsupported
inputs. It accepts no case selector. `test_installed.py ABSOLUTE_BINARY ABSOLUTE_BACKEND`
uses an owned loopback backend and scratch home for names/files, sources,
controls, secrecy, cache/replay, cancellation and EOF. Required cells beyond the
executed consumer remain failed in the shared parity runner; fixture success
never qualifies parity. Final qualification also requires the owning shared parity
run, bounded timing and real Windows checks.

Unix uses owned unbuffered descriptors and bounded polling. Windows uses owned
pipe handles with a joined cancellation watcher. Each synchronous operation
registers its actual thread; cancellation cannot target unrelated native I/O.
Flush does not wait for a peer to drain. The exact Windows-only unsafe leaf is
`src/mcp/input/windows/ffi.rs`; the existing `windows-sys` package adds only its
required `Win32_System_IO` feature. Windows compilation and pipe regressions
remain pending on the existing runner. Linux checks do not qualify Windows.

## Landing work

Native complete results retain optional typed `source` for explicit physical
inputs. Dynamic choose retains authored non-root `on` selection through native
admission. Explicit MCP model controls use `with_model_override`; the native
`model` builder continues to refuse a duplicate saved model. Context declarations
still pass directly to native `RecordReading`. These paths use the packaged
native schema and need no host question parser or result-field fabrication.

- Root owns the fresh whole High review, full tests/lint, affected installed/spec/
  surface checks, complete shared parity and final real Windows qualification.
  Keep the one short ticket record at landing, release rehearsal and Ian's
  publication approval. C/family/shared parity declarations stay with their owners.
- The bounded paired native/installed MCP timing case is registered under the
  existing explicit `test-stress --run` entry point. Atomic, record and whole-set
  samples report payload bytes, counted requests, cold startup, warm median,
  exploratory p95 and maximum. Measurement remains pending; future proxy latency
  remains unmeasured. The current entry point runs all load campaigns and surface
  stress checks together; a bounded MCP builder handoff leaves that run to root.

Ian can overturn the local MCP contract and measurement plan. A pushed ticket
checkpoint is not a main landing, publication or complete support claim.
