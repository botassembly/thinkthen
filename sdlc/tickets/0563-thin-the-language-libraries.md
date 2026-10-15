# 0563: Move repeated language-library code into the C layer and generators

Status: OPEN.

Milestone: 0.2

Depends on: 0561, 0562

## Outcome

Each language library that uses the C ABI keeps only its own idioms: futures or threads, ownership and native types. The C layer and the generators own the named calls, the wait for the next packet, request construction, error kinds and usage status. Known defects in the JVM and Go libraries are fixed.

## Evidence

- Starts from: the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md) and a read-only review of the 0.2 combined branch.
  - Hand-written source grew from 0.1.2 in Swift (271 to 635 lines), PHP (204 to 498), JVM (543 to 703), Go (548 to 732) and C# (297 to 440). Ada and COBOL shrank, while their generated code grew by about 10,000 lines each.
  - Nearly every language repeats four pieces by hand:
    - the ten named-call wrappers;
    - a read loop that polls `thinkthen_session_wait` every 50 ms on its own thread;
    - the error-code-to-kind mapping;
    - the usage-status wrappers.
  - Go builds its request by hand. It maps records to units for find and to entities for relate, then picks one of ten request types in a switch. `thinkthen_session_new_named_with_surface` already does this, and PHP uses it.
  - Ada and COBOL each carry a generated JSON request encoder for the same schema. The COBOL encoder alone is about 4,000 lines.
  - The generated Kotlin and Scala results, about 2,900 lines, re-decode every Java packet into separate result trees.
  - Defects:
    - JVM `NativeSession.check` drops the retryable flag and the facts on session errors.
    - JVM `execute` has no feed form.
    - C# starts a new long-running thread for every 50 ms wait.
    - Ada `Start` without an engine quietly creates a default engine for each session.
    - Go has dead code: the nil-settings branch of `newNativeEngine`, and `checkedCString` on JSON from `json.Marshal`.
- Keeps: each library's public 0.2 API and its routine installed checks. Renames need no upgrade notes.
- Changes: these moves.
  - Add a C next-packet call that blocks until a packet is ready or the session closes, with cancellation. Each library uses it in place of its own 50 ms poll loop.
  - Move Go, and every other library that builds requests by hand, onto `thinkthen_session_new_named_with_surface`.
  - Give Ada and COBOL one typed request entry in the C layer, and delete their generated encoders.
  - Generate the named-call wrappers, the error-kind enum and the usage-status wrappers for every C-ABI language, and delete the hand-written copies.
  - Let Kotlin and Scala use the Java result records directly. Delete their generated result trees and Scala's `transport()`.
  - Fix the defects listed above.
  - Lower each changed ratchet to its measured value.
- Proof: these checks.
  - Every language library's routine installed checks pass.
  - The JVM session-error check shows the retryable flag and the facts.
  - A cancellation check in each changed library shows the blocking next-packet call returns when the session is cancelled.
  - No library source contains its own 50 ms wait loop.
- Defers: Python, R, Ruby and TypeScript, which call the core through their own Rust crates and are outside this ticket.



PM amendment, 2026-10-13. Remaining work. Close 0563 now and move its remainder into the 0576 language lanes: PHP replaces its 50 ms `wait()` loop in `libraries/php/src/session/operation.php` with `thinkthen_session_next_packet`; Objective-C does the same in `Sources/Foundation/ThinkThen.m`; Zig merges e0afccd3a from `ticket/0563-php-dart-zig-runtime` and uses the blocking call in its examples; COBOL's session check uses the blocking call; Swift, Objective-C and Zig start sessions through `thinkthen_session_new_named_with_surface`. The no-poll rule covers only C-ABI libraries; the ticks in the Python, R, Ruby and TypeScript crates stay under ADR 0047. Proof, in each lane: the library's `check.sh` passes with a cancellation case, and `git grep -n 'session_wait.*50' libraries` returns nothing.

The Objective-C polling loop is gone: 6ba9f3c59 replaced it with event waits, and the final review fix 8e1615cdc kept settlement waits correct after cancellation. This ticket closes with the passing 0543 candidate.
