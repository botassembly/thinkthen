Status: Open. Filed 2026-09-30 by ticket 0304 slice 3a. Ticket 0304 slice 3b fixes it.

# The static libraries export the bundled SQLite's symbols

## What happens

The library bundles SQLite for the question store, by ADR 0111 section 3. The shared library keeps SQLite's symbols private. The static libraries do not. `libthinkthen.a` from the C door exports about 285 global `sqlite3_` symbols, and the R binding's static library does the same. A program that links one of them beside its own SQLite meets duplicate symbols at link time, or silently mixes two SQLite builds.

The slice 2 check runs `nm -D` on `libthinkthen.so` only, so it never saw the static library.

## Proof

`libraries/c/tests/door/main.rs` `the_static_library_still_exports_sqlite_symbols_until_slice_3b` runs `nm -g --defined-only` on the archive's `libthinkthen.a`. It asserts at least one `sqlite3_` symbol, so it pins the known failure and fails once the leak is fixed. The inverted test is a placeholder, not a gate. It must become a gate that requires zero `sqlite3_` symbols before any release.

## Where to fix it

Slice 3b of ticket 0304. Link the static library's objects into one relocatable object with a partial link (`ld -r`), localize every symbol outside the header's `thinkthen_` names, and archive that object. Apply the same step to R's static library. Then flip the test to require zero `sqlite3_` symbols.

## Done when

Neither static library exports a `sqlite3_` symbol, the C door test asserts zero as a gate that runs before any release, and this issue moves to `closed/`.
