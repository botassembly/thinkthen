# 0648: Keep one Ada call family on run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- One family remains. Named functions return an owned result and take an optional producer for fed input and a cancellation handle. The other procedure set is deleted.
- The Ada session loop (`native_sessions.adb`) and its private closed, reader and producer error text are deleted. Each call runs the run call once; a task that needs to cancel uses the handle.
- Calls go through the run-call entry point. Value and probability come from the packet's answer fields, and the projection parts of thinkthen-answers.adb and its template are deleted.
- The README describes only the remaining family.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, an outside GNAT project imports the unpacked thinkthen.gpr, runs examples/session_demo.adb, and its packet dump matches the 0642 oracle.

## Evidence

- Ada exposes two named-call families: procedures on a session owner in `Thinkthen.Sessions.Calls` (libraries/ada/src/thinkthen-sessions-calls.ads:1-24) and blocking functions in `Thinkthen.Inputs.Calls` (thinkthen-inputs-calls.ads). That conflicts with "one public API" (BINDING-AUTHOR.md).
- Answers and probabilities are projected in 6,028 generated lines (thinkthen-answers.adb) from six Ada generator templates of roughly 1,500 lines (sdlc/generators/results/templates/ada*.py).
