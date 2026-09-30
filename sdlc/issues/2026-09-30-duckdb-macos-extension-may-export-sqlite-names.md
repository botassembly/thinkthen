Status: open. Found while building ticket 0351. Owner: none yet; the release rehearsal's macOS DuckDB job can confirm it.

Kind: debt

Pay when: before 0.1, if the rehearsal's macOS DuckDB extension shows a `sqlite3_` name.

Debt: 026

Severity: medium

Keeping it may ship a macOS DuckDB extension whose `sqlite3_` names bind to another extension's or the host's SQLite.

# The macOS DuckDB extension may export SQLite's names

## The problem

Since ticket 0304 slice 3a, the DuckDB bridge bundles SQLite for the question store. `databases/duckdb/cpp/CMakeLists.txt` hides the bridge's names with `-Wl,--exclude-libs,ALL` on Linux only (lines 69 and 70). The macOS link has no matching step, so the extension may export the bundled SQLite's names. Ticket 0351 fixed the same gap in the C archive and the R package, and did not build DuckDB.

## A fix

Check the built macOS extension with `nm -gU` for `sqlite3_`. If any show, pass Apple's linker `-exported_symbol` for the extension's entry points only, as the R package does with `_R_init_thinkthen`, and add that `nm -gU` count to the DuckDB check on macOS.
