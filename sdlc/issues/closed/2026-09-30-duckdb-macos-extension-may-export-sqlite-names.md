Status: Closed by the quick fix landed as `Land quick fix: the fourth rehearsal's build failures and the macOS DuckDB SQLite exports`. Found while building ticket 0351. Confirmed on the M5 from the fourth rehearsal's commit `91878c233`, because that run's macOS jobs stopped before packing DuckDB.

Kind: debt

Pay when: before 0.1, if the rehearsal's macOS DuckDB extension shows a `sqlite3_` name.

Debt: 026

Paid: 2026-09-30

Severity: medium

Keeping it may ship a macOS DuckDB extension whose `sqlite3_` names bind to another extension's or the host's SQLite.

# The macOS DuckDB extension may export SQLite's names

## The problem

Since ticket 0304 slice 3a, the DuckDB bridge bundles SQLite for the question store. `databases/duckdb/cpp/CMakeLists.txt` hides the bridge's names with `-Wl,--exclude-libs,ALL` on Linux only (lines 69 and 70). The macOS link has no matching step, so the extension may export the bundled SQLite's names. Ticket 0351 fixed the same gap in the C archive and the R package, and did not build DuckDB.

## A fix

Check the built macOS extension with `nm -gU` for `sqlite3_`. If any show, pass Apple's linker `-exported_symbol` for the extension's entry points only, as the R package does with `_R_init_thinkthen`, and add that `nm -gU` count to the DuckDB check on macOS.

## Resolution

The debt was real. The macOS ARM extension built at `91878c233` exported 285 `_sqlite3_` names, counted with `nm -gU`, out of 39,527 exported names.

On Apple, `databases/duckdb/cpp/CMakeLists.txt` now links with `-unexported_symbol,_sqlite3_*`. After `strip_macos.py`, `databases/duckdb/cpp/build.sh` counts the stripped extension's `_sqlite3_` exports with `nm -gU` and stops the build unless the count is 0. On the M5, the rebuilt extension exports 39,242 names, exactly 285 fewer, and none is a SQLite name.

The fix hides only SQLite's names, not everything but the entry points as "A fix" proposed. Exporting only the entry points would also hide DuckDB's C++ names, such as type information that exceptions need when they cross between the host and the extension. The Linux link hides the bundled archives' names with `--exclude-libs,ALL` and keeps the extension's own. The macOS link now hides the one bundled library that can clash.
