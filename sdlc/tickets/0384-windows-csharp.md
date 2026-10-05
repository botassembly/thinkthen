# 0384: Windows stage 1: the C# package loads the Windows DLL

Status: ready. Deferred to 0.3 under Ian's 2026-10-05 scope ruling; the ticket stays open.

Milestone: 0.3

## Outcome

- On Windows the package finds `thinkthen.dll` from ticket 0381's archive, loads it and answers from a loopback backend. The package still ships no native library, as `libraries/csharp/README.md` says.
- The C# tests that need `bwrap`, `flock` or `killpg` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/csharp/README.md` names Windows. Linux and macOS loading is unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build C#. `ThinkThen.cs` line 45 imports `libthinkthen.so.0` by name. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 150 to 400 lines and 1 slice, with low to medium Linux and macOS risk: a resolver replaces the fixed name, so it touches the Linux load path. No experiment preceded this ticket.
- Keeps: the Linux and macOS packages load the same library files as today. The package's public API.
- Changes: a native library resolver picks `thinkthen.dll` on Windows and keeps today's name elsewhere. The README says where to put the DLL, beside the application or on `PATH`. The Unix-only tests get skips with reasons.
- Proof: a load and smoke case on `windows-2025` that counts loopback requests. The Linux and macOS C# checks and the installed-file check stay green.
- Defers: the main unknown, whether `NativeLibrary.SetDllImportResolver` finds the DLL the same way on every .NET version the binding supports. The builder tests each supported version and records the answer. A native NuGet package, such as the reserved `Botassembly.ThinkThen.C`, stays out of scope.
