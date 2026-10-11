# 0556: Wait for session results in Rust and delete binding poll loops

Status: OPEN.

Milestone: 0.2

Depends on: 0550, 0553, 0554

Reviews: revision 597ef8ae60b452689aa5f3813d6aea3967a5fe7b, accept

Reviews: revision 91d50009bf4cd7109c98cb6297a59b380cd082a4, reject

Reviews: revision 1268c1fe4bc523a39b1d9f6dbafcd0892de20a9d, accept

## Outcome

A binding waits for the next session result, or for room to push, through one native wait. The wait blocks for a caller-given timeout and wakes early on cancel. No binding sleeps or polls on a timer, and no binding blocks its host's scheduler.

## Evidence

- Starts from: [Ian's 2026-10-11 approval](../decisions/2026-10-11-efficiency-approvals.md) and the bindings section of the [adversarial review](../records/2026-10-11-0-2-adversarial-review.md).
  - The C API offers only `thinkthen_session_try_read` and `thinkthen_session_try_push` to wait on a session.
  - Each binding sleeps or polls on a 1 to 10 ms timer:

    | Binding | Where |
    |---|---|
    | Python | `_calls.py:270,294,322` |
    | R | `requests.R:153` |
    | Ruby | `session.rb:196` |
    | JavaScript | `native.js:116` |
    | PHP | `operation.php:80` |
    | C# | `OwnedSession.cs:52,75` |
    | JVM | `scheduleWithFixedDelay` at `Engine.java:110` |
    | C++ | `client.hpp:78` |
    | Go | 1 ms ticker at `session.go:199` |
    | Swift | `OwnedSession.swift:92,125` |
    | Objective-C | 10 ms dispatch timer at `ThinkThen.m:109` |
    | Dart | `client.dart:104,144` |
    | COBOL | `tt_session.c:12` |

  - The Rust session queue wakes every 10 ms (`public/request/session_queue.rs:110`). 0554 removes this timer, and 0556 builds on it.
- Keeps:
  - `try_read`, `try_push` and the frozen 0.1 C ABI.
  - Cancellation from a host interrupt in every binding.
  - Every installed case.
- Changes:
  - Add a timed wait to the Rust session type, and export it as `thinkthen_session_wait` in the C API. Rust-native bindings call the Rust method.
  - The wait returns when one of these happens:
    - a result is ready;
    - the input has room;
    - the session ends;
    - cancel is called;
    - the timeout passes.
  - Each binding waits without blocking its host's scheduler:
    - Python releases the GIL with `py.detach`. Its asyncio path runs the wait through `asyncio.to_thread`.
    - Ruby releases the GVL.
    - R loops in R with a wait of at most 50 ms, so an interrupt reaches its `interrupt` handler.
    - JavaScript runs the wait on a libuv worker thread through a napi async task, never on the main thread.
    - Dart runs the wait in a helper isolate, never on the calling isolate.
    - Swift and Objective-C run the wait on a dedicated thread or dispatch queue, outside Swift's cooperative pool.
    - C# and JVM run the wait on a dedicated thread.
    - Go runs the wait inside the locked-thread goroutine and selects on the context.
    - PHP, C++, COBOL, Zig and Ada block directly with a short timeout.
  - Delete every binding's sleep or timer loop. Update the generated header and each binding's declarations.
- Proof:
  - A Rust test shows the wait returns on a result, on cancel and on timeout.
  - Each language's routine installed checks pass, including its cancellation case.
  - A search of each binding's session code finds none of these: sleep, `usleep`, `nanosleep`, `Task.Delay`, `Task.sleep`, `setTimeout`, `Future.delayed`, `time.NewTicker`, `scheduleWithFixedDelay` or a dispatch timer.
  - A JavaScript check shows a `setImmediate` callback runs while `next()` waits.
- Defers: nothing.

## Progress

- 2026-10-11 started
