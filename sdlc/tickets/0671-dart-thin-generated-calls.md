# 0671: Delete Dart answer shaping and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `libraries/dart/lib/`, `libraries/dart/flutter/lib/`, `libraries/dart/checks/session_consumer/`, `libraries/dart/example/`, `libraries/dart/README.md`, `build-tools/results/templates/dart.py`, `build-tools/results/answer_rules.py`, `conformance/library-surface.json`.
- Dart named calls go through `thinkthen_run_call` and return the packet's final answer, probability, pick and resolution.
- The result-path walking in `build-tools/results/templates/dart.py` and in the generated `calls_generated.dart` is deleted, along with the recognition odds join in `values.dart`. The template keeps only naming and type mapping.
- The session loop in `client.dart` (`OwnedSession`, `_NativeWaitPool`, `_NativeWaiter` and the read loop in `Engine._call`) and its private closed, reader and producer error text, including `StateError('Native owner is closed')`, are deleted. One call runs `thinkthen_run_call` on a helper isolate and cancels through the cancel handle. The producer callback runs on that isolate's own thread, as 0638 requires.
- `build-tools/results/templates/dart.py` and `libraries/dart/lib/src/session/values.dart` leave the `LEGACY_FILES` exceptions in `build-tools/results/answer_rules.py`.
- The Dart entry in `conformance/library-surface.json` names the lifecycle owners that remain.

### Proof

- On the M5 through `pm job run --machine m5 0671 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --target dart --check && python3 build-tools/results/generate.py --target dart --bridge --check && cd libraries/dart && dart analyze --fatal-infos'`. Before the change, `answer_rules.py` fails once the two Dart exceptions are removed.
- The Dart check is declared for `linux-x86_64` only in `sdlc/tools.json`, and Flutter is installed only on Linux, so the installed Dart consumer and the Flutter consumer run all ten named calls, cancellation and failures against a recorded backend at the 0619 container checkpoint on Yellow (x86_64), through `sh libraries/dart/check.sh`. Their packet dumps match the 0642 oracle.

## Evidence

- The generated named calls in `libraries/dart/lib/src/session/calls_generated.dart` walk result paths such as `["answer", "probability"]` and `["answer", "pick"]` through `_atPath` to shape answers. `dart.calls` in `build-tools/results/templates/dart.py` writes them; `build-tools/results/generate.py` calls it for `--target dart`.
- `libraries/dart/lib/src/session/values.dart` builds `CallResult` and joins recognition proposals to their odds.
- `libraries/dart/lib/src/session/client.dart` runs its own push, wait and read loop on helper isolates (`_NativeWaitPool`, `_NativeWaiter`, `_nativeWaitLoop`).
- The guide says moving rules into templates does not remove duplication (`libraries/BINDING-AUTHOR.md`). The public API was reshaped several times as the core changed (77ffc568a, d804c46cd, 6705bee33).
