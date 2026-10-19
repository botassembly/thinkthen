# 0671: Delete Dart answer shaping and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0670, 0688, 0727

## Outcome

- Changes: `libraries/dart/lib/`, `libraries/dart/flutter/lib/`, `libraries/dart/checks/session_consumer/`, `libraries/dart/checks/fixture_types.py`, `libraries/dart/check.sh`, `libraries/dart/example/`, `libraries/dart/README.md`, `build-tools/results/templates/dart.py`, `build-tools/results/legacy/dart.txt`, `conformance/library-surface/dart.json`.
- Dart named calls go through `thinkthen_run_call` and return the packet's final answer, probability, pick and resolution.
- The result-path walking in `build-tools/results/templates/dart.py` and in the generated `calls_generated.dart` is deleted, along with the recognition odds join in `values.dart`. The template keeps only naming and type mapping.
- The session loop in `client.dart` (`OwnedSession`, `_NativeWaitPool`, `_NativeWaiter` and the read loop in `Engine._call`) and its private closed, reader and producer error text, including `StateError('Native owner is closed')`, are deleted. One call runs `thinkthen_run_call` on a helper isolate and cancels through the cancel handle. The producer callback runs on that isolate's own thread, as 0638 requires.
- `build-tools/results/templates/dart.py` and `libraries/dart/lib/src/session/values.dart` leave this language's exception file `build-tools/results/legacy/dart.txt`.
- The Dart entry in `conformance/library-surface/dart.json` names the lifecycle owners that remain.

- The Dart conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer (`checks/session_consumer/bin/parity_consumer.dart` and the fixture typing it needs from `checks/fixture_types.py`) is deleted.

### Proof

- On the M5 through `pm job run --machine m5 0671 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --target dart --check && python3 build-tools/results/generate.py --target dart --bridge --check && cd libraries/dart && dart analyze --fatal-infos'`. Before the change, `answer_rules.py` fails once the two Dart exceptions are removed.
- On the M5 through `pm job run --machine m5 0671 SHA --memory 8 -- sh libraries/dart/check.sh`, after 0670 declares `darwin-arm64` and Flutter for macOS, the installed Dart consumer and the Flutter consumer run all ten named calls, cancellation and failures against a recorded backend, and their packet dumps match the 0642 oracle. Linux runs at the 0619 container checkpoint.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0670 lands before the call ticket 0671, so 0671 can prove on the M5. The packet dumper belongs to 0671, written once against the run call.
- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/dart.txt` and its surface entry in `conformance/library-surface/dart.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- The generated named calls in `libraries/dart/lib/src/session/calls_generated.dart` walk result paths such as `["answer", "probability"]` and `["answer", "pick"]` through `_atPath` to shape answers. `dart.calls` in `build-tools/results/templates/dart.py` writes them; `build-tools/results/generate.py` calls it for `--target dart`.
- `libraries/dart/lib/src/session/values.dart` builds `CallResult` and joins recognition proposals to their odds.
- `libraries/dart/lib/src/session/client.dart` runs its own push, wait and read loop on helper isolates (`_NativeWaitPool`, `_NativeWaiter`, `_nativeWaitLoop`).
- The guide says moving rules into templates does not remove duplication (`libraries/BINDING-AUTHOR.md`). The public API was reshaped several times as the core changed (77ffc568a, d804c46cd, 6705bee33).
