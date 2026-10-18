# 0671: Delete Dart answer shaping and use the run call

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Dart named calls go through the single run-call entry point and return the packet's final answer, probability, pick and resolution.
- The result-path walking in the template and in the generated calls is deleted. The template keeps only naming and type mapping.
- The session loop in `client.dart:40-140` and its private closed, reader and producer error text, including `StateError('Native owner is closed')`, are deleted. One call runs the run call on a helper isolate and cancels through the handle. The producer callback runs on that isolate's own thread, as 0638 requires.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, the installed Dart consumer and the Flutter consumer run all ten named calls, cancellation and failures against a recorded backend. Their packet dumps match the 0642 oracle.

## Evidence

The generated Dart named calls walk result paths such as `["answer","probability"]` to shape answers (`libraries/dart/lib/src/calls_generated.dart:43,91,184`). Those rules live in a 544-line per-language template (`sdlc/generators/results/templates/dart.py`). The guide says moving rules into templates does not remove duplication (`libraries/BINDING-AUTHOR.md:52`). The public API was reshaped several times as the core changed (77ffc568a, d804c46cd, 6705bee33).
