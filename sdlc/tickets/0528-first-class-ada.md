# 0528: Generate Ada access from the complete C views

Status: OPEN.

Milestone: 0.2

Depends on: 0505
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision 0ab21ff1e58ddfbc31255c738d1fe035b3df8a94, accept

## Outcome

Ada callers use one named package API with generated package specifications, fixed records and discriminated types for every known result field. Failures arrive as typed exceptions or declared status with error facts. Rust owns every rule, so the Ada JSON validator and label grammar go. Hand-mirrored specs and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). `libraries/ada/src/thinkthen_c*.ads` hand-mirror the C carriers. `libraries/ada/src/thinkthen.adb` restates a JSON validator (`Validate`) and the label grammar. These specs cannot be generated completely until 0505 supplies the complete C session views.
- Keeps: Every frozen 0.1 C symbol and its Ada compatibility, documented Ada representation limits and explicit overflow refusals, all ten functions, failure facts, cache and replay behavior and installed package checks.
- Changes: Meet the caller acceptance and the Ada section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - generated Ada specifications from the complete C header produced by 0505;
  - presence discriminants, typed errors with facts, owned cancellation and controlled or explicit cleanup;
  - generated Ada request records, which the binding may serialize to JSON internally for the session, so callers never build JSON;
  - deletion of the Ada JSON validator, label grammar and hand-mirrored specs once installed typed cases pass;
  - the package slice carrying the native library under 0517's design;
  - `libraries/ada/README.md`, with a short old-to-new call mapping, and removal of old public names after parity.
  One public API is one coherent family of named typed calls. Claim `libraries/ada/**` narrowed and named per slice.
- Proof: An installed Ada consumer reads every known field and failure facts through shared cases, including a value that exceeds an Ada representation limit and receives the explicit refusal. Record handwritten code removed and added, counting generator changes, in the landing record.
- Defers: Final distribution assembly belongs to 0530.

## Progress

- 2026-10-10 landed e07d475bd; next: Generated typed Ada requests and native sessions are landed with counted-string ownership repaired. Final installed archive parity and platform qualification remain.
