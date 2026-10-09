# 0512: Run the CLI and MCP through the public request path

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

The review amendment below governs the initial slice and supersedes the undefined machine-readable CLI error feature.

The CLI and MCP admit and execute every call through the same public request path as the libraries. Their private pipelines and their own argument types go.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). `cli/request.rs` admits through the public API, discards the result and runs a private pipeline; the CLI makes about 391 references to private engine modules. `mcp/admission.rs` and `mcp/request.rs` keep their own types and rules beside the public Request.
- Keeps: CLI output formats, exit codes, help, MCP framing, file authority and cancellation.
- Changes: Map CLI arguments and MCP tool input into the public Request and execute it. Derive CLI exit codes from public error kinds. Add a machine-readable CLI error output. Claim `crates/thinkthen/src/cli/**` and `libraries/mcp/**` in narrowed slices.
- Proof: Existing CLI and MCP outside-in cases pass unchanged. A check fails when `cli/` or `mcp/` imports a private engine module.
- Defers: New CLI features. Shell completions go to an idea.

## Review amendment

CLI converts transport arguments into Request and uses public execution instead of private judgment pipelines; its host argument parser remains. MCP already executes Request. Remove its duplicate semantic admission while preserving protocol rendering and host authority. Do not add a new machine-readable error feature in this ticket.

The first MCP slice claims `crates/thinkthen/src/mcp/admission.rs`, `crates/thinkthen/src/mcp/request.rs`, `crates/thinkthen/src/mcp/inputs.rs`, `crates/thinkthen/src/mcp/tests/admission.rs`, `crates/thinkthen/src/mcp/tests/execution.rs` and `libraries/mcp/conformance.py`. Name actual CLI files per later slice. This amendment supersedes the whole-folder claim and private argument-type removal above.
