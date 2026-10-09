# MCP input admission fixtures

[ADR 0128](../../../sdlc/planning/adr/0128-mcp-retained-attachment-bound.md) cites the owned baseline reproduction committed at `20ca32e0532fc829a028120f8f7b77b4ff092d02`. It repeated a valid synthetic image across separate descriptors under default unlimited records, reached a later missing path after exceeding the framing ceiling, and observed zero backend connections. Separate owned FIFO and stdin children demonstrated an occupied worker and competing protocol reads. The fixture killed and reaped its children and removed its own files.

Behavioral regressions replace the standalone baseline reproduction. The existing [installed MCP suite](../../../libraries/mcp/test_installed.py) owns repeated image rows, refusal before later paths, zero sends, stable nonregular selector refusal, next framed tool completion and normal shutdown. Its existing named-file and ordered-image cases preserve named lookup, regular-file symlinks and duplicates. The [native MCP reader case](../../../crates/thinkthen/src/mcp/tests/input_bounds.rs) owns cumulative source admission, stopping before the tail, exact byte boundaries and preservation of the ordinary native reader ceiling. The existing question-file tests own shared loader and command error mapping. No additional runner is required.

Run the installed suite with the existing native command and offline backend:

```sh
python3 libraries/mcp/test_installed.py target/debug/thinkthen target/debug/conformance-backend
```

The installed suite supplies an owned home, fake environment-only key, temporary files and loopback backend. FIFO and stdin recovery cases run on Linux; they make no Windows device-path claim. These checks bound original attachment retention rather than total process memory and do not establish protection against path replacement between metadata and open.
