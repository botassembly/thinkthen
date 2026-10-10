# 0512: Run the CLI and MCP through the public request path

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision e90ea2c2ad34616f3f1a7221ec9e07c8e76cf476, accept

Reviews: revision a54ef96da16d696b3f37e33093d0a66782c9c3b4, reject

Reviews: revision bf05f50d7c873d690dc5537ce39e48b8e821bd29, reject

Reviews: revision 3fcbfae5dde00ddf1778c89467410663fb002ec0, accept

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
- 2026-10-09 landed 760ff5b25; next: MCP now admits controls through Request owners and its installed shared cases pass. Migrate CLI judgment execution next; preserve output, pipe interruption and exit codes.
- 2026-10-09 landed 8144acf6424d9cc8fd27568b5c1dd47e66569c0b; next: Find executes admitted native Requests and retains saved-file diagnostics, deadlines, interruption, original JSON and accounting. Migrate the remaining nine CLI judgment commands and finish combined checks before whole closure.
- 2026-10-09 landed 8144acf6424d9cc8fd27568b5c1dd47e66569c0b; next: Find is landed; atomic migration is pushed as incomplete uncompiled WIP d76312353. Ian stopped stress-heavy testing because it affected the computer. Combined lint and all 1904 default-profile tests passed; library-only run was interrupted, so full checks are not complete. Resume light implementation and focused behavior checks; keep stress-heavy runs stopped.
- 2026-10-10 landed b2c652a62; next: CLI native execution and follow-up repairs for rank counts, document details, shared context, profile diagnostics, image-only records and literal relation errors are landed. The routine Rust run passes; changed reader and adapter checks pass locally. Finish contract adoption and documentation before closure; release-only checks remain held.
