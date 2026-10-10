# 0487: Isolate child test environments through shared helpers

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision f1a49f8bc78b16bd499e1f22e052226a4e88ca0c, accept

Reviews: revision 8a695108fae973bb675f1c4bcc72d2609e5ec860, accept

Reviews: revision 86b65620c, accept

Reviews: revision 4e0ef1545185c397c8a645563ba0a08c4f102d34, accept

Reviews: revision 18c72234b644b6c8a87ba40be101c578a5ea43a6, accept

Reviews: revision 334204898a22e395abbd909292ea48332c586d1a, accept

Reviews: revision 6ee0992101afb3ddbe111baee7b0d12a0464119a, accept

## Outcome

Tests use one child-environment entry point per language, preserving intentional environment cases while preventing accidental inherited configuration.

## Evidence

- Starts from: architecture/test-sediment PM message of 2026-10-08, ask 2; conformance/children/children.py already supplies a Python helper, while native Rust and several language runners build separate maps or mutate XDG values afterward. The reported 113-file count has not been independently verified. 0471 separately fixes executable spec/demo isolation.
- Keeps: Named-question configuration, explicit test overrides, Windows path/APPDATA behavior, Cargo/toolchain caches, installed package fixtures and all distinct secrecy/error cases.
- Changes: Inventory direct child-environment writes, reuse one existing helper per language and migrate same-language callers in a coherent family pass. Add a narrow lint rejecting direct XDG/APPDATA writes in ordinary test callers; permit only the actual helper and explicit environment-behavior/container fixtures with a documented reason. Avoid marker-based escape hatches and a new test runner. Settle helper semantics before wide edits, with owned fake configurations only.
  Claim `sdlc/scripts/**`, `crates/thinkthen/tests/**`, `crates/thinkthen/src/mcp/tests/**`, `libraries/**/tests/**` and `databases/**/tests/**`.
- Proof: Representative named-question, cache and installed children preserve expected behavior without ambient configuration. One planted direct write in the ordinary scope fails lint; legitimate environment behavior cases still run. Report actual migration scope and retain shared cases.
- Defers: Broader architecture redesign. This is approved 0.2 work; do not silently defer it based on lane availability. Coordinate source claims and keep must-fix product defects first.

## Progress

- 2026-10-08 started
- 2026-10-08 landed 98a876029; next: Slice B unifies Python child homes. Full routine Python tests, malformed ambient configuration cases, examples and the existing child check pass after rebuilding the matching extension. Remaining Ruby, JavaScript, R, native and shared fixture families stay open; migrate them without colliding with the contract rollout.
- 2026-10-08 landed 3f64dbca49b46c80585e13b2406f56834bd42283; next: Slice C unifies Ruby child homes and preserves installed GEM_PATH. Scoped environment, package, settings, public-name, token-cap and example checks pass. Two recognition failures reproduce with the original helper and remain assigned to 0497. JavaScript, R, native and shared-fixture migrations remain open.
- 2026-10-09 landed c0338e67e; next: JavaScript child-home isolation landed and the combined lint, test and spec pass. Finish R, native and shared-fixture environment isolation; retain installed package paths and existing request-count cases.
- 2026-10-10 landed 433abe142; next: The outer R wrapper now isolates credentials, settings and startup configuration with the existing child helper. Verify remaining declared fixture boundaries and close the implemented environment scope; installed parity stays separate.
- 2026-10-10 landed 63b3aa677; next: CLI, C and native Python fixture children use the shared isolated environment helper. Finish remaining ordinary JVM, C# and installed-consumer child maps; full installed qualification remains separate.
