# 0641: Make the C archive link with plain pkg-config on every platform

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0619

Reviews: revision f8a4d0d, accept

Reviews: revision fcfddcb, accept

Reviews: revision 59e6cd1, accept

## Outcome

- Changes: `libraries/c/tests/door/main.rs`, `sdlc/scripts/release-pack`, `sdlc/scripts/release-pack.d/c`, `sdlc/scripts/installed.sh`, `sdlc/scripts/coverage_installed_c_hosts.py`, `sdlc/scripts/coverage_installed_initial.py`, `libraries/c/build.rs`, `libraries/c/localize.sh`, `libraries/c/check.sh`, `libraries/c/README.md`, `libraries/c/README-PUBLIC.md`.
- The archive's one `lib/pkgconfig/thinkthen.pc` follows standard pkg-config semantics: `--static` adds the native static dependencies and the caller selects static linkage. Add no variable or second package name. The README shows the Linux and macOS static link commands without `sed`; macOS names the archive path because its linker prefers the dylib. The Windows archive keeps its `lib/thinkthen.dll.lib` import library, and the README documents the MSVC command.
- Symbol hiding moves into the Rust build (an export list or version script passed by `build.rs`) where the linker supports it. `localize.sh` shrinks to what remains, or is deleted.
- Coverage-only branches leave `localize.sh`; the coverage driver passes what it needs itself.
- The README's compile command has no `sed`.
- The routine source check in `libraries/c/check.sh` links the platform's own library name, so it runs on macOS as well as Linux.

### Proof

- On the M5 natively: `pm job run --machine m5 0641 SHA --memory 8 -- sh -c 'out=$(mktemp -d) && sh sdlc/scripts/release-pack aarch64-apple-darwin "$out" c && THINKTHEN_ARTIFACT="$(ls "$out"/thinkthen-c-*.tar.gz)" sh libraries/c/check.sh'`. The installed check compiles an outside C program against the real unpacked static archive using the README's platform-specific command and pkg-config dependencies without `sed`, and a second program that also links its own SQLite links without symbol clashes. Before the change the README command without `sed` does not link the static archive.
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
- 2026-10-20 started
- 2026-10-20 next: Fresh fixing review acceptedf8a4d0d39 after corrected job1792491843921-50434 passed real native static linkage own-SQLite dependency and installed-source C checks, preserving first receipt1792491463122-29115 with policy and clippy passes and formatting README failures, while Linux Windows Intel macOS and coverage remain checkpoint proof and the integrated closing gate is pending.
- 2026-10-20 next: Combined gate receipt gate-runs/thinkthen/0632/m5/5f67fbb64aa8499ee58d9f63ab21d3678a3bdca1 stopped before tests at an unsupported README bat fence; fixing review accepted 59e6cd14a with commands unchanged and M5 job 1792494861621-11079 passed the named-answer page public-README and whitespace checks, while the repaired batch closing gate remains required.

## Existing caller paths, 2026-10-20

The release-pack dispatcher now delegates C packaging to release-pack.d/c. The initial coverage driver also calls localize.sh and must retain its distinct collection when that calling convention changes. The claim follows these existing implementation and caller paths; the Outcome is unchanged.

## Static linkage ruling, PM 2026-10-20

Use one conventional thinkthen.pc. Its --static output supplies static dependencies; callers choose the archive with the documented platform command. The native proof must link that real archive, not accidentally select its neighboring shared library. No new pkg-config variable or second package is introduced.

## Native source check formatting, 2026-10-20

The first native proof reached an existing rustfmt difference in the door fixture. Apply only the pinned formatter whitespace change to its assert_eq invocation; preserve the exact tokens, assertions and behavior. The same proof reached the public README line guard before archive creation, so archive checks remain unrun until the in-scope README correction.
