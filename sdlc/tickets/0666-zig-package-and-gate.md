# 0666: Package Zig for every supported target and simplify its gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688, 0728

## Outcome

- Changes: `libraries/zig/build.zig`, `libraries/zig/build.zig.zon`, `libraries/zig/src/thinkthen.zig`, `libraries/zig/check.sh`, `libraries/zig/Tests/package_local.py`, `libraries/zig/README.md`, `libraries/zig/README-PUBLIC.md`, `sdlc/tools/zig.json`, `sdlc/scripts/release-pack.d/zig`, `sdlc/scripts/coverage_installed_c_hosts.py`.
- Zig adds darwin-arm64 and linux-arm64 to its `platforms` and `check_platforms` in `sdlc/tools.json`. The package carries the native archive for each supported target. `linkNative` in build.zig selects it by target and links that target's system libraries.
- Engine construction calls `thinkthen_version` once and compares the number it writes with the header's `THINKTHEN_INTERFACE_NUMBER`, as 0692's contract says. `headerVersionMatches` and the header symbol search in build.zig are deleted.
- The supported Zig version range is stated in the README. Its lowest and newest versions are both in the tool manifest, and the gate tests both.
- The smoke profile in check.sh links through `zig build` instead of a hand-written list of system libraries.

### Proof

- On the M5 through `pm job run --machine m5 0666 SHA --memory 8 -- sh libraries/zig/check.sh`, an outside Zig project builds and runs `examples/decide.zig` with `zig build` against the packed darwin-arm64 package, with each manifest Zig version. Before the change the check refuses darwin-arm64 and `linkNative` panics off x86_64-linux-gnu.
- A Zig test that loads a library whose interface number differs fails construction with the 0692 mismatch message.
- linux-arm64 runs at the 0619 container checkpoint on the M5, and linux-x86_64 in the 0619 x86_64 container on Yellow.

## Evidence

- `linkNative` in `libraries/zig/build.zig` panics on anything but x86_64-linux-gnu, "in this rehearsal", and links a fixed list of system libraries for the static mode.
- `headerVersionMatches` in build.zig parses `#define THINKTHEN_VERSION_*` lines out of the C header by hand, and `linkNative` searches the header text for three symbol names. 0692 landed `thinkthen_version` and `THINKTHEN_INTERFACE_NUMBER` in `libraries/c/include/thinkthen.h`, which `src/native.zig` already imports. No Zig code calls `thinkthen_version` yet. The 2026-10-19 no-version ruling keeps this check as a build identity.
- `sdlc/tools.json` lists Zig 0.15.2 only, and Zig's `platforms` and `check_platforms` list only linux-x86_64. `build.zig.zon` sets `minimum_zig_version` to 0.15.2.
- `part_source_wrapper` in `sdlc/scripts/release-pack` packs Zig with the one native archive of the running target. `sdlc/scripts/coverage_installed_c_hosts.py` copies the Zig package with a fixed `native/x86_64-unknown-linux-gnu` folder.
- The smoke profile in `libraries/zig/check.sh` runs `zig build-exe` with `-lgcc_s -lutil -lrt -lpthread -lm -ldl`.
- The Zig packet dumper is 0716's work. Assembling one package from several runners' archives, if the design needs it, uses the 0620 per-target release entrypoint.
