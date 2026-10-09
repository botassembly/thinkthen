# 0512: Run the CLI and MCP through the public request path

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision e90ea2c2ad34616f3f1a7221ec9e07c8e76cf476, accept

## Outcome

The CLI and MCP admit and execute every judgment call through the same public Request path as the libraries. Their private judgment pipelines and duplicate semantic admission go. Callers see the same output, exit codes and protocol behavior.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). `cli/request.rs` admits through the public API, discards the result and runs a private pipeline. `mcp/admission.rs` and `mcp/request.rs` keep their own types and rules beside the public Request, although MCP already executes Request.
- Keeps: The CLI's host argument parser, output formats, pipes, pipe interruption, exit codes and help. MCP tool discovery, generated input schemas, framing, protocol errors, file authority and cancellation. The maintenance commands in `cli/cache.rs` and `cli/status.rs` stay on their private maintenance APIs because they sit outside Request's ten-function dispatch.
- Changes: Slices, in order:
  - MCP first: remove duplicate semantic admission and keep protocol rendering and host authority. Claim `crates/thinkthen/src/mcp/admission.rs`, `crates/thinkthen/src/mcp/request.rs`, `crates/thinkthen/src/mcp/inputs.rs`, `crates/thinkthen/src/mcp/tests/admission.rs`, `crates/thinkthen/src/mcp/tests/execution.rs` and `libraries/mcp/conformance.py`.
  - CLI: convert transport arguments into Request and use public execution for judgments. Derive CLI exit codes from public error kinds, keeping today's codes. Name the actual CLI files before each slice.
- Proof: Existing CLI and MCP outside-in cases pass unchanged. A check fails when production judgment execution or semantic admission in `cli/` or `mcp/` imports a private engine module. The check excludes the maintenance paths in `cli/cache.rs` and `cli/status.rs`.
- Defers: A machine-readable CLI error output is not part of this ticket and needs no ticket under the scope freeze. New public maintenance APIs are not needed. Shell completions go to an idea.

## Progress

- 2026-10-09 started
