# 0493: Derive MCP schemas from the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Derive MCP tool input schemas and ordinary admission from the shared Request contract.

## Evidence

- Starts from: PM architecture asks2/5; mcp/tools.rs duplicates admission rules.
- Keeps: Tool names, framed stdio, IDs, cancellation, local file authority and protocol-specific resource limits.
- Changes: Map Request variants into advertised tools and native admission. Depends on0491; coordinate0464,0481,0488. Claim `crates/thinkthen/src/mcp/**` and installed MCP fixtures.
- Proof: Schema combinations match runtime refusals and valid typed calls; malformed input sends nothing; existing installed MCP suite remains green.
- Defers: HTTP services and new tools. Size: medium surface migration.

## Progress

- 2026-10-08 started
