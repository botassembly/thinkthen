# 0522: Make Dart and Flutter thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision a087f6dc3, accept

Reviews: revision 36382970a, accept

Reviews: revision dbcd3cbc5c8e736cbd515928d39a9cd1034df4a1, reject

Reviews: revision 3dfd812c21f4308c808148ee48093792578acc68, accept

## Outcome

Dart and Flutter callers install one package, call the ten functions with Dart values, await `Future` and `Stream` results as generated presence-aware Dart classes, and handle typed exceptions. Cancellation is explicit and cleanup is deterministic, with a finalizer as a backstop. Flutter ships as a real FFI plugin with its platform libraries. Hand-copied layouts, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). Dart blocks the caller, has no finalizer, so a forgotten dispose leaks native memory, and keeps hand-copied layouts in `libraries/dart/lib/src/native/`. The 0502 experiment found generated Dart JSON classes erase missing versus null. `libraries/dart/flutter/pubspec.yaml` sets `publish_to: none` and bundles no platform libraries.
- Keeps: All ten functions, files and images, failure facts, cache and replay behavior and installed package checks.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the Dart and Flutter section of `../../libraries/BINDING-AUTHOR.md`. Dart and Flutter share one implementation. This ticket owns:
  - the Dart target template and generated outputs;
  - native input conversion, typed exceptions, `Future` and `Stream` scheduling, cancellation and cleanup;
  - the pub package and a real Flutter FFI plugin for the targets declared by 0517, fetching the prebuilt library at build time under [the pub ruling](../decisions/2026-10-09-pub-build-time-native-asset.md);
  - `libraries/dart/README.md` and `libraries/dart/flutter/README.md`, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/dart/**`, including `libraries/dart/flutter/**`, narrowed and named per slice.
- Proof: Shared conformance runs through an installed Dart consumer and an installed Flutter plugin consumer, covering absent, null, failures, unknown output fields, files, images and cancellation. One installed held-provider case shows another isolate task progressing, and cancel and cleanup returning before the provider is released. Pending final facts stay pending. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Unused Dart `complete/` code and the `door.dart` probe go in 0515. Final distribution assembly belongs to 0530. Platform qualification beyond this machine waits for Ian's release hold.

## Progress

- 2026-10-10 started
- 2026-10-10 landed 516e1ea57; next: Dart and Flutter now settle recognized stream errors with completed rows and native facts, and cancellation does not await held stream cleanup. Fresh review and focused extracted package cases pass. Final installed distribution and platform qualification remain held.
