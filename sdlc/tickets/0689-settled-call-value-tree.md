# 0689: Turn a settled call into one neutral value tree for the direct Rust bridges

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0629, 0642

## Outcome

- Changes: `crates/thinkthen/src/public/request/value_tree.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/lib.rs`, `crates/thinkthen/tests/`, `libraries/rust/consumer/`, `specification/rust-api-0.2.md`.
- One Rust layer, beside `Engine::call` from 0629, turns a settled `Call<RequestValue>` into a neutral value tree: maps, lists, strings, numbers kept exactly, booleans, null and an explicit absent marker, with the answer, probability, pick and resolution from 0612.
- pyo3, napi, magnus and extendr will only map that tree to host objects and hold no per-function or per-type result code. Each language's switch to the tree is its own phase 3 ticket: 0662, 0665, 0682 and 0683. This ticket changes no language bridge.
- The new public types and methods are declared in `specification/rust-api-0.2.md` in the same change: an Authority entry for 0689 and the declarations in its Added public declarations block, so `sdlc/scripts/inventory` in `lint` passes.

### Proof

- On the M5 natively: `pm job run --machine m5 0689 SHA --memory 8 -- sh -c 'cargo package --locked --offline --no-verify -p thinkthen && THINKTHEN_ARTIFACT="$(ls target/package/thinkthen-*.crate)" sh libraries/rust/check.sh'`. The installed Rust consumer prints the value tree for every shared case, and its answer fields match the 0642 oracle `conformance/fixtures/packets.json`. Before the change the consumer does not compile, because the tree does not exist.
- A test in `crates/thinkthen/tests/` shows absent and null stay distinct in the tree for the partial annotate case and for `19-find-none`.
- On the M5 through pm job, `sdlc/scripts/lint` passes, including the API inventory.

## Evidence

- `crates/thinkthen/src/public/request/call.rs` defines `Engine::call`, which returns the settled `Call<RequestValue>`. `crates/thinkthen/src/public/request/result.rs` defines `RequestValue`, an untagged enum of typed complete records per function (`Decisions`, `Choices`, ..., `Related`), and `RequestOutcome`.
- Python, TypeScript, Ruby and R bind to Rust directly, yet each converts results through its own generated or hand-written layer: Python's `libraries/python/src/results_generated.rs` (2,762 lines) and `native_result.rs`, R's `libraries/r/thinkthen/src/rust/src/results_generated.rs` (3,195) and `native_results.rs`, Ruby's Rust sources under `libraries/ruby/src/`, and TypeScript's generated `libraries/typescript/results_generated.js` (4,771 lines). Each layer must change when the result shape changes, which defeats the point of a direct Rust bridge.
- The 0629 landing first failed its gate because new public declarations were missing from `specification/rust-api-0.2.md`, though prose described them.
