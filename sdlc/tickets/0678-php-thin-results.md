# 0678: Delete PHP result building and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0677, 0688, 0727

## Outcome

- Changes: `libraries/php/src/session/`, `libraries/php/fixtures/`, `libraries/php/examples/`, `libraries/php/README.md`, `libraries/php/check.sh`, `build-tools/results/templates/php.py`, `build-tools/results/legacy/php.txt`, `conformance/library-surface/php.json`.
- PHP named calls go through `thinkthen_run_call` and return the packet's final answer, probability, pick and resolution.
- `ResultBuilder` in `results.php` and its per-function projection are deleted.
- The session loop in `operation.php` (`Operation::poll`, `readPacket`, `wait` and `feed`) and its private closed, reader and producer error text are deleted. The producer callback runs on the calling thread, as 0638 requires.
- `build-tools/results/templates/php.py` and `libraries/php/src/session/results.php` leave this language's exception file `build-tools/results/legacy/php.txt`. The PHP entry in `conformance/library-surface/php.json` names the lifecycle owners that remain.
- The PHP conformance runner becomes a packet dumper following the 0642 contract, and `fixtures/session_consumer.php`, `fixtures/session_parity.php` and the parity path of `fixtures/session_installed.py` are deleted.

### Proof

- On the M5 through `pm job run --machine m5 0678 SHA --memory 8 -- sh -c 'python3 build-tools/results/answer_rules.py && python3 build-tools/results/generate.py --target php --check'`. Before the change, `answer_rules.py` fails once the two PHP exceptions are removed.
- On the M5 through `pm job run --machine m5 0678 SHA --memory 8 -- sh libraries/php/check.sh`, after 0677 declares `darwin-arm64`, the Composer-installed archive consumer runs all ten named calls against a recorded backend, and its packet dump matches the 0642 oracle. Linux runs at the 0619 container checkpoint.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0677 lands before the call ticket 0678, so 0678 can prove on the M5. The packet dumper belongs to 0678, written once against the run call.
- After 0727 lands, this language's answer-rule exceptions live in `build-tools/results/legacy/php.txt` and its surface entry in `conformance/library-surface/php.json`. This ticket edits only those two files, so it does not collide with other languages' call tickets.
- `ResultBuilder::complete` in `libraries/php/src/session/results.php` works out values and probabilities for each function in its private `probability` and `resolved` helpers. It pairs recognized entities with kept proposals by start, end and kind. Ruby repeats the same rule (`libraries/ruby/lib/thinkthen/session.rb`), and so does Swift.
- `Operation::result` in `libraries/php/src/session/operation.php` calls `ResultBuilder::complete` after its own push, wait and read loop.
- `php.outputs` in `build-tools/results/templates/php.py` writes `results_generated.php`, `constants_generated.php` and `ffi_generated.h`.
- Of 134 PHP commits since 2026-09-15, 51 are fixes, for example 1ae8c48ef.
