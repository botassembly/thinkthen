# 0678: Delete PHP result building and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Changes: `libraries/php/src/session/`, `libraries/php/fixtures/`, `libraries/php/examples/`, `libraries/php/README.md`, `build-tools/results/templates/php.py`, `build-tools/results/answer_rules.py`, `conformance/library-surface.json`.
- PHP named calls go through `thinkthen_run_call` and return the packet's final answer, probability, pick and resolution.
- `ResultBuilder` in `results.php` and its per-function projection are deleted.
- The session loop in `operation.php` (`Operation::poll`, `readPacket`, `wait` and `feed`) and its private closed, reader and producer error text are deleted. The producer callback runs on the calling thread, as 0638 requires.
- `build-tools/results/templates/php.py` and `libraries/php/src/session/results.php` leave the `LEGACY_FILES` exceptions in `build-tools/results/answer_rules.py`. The PHP entry in `conformance/library-surface.json` names the lifecycle owners that remain.

### Proof

- On the M5 through `pm job run --machine m5 0678 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --target php --check'`. Before the change, `answer_rules.py` fails once the two PHP exceptions are removed.
- PHP is declared for `linux-x86_64` only in `sdlc/tools.json`, so at the 0619 container checkpoint on Yellow (x86_64), `sh libraries/php/check.sh` runs the Composer-installed archive consumer (`fixtures/session_installed.py`) through all ten named calls against a recorded backend, and its packet dump matches the 0642 oracle.

## Evidence

- `ResultBuilder::complete` in `libraries/php/src/session/results.php` works out values and probabilities for each function in its private `probability` and `resolved` helpers. It pairs recognized entities with kept proposals by start, end and kind. Ruby repeats the same rule (`libraries/ruby/lib/thinkthen/session.rb`), and so does Swift.
- `Operation::result` in `libraries/php/src/session/operation.php` calls `ResultBuilder::complete` after its own push, wait and read loop.
- `php.outputs` in `build-tools/results/templates/php.py` writes `results_generated.php`, `constants_generated.php` and `ffi_generated.h`.
- Of 134 PHP commits since 2026-09-15, 51 are fixes, for example 1ae8c48ef.
