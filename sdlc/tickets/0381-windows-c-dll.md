# 0381: Windows stage 1: the C library ships as a DLL for Windows x86-64

Status: ready. It waits for the `release/0.1` cut and for ticket 0380 slice A, which adds the fifth release target. No slice lands on main before the cut (ADR 0116 item 2). Plan: `sdlc/planning/windows.md`, stage 1. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. The coordinator assigns a lane after the cut.

Milestone: 0.2

## Outcome

- The Windows target builds `thinkthen.dll`, its import library `thinkthen.dll.lib` and a static library, and packs them with the header.
- The DLL exports exactly the C door's public functions. A static link against the static library exposes no Rust standard library symbol, as on Linux and macOS.
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
- Defers: the main unknown, how to hide the Rust standard library's symbols in a COFF static library. If slice B finds no clean way, the builder ships the DLL alone, files the static library as an issue for stage 3, and says so in `DESIGN.md`. `fork.c` stays Unix-only. The stage 2 bindings that load this DLL (C++, Go, Ruby, R, PHP, Swift, Dart, Zig) wait for stage 2.
