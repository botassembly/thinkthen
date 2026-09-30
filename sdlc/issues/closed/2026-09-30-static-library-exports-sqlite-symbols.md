Status: Closed by ticket 0351, which localizes the macOS archive and hides the macOS R package's names; both proved on the M5. Filed 2026-09-30 by ticket 0304 slice 3a. Ticket 0304 slice 3b fixed Linux.

Kind: debt

Pay when: before 0.1. Ticket 0304 slice 3b paid the Linux part; ticket 0351 paid the macOS part.

Paid: 2026-09-30

Debt: 001

Severity: medium

Keeping it ships a macOS static library and R package whose `sqlite3_` names clash with a consumer's own SQLite.

# The static libraries export the bundled SQLite's symbols

## What happens

The library bundles SQLite for the question store, by ADR 0111 section 3. The shared library keeps SQLite's symbols private. Cargo's static library does not. `libthinkthen_c.a` exports about 285 global `sqlite3_` symbols and Rust's own runtime names. A program that links it beside its own SQLite meets duplicate symbols at link time, or silently mixes two SQLite builds.

## What slice 3b did

`libraries/c/localize.sh` writes the release `libthinkthen.a` from Cargo's archive. On Linux it joins the members the header's functions need with `ld -r`, turns every other global name local with `objcopy --keep-global-symbol`, and archives the one object. `release-pack` and the Go, C++ and Zig checks use it. `libraries/c/tests/door/main.rs` `the_static_library_exports_exactly_the_header_symbols` gates the exact set: the header's 30 functions and nothing else. The Go check's `fixtures/abi.py` asserts the same set on the archive it installs.

R links its static library into the package's shared object and deletes the archive, so R ships no archive. That shared object still exported about 285 `sqlite3_` names. On Linux the package now links with `-Wl,--exclude-libs,ALL`, and `libraries/r/check.sh` requires `nm -D` to show no `sqlite3_` name. R finds its routines through their registration, so no name needs to stay global.

## What 0351 found on macOS

On macOS, `localize.sh` copies Cargo's archive unchanged and says so on standard error. The R package's shared object on macOS has no `--exclude-libs` step either. One M5 try on 2026-09-30 ran `cc -r -nostdlib -arch arm64 -Wl,-exported_symbols_list,LIST` with `-u` for each header function. It wrote an object that Apple's `nm` refused with "Unknown attribute kind (105) (Producer: 'LLVM22.1.3' Reader: 'LLVM APPLE_1_2100.1.1.101_0')". The Homebrew Rust 1.95 embeds LLVM 22 bitcode, which the Apple tools read as bitcode after the partial link. A fix might strip the embedded bitcode first, or run the partial link with the LLVM tools that match Rust's.

## What 0351 did

On macOS, `localize.sh` runs the same partial link with Apple's linker and `-exported_symbols_list`, then removes the `__LLVM,__bitcode` and `__LLVM,__cmdline` sections with the `rust-objcopy` that ships inside every Rust toolchain. Apple's `nm` then reads the object, and the door gate passes on the M5. The R package's macOS link passes `-Wl,-exported_symbol,_R_init_thinkthen`, and `check.sh` reads the shared object with `nm -gU`.

## Done when

The macOS archive defines only the header's `thinkthen_` functions, the macOS R package exports no `sqlite3_` name, a check proves both on M5, and this issue moves to `closed/`.
