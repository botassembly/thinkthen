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

Reviews: revision a087f6dc3, accept

Reviews: revision 7470fabe038c3877ee739aaac1da29f09f19045f, accept

Reviews: revision 13cc0ccd443bd97696970af382924bd411b264c9, accept

Reviews: revision fcb87c804225e8daf212bf918195d5686a6f56d0, reject

Reviews: revision 5df36c0f8e2d5e2d606d0883b70e5ee142f889c9, reject

## Outcome

A C# developer installs the NuGet package and calls named functions with native typed inputs. They get generated typed results and typed exceptions, cancel with a `CancellationToken`, and release resources with `SafeHandle` and disposal. No library path, hand-built JSON or result decoding is needed. The pilot sets the wait, cancellation, cleanup and packaging pattern that every other C-interface language follows.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 3 of [the surface assessment](../records/0521-surface-contract-assessment.md). C# blocks a thread for batches, has no finalizer, takes JSON strings for call, plan and settings, and keeps about 900 lines of layout copies. [ADR 0129](../planning/adr/0129-owned-json-sessions.md) allows Pending after cancellation.
- Keeps: Every C# behavior the shared suite covers. Already delivered stream output.
- Changes: Rebuild `libraries/csharp` on 0503's session and 0513's generated C# results. Meet the caller acceptance and the C# section of `../../libraries/BINDING-AUTHOR.md`. Internal JSON is session transport only. One API is one coherent family of named typed calls. Slices:
  - Session and calls. Claim `libraries/csharp/src/ThinkThen.cs`, `libraries/csharp/src/CompleteEngine.cs`, `libraries/csharp/src/NativeMethods.cs`, `libraries/csharp/tests/source/NativeChecks.cs`, `libraries/csharp/tests/source/Installed.cs` and `libraries/csharp/tests/package_check.py`. Name new session and `SafeHandle` files in the reviewed design before coding.
  - Wait and cancellation design. State the wait strategy. A cancelled Task completes promptly with the host's cancellation exception and does not wait for a held provider. Cancel stops reads and submissions, signals native cancellation and releases host ownership safely. Disposal never frees state a native operation or cancellation callback still references. Copy results into owned host values before releasing native handles. Never fabricate final facts while settlement is pending, and say where an explicit drain gets them if the API has one. Use no busy polling and block no calling or UI thread. Name any worker or polling cost and bound it per active session.
  - NuGet package. Claim `libraries/csharp/ThinkThen.csproj`, `libraries/csharp/Botassembly.ThinkThen.nuspec` and `libraries/csharp/README.md`, under 0517's design. Name the loader file before coding.
  - Generated settings, reader failures and shipping inventory. Claim the owning C# session files and `sdlc/generators/results/templates/csharp.py`, `sdlc/scripts/package-inventory.py`, `libraries/csharp/check.sh` and the existing installed session consumer. Derive settings and failures from admitted native definitions and shipping members from project/compiler output; keep independent installed consumers.
  - Removal. Delete copied layouts and readers as their callers migrate. Remove old public names after installed parity, with an old-to-new mapping in the README.
- Proof: The shared conformance suite passes through the installed local NuGet package with no library-path override. One installed held-provider case shows an unrelated Task progressing while cancellation and disposal return before the provider is released. Deterministic cases cover cancelled, full and closed states, stopped readers, cancellation racing disposal, full output, and failure facts through existing runners. A Task wrapping a blocking call does not count. A fresh reviewer scores all ten guide items as met. The landing record gives handwritten code removed and added, counting generator templates.
- Defers: Windows qualification goes to 0384 at the first authorized candidate. Other packages go to their migrations, and final assembly to 0530. Other languages follow in 0504, 0518 and 0522–0526.

## Progress

- 2026-10-09 started
- 2026-10-09 landed 814fd7e11bdaa4714f82fdb64a71c9a3af885d63; next: Installed owned-session initial slice is accepted and landed. Finish generated settings and reader failures, canonical planning and retained legacy removal before whole adoption.
- 2026-10-09 landed e95220d2963c21762926c42bd3e1e8d4f6359b2f; next: Generated settings, reader failures and NuGet inventory are landed. Finish generated Plan, actual shared cases through new Async methods, and obsolete API removal.
- 2026-10-10 landed ccf724c3a; next: Generated Plan and all ten typed Async shared-case routes are implemented; generator freshness passes and the installed fixture uses the extracted NuGet assembly. Final installed/platform qualification and compatibility retirement remain, subject to the release hold.
