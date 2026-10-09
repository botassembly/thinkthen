# 0524: Make Go thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

## Outcome

Go callers add one module that carries the native library, call the ten functions with Go values, cancel through `context.Context`, and read generated Go result types with explicit presence. Failures are typed errors with retained facts. Hand-copied layouts, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). Go reads the C header through cgo but keeps hand-written native views and readers in `libraries/go/native_*.go` and `libraries/go/complete_*.go`. Callers install the native library themselves.
- Keeps: Go's existing context-based cancellation, all ten functions, files and images, failure facts, cache and replay behavior and installed package checks.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the Go section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the Go target template and generated outputs;
  - native input conversion, typed errors, `context.Context` cancellation and cleanup;
  - the Go module packaging slice under 0517's design;
  - the Go README, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/go/**` narrowed and named per slice.
- Proof: Shared conformance runs through an installed Go consumer, covering absent, null, failures, unknown output fields, files, images and cancellation. One installed held-provider case shows another goroutine progressing, and context cancellation and cleanup returning before the provider is released. Pending final facts stay pending. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Final distribution assembly belongs to 0501. Native qualification beyond this machine follows Ian's release hold.
