# rusqlite's loadable headers stop at SQLite 3.34

Status: Open

Found by experiment 207 (`experiments/207-thinkthen-db/sqlite/NOTES.md`, first entries). Nobody opens an issue upstream. Ian decides.

## What breaks

rusqlite 0.40.2 with `loadable_extension` routes the SQLite calls through `libsqlite3-sys`, whose bundled headers are from the SQLite 3.34 era. Any call added after 3.34 is missing from the routed API. The one that matters here is `sqlite3_is_interrupted`, added in SQLite 3.41.0 on 2023-02-21, which is the extension's only way to see an interrupt while a function call blocks.

A second trap from the same family: rusqlite refuses `loadable_extension` beside `load_extension`, and a workspace unifies features through `libsqlite3-sys`, so any harness in the same workspace that loads extensions through rusqlite breaks the loadable bindings with "SQLite API not initialized".

## Smallest reproduction

Build a loadable extension on rusqlite 0.40.2 with `loadable_extension` + `functions` and call `sqlite3_is_interrupted` from inside a function. The symbol is absent. The 207 fix declares that one call against the host library directly with an `extern "C"` block and links `-lsqlite3`; `ldd` then shows `libthinkthen0.so` gains `DT_NEEDED libsqlite3.so.0`.

## What it costs

Every post-3.34 call must be declared by hand against the host library, and the binary link to the host SQLite becomes part of the build. The support floor is stated as SQLite 3.41 with visible symbols, and the macOS system SQLite is the holdout. rusqlite stays regardless: the alternative cannot register an aggregate at all.

## The decision that is Ian's

Whether to ask rusqlite to refresh the bundled headers for the loadable path, keep the direct declarations, or both. Nobody opens an issue upstream without his word.
