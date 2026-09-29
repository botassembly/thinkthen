# Dart consumer proof needs a supported package

Status: the local package experiment (local experiment 300) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized Dart on 2026-09-28; the team uses it with Flutter.

## What was proven

A `thinkthen_dart` 0.0.1 package over the C door at pins `5f069321` (stage one) and `79244123` (stage two), Dart SDK 3.13.4, `package:ffi` 2.2.0, on this Linux x86_64 host. Two independent pub consumers pass the full gate: exact 16-arrival multisets, strict cancellation from a second Dart isolate (held scalar and bulk 5, held deadline 3, fired-token reuse refused, fresh-token recovery, released reply cached), both constructors including `thinkthen_engine_new_with` with settings-selected backend routing, and the borrowed `thinkthen_error_facts_json` copied on the calling isolate. The ABI check derives the 21-export list from the sealed header per gate — never a pinned count. J1 parity at gate time: shared corpus 53 cases / 66 checks Draft 2020-12, five typed samples per consumer, shared non-BMP case 41 proving `[10,20)` Unicode-scalar offsets against Dart's UTF-16 indexing. Eighteen planted negatives (nine variants × two consumers) fail for their stated reasons; seven invalid-input no-leak regressions run three times each after the review-1 fix.

## Reviews

Review 1 found a real leak: invalid input with an embedded NUL thrown after partial allocation leaked caller-owned native pointers. Fixed (cleanup-on-throw, reverse-order release) and verified. Review 2 found one records defect only; no package-code blocker. All reviews and fix reports are in the experiment folder.

## Handoff to J8

Copy only `stage2/package/` into `libraries/dart/`. Then: rebuild at the final release pin and rerun the gate; run the full J1 runtime corpus through the public binding; run a real Flutter-embedder test — an actual Flutter application calling through the binding on a Flutter target, per Ian's ruling 2026-09-28 ('flutter yes') — and decide platform packaging (pubspec assets vs. runtime download of the native archive — the README must stay honest that the native library installs separately); adopt trusted publishing per Ian's registration to-do. Ian's compute grant covers installing the Flutter SDK on the Linux host and the M5 Mac for this. Note for every port: derive export lists per pin; `thinkthen_error_facts_json` exists since pin `5f069321`.

## Evidence

Local experiment 300: `FINDINGS.md` (stage one), `stage2/FINDINGS.md`, `BUILD-REPORT.md` files, `reviews/` and `FIX-REPORT.md` in both stages, gate logs under `logs/`. Toolchain hashes in `inputs/toolchain.json` (Dart SDK) and `stage2/package/pubspec.lock` (package:ffi).

## Limits

One Linux host and SDK, synthetic loopback, shared-library loading only (no static-link mode), no Flutter embedder run, no publication, rehearsal archives never ship. GH Actions direction: `ubuntu-24.04`, setup-dart, offline native build from the release archive, `dart pub publish --dry-run` in CI and real publication through the release job.

## Flutter surface proven (2026-09-28, stage three)

The ruling "flutter yes" now has executed evidence in local experiment 300 `stage3/`: the Flutter host test toolchain runs the full strict contract (17 exact arrivals, isolate cancellation, planted negatives), and a real Flutter engine app — `flutter build linux` under xvfb — decides through the binding with one counted backend arrival and the literal `FLUTTER_EMBEDDER_PASS` marker. Ian installed `ninja-build` and `libgtk-3-dev` to unblock the Linux build. The J8 ticket inherits the sharpened packaging decisions: no `path:` dependencies at publish, per-platform native archives (a Linux `.so` satisfies no other target), no untested runtime download, README names archive/version/discovery/platforms. Flutter 3.47.5 hash in the experiment's `inputs/flutter-toolchain.json`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

UNCHANGED-PASS (pin already postdated the contract changes; take the repin README). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Source integration checkpoint (2026-09-28)

Ticket 0249's Dart lane now contains `libraries/dart/` and a Linux Flutter consumer under `libraries/dart/flutter/`. The current-pin local source gate proved two installed Dart pub consumers, complete 16-arrival body multisets each, all 29 executable shared J1 cases through the public binding, 18 historical Dart planted negatives, the 17-arrival Flutter host, and an actual one-arrival Linux app. The source package still installs its native C library separately. See `sdlc/records/0249-dart-build.md` for exact artifact hashes and the embedder cache repeatability fix. This checkpoint does not close the issue's final release pin, Ubuntu 24.04 CI, native archive install, pub dry run, trusted publishing, or untested hosts.

The registration candidate now has one `libraries/dart` surface entry, a checked `pubspec.yaml`, nested private Flutter wrapper and app manifests, and an exact Dart source ratchet. Policy and the surface registry pass. Independent recheck of the shared J1 backend startup correction and registration is pending; the release criteria above remain open.
