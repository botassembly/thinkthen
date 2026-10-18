# 0689: Turn a settled call into one neutral value tree for the direct Rust bridges

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612, 0629, 0642

## Outcome

- One Rust layer, beside the run function from 0629, turns a settled call into a neutral value tree: maps, lists, strings, numbers kept exactly, booleans, null and an explicit absent marker, with the answer, probability, pick and resolution from 0612.
- pyo3, napi, magnus and extendr only map that tree to host objects. They hold no per-function or per-type result code.
- Each language's switch to the tree is its own phase 3 ticket: 0662, 0665, 0682 and 0683.

### Proof

- On the M5 through pm job, the Rust library consumer prints the value tree for every shared case, and it matches the 0642 oracle's answer fields.
- Absent and null stay distinct in the tree for the partial annotate case and for `19-find-none`.

## Evidence

Python, TypeScript, Ruby and R bind to Rust directly, yet each one converts results through its own generated or hand-written layer: Python's `src/results_generated.rs` is 2,433 lines, R's result layer about 2,840, Ruby's about 969, and TypeScript rebuilds results in 4,014 lines of generated JavaScript readers. Each layer must change when the result shape changes, which defeats the point of a direct Rust bridge.
