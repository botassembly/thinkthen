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

## Lane B item 8 findings, 2026-09-21 (branch copy; the main copy is the record)

**The road is the one already taken, plus a load-time floor check.** Vendoring `sqlite3.c` is rejected with reasons: a loadable extension rides the host's own SQLite, and a second copy in-process invites symbol interposition and adds roughly 1.5–2 MB to a 4.4 MB artifact for no behavior gain. The direct link stays; `init` now calls the routed `sqlite3_libversion_number()` and refuses a host below 3.41.0 by name — `thinkthen needs SQLite 3.41.0 or newer (the interrupt check uses sqlite3_is_interrupted); this host is X` — through a pure `version_refusal` that is unit-tested, with the live call exercised by the stock-CLI load (host 3.53.4 passes). An old-host live test is impossible on this box and is recorded as such. Numbers and the full evaluation: `databases/sqlite/NOTES.md`, "lane B item 8".
