# 0641: Make the C archive link with plain pkg-config on every platform

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0619

## Outcome

- Changes: `sdlc/scripts/release-pack`, `sdlc/scripts/installed.sh`, `sdlc/scripts/coverage_installed_c_hosts.py`, `libraries/c/build.rs`, `libraries/c/localize.sh`, `libraries/c/check.sh`, `libraries/c/README.md`, `libraries/c/README-PUBLIC.md`.
- The archive's `lib/pkgconfig/thinkthen.pc` gives separate shared and static link lines, so `pkg-config --libs --static thinkthen` works unedited on Linux and macOS. The Windows archive keeps its `lib/thinkthen.dll.lib` import library, and the README documents the MSVC command.
- Symbol hiding moves into the Rust build (an export list or version script passed by `build.rs`) where the linker supports it. `localize.sh` shrinks to what remains, or is deleted.
- Coverage-only branches leave `localize.sh`; the coverage driver passes what it needs itself.
- The README's compile command has no `sed`.
- The routine source check in `libraries/c/check.sh` links the platform's own library name, so it runs on macOS as well as Linux.

### Proof

- On the M5 natively: `pm job run --machine m5 0641 SHA --memory 8 -- sh -c 'out=$(mktemp -d) && sh sdlc/scripts/release-pack aarch64-apple-darwin "$out" c && THINKTHEN_ARTIFACT="$(ls "$out"/thinkthen-c-*.tar.gz)" sh libraries/c/check.sh'`. The installed check compiles an outside C program against the unpacked archive with only the README's `pkg-config` command, and a second program that also links its own SQLite links without symbol clashes. Before the change the README command without `sed` does not link the static archive.
- On the M5 natively, `sdlc/scripts/surfaces` runs `libraries/c/check.sh` in its source mode; before the change it fails on macOS copying `libthinkthen_c.so`.
- Linux arm64 and x86_64 run at the 0619 container checkpoint. Windows and Intel macOS are proven on the candidate in 0687.

## Evidence

- The documented C link command strips `-lthinkthen` out of pkg-config's output with `sed` (`libraries/c/README.md`, "Use a prebuilt development archive").
- `sdlc/scripts/release-pack` `part_c` writes `thinkthen.pc` with one `Libs: -L${libdir} -lthinkthen` line and a per-platform `Libs.private`, and builds the static library through `localize.sh`. `sdlc/scripts/installed.sh` `native_install` and `sdlc/scripts/coverage_installed_c_hosts.py` also call `localize.sh`.
- `libraries/c/localize.sh` hides symbols by partial linking, with a separate macOS path that removes LLVM bitcode using `rust-objcopy` and `THINKTHEN_COVERAGE_INITIALIZER` branches on both paths.
- The README says only Linux static linking is proven. `libraries/c/check.sh` in source mode copies `target/debug/libthinkthen_c.so` and links `libthinkthen.so`, though the manifest declares Darwin support.
- `sdlc/scripts/release-windows-c.py` already packs `include/thinkthen.h`, `bin/thinkthen.dll` and `lib/thinkthen.dll.lib`.
- Recent repairs for MSVC and Windows output show the other platforms are untested until late: 0abd01a6f, 1b250072f, dc155aec2.

## Progress

- 2026-10-19 PM assigned the routine C source-check Darwin linking defect here: libraries/c/check.sh copies target/debug/libthinkthen_c.so and links libthinkthen.so despite declaring Darwin support; 0636 proves the installed CLI, Rust and C entrypoints and makes no source-linking repair.
