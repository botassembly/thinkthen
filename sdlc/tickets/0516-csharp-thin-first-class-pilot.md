# 0516: Prove the thin first-class pattern in C#

Status: OPEN.

Milestone: 0.2

Depends on: 0503
Depends on: 0511
Depends on: 0513
Depends on: 0514
Depends on: 0517

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

A C# developer installs the NuGet package and calls named functions with native typed inputs. They get generated typed results and typed exceptions, cancel with a `CancellationToken`, and release resources with `SafeHandle` and disposal. No library path, hand-built JSON or result decoding is needed. The pilot sets the wait, cancellation, cleanup and packaging pattern that every other C-interface language follows.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 3 of [the surface assessment](../records/0521-surface-contract-assessment.md). C# blocks a thread for batches, has no finalizer, takes JSON strings for call, plan and settings, and keeps about 900 lines of layout copies. [ADR 0129](../planning/adr/0129-owned-json-sessions.md) allows Pending after cancellation.
- Keeps: Every C# behavior the shared suite covers. Already delivered stream output.
- Changes: Rebuild `libraries/csharp` on 0503's session and 0513's generated C# results. Meet the caller acceptance and the C# section of `../../libraries/BINDING-AUTHOR.md`. Internal JSON is session transport only. One API is one coherent family of named typed calls. Slices:
  - Session and calls. Claim `libraries/csharp/src/ThinkThen.cs`, `libraries/csharp/src/CompleteEngine.cs`, `libraries/csharp/src/NativeMethods.cs`, `libraries/csharp/tests/source/NativeChecks.cs`, `libraries/csharp/tests/source/Installed.cs` and `libraries/csharp/tests/package_check.py`. Name new session and `SafeHandle` files in the reviewed design before coding.
  - Wait and cancellation design. State the wait strategy. A cancelled Task completes promptly with the host's cancellation exception and does not wait for a held provider. Cancel stops reads and submissions, signals native cancellation and releases host ownership safely. Disposal never frees state a native operation or cancellation callback still references. Copy results into owned host values before releasing native handles. Never fabricate final facts while settlement is pending, and say where an explicit drain gets them if the API has one. Use no busy polling and block no calling or UI thread. Name any worker or polling cost and bound it per active session.
  - NuGet package. Claim `libraries/csharp/ThinkThen.csproj`, `libraries/csharp/Botassembly.ThinkThen.nuspec` and `libraries/csharp/README.md`, under 0517's design. Name the loader file before coding.
  - Removal. Delete copied layouts and readers as their callers migrate. Remove old public names after installed parity, with an old-to-new mapping in the README.
- Proof: The shared conformance suite passes through the installed local NuGet package with no library-path override. One installed held-provider case shows an unrelated Task progressing while cancellation and disposal return before the provider is released. Deterministic cases cover cancelled, full and closed states, stopped readers, cancellation racing disposal, full output, and failure facts through existing runners. A Task wrapping a blocking call does not count. A fresh reviewer scores all ten guide items. The landing record gives handwritten code removed and added, counting generator templates.
- Defers: Windows qualification goes to 0384 at the first authorized candidate. Other packages go to their migrations, and final assembly to 0501. Other languages follow in 0504, 0518 and 0522–0526.
