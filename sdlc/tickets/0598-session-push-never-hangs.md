# 0598 — session-push-never-hangs

Status: OPEN.

Milestone: 0.2

Reviews: revision c4b48c5, accept

## Outcome

A caller who pushes every record into a session, finishes it, and only then reads gets every answer in input order on every surface. No push waits on answers the caller has not read.

## Evidence

- Release diagnostics at rc.16: the C++, COBOL and Ada session checks hung until their timeout. A debugger showed each pushed all input before reading; with `max_requests` set, the engine's one output slot filled and the blocking push waited for input room forever. The engine behaved as its contract says.
- Ian, 2026-10-15: "that seems like a design problem. What do we do to make it so humans don't have footguns when they're making their code, or agents are making code? I want to make sure that there are no issues."
- Commit 87cf274f3 (2026-10-13) added the blocking push on `ticket/0543-final`; current main has no blocking push. `push_wait` in `crates/thinkthen/src/public/request/session_queue.rs` waits only for input room; `publish` waits for a reader; `thinkthen_session_push` in `libraries/c/include/thinkthen.h` says "Pending output cannot wake this producer." Ada and COBOL push through `thinkthen_session_push_typed`, which reaches the same `push_with`. The C++, COBOL and Ada wrappers moved onto it on 2026-10-13 and 2026-10-14. Their checks read only on a "full" status that a blocking push never returns.
- Read-ahead is bounded by (jobs + 1) × batch (`engine/pipeline.rs`), so how many pushes succeed before the hang depends on timing. Small tests pass. Past a `max_requests` refusal, the session keeps accepting pushes until it settles (`engine/pipeline/run.rs`, `session.rs`), and it cannot settle until the caller reads.
- Raw push and read are public in C, C++, Ada, COBOL, Zig, the Rust core session and recovering session, and beside the safe calls in Java, Kotlin, Scala, C#, Swift and Dart. Python, TypeScript, Ruby, PHP, R, Go, Objective-C's feed, the three SQL hosts and Polars own both feeding and reading and cannot hang this way.
- ADR 0129 keeps output backpressure on purpose and bounds output to one queued packet. It never names this hang. `sdlc/records/0024` fixed the same shape in the command scheduler by waiting on input and answers together.
- A read-only design review on 2026-10-15 compared six options: iterator-only APIs, parking unread answers, a fail-fast error, a stall timeout, library-owned producer threads, and a combined push-or-read call.

## Decisions

- PM decision, 2026-10-15: fix it in the core before 0.2 ships. A warning does not reach coding agents, and the hang is silent on the most natural loop.
- PM decision, 2026-10-15: when a push, blocking or not, finds input full while an answer waits, the session moves that answer into an ordered list it owns, and the engine continues. A blocking push repeats this each time it wakes. Reads serve that list first, then the output slot, then the terminal. Cancel keeps the list readable, as ADR 0129 keeps the output slot after cancel; free discards it. Unread answers then cost memory, as a collecting call does. A slow reader no longer slows a separate producer thread; the provider and the request caps still do. This amends ADR 0129's one-packet output bound.
- PM decision, 2026-10-15: no separate early-close signal. Parking alone ends the hang: past a `max_requests` stop the session settles and a later push returns closed. The engine scheduler does not change.
- PM decision, 2026-10-15: Ada keeps its `Full` status, so no public type changes; the README says a push no longer waits on unread answers.
- PM decision, 2026-10-15: build on `ticket/0543-final`, where the blocking push lives, and merge there; 0543 depends on this ticket.
- PM decision, 2026-10-15: no public signature changes. The fix also stops a try-push retry loop that never reads from spinning, because try-push parks the same way.

## Scope

- Changes: `crates/thinkthen/src/public/request/session_queue.rs`, `crates/thinkthen/src/public/request/session.rs`, `crates/thinkthen/src/public/request/recovering_session.rs`, `crates/thinkthen/tests/request_session/push.rs`, `crates/thinkthen/tests/recovering_stream.rs`, `sdlc/planning/adr/0129-owned-json-sessions.md`, `sdlc/generators/results/operations.py`, `libraries/c/src/ffi/session/operations_generated.rs`, `libraries/c/src/ffi/native_request/operations_generated.rs`, `libraries/c/include/thinkthen.h`, `libraries/cpp/README.md`, `libraries/zig/README.md`, `libraries/cobol/README.md`, `libraries/ada/README.md`, and one push-all-then-read case in the existing checks under `libraries/c/`, `libraries/cpp/`, `libraries/cobol/`, `libraries/zig/`, `libraries/jvm/`, `libraries/csharp/`, `libraries/swift/` and `libraries/dart/`. The recovering session has its own one-slot output in `recovering_session.rs`, so its push parks from that output. The generator's comments for `thinkthen_session_push` and `thinkthen_session_push_typed` regenerate both Rust files and the header.
- Keeps: answer order; every request body, cap and cache behavior; cancel and close semantics; the safe iterator calls unchanged.

## Proof

Routine suite: `push.rs` flips to push all, finish, then read every row in order with `max_requests` below the record count and jobs 1, with each push bounded by a timeout so the old code fails instead of hanging. `recovering_stream.rs` runs the same case, and a cancel after parking still reads every parked row before the terminal. The same case runs in the C, C++, COBOL, Ada, Zig, Java, C#, Swift and Dart session checks with zero timeouts. It fails on `ticket/0543-final`.

## Defers

The other footguns the review found go to ticket 0599. These include read-before-finish on recognize, rank, find and relate; C++ cancel and close races; JVM `Error` handling; and making the iterator call the documented front door.
