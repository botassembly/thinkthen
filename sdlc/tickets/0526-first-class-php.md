# 0526: Make PHP thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision 47e7b9d518c372cbdaaf2eda003e26fd20dff5d2, reject

## Outcome

PHP callers install one Composer package that carries the native library, call the ten functions with PHP arrays and values, and read generated PHP result classes with explicit presence. Failures are typed exceptions with retained facts. Every native resource is freed when its object is destroyed, including an engine with open batches. Hand-copied views, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). The engine destructor in `libraries/php/src/native/engine.php` skips the free while batches are open. Native views in `libraries/php/src/native/` are hand-written. The 0502 experiment showed `FFI::cdef` can load declarations stripped directly from the C header. Callers install the native library themselves.
- Keeps: All ten functions, files and images, failure facts, cache and replay behavior and installed package checks. PHP stays synchronous and gains no async runtime.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the PHP section of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - the PHP target template and generated outputs, with FFI declarations taken from the generated header;
  - native input conversion, typed exceptions, cancellation and destructor cleanup that releases batches and the engine in a safe order;
  - the Composer packaging slice under 0517's design;
  - `libraries/php/README.md`, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/php/**` narrowed and named per slice.
- Proof: Shared conformance runs through an installed Composer consumer, covering absent, null, failures, unknown output fields, files, images and cancellation. One installed case destroys an engine with an open batch and shows no leaked native handle. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Dead `libraries/php/src/complete/` removal goes in 0515. Final distribution assembly belongs to 0530.

## Progress

- 2026-10-10 landed a7ff49569; next: PHP polling now pumps bounded producer input through the same feed method as blocking result; poll-only and held-generator cleanup callers pass with exact request counts. Installed Composer distribution, full parity and compatibility retirement remain open.
