# 0678: Delete PHP result building and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- PHP named calls go through the single run-call entry point and return the packet's final answer, probability, pick and resolution.
- `ResultBuilder` and its per-function projection are deleted.
- The session loop in `operation.php:7-101` and its private closed, reader and producer error text are deleted. The producer callback runs on the calling thread, as 0638 requires.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the Composer-installed archive consumer (`fixtures/session_installed.py`) runs all ten named calls against a recorded backend. Its packet dump matches the 0642 oracle.

## Evidence

`ResultBuilder` in `libraries/php/src/session/results.php:49-107` works out values and probabilities for each function. It pairs recognized entities with kept proposals by start, end and kind (`results.php:97-105`). Ruby repeats the same rule (`libraries/ruby/lib/thinkthen/session.rb:21-64`), and so does Swift. Of 134 PHP commits since 2026-09-15, 51 are fixes, for example 1ae8c48ef.
