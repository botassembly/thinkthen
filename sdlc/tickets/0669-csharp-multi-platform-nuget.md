# 0669: Ship and check the C# package on every supported platform

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- Changes: `libraries/csharp/ThinkThen.csproj`, `libraries/csharp/Botassembly.ThinkThen.nuspec`, `libraries/csharp/src/NativeLoader.cs`, `libraries/csharp/check.sh`, `libraries/csharp/tests/`, `libraries/csharp/README.md`, `build-tools/results/csharp/`, `sdlc/tools.json`, `sdlc/scripts/package-inventory.py`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-managed-pair.py`.
- One NuGet package carries the native library for every supported runtime under standard `runtimes/<rid>/native` folders, so the default .NET loader finds it.
- The custom resolver in `NativeLoader.cs` and its filename special case are deleted, or reduced to what the standard layout cannot do.
- The native library name cannot collide with the managed assembly name.
- Supported runtimes come from the `csharp` entry under `libraries` in `sdlc/tools.json`. Windows arm64 is either supported or listed as unsupported there. `check_platforms` adds `darwin-arm64` and `linux-arm64`.

### Proof

- On the M5 through `pm job run --machine m5 0669 SHA --memory 8 -- sh libraries/csharp/check.sh`, the C# check runs natively on macOS arm64: it installs the single package from a local feed, runs a named call with no library path. Before the change the check exits 77 on the M5 because `sdlc/tools.json` declares only `linux-x86_64`.
- The same check in the arm64 container runs at the 0619 container checkpoint.
- Windows x64 and Linux x64 are proven on the candidate in 0687.

## Evidence

- Order (coordinator, 2026-10-19): the packaging ticket 0669 lands before the call ticket 0668, so 0668 can prove on the M5. The packet dumper belongs to 0668, written once against the run call.
- **Late Windows qualification.** Windows was qualified on the release day. Four commits on 10-17 fixed Windows loading the managed `ThinkThen.dll` in place of the native `thinkthen.dll` (547dae8ef, 6dc2d80ff, b3e9dd194, 98322d5ff).
- **Filename hack.** `Resolve` in `libraries/csharp/src/NativeLoader.cs` skips a candidate path equal to the managed assembly's location on Windows.
- **Windows on arm64 refused.** The same `Resolve` throws `PlatformNotSupportedException` for Windows on anything but x64.
- **One runtime per pack.** `ThinkThen.csproj` packs one `ThinkThenNativeRid` per pack, and `check.sh` packs only `linux-x64` with `libthinkthen.so`. `sdlc/scripts/release-workflow` packs per target from `csharp_inventory` in `sdlc/scripts/package-inventory.py`, and `sdlc/scripts/release-managed-pair.py` (`check_nupkg`) checks the assembled package.
- **One checked platform.** `sdlc/tools.json` lists five C# platforms but `check_platforms` holds only `linux-x86_64`.
