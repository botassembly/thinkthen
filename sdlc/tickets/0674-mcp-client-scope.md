# 0674: Mark the MCP client as a test client

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0639, 0688

## Outcome

- Changes: `libraries/mcp/client.py`, `libraries/mcp/tests/`, `libraries/mcp/test_client.py`, `libraries/mcp/test_installed.py`, `libraries/mcp/installed.py`, `libraries/mcp/conformance.py`, `libraries/mcp/check.sh`, `libraries/mcp/README.md`.
- `client.py` moves to `libraries/mcp/tests/` and is named as a test client, together with the scripts that import it.
- The README's "Try a saved answer" example and the text that presents `client.py` as a consumer library are deleted. The README keeps the launch entry and points agent builders at their own MCP client.
- The strict-JSON rechecks the server already guarantees (`_json`, `_same`) are deleted from the client.
- The 5 ms sleep loop in the cancellation test is replaced by waiting on the process or stream event.
- The FIFO and stdin cases run on macOS too, or the test states why they cannot.

### Proof

- On the M5 through `pm job run --machine m5 0674 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && cargo build --locked --offline --bin thinkthen && python3 libraries/mcp/tests/test_client.py && python3 libraries/mcp/tests/test_installed.py target/debug/thinkthen target/debug/conformance-backend'`, the installed-binary MCP tests drive the built `thinkthen` over stdio with tool discovery, errors and cancellation intact, and the FIFO and stdin cases run. On current main those cases are skipped on macOS.
- `git grep -n 'time.sleep' libraries/mcp` finds nothing; on current main it finds the loop in `test_client.py`.
- The arm64 container run is part of the 0619 container checkpoint.

## Evidence

- **Unpackaged client.** The MCP server is Rust that delegates to Request (2092361d6, 1b303b912); 0639 moves it into `crates/thinkthen-cli/src/mcp/` and onto the Rust run function. `libraries/mcp/client.py` is an unpackaged client imported through `PYTHONPATH=libraries/mcp`, which the README teaches as a consumer library.
- **One call at a time.** `Client` serializes calls with `_call_lock`.
- **Duplicated checks.** `_json` refuses duplicate members and nonfinite numbers, and `_same` compares structured and text results, which the server already guarantees.
- **Linux-only cases.** `test_installed.py` skips the FIFO and stdin cases unless `sys.platform` is Linux.
- **Sleep loop.** `test_client.py` polls `client.pending_id` with `time.sleep(0.005)`.
- `libraries/mcp/check.sh` runs these scripts by path; `sdlc/surfaces.txt` lists MCP as planned, so `surfaces` does not run it. `VERSION` in `client.py` is the MCP protocol revision, not a ThinkThen format version, and stays.
