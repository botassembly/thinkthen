# 0351: The macOS static library and R package export no SQLite names

Status: COMPLETE.

Opened as: 2026-10-11. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B5. Pays the macOS part of Debt 001, `sdlc/issues/closed/2026-09-30-static-library-exports-sqlite-symbols.md`. Starts after ticket 0335 slice 2 lands, because that slice edits `libraries/r/check.sh` and `libraries/r/thinkthen/src/Makevars.in`. Its M5 proof should come before Ian dispatches the release rehearsal (ticket 0128 phase 3b), so the rehearsal's macOS jobs confirm it.

## Outcome

On macOS, `libraries/c/localize.sh` writes a `libthinkthen.a` that defines only the header's `thinkthen_` functions, as it does on Linux. The macOS R package's shared object exports no `sqlite3_` name. A check proves both on the M5.

## Evidence

- Starts from: the debt issue. On Linux, 0304 slice 3b joins the needed members with `ld -r`, localizes every other global with `objcopy --keep-global-symbol`, and gates the set in `libraries/c/tests/door/main.rs` `the_static_library_exports_exactly_the_header_symbols`; R links with `-Wl,--exclude-libs,ALL` and `libraries/r/check.sh` requires `nm -D` to show no `sqlite3_` name. On macOS, `localize.sh` copies Cargo's archive unchanged. One M5 try of `cc -r -nostdlib -arch arm64 -Wl,-exported_symbols_list,LIST` wrote an object Apple's `nm` refused, because Homebrew Rust 1.95 embeds LLVM 22 bitcode the Apple tools cannot read. Ticket 0336 ran the PostgreSQL build on the M5, which has zerobrew, not Homebrew.
- Keeps: the Linux archive, its gate and the Go, C++ and Zig checks that read it; the R package's Linux link; `release-pack`'s layout; the header's 31 functions.
- Changes: `localize.sh` on macOS strips embedded bitcode first, or runs the partial link with the LLVM tools that match Rust's; the builder records which worked. The R package's macOS link hides every name its routine registration does not need, through `-exported_symbols_list` or the matching linker flag. The symbol checks gain a macOS form. Mach-O names carry a leading underscore, so the macOS check strips it before comparing with the header's names and before looking for `sqlite3_`. The door harness finds `libthinkthen_c.dylib` on macOS, where it looks for `libthinkthen_c.so` today. The R check reads the package's Mach-O shared object with `nm -gU` in place of `nm -D`. If the door harness cannot run on the M5 without wider changes, a macOS-only check script runs the same exact-set comparison on the archive, and the builder records why. The issue moves to `closed/`.
- Proof: on the M5, the exact-set gate passes on the macOS archive and fails once on Cargo's unchanged archive, and `libraries/r/check.sh` shows no `sqlite3_` name in the package's shared object. On Linux, the C door tests, the Go, C++, Zig and R checks pass unchanged. `policy.py`, `tickets`, and `lint` in a clean checkout.
- Defers: the full macOS surface run, which the release rehearsal does.

## What the build taught us

- The M5 proof ran on macOS 26.4, arm64, Apple ld-1267 and Apple nm (LLVM 21), with zerobrew's Rust 1.95 (LLVM 22.1.3), in a scratch copy removed afterwards.
- Stripping the bitcode worked; no matching LLVM linker was needed. Apple's linker runs the partial link with `-exported_symbols_list`, which turns every other global local. The object still carries `__LLVM,__bitcode` and `__LLVM,__cmdline` from Rust's standard library, and Apple's `nm` reads it as LLVM 22 bitcode it cannot parse. `rust-objcopy --remove-section` drops both. Every Rust toolchain ships `rust-objcopy` in its sysroot, zerobrew's and the official rustup 1.95 alike, so the release runners need nothing new. Apple's `bitcode_strip -r` fails on this Xcode, because it calls the new linker with an option it lacks.
- Apple's `nm` refuses even Cargo's unchanged archive for one standard library member, and still lists the other members' names. The gate therefore reads names without requiring `nm` to exit 0, so Cargo's archive fails on its 304 `sqlite3_` names.
- The door harness needed two changes to run its gate on macOS: the `.dylib` file name and the leading underscore. The M5 ran that one test by name; the other door tests still use `readelf` and the Linux linker.
- On the M5, a C program linked its own SQLite 3.50.0 beside the new archive and ran. Cargo's unchanged archive gave 274 duplicate symbols.
- R on macOS needs only `_R_init_thinkthen` exported. The installed package reached native code with 21 registered routines. Without the flag, `check.sh` refused 285 `sqlite3_` names.
- The M5's zerobrew R names `libR.dylib` by a path that does not exist, so the package's `document` step cannot run there. The M5 proof skipped that step in its scratch copy only; the tarball shape skips it the same way. The full R check also stops there as not run, because `dbplyr` and `igraph` are missing.
- DuckDB's extension passes `--exclude-libs,ALL` on Linux only, so its macOS build may export `sqlite3_` names as well. This ticket did not check it; the release rehearsal's macOS DuckDB job should.
- The DuckDB gap is filed as Debt 026, `sdlc/issues/2026-09-30-duckdb-macos-extension-may-export-sqlite-names.md`.
