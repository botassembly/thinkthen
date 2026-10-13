# 0574: Let every library read one answer simply and safely

Status: OPEN.

Milestone: 0.2

Depends on: 0562

## Outcome

In every language library, a caller reads one call's answer in one short, safe line, writes recognize and relate questions with typed builders, and reads a relate edge's source and target names directly. Each library's own example uses the 0.2 API.

## Evidence

- Starts from: a 2026-10-12 replay of the ten function samples in every language library.
  - Reading one answer means picking a packet type out of a results loop in Java, Kotlin, Scala, C, C++, Swift, Zig, Ada and COBOL.
  - Swift's decide reads `.alternative0(.alternative0(let yes))`, and Dart's reads `.value.value.asAlternative0`. Zig has no session wait method.
  - The Objective-C decide sample takes 44 lines, because the API offers only an async call and a packet loop.
  - In C++, `value()` returns a temporary, so `for (x : *found->value().value)` reads freed memory. The rank sample hit `free(): invalid next size`, and recognize read wrong names.
  - `libraries/csharp/examples/Program.cs` still calls the removed API (`engine.Decide`, `Answer`).
  - The Java, Kotlin, Scala, C# and Swift recognize and relate samples fall back to `fromJson` or `parseQuestion`, which the guides tell callers not to use.
  - Relate edges' source and target share no name field in Java, Kotlin, Scala, C# and Objective-C.
- Keeps: the packet stream for callers who want it.
- Changes:
  - Generate a typed answer accessor for each of the ten functions in every library: decide returns a yes or no, score a number, rank a list and so on. Each waits for the result and returns it. Objective-C gets this blocking call beside its async call.
  - Add a session wait method to Zig.
  - Make the C++ accessors return owned values or references that outlive the loop.
  - Generate typed recognize and relate question builders wherever a library lacks them.
  - Give relate edges a name on source and target in every library.
  - Rewrite the C# example on the 0.2 API, and run every library's example in its routine installed checks.
- Proof:
  - The ten samples in each language read one answer without a results loop or `fromJson`, and replay green.
  - The C++ rank and recognize samples run under AddressSanitizer with no error.
  - Every language library's routine installed checks pass, including its example.
- Defers: long generated names belong to 0562, which renames them.

## Progress
