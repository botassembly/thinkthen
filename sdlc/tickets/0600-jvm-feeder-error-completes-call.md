# 0600 — jvm-feeder-error-completes-call

Status: OPEN.

Milestone: 0.2

## Outcome

When the caller's iterator throws any `Throwable` inside `Engine.execute(request, Iterable)`, the returned future completes exceptionally with that throwable, and the session is cancelled and closed.

## Evidence

- Item 5 of ticket 0599, moved to 0.2 by the PM on 2026-10-15 after the coordinator reproduced it in documented use. An ordinary iterator that throws `AssertionError` escapes the feeder. The future stays pending after a bounded wait, with zero backend sends, though cleanup completed.
- `libraries/jvm/session/thinkthen/Engine.java` on `ticket/0543-final`: the feeder catches only `RuntimeException` before `result.completeExceptionally`.
- Ian, 2026-10-15: "I want to make sure that there are no issues."

## Decisions

- PM decision, 2026-10-15: the feeder catches `Throwable`. It cancels the session through the existing cancel path and completes the future exceptionally with the original throwable, unwrapped. A Java `Error` is rethrown to no one; the future carries it.

## Scope

- Changes: `libraries/jvm/session/thinkthen/Engine.java` and one case in the JVM checks under `libraries/jvm/`.
- Keeps: every other `execute` behavior and request byte; `RuntimeException` handling unchanged.
- Builds on and merges into `ticket/0543-final`.

## Proof

A JVM check feeds an iterator that throws `AssertionError` on its second item. The future completes exceptionally with that error within a bounded wait, and zero further sends follow. The check fails on `ticket/0543-final`.

## Defers

Nothing.
