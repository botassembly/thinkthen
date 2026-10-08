# Owned MCP input reproductions

Run from the repository root on Linux after building the native command:

```sh
python3 specification/fixtures/mcp-input-bounds/reproduce.py target/debug/thinkthen
```

This fixture reproduces the unbounded aggregate retention and nonregular selector failures motivating [proposed ADR 0128](../../../sdlc/planning/adr/0128-mcp-retained-attachment-bound.md). It checks the existing failures and is not a passing regression for the proposed contract. Replace its failure assertions with behavioral regressions after ADR acceptance; do not add it to a gate as proof of corrected behavior.

The fixture creates its own synthetic valid PNG, home/config/state/cache directories, FIFO and loopback listener in a temporary directory. It repeats the same image path across separate image-only descriptors under the default unlimited record setting, then supplies a missing final path. It reports actual frame bytes, aggregate original bytes and Linux process high-water memory. Reaching the final missing file proves that the earlier rows were composed; memory observations illustrate retained buffers without claiming a process-memory limit. No external address receives a request and no key is read from the caller's environment. The fake fixture key exists only in the child's environment.

FIFO and stdin selectors run in separate owned MCP children. Ping response and busy refusal describe protocol responsiveness separately from tool completion. The stdin file reader races the protocol reader; it can consume or split the next framed ping, so this observation is reported separately. Cancellation followed by EOF checks joining. The fixture kills and reaps each remaining child before removing its own files. The backend listener must observe zero connections. All waits have deadlines; this fixture makes no Windows device-path claim.
