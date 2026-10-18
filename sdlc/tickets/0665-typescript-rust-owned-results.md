# 0665: Return Rust-owned results from the TypeScript addon

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0688, 0689

## Outcome

- The addon takes JavaScript values and calls the Rust run function from 0629. It builds results by mapping the 0689 value tree, so answer fields come from the core.
- The JavaScript polling loop around `poll` and its private closed, reader and producer error text are deleted. Async calls run the run function on a worker and cancel through its handle.
- `callResult`, `callResolution`, the JSON parsers in results_generated.js and the `dump` validator are deleted. Generated `.d.ts` declarations remain.
- Presence stays distinct from null in the declarations.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the packed npm package installs into an outside project. `tsc --strict` type-checks a caller, and its packet dump matches the 0642 oracle.

## Evidence

- The contract asks for a "Direct Rust addon; no hand-copied JSON readers" (BINDING-AUTHOR.md, TypeScript table).
- The package passes JSON strings in both directions instead. `dump` serializes and checks every input in JavaScript (libraries/typescript/native.js:10-24). `poll` returns text that `readJson` and `readPacket` decode (:110-112).
- Results are rebuilt by 4,014 lines of generated JavaScript readers (results_generated.js). Those include a second copy of the per-function value and probability projection (`callResult`, results_generated.js:3820-3873) and resolution tracking (`callResolution`).
