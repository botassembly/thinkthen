# 0679: Build Polars frames from typed packet results

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0626, 0629, 0688

## Outcome

- Changes: `crates/thinkthen/src/public/frame/`, `crates/thinkthen/tests/polars/`, `specification/rust-api-0.2.md`.
- Frame building calls `Engine::call` (0629) and reads the typed final answer, probability, pick and resolution straight from the Rust result types (`Call`, `SettledAnswer`).
- Polars column types come from those Rust types directly, with no schema generation step.
- The runtime schema interpreter in `frame/schema.rs` and the `serde_json::Value` hops in `frame/result.rs` are deleted.
- When a public frame declaration changes, `specification/rust-api-0.2.md` gains its Authority entry and its Added and Retired declaration lines in the same change, so `sdlc/scripts/inventory` passes.

### Proof

- On the M5 through `pm job run --machine m5 0679 SHA --memory 12 -- sh libraries/polars/check.sh`, `cargo test --test polars` and the frame consumer run all ten functions over a frame against a recorded backend. Its columns, types and nulls match the 0642 oracle's answer fields.
- `git grep -n serde_json crates/thinkthen/src/public/frame` finds nothing; on current main it finds `schema.rs` and `result.rs`.
- `python3 sdlc/scripts/inventory` passes in the same job.

## Evidence

- **Runtime schema interpreter.** `crates/thinkthen/src/public/frame/schema.rs` parses `complete_call_schema()` with `serde_json` and turns it into Polars types at run time (`Shape`, `merge`, `definition`, `aggregate_question`).
- **JSON in the frame path.** `frame/result.rs` serializes `RequestValue` rows into `serde_json::Value` (`json`, `probabilities`, `documents`, `value_column`, `annotation_values`, `restore`) before it builds columns.
- **Old entry point.** `execute_feed` in `frame/request.rs` runs calls through `Engine::execute_request`, not `Engine::call`.
- **Likely break point.** This is the part of Polars most likely to break when the result schema changes (uncertain).
