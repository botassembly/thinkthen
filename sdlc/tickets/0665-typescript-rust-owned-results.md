# 0665: Return Rust-owned results from the TypeScript addon

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0629, 0664, 0688, 0689

## Outcome

- Changes: `libraries/typescript/src/`, `libraries/typescript/native.js`, `libraries/typescript/native.d.ts`, `libraries/typescript/results_generated.js`, `libraries/typescript/results_generated.d.ts`, `libraries/typescript/tests/`, `build-tools/results/templates/typescript.py`, `build-tools/results/generate.py`, `build-tools/results/registry.py`, `build-tools/results/answer_rules.py`.
- The addon takes JavaScript values and calls `Engine::call`, the Rust run function from 0629. It builds results by mapping the 0689 value tree, so answer fields come from the core.
- The JavaScript polling loop in `Operation` around `poll` and its private closed, reader and producer error text are deleted. Async calls run the run function on a napi worker and cancel through its handle.
- `callResult`, `callResolution`, the JSON parsers in `results_generated.js` and the `dump` input serializer in `native.js` are deleted. The generator stops emitting the JavaScript readers; the generated `.d.ts` declarations remain.
- Presence stays distinct from null in the declarations.
- `templates/typescript.py` leaves the `LEGACY_FILES` exceptions in `build-tools/results/answer_rules.py`.
- This changes no public Rust API in `crates/thinkthen`, so `specification/rust-api-0.2.md` is untouched. If the work needs a new public Rust declaration, it updates that file's Authority entry and Added block in the same change.

### Proof

- On the M5 natively: `pm job run --machine m5 0665 SHA --memory 8 -- env THINKTHEN_TEST_PROFILE=full sh libraries/typescript/check.sh` installs the packed npm package into an outside project, `tsc --strict` type-checks a caller, and the 0664 dump matches the 0642 oracle for every shared case.
- `python3 build-tools/results/answer_rules.py` passes with `templates/typescript.py` removed from `LEGACY_FILES`; before the change the template's `callResult` projection fails it.
- `git grep -n 'callResult\|callResolution\|\.poll(' libraries/typescript` finds nothing.

## Evidence

- The contract asks for a "Direct Rust addon; no hand-copied JSON readers" (`libraries/BINDING-AUTHOR.md`, TypeScript table).
- The package passes JSON strings in both directions instead. `dump` in `libraries/typescript/native.js` serializes and checks every input in JavaScript. `Operation.next` calls `poll` on the native session (`NativeSession::poll` in `src/request_node.rs`) and decodes its text with `readJson` and `readPacket`. `Client.plan` does the same through `requestPlan`.
- Results are rebuilt by 4,771 lines of generated JavaScript readers in `results_generated.js`, from `build-tools/results/templates/typescript.py` through the TypeScript branch of `build-tools/results/generate.py`. They include a second copy of the per-function value and probability projection (`callResult`) and resolution tracking (`callResolution`).
- `build-tools/results/answer_rules.py` lists `templates/typescript.py` in `LEGACY_FILES`.
