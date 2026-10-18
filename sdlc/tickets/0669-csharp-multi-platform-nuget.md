# 0669: Ship and check the C# package on every supported platform

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0668, 0688

## Outcome

- One NuGet package carries the native library for every supported runtime under standard `runtimes/<rid>/native` folders, so the default .NET loader finds it.
- The custom resolver and its filename special case are deleted, or reduced to what the standard layout cannot do.
- The native library name cannot collide with the managed assembly name.
- Supported runtimes come from the shared product inventory. Windows arm64 is either supported or listed as unsupported in that inventory.
- The C# conformance runner becomes a packet dumper following the 0642 contract, and the old fixture consumer is deleted.

### Proof

- On the M5 through pm job, natively and in the pinned arm64 container, the installed-consumer check installs the single package from a local feed and runs a named call with no library path, and its dump matches the 0642 oracle.
- Windows x64 and Linux x64 are proven on the candidate in 0687.

## Evidence

- **Late Windows qualification.** Windows was qualified on the release day. Four commits on 10-17 fixed Windows loading the managed `ThinkThen.dll` in place of the native `thinkthen.dll` (547dae8ef, 6dc2d80ff, b3e9dd194, 98322d5ff).
- **Filename hack.** The fix is a filename special case in `libraries/csharp/src/NativeLoader.cs:20-21`.
- **Windows on arm64 refused.** `NativeLoader.cs:15` refuses it.
- **One runtime per pack.** `ThinkThen.csproj:18-23` packs one runtime per pack, and `check.sh:30,84` packs only linux-x64. `sdlc/scripts/release-managed-pair.py:119-137` checks the assembled NuGet package.
- **Silent skip.** `check.sh:10-11` exits 77 when dotnet is missing.
