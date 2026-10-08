# 0491: Define one typed request contract for all functions

Status: OPEN.

Milestone: 0.2

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

## Outcome

Define one versioned Rust Request contract for all ten functions and native execution; generate specification/request.schema.json and parse thinkthen_call through a compatibility translation.

## Evidence

- Reviewed design: [ADR 0125](../planning/adr/0125-one-request-contract-and-native-admission.md) defines the versioned Request, legacy C translation, shared CLI/native admission and offline schema generation. Fresh design review corrected the legacy repeated-key and recognize-record refusal boundary before accepting it.

- Starts from: PM binding architecture asks 2 and 5; libraries/c/src/call.rs and MCP independently parse envelopes.
- Keeps: Frozen 0.1 C symbols and accepted thinkthen_call grammar, named typed methods, current result contract, pure core and one endpoint.
- Changes: Review an ADR for tagged inputs, closed request objects, optional controls and absent/null semantics. Serde/schemars derive the schema from one edge-owned type; native callers use that type without JSON round trips. Claim `crates/thinkthen/src/public/**`, shared admission, `libraries/c/src/call.rs`, schema generation and `specification/request.schema.json`. Depends on0489; coordinate0468 facts before readers settle.
  CLI commands construct the same typed Request and use shared admission before file reads or model calls; no CLI JSON round trip or second validation implementation. Claim `crates/thinkthen/src/cli/**` alongside native/C admission. Preserve valid command behavior and legacy flag spellings. The PM's corrected order starts this ticket alongside 0489's final checks and the binding experiment; integrate 0489 before the final contract landing.
- Proof: Legacy/canonical requests and valid CLI commands agree; CLI and native invalid selectors, unknown input members and invalid media refuse before reads/sends. Count file-reader access and loopback sends on invalid CLI cases. Shared typed cases and committed schema regeneration detect drift.
- Defers: Proxy policy, model routing and replacing public named methods with raw JSON. Size: large shared execution change.
