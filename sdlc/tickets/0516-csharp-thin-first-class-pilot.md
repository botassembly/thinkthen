# 0516: Prove the thin first-class pattern in C#

Status: OPEN.

Milestone: 0.2

Depends on: 0503
Depends on: 0511
Depends on: 0513
Depends on: 0514

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

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

## Surface assessment amendment

Prove the complete caller path: native typed inputs, named calls, typed values and failures, observations, cancellation, cleanup and installation. A caller must not construct JSON for ordinary input, call settings or planning. Internal JSON is session transport. Generated declarations alone do not meet the outcome.

Settle the wait strategy and caller-visible cancellation in the pilot design. A cancelled Task must complete promptly without waiting for a held provider; stop reads and submissions, signal native cancellation and safely release host ownership. Expose cancellation through the host's cancellation exception form. Preserve already delivered stream output; do not fabricate final facts while native settlement is pending. State where an explicit drain can obtain actual terminal facts, if the API exposes one. Avoid a busy polling loop or a blocked calling/UI thread; name any worker or polling cost and keep it bounded per active session. The native contract does not promise immediate termination of a blocked worker.

One installed held-provider case must prove an unrelated host task can progress and cancellation and disposal return before the provider is released. Retain the full-output and cancellation/disposal race cases. Returning a Task that wraps a blocking call is insufficient evidence by itself.

The pilot also owns the first local NuGet native assets and loader, using 0517's early package design and existing inventory machinery. Add `libraries/csharp/ThinkThen.csproj`, `libraries/csharp/Botassembly.ThinkThen.nuspec` and `libraries/csharp/README.md` to its package slice; name the loader before coding. Install that actual package without library-path overrides. This transfers 0517's initial NuGet implementation slice here so the pilot can prove all ten guide items before family rollout. 0517 reuses its evidence and owns remaining targets and final distribution assembly. Native platform qualification remains under the release hold.
