# 0384: Load the bundled Windows library from the C# package

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0530

Reviews: revision 4d59e7c6a, accept

## Outcome

On Windows a caller installs the final NuGet package and makes a real call. The package selects its bundled `thinkthen.dll` and declared dependencies with no user library-path setting. Linux and macOS loading is unchanged. The C# README names Windows as supported.

## Evidence

- Starts from: Windows stage 0 (0373) did not build C#. `ThinkThen.cs` imports `libthinkthen.so.0` by name. The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) requires every package to carry its native library, which replaces the earlier manual DLL placement and PATH instructions. The [0521 assessment](../records/0521-surface-contract-assessment.md) replaced the 0.3 deferral.
- Keeps: Linux and macOS packages load the same library files as today. The public API from 0516.
- Changes: Consume 0516's packaged loader and the Windows native asset assembled by 0530 under 0517's package design. Unix-only test machinery (`bwrap`, `flock`, `killpg`) may use Windows equivalents. Required cancellation and cleanup cases cannot pass by skipping. Claim `libraries/csharp/**` test files named per slice and `.github/workflows/windows.yml`.
- Proof: On `windows-2025`, install the final NuGet artifact, make one call and count its loopback requests, on each supported .NET version declared by the package design. The Linux and macOS C# checks and the installed-file check stay green. Windows execution is owed to the first authorized candidate, and the ticket stays open until it passes.
- Defers: Candidate dispatch and publication wait for Ian's permission.
