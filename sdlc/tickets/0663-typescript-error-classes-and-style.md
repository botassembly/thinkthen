# 0663: Give TypeScript typed errors and readable source

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0665, 0688

## Outcome

- `UsageError`, `BackendError`, `LocalError`, `CancelledError`, `DeadlineError` and `DefectError` extend a common `ThinkThenError` that carries `retryable`, `facts`, `results` and `terminal`. The addon throws them directly.
- The wrapper source is TypeScript compiled at package time, formatted by a standard formatter checked in the gate. Handwritten native.d.ts is replaced by compiler output.
- The `prototype` patching in `crossing` is deleted.

### Proof

On the M5 through pm job, an installed caller catches each error class with `instanceof` in the shared failure cases. `tsc --strict` passes over the caller.

## Evidence

- The contract asks for "Typed Error subclasses" (BINDING-AUTHOR.md, TypeScript table). The package has one `ClientError` with a `kind` string (libraries/typescript/native.js:4-6). It patches native errors onto that class with `Object.setPrototypeOf` (:25-29).
- native.js packs many statements per line in a minified style (for example :49-51 and :100-138). That makes review and debugging hard.
- The package is JavaScript with handwritten native.d.ts, not TypeScript source.
