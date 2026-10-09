# 0493: Derive MCP schemas from the shared request contract

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision 236db58831f6a81d8baef4b50f7d9a97e76417d6, accept

## Outcome

Derive MCP tool input schemas and ordinary admission from the shared Request contract.

## Evidence

- Starts from: PM architecture asks2/5; mcp/tools.rs duplicates admission rules.
- Keeps: Tool names, framed stdio, IDs, cancellation, local file authority and protocol-specific resource limits.
- Changes: Map Request variants into advertised tools and native admission. Depends on0491; coordinate0464,0481,0488. Claim `crates/thinkthen/src/mcp/**` and installed MCP fixtures.
- Proof: Schema combinations match runtime refusals and valid typed calls; malformed input sends nothing; existing installed MCP suite remains green.
- Defers: HTTP services and new tools. Size: medium surface migration.

## Reviewed transport admission

The design at `236db58831f6a81d8baef4b50f7d9a97e76417d6` retains ADR 0128 while moving execution into Request. Use a crate-private transport attachment limit during admission and composition. Unresolved file descriptors enter the shared composer before attachment reads; already charged immutable handles are not charged again. Preserve attachment-first ordering, finite zero-send admission, source prefixes, locations and ordinary native defaults. Derive advertised structural schemas and semantic restrictions from the shared contract rather than introducing another MCP applicability table.

After 0499 relinquishes its shared writer, this change may edit `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/admission.rs`, `crates/thinkthen/src/public/request/composition.rs`, `crates/thinkthen/src/public/request/execution.rs`, a private `crates/thinkthen/src/public/request/transport.rs`, and `crates/thinkthen/src/public/request/inline.rs` only if needed for the preflight. The existing bounded reader in `crates/thinkthen/src/public/input_files.rs` may lose its CLI-only guard if library-only compilation requires it. Add no canonical transport setting or cache identity field. Keep implementation and its actual API inventory together; do not publish future exports ahead of code.

The installed shared cases also require `crates/thinkthen/src/public/request/definition.rs` to expose a crate-private entry point into the existing authored-question parser. Reuse its safe unsupported-feature diagnostic for inline declarations and saved selectors, retaining the saved-definition error class. Do not expose raw deserializer errors or change the generated schema. Annotation document composition must apply the existing selection and admission rules through the shared path.

## Progress

- 2026-10-08 started
