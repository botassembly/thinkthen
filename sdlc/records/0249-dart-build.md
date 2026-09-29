# 0249 Dart and Flutter source integration build

Built 2026-09-28 on Linux x86_64 from `4de52c8f764bdf0cfd181dc404b4353d14d8e688` on `ticket/0249-dart-integration`. The Dart source is `libraries/dart/`; `libraries/dart/flutter/` contains the Flutter wrapper and a real Linux application. Product checks, independent pub consumers, exact request fixtures, privacy guard, and source ratchet live beside the package. No generated cache, native binary, or experiment archive is committed.

## Proof

| Boundary | Current result |
| --- | --- |
| Native ABI | Offline release build from this checkout; installed `.so` SHA-256 `1106ff50249b54422c2bad27c4f3b554b7198dd4bf79f21ec7deadaac05a4606`; header SHA-256 `1aa49b91a157b4ef6baed1e72a2195f07e453d61e14c81f5459e7fa55edc7089`; 30 declared/defined exports match. |
| Dart package | Offline pub resolution, format check, `dart analyze lib`, two independent pub consumers with analysis and typed structural samples. Dart 3.13.4 Linux x64, `ffi` 2.2.0. |
| Dart runtime | Alpha and Bravo each passed an exact 16-arrival complete request-body multiset at the current pin. JSON `value` and `facts`, named outcomes and errors, distinct null/failure, map descriptions, configured engine, copied failure facts, ordered packed results, cache and deadlines passed. Held scalar/bulk cancellation fired from another isolate; joins preceded token/engine frees. Seven invalid-input allocation regressions ran three times per consumer. |
| Shared J1 corpus | Draft 2020-12 schema: 55 cases, 68 checks, 41 valid and 27 invalid. Public Dart `Door` API through an installed consumer: all 29 executable cases passed using the shared loopback backend. Case 41 checks `[10,20)` Unicode scalar offsets past an accent and emoji. The two 0258 record-error cases are schema controls only. |
| Dart negatives | Nine original planted variants for each consumer, 18 total, failed for their intended reasons. A changed question with the same request count failed the full-body comparator. Three planted private path/key markers were refused by the source guard. |
| Flutter host | Offline Flutter pub and analysis passed. The strict host consumer passed 17 exact complete-body arrivals, including facade call, with three historical planted negatives. |
| Flutter app | `flutter build linux` and an actual Xvfb Linux engine run printed `FLUTTER_EMBEDDER_PASS`; the loopback backend counted exactly one `flutter-embedder` request and its full body matched the pinned fixture. Flutter 3.47.5 on this Linux host. |
| Source | Dart ratchet: 13 files, 2,537 nonblank lines. `git diff --check` passed. |

The final full gate's Dart and Flutter host sections passed. Its first embedder repeat found that the experiment script reused a cache tied to an earlier loopback port, causing a local backend-binding refusal. The script now creates a new home and cache per run; the affected embedder check passed on rerun, including the full-body assertion. This repeatability fix changed only the embedder runner. Receipts remain in ignored `libraries/dart/checks/logs/` and `libraries/dart/flutter/logs/`; the source gate is `libraries/dart/check.sh`.

`CARGO_BUILD_RUSTC_WRAPPER= CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` reports only the expected shared registration failure: `libraries/dart` needs its `surfaces.txt` mapping. The coordinator's registry lane owns that file and `policy.py`; this source branch does not edit them. The same policy run checked 189 resolved Cargo packages. This source claim is ready for independent review before shared registration and landing.

## What the build taught us

The accepted experiment's `stage3/package/` was a second Dart binding copy, while the actual Flutter wrapper was at stage-three root. Keeping one Dart package and placing the Flutter wrapper under it removes that duplicate source. A loopback cache must be fresh for each ephemeral backend address; the one-time experiment run did not reveal this until the product gate repeated. The prior re-pin receipts recorded names and question bodies only; the current gate freezes the complete `model`, `state`, and `questions` body after matching the prior 16/17 question multisets. A count alone would have missed a changed question.

The public Dart API's JSON route already carries success facts, and `DoorFailure` carries copied failed-call facts. Bare typed convenience methods remain available for callers that only need values. No extra typed C convenience symbols were bound. This keeps the binding surface aligned with actual Dart use. The current source package has no native artifact installation or runtime download.

## Remaining limits

The proof is Linux x86_64, shared-library loading, local loopback, and one installed Dart and Flutter SDK. It does not claim Android, iOS, macOS, Windows, static linkage, model-produced non-BMP offsets, CI publication, registry ownership, or a self-contained native-asset package. The existing Dart consumer issue and release/install issue remain open for the release pin, Ubuntu 24.04 CI, `dart pub publish --dry-run`, trusted publishing, per-platform artifacts, and other-host proof. The shared registry/policy lane and ticket 0249 remain with the coordinator.

## Review correction: shared J1 backend startup

Fresh review found that `specification/fixtures/types/check.py` built `conformance-backend` only in its C-only `build()` path. Public Dart, C#, and JVM callers use `start_backend()` directly and could inherit a stale or missing executable. `start_backend()` now builds the backend offline before launching it, with `--target-dir` fixed to this checkout's `target/` so its output matches `BACKEND` even when `CARGO_TARGET_DIR` is set. `build()` still builds the C library for the C-only path; it no longer builds the backend twice. Schema validation remains build-free.

Focused proof moved this worktree's `target/debug/conformance-backend` into an owned temporary folder, asserted the original path absent, set `CARGO_TARGET_DIR=/tmp/thinkthen-dart-unselected-target`, and ran `python3 libraries/dart/checks/types.py` with `TT_DART`, `TT_NATIVE_LIBRARY`, `PUB_CACHE`, and `RUSTC_WRAPPER=`. The 29 public cases passed, a new executable appeared at the expected checkout path, and the prior executable was restored. The command exited 0. No native library or Flutter source changed, so their gate was not repeated for this helper correction.
