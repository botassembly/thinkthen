# 0679: Build Polars frames from typed packet results

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0626, 0629, 0688

## Outcome

- Frame building calls the Rust run function from 0629 and reads the typed final answer, probability, pick and resolution straight from the Rust result types.
- Polars column types come from those Rust types directly, with no schema generation step.
- The runtime schema interpreter and the `serde_json::Value` hops are deleted.

### Proof

On the M5 through pm job, the installed Polars consumer runs all ten functions over a frame against a recorded backend. Its columns, types and nulls match the 0642 oracle's answer fields.

## Evidence

- **Runtime schema interpreter.** `crates/thinkthen/src/public/frame/schema.rs` holds a 575-line interpreter that turns the JSON schema into Polars types at run time.
- **JSON in the frame path.** The `frame/` module moves results through `serde_json::Value` at 8 call sites.
- **Likely break point.** This is the part of Polars most likely to break when the result schema changes (uncertain).
