# ThinkThen local MCP

The development command exposes `thinkthen mcp` over local stdio with exactly
`decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`,
`recognize` and `relate`. It uses one environment-resolved native Engine and its
default persistent cache. No additional package, daemon or publication is made.
Full 0455 acceptance remains open: fresh whole MCP review, landing gates,
complete shared parity, timing and final Windows qualification are still required.

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
config questions folder. These three selectors are exclusive. The agent writes
its own question files; the server creates none. Authored question names,
wording versions, declarations, descriptions and ordering remain native.

Evidence, original JSON `records` and an explicit `source` are exclusive. Source
is `{paths:[...],unit:"line"|"window"|"file",window?,media?}`; it uses the native
reader and composition, with folder order and repeated paths preserved. No path
silently becomes evidence. Explicit `images` are ordered PNG/JPEG attachments,
including duplicates, optionally beside text evidence. Only decide/choose/score
admit images. Image source requires file units. Seven other functions refuse
images before reading files or sending.

Per-call `options` admits model, threshold, shared context, field or ordered
field list, per-item context/candidate pointers, batch (integer or `max`), rank top, find none,
filter files-only, deadline, attempt capture and send limits. The deadline starts
before preparation and remains shared through execution. Literal evidence never
becomes parsed JSON to satisfy an authored pointer or declaration. Structural
selection requires explicit records or native source composition.

Successes return unchanged native complete call envelopes and the packaged native
output schema. Safe errors use `Error::complete()`, including joined started-call
facts. Admission failures invent no call facts. No adapter rewrites schemas,
origins, IDs, timing, models or cache hits. Native calls stamp the closed `mcp`
surface and compiled-engine User-Agent, call ID and request ID.

Run the independent client fixtures with `python3 libraries/mcp/test_client.py`.
`installed.py ABSOLUTE_BINARY` initializes, pings, reads the ten-tool catalog and
makes an actual recorded decide call. Release smoke runs it against the unpacked
executable and checks its existing backend counter for zero sends.
`conformance.py LOOPBACK_PORT ABSOLUTE_BINARY` executes 25 shared behavior cases
through named public methods. `test_installed.py ABSOLUTE_BINARY ABSOLUTE_BACKEND`
uses an owned loopback backend and scratch home for names/files, sources,
controls, secrecy, cache/replay, cancellation and EOF. Required cells beyond the
executed consumer remain missing in the shared parity runner; fixture success
never qualifies parity.

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
