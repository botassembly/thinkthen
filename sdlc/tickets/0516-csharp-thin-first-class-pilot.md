# 0516: Prove the thin first-class pattern in C#

Status: OPEN.

Milestone: 0.2

Depends on: 0503
Depends on: 0511
Depends on: 0513
Depends on: 0514

Reviews: revision 4cd756859, reject

## Outcome

The review amendment below narrows the initial implementation and adds explicit session ownership proof.

C# is thin and first-class end to end: the JSON session interface, generated typed results, `Task` with `CancellationToken`, `SafeHandle` cleanup, nullable reference types, typed exceptions and one entry point. Its measured result sets the pattern for 0504.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). C# blocks a thread for batches, has no finalizer, takes JSON strings for call, plan and settings, and keeps about 900 lines of layout copies.
- Keeps: Every C# behavior the shared suite covers.
- Changes: Rebuild `libraries/csharp` on the session interface and generated results. Remove the native layout and reader files. Claim `libraries/csharp/**`.
- Proof: The shared conformance suite passes through the installed NuGet package. A fresh reviewer scores all ten guide items as met. The record gives lines removed and lines left.
- Defers: Other languages follow in 0504 using this pattern.

## Review amendment

Use 0503's settled ownership and capacity rules. Cancellation stops further host reads and submissions. Disposal cannot free state still referenced by a native operation or cancellation callback. Retained native work references no freed host storage; copy generated results into owned host values before releasing native handles.

Initial claims are `libraries/csharp/src/ThinkThen.cs`, `libraries/csharp/src/CompleteEngine.cs`, `libraries/csharp/src/NativeMethods.cs`, `libraries/csharp/tests/source/NativeChecks.cs`, `libraries/csharp/tests/source/Installed.cs` and `libraries/csharp/tests/package_check.py`. Name new session and SafeHandle files in the reviewed design before coding. Add deterministic cancelled/full/closed, stopped-reader, cancellation-versus-disposal and failure-facts cases through existing runners. Delete copied layouts and readers only as their callers migrate; retain all four dependencies.
