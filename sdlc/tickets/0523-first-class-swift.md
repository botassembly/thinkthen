# 0523: Make Swift thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision 419c4d625474638ad8a59d7e7398e5ff0bcaaa71, reject

## Outcome

Swift callers add one SwiftPM dependency that carries the native library, call the ten functions with Swift values, and use `async`/`await` with task cancellation. Results are generated Swift types with optionals plus explicit presence, and failures are typed thrown errors. Hand-copied layouts, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). Swift blocks the caller and keeps hand-copied layouts in `libraries/swift/Sources/ThinkThen/Native*.swift`. Callers install the native library and set its path themselves.
- Keeps: All ten functions, files and images, failure facts, cache and replay behavior and installed package checks.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the Swift section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the Swift target template and generated outputs;
  - native input conversion, typed errors, `async`/`await` scheduling, cancellation and deterministic cleanup;
  - the SwiftPM binary target slice under 0517's Apple binary strategy;
  - `libraries/swift/README.md`, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/swift/**` narrowed and named per slice.
- Proof: Shared conformance runs through an installed Swift consumer, covering absent, null, failures, unknown output fields, files, images and cancellation. One installed held-provider case shows another task progressing, and cancel and cleanup returning before the provider is released. Pending final facts stay pending. Apple platform proof runs on an Apple machine; Linux checks do not replace it. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Dead `Complete.swift` removal goes in 0515. Final distribution assembly belongs to 0530. Native qualification follows Ian's release hold.

## Progress

- 2026-10-10 landed 00ad0813e; next: Swift exposes owned native usage observation and finalization in the shared reviewed 0505 batch. Its corrected thread diagnostic and focused installed status checks pass. The canonical batch record is 0505; final installed Apple and distribution qualification remain held.
- 2026-10-10 landed 00ad0813e; next: Owned usage-status observation is landed, but the full Swift migration remains implementation work: generated Swift session inputs and results, typed failures, async calls and task cancellation, and the SwiftPM native package. Complete that implementation before final installed and Apple qualification.
- 2026-10-10 started
