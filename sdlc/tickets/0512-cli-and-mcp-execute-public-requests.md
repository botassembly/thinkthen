# 0512: Run the CLI and MCP through the public request path

Status: OPEN.

Milestone: 0.2

Depends on: 0511

## Outcome

The CLI and MCP admit and execute every call through the same public request path as the libraries. Their private pipelines and their own argument types go.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). `cli/request.rs` admits through the public API, discards the result and runs a private pipeline; the CLI makes about 391 references to private engine modules. `mcp/admission.rs` and `mcp/request.rs` keep their own types and rules beside the public Request.
- Keeps: CLI output formats, exit codes, help, MCP framing, file authority and cancellation.
- Changes: Map CLI arguments and MCP tool input into the public Request and execute it. Derive CLI exit codes from public error kinds. Add a machine-readable CLI error output. Claim `crates/thinkthen/src/cli/**` and `libraries/mcp/**` in narrowed slices.
- Proof: Existing CLI and MCP outside-in cases pass unchanged. A check fails when `cli/` or `mcp/` imports a private engine module.
- Defers: New CLI features. Shell completions go to an idea.
