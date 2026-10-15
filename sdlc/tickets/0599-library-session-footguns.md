# 0599 — library-session-footguns

Status: OPEN.

Milestone: 0.3

Draft: filed 2026-10-15 by the 0.2 PM from the design review behind ticket 0598, for the 0.3 PM to order. Any item found to cause a wrong answer or a crash in normal use moves to 0.2.

## Outcome

No library's session or incremental API can hang, spin, race or lose a result on a plausible caller's code, and every library's documented front door hands ThinkThen an iterator or stream and receives answers back.

## Evidence

- Ian, 2026-10-15: "What do we do to make it so humans don't have footguns when they're making their code, or agents are making code? I want to make sure that there are no issues."
- The read-only design review of 2026-10-15 found these beside the push hang that 0598 fixes. The paths are on `ticket/0543-final` and must be re-read on main:
  1. **Read before finish hangs.** Single-thread code that waits for an answer before finishing hangs on recognize, rank, find and relate. They answer only after input ends (`crates/thinkthen/src/public/request/session.rs`, `settings.rs`).
  2. **C++ race.** `cancel` writes a plain bool that `admit` reads, which is a data race (`libraries/cpp/` `native_implementation.hpp`).
  3. **C++ use after free.** `close` while another thread is blocked in a call is a use after free (the same file).
  4. **Free without cancel.** Ada `Close` and `Finalize`, and Zig `deinit`, free a session without cancelling it.
  5. **JVM.** Moved to 0.2 as ticket 0600 on 2026-10-15.
  6. **Python.** Leaving `iterate` early without `close` keeps the session alive until `engine.close`.
  7. **PHP.** `wait` has no timeout without a token.
  8. **Dart.** `finishUsageStatus` blocks the UI isolate.
  9. **Go.** `Close` hangs on a producer that ignores its context. This is documented.
  10. **Ruby.** Any interrupt during a wait cancels the whole session.
  11. **SQL row order.** The shared SQL host puts failure rows after recovered rows (`sql_request.rs`).
  12. **Repeated loop.** The step loop is repeated in Python, TypeScript, Ruby, R, PHP and Objective-C. One core call could own it.

## Scope

- Changes: decided per item by the 0.3 PM. Expect `crates/thinkthen/src/public/request/`, `libraries/cpp/`, `libraries/ada/`, `libraries/zig/`, `libraries/jvm/`, `libraries/python/`, `libraries/php/`, `libraries/dart/`, `libraries/ruby/` and the SQL host.
- Keeps: every request body and answer.

## Proof

One outside-in case per item at the library's public entry point, under replay. Each fails before its fix and passes after. A thread sanitizer run covers the C++ race.

## Defers

The push hang itself is ticket 0598, in 0.2.
