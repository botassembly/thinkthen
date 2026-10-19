# 0641: Make the C archive link with plain pkg-config on every platform

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0619

## Outcome

- The archive ships `thinkthen.pc` with separate static and shared link lines, so `pkg-config --libs --static thinkthen` works unedited on Linux and macOS. Windows ships the matching import library and a documented MSVC command.
- Symbol hiding moves into the Rust build (an export list or version script passed by build.rs) where the linker supports it. localize.sh shrinks to what remains, or is deleted.
- Coverage-only branches leave the packaging script.
- The README's compile command has no `sed`.

### Proof

- Through pm job, an outside C program compiles and runs against the unpacked archive using only the README's command: natively on the M5, and in the pinned Linux containers on the M5 (arm64) and Yellow (x86_64). A program that also links its own SQLite links without symbol clashes.
- Windows and Intel macOS are proven on the candidate in 0687.

## Evidence

- The documented C link command has to strip `-lthinkthen` out of pkg-config's output with `sed` (libraries/c/README.md, "Use a prebuilt development archive").
- The release archive is made by libraries/c/localize.sh. That script hides symbols by partial linking, with a separate macOS path that removes LLVM bitcode using rust-objcopy (localize.sh:24-40) and coverage special cases (localize.sh:25-27, 42-45).
- The README says only Linux static linking is proven, and the C gate checks Linux only (check.sh:52-55 copies `libthinkthen_c.so`).
- Recent repairs for MSVC and Windows output show the other platforms are untested until late: 0abd01a6f, 1b250072f, dc155aec2.

## Progress

- 2026-10-19 PM assigned the routine C source-check Darwin linking defect here: libraries/c/check.sh copies target/debug/libthinkthen_c.so and links libthinkthen.so despite declaring Darwin support; 0636 proves the installed CLI, Rust and C entrypoints and makes no source-linking repair.
