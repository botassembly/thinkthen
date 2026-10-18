# 0674: Mark the MCP client as a test client

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0639, 0688

## Outcome

- `client.py` is moved under the MCP tests and named as a test client.
- Any README text that presents it as a consumer library is deleted.
- Strict-JSON rechecks the server already guarantees are deleted from the client.
- The sleep loop is replaced by waiting on the process or stream event.
- The FIFO and stdin cases run on macOS too, or the reason they cannot is stated in the test.

### Proof

On the M5 through pm job, the installed-binary MCP tests (`installed.py`, `test_installed.py`, `conformance.py`) drive the extracted `thinkthen` binary over stdio. They pass natively on the M5 and in the pinned arm64 container, with tool discovery, errors and cancellation intact.

## Evidence

- **Unpackaged client.** The MCP server is Rust that delegates to Request (2092361d6, 1b303b912); 0639 moves it onto the Rust run function. `libraries/mcp/client.py` is an unpackaged client imported through `PYTHONPATH=libraries/mcp`.
- **One call at a time.** `_call_lock` serializes calls (`client.py:134`).
- **Duplicated checks.** The client re-checks strict JSON and compares structured and text results (`client.py:17-41`).
- **Linux-only cases.** FIFO and stdin cases run only on Linux (`test_installed.py:116`).
- **Sleep loop.** `test_client.py:168` sleeps 5 ms in a loop.
