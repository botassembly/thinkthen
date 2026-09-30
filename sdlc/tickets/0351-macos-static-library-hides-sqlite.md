# 0351: The macOS static library and R package export no SQLite names

Status: ready. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B5. Pays the macOS part of Debt 001, `sdlc/issues/2026-09-30-static-library-exports-sqlite-symbols.md`. Starts after ticket 0335 slice 2 lands, because that slice edits `libraries/r/check.sh` and `libraries/r/thinkthen/src/Makevars.in`. Its M5 proof should come before Ian dispatches the release rehearsal (ticket 0128 phase 3b), so the rehearsal's macOS jobs confirm it.

## Outcome

On macOS, `libraries/c/localize.sh` writes a `libthinkthen.a` that defines only the header's `thinkthen_` functions, as it does on Linux. The macOS R package's shared object exports no `sqlite3_` name. A check proves both on the M5.

## Evidence

- Starts from: the debt issue. On Linux, 0304 slice 3b joins the needed members with `ld -r`, localizes every other global with `objcopy --keep-global-symbol`, and gates the set in `libraries/c/tests/door/main.rs` `the_static_library_exports_exactly_the_header_symbols`; R links with `-Wl,--exclude-libs,ALL` and `libraries/r/check.sh` requires `nm -D` to show no `sqlite3_` name. On macOS, `localize.sh` copies Cargo's archive unchanged. One M5 try of `cc -r -nostdlib -arch arm64 -Wl,-exported_symbols_list,LIST` wrote an object Apple's `nm` refused, because Homebrew Rust 1.95 embeds LLVM 22 bitcode the Apple tools cannot read. Ticket 0336 ran the PostgreSQL build on the M5, which has zerobrew, not Homebrew.
- Keeps: the Linux archive, its gate and the Go, C++ and Zig checks that read it; the R package's Linux link; `release-pack`'s layout; the header's 30 functions.
- Changes: `localize.sh` on macOS strips embedded bitcode first, or runs the partial link with the LLVM tools that match Rust's; the builder records which worked. The R package's macOS link hides every name its routine registration does not need, through `-exported_symbols_list` or the matching linker flag. The door test's symbol gate and the R check's `sqlite3_` check run on macOS too. The issue moves to `closed/`.
- Proof: on the M5, the door test's exact-set gate passes on the macOS archive, and `libraries/r/check.sh` shows no `sqlite3_` name in the package's shared object. On Linux, the C door tests, the Go, C++, Zig and R checks pass unchanged. `policy.py`, `tickets`, and `lint` in a clean checkout.
- Defers: the full macOS surface run, which the release rehearsal does.

## What the build taught us
