# 0487: Isolate child test environments through shared helpers

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision f1a49f8bc78b16bd499e1f22e052226a4e88ca0c, accept

## Outcome

Tests use one child-environment entry point per language, preserving intentional environment cases while preventing accidental inherited configuration.

## Evidence

- Starts from: architecture/test-sediment PM message of 2026-10-08, ask 2; conformance/children/children.py already supplies a Python helper, while native Rust and several language runners build separate maps or mutate XDG values afterward. The reported 113-file count has not been independently verified. 0471 separately fixes executable spec/demo isolation.
- Keeps: Named-question configuration, explicit test overrides, Windows path/APPDATA behavior, Cargo/toolchain caches, installed package fixtures and all distinct secrecy/error cases.
- Changes: Inventory direct child-environment writes, reuse one existing helper per language and migrate same-language callers in a coherent family pass. Add a narrow lint rejecting direct XDG/APPDATA writes in ordinary test callers; permit only the actual helper and explicit environment-behavior/container fixtures with a documented reason. Avoid marker-based escape hatches and a new test runner. Settle helper semantics before wide edits, with owned fake configurations only.
  Claim `sdlc/scripts/**`, `crates/thinkthen/tests/**`, `crates/thinkthen/src/mcp/tests/**`, `libraries/**/tests/**` and `databases/**/tests/**`.
- Proof: Representative named-question, cache and installed children preserve expected behavior without ambient configuration. One planted direct write in the ordinary scope fails lint; legitimate environment behavior cases still run. Report actual migration scope and retain shared cases.
- Defers: Broader architecture redesign. This is approved 0.2 work; do not silently defer it based on lane availability. Coordinate source claims and keep must-fix product defects first.
