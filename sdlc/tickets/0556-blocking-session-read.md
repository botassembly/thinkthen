# 0556: Wait for session results in Rust and delete binding poll loops

Status: OPEN.

Milestone: 0.2

Depends on: 0550, 0553

## Outcome

A binding waits for the next session result or for room to push through one native call. That call blocks for a caller-given timeout and wakes early on cancel. No binding sleeps and polls. The Rust session queue does not wake on a fixed timer.

## Evidence

- Starts from: [Ian's 2026-10-11 approval](../decisions/2026-10-11-efficiency-approvals.md) and the bindings section of the [adversarial review](../records/2026-10-11-0-2-adversarial-review.md).
  - The C API offers only `thinkthen_session_try_read` and `thinkthen_session_try_push`.
  - Each binding sleeps for 1 to 10 ms in a loop: Python `_calls.py:270,294,322`, R `requests.R:153`, Ruby `session.rb:196`, JavaScript `native.js:116`, PHP `operation.php:80`, C# `OwnedSession.cs:52,75`, JVM `Engine.java:110`, C++ `client.hpp:78`, Go `session.go`, Swift `OwnedSession.swift:92,125`, Dart `client.dart:104,144`, and COBOL `tt_session.c:12`.
  - The Rust session queue wakes every 10 ms (`public/request/session_queue.rs:110`). 0554 also names this.
- Keeps:
  - `try_read` and `try_push`, and the frozen 0.1 C ABI.
  - Cancellation from a host interrupt in every binding.
  - Every installed case.
- Changes:
  - Add `thinkthen_session_wait` (or a better-named equivalent) to the C API. It takes a timeout in milliseconds and returns when a result is ready, the input has room, the session ends, cancel is called or the timeout passes.
  - Each binding calls it with a short timeout, so host interrupts and producers still get a turn, and deletes its sleep loop. Bindings with an async runtime may keep their own wait, as long as they block on the native wait and not on a timer.
  - Update the generated header and each binding's declarations.
- Proof:
  - A Rust test shows the wait returns on a result, on cancel and on timeout.
  - Each language's routine installed checks pass, including its cancellation case.
  - A search finds no sleep in a binding's session code.
- Defers: nothing.
