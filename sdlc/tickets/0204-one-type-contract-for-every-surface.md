---
flow: build
priority: 204
opens: specification/types.md specification/result.schema.json specification/fixtures/types sdlc/scripts/test sdlc/planning/adr/0082-one-type-contract-for-every-surface.md sdlc/records sdlc/tickets
---

# 0204: Publish one type contract for every surface

Status: done 2026-09-27 after fresh code review and correction acceptance at `8fef9b85`. Owner: Codex.

## Outcome and authority

A library or language port can read one written type contract, validate the C JSON door's request and result shapes, and test its mapping against shared examples. This is Batch J1 in `sdlc/planning/work-plan-2026-09-27.md`, authorized by `sdlc/issues/2026-09-27-one-type-contract-for-every-surface.md` and proposed ADR 0082. J1 supports the ideal state's one Rust engine with the same answers across surfaces. Ian can overturn the issue's recommended mappings. Leave existing tickets unchanged and do not build J2–J8 here.

## Starts from and keeps

Ticket 0069 already landed `specification/question-file.schema.json`, a corpus, and schema/parser parity. Keep that grammar. Keep `libraries/c/include/thinkthen.h`'s JSON request and result path, with descriptions in `question_json`; add no C structs. Keep Rust `Choice`, `choices!`, and `Annotated::Failed`, TypeScript's typed output union, Python's `py.typed` stub and dependency-free core, and plain SQL answer types. The issue identifies the remaining contract and surface gaps accurately. `conformance/cases.json` already contains case 41 with `Le café 😀 Maria Chen arrived.` and a code-point span of 10 to 20; reuse it for offset parity rather than adding a second recording.

## Change

1. Write `specification/types.md` as a compact mapping for question inputs, ordered label sets and optional descriptions, thresholds, record selections, deadlines and cancellation, the ten answer shapes, six named call errors, failure markers, and offsets. Link the settled question-file, result, annotate, recognize, and relate pages instead of copying their full grammar. State `null` versus failure, numeric score, successful empty tag list, and string or structured descriptions. Name each surface's offset unit and point to shared case 41.
2. Write one hand-maintained Draft 2020-12 `specification/result.schema.json`. Give the ten bare verb results distinct `$defs`, plus record rows, detailed `thinkthen.result/1`, annotate failure marker, usage result, and a door request definition. Require the discriminating fields and value kinds, including six error-kind names where represented. Allow compatible added detailed-result members. Document structural limits that production parsers still check. Do not regenerate or replace the 0069 input schema, and do not make a broad union silently accept the wrong answer for a verb.
3. Add a focused `specification/fixtures/types/` corpus and a self-test. Cases cover yes/no/unresolved, described choice, empty and nonempty tag, numeric score, find-none, record collections, annotate answered and failed members, recognize and relate objects, detailed result, usage, and malformed request/value discriminators. Reuse selected accepted `conformance/cases.json` ids and their stored backend responses; do not copy captured recordings. Include case 41 as the one non-BMP offset case and assert each stated unit. The self-test checks schema verdicts with the installed test-only `jsonschema` and compares public C JSON door outputs or error kinds with independent expected values against the offline conformance backend. Use a small Python `ctypes` driver under the fixture if it can load the C artifact, start the existing backend binary, and clean both up deterministically. The self-test must fail if either the schema or real door disagrees. Do not count schema-only validation as runtime parity.
4. Add one call to that self-test in `sdlc/scripts/test`, after its existing question-file self-test. This gate hook is necessary even though the issue initially described J1 as specification-only. The existing install rung already checks `jsonschema`. Do not edit `conformance/settings.json` or the 0148-held consumer or C test files. If the self-contained runner cannot call the real door from these paths, stop and amend the reviewed ticket with the exact needed file claim before claiming parity.

The budget is a result schema under 500 nonblank lines, a corpus smaller than the 0069 corpus's 468 lines, and a short self-test patterned on its 55-line checker plus only the C boundary setup it needs. Use `$defs` to share shapes. Do not introduce a generator, new runtime dependency, or a second parser. If that budget cannot express the settled result shape, stop and amend this reviewed design before splitting the schema or claiming another path.

## Proof and routing

The red proof is a corpus case whose wrong answer kind or malformed door request the current unchecked contract cannot reject; record the schema/checker failure before the fix. The green proof checks `Draft202012Validator.check_schema`, each corpus verdict, real door result or named error, and the existing case-41 offset expectation through its current conformance route. Give every new case an independent expected value; do not compute it from the result under test. A duplicate corpus case that an existing conformance case already proves without a new type boundary should be removed. Run `sdlc/scripts/tickets`, `git diff --check`, and focused schema/fixture checks during the build. Run the test rung once with the shared heavy lock at the related-ticket batch boundary, along with the other required integration rungs under the new work-plan ruling. No paid backend call enters a gate.

Design claim opens only this ticket and ADR 0082. After fresh read-only Codex design review and coordinator acceptance, claim the exact implementation paths in `opens`. A fresh read-only code reviewer checks the hand-written schema against the settled pages, C door parser parity, no secret or live request in the checker, offset units, and the gate hook. The coordinator records, lands, and pushes the reviewed result. J2–J8 retain their plan order and own their implementations.

## Evidence

- Starts from: The revised 2026-09-27 type-contract issue, Batch J, ticket 0069's schema/corpus/checker, current C door header and parser, Rust typed results, TypeScript declarations, Python stub, and conformance case 41.
- Keeps: The existing question-file schema, JSON C ABI and descriptions, typed Rust and TypeScript results, Python's standard-library core, plain SQL answer types, and each surface's current offset convention.
- Changes: A written cross-surface type page, hand-written result and door-request schema, shared parity corpus with a real C door check, and one test-rung hook.
- Proof: Structural valid/invalid schema verdicts, independent expected answers and errors through the offline C door, case-41 offsets, focused self-test, and the related-batch integration gates.
- Defers: J2 stable frame columns, J3 TypeScript inputs, J4 Rust choice descriptions, J5 Python Enum/Literal and optional Pydantic, J6 Ruby/R parity, J7 SQL recipes, and J8 port integration. Existing tickets stay unchanged.
