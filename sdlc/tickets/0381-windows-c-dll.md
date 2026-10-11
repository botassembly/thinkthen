# 0381: Windows stage 1: the C library ships as a DLL for Windows x86-64

Status: COMPLETE.

Opened as: 2026-10-11. C DLL/MSVC loading and native behavior passed on c64b71859 in runs 37409599783 and 37409602101. Static-library distribution remains deferred; 0425 owns final expanded-commit qualification.

Milestone: 0.2

## Outcome

- The Windows target builds `thinkthen.dll` and its import library `thinkthen.dll.lib`, and packs them with the header in their own archive.
- The DLL exports exactly the C door's public functions.
- Either a static link against the static library exposes no Rust standard library symbol, as on Linux and macOS, or the DLL ships alone, `DESIGN.md` says so, and an issue for stage 3 holds the static library.
- The door's C tests build and pass with MSVC on `windows-2025`, apart from `fork.c`, which stays Unix.
- `libraries/c/DESIGN.md` states the DLL, import library and static library rules.
- Linux and macOS libraries are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0). `libraries/c` builds, passes Clippy and passes its Rust tests on Windows. Its C tests build C with a Unix compiler under ASan, so they stay on Unix today. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 400 to 900 lines and 2 slices, with low to medium Linux and macOS risk: `build.rs` and the pack script are shared, but each change sits in a Windows arm.
  - `libraries/c/localize.sh` hides Rust's symbols with `nm`, `ld -r` and `objcopy`. MSVC has no twins for them.
  - The door tests use `cc`, ASan, `readelf` and `fork.c`.
  - No experiment preceded this ticket.
- Keeps: the C header, every exported name and every Linux and macOS archive's contents. The `.pc` file's meaning on Unix. The rule that a static link exposes no Rust standard library symbol.
- Changes: slice A, the DLL: `libraries/c/build.rs` makes the import library and a `.def` export list; `release-pack` and `release.yml` build and pack the DLL, import library and header on the Windows target; an export check with `dumpbin`; the MSVC build of the door's C tests. Slice B, the static library: hide the Rust internal symbols in the COFF static library, the `.pc` file's `Libs.private` for Windows, and a static-link smoke. `DESIGN.md` gains the rules in each slice.
- Proof: `dumpbin /exports` lists exactly the header's functions. The MSVC door tests pass on the runner. MSVC's ASan gets its own check, or the builder records why it cannot run. A static-link smoke links a small C program and finds no Rust standard library symbol. The Linux and macOS C door tests and `release-pack` self-tests stay green.
- Defers: Windows static-library localization, coexistence and metadata wait for stage 3 in [the static-library issue](../issues/2026-10-05-windows-static-library-waits-for-stage-3.md). No native localization experiment ran or failed. `fork.c` stays Unix-only. The stage 2 bindings that load this DLL (C++, Go, Ruby, R, PHP, Swift, Dart, Zig) wait for stage 2.
