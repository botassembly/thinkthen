# 0384: Windows stage 1: the C# package loads the Windows DLL

Status: ready. It waits for the `release/0.1` cut, for ticket 0380 slice A, which adds the fifth release target, and for ticket 0381 slice A, which ships the DLL. No slice lands on main before the cut (ADR 0116 item 2). Plan: `sdlc/planning/windows.md`, stage 1. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. The coordinator assigns a lane after the cut.

Milestone: 0.2

## Outcome

- The NuGet package carries `runtimes/win-x64/native/thinkthen.dll`, and on Windows it loads the DLL and answers from a loopback backend.
- The C# tests that need `bwrap`, `flock` or `killpg` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/csharp/README.md` names Windows. Linux and macOS loading is unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build C#. `ThinkThen.cs` line 45 imports `libthinkthen.so.0` by name. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 150 to 400 lines and 1 slice, with low to medium Linux and macOS risk: a resolver replaces the fixed name, so it touches the Linux load path. No experiment preceded this ticket.
- Keeps: the Linux and macOS packages load the same library files as today. The package's public API.
- Changes: a native library resolver, or plain `runtimes/` probing, picks the library per platform. The NuGet layout gains the Windows entry. The Unix-only tests get skips with reasons. `release.yml` puts the DLL into the package.
- Proof: a load and smoke case on `windows-2025` that counts loopback requests. The Linux and macOS C# checks and the installed-file check stay green.
- Defers: the main unknown, whether `NativeLibrary.SetDllImportResolver` or plain `runtimes/` probing is enough on every .NET version the binding supports. The builder tests each supported version and records the answer.
