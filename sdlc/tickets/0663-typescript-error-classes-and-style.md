# 0663: Give TypeScript typed errors and readable source

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0665, 0688, 0727

## Outcome

- Changes: `libraries/typescript/native.js`, `libraries/typescript/native.ts`, `libraries/typescript/native.d.ts`, `libraries/typescript/index.js`, `libraries/typescript/index.mjs`, `libraries/typescript/index.d.ts`, `libraries/typescript/src/request_node.rs`, `libraries/typescript/package.json`, `libraries/typescript/package-lock.json`, `libraries/typescript/check.sh`, `libraries/typescript/README-DEVELOPMENT.md`, `libraries/typescript/tests/`, `conformance/library-surface/typescript.json`, `site/src/data/catalog.mjs`.
- `UsageError`, `BackendError`, `LocalError`, `CancelledError`, `DeadlineError` and `DefectError` extend a common `ThinkThenError` that carries `retryable`, `facts`, `results` and `terminal`. The addon throws them directly. `ClientError` and its `kind` string are deleted.
- The wrapper source is `native.ts`, compiled at package time by the locked TypeScript devDependency and formatted by a standard formatter that the gate checks. The handwritten `native.d.ts` is replaced by compiler output.
- The `Object.setPrototypeOf` patching in `crossing` is deleted.
- The error class change updates its docs and registry in the same change: `README-DEVELOPMENT.md`, the TypeScript errors line in `site/src/data/catalog.mjs`, and the `wrapper_members` entry for `ClientError` in `conformance/library-surface/typescript.json`.

### Proof

- On the M5 natively: `pm job run --machine m5 0663 SHA --memory 8 -- env THINKTHEN_TEST_PROFILE=full sh libraries/typescript/check.sh` passes. An installed caller in `tests/` catches each error class with `instanceof` in the shared failure cases, and `tsc --strict` passes over it. Before the change `instanceof UsageError` fails because the package exports only `ClientError`.
- The gate's formatter check fails on the current packed, many-statements-per-line style and passes on the compiled source.
- `python3 conformance/library-surface.py --library typescript` passes.

## Evidence

- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/typescript.txt` and its surface entry in `conformance/library-surface/typescript.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- The contract asks for "Typed Error subclasses" (`libraries/BINDING-AUTHOR.md`, TypeScript table). The package has one `ClientError` with a `kind` string in `libraries/typescript/native.js`, exported again by `index.mjs`. `crossing` patches native errors onto that class with `Object.setPrototypeOf`. The addon builds those errors in `failure` in `src/request_node.rs`.
- native.js packs many statements per line in a minified style, for example `Client` and `Operation`. That makes review and debugging hard.
- The package is JavaScript with handwritten `native.d.ts`, not TypeScript source. `package.json` already locks `typescript` 7.0.2 as a devDependency.
- `ClientError` appears in `README-DEVELOPMENT.md`, in the TypeScript errors line of `site/src/data/catalog.mjs`, in `wrapper_members` in `conformance/library-surface.json`, and in `tests/owned_session.test.mjs` and `tests/owned_types.test.ts`.
