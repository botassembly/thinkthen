# 0388: The macOS DuckDB extension exports no Rust names

Status: in progress. Lane claude-2. Branch `ticket/0388-macos-duckdb-exports`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Parent: ticket 0128, phase 3b.

Milestone: 0.1

## Outcome

1. On both Macs, the shipped `thinkthen.duckdb_extension` exports no name from the Rust bridge archive. Its Rust standard library, its panic hook state and its bundled SQLite stay out of the export table, as `--exclude-libs,ALL` already keeps them out on Linux.
2. The installed DuckDB check's `own_panic_hook` case passes on both Macs unchanged in what it checks. It still fails any shipped library that imports or exports a panic symbol.
3. The macOS DuckDB build refuses an extension that exports any Rust or SQLite name, so the leak fails the build job, not only the smoke job.
4. A failing `own_panic_hook` names up to three of the panic symbols it found. The `printf: write error: Broken pipe` line no longer appears.

## Evidence

- Starts from: rehearsal run 36903959951 at main `142e8e5af`.
  - ARM Mac smoke job 110530622023 failed `databases/duckdb` with `FAIL thinkthen.duckdb_extension exports a panic symbol`. The same job passed `own_panic_hook` for `thinkthen.bundle`, `_thinkthen.abi3.so`, `thinkthen-darwin-arm64.node`, `libthinkthen.dylib` and `libthinkthen0.dylib`. Every other surface passed. PostgreSQL does not run the case; its macOS `thinkthen.dylib` exports 99 names and none names a panic.
  - The run's ARM Mac DuckDB archive holds a Mach-O dynamic library (`MH_DYLIB`) with flags `NOUNDEFS DYLDLINK TWOLEVEL WEAK_DEFINES BINDS_TO_WEAK NO_REEXPORTED_DYLIBS MH_HAS_TLV_DESCRIPTORS`. `nm -gU` lists 39,247 exported names, 106 of them naming a panic. They include `std::panicking::HOOK` (data), `std::panicking::set_hook`, `take_hook`, `default_hook`, `rust_panic` and `__rust_start_panic`. All 106 appear in dyld's export trie (`llvm-objdump --macho --exports-trie`). None is weak, and none appears in a bind, lazy-bind or weak-bind entry.
  - The Intel Mac build job passed, and its DuckDB archive has the same fault: 43,008 exported names, 106 naming a panic, including `std::panicking::HOOK` and `set_hook`. Its smoke job was still running when this ticket was written; it runs the same check on that file.
  - The run's x86 Linux DuckDB extension exports 177 dynamic names. None names a panic and none is a Rust symbol. They are `thinkthen_duckdb_cpp_init`, the C callback `thinkthen_cpp_interrupt_busy`, and C++ names from the extension's own source files.
  - Cause: `databases/duckdb/cpp/CMakeLists.txt` links the Rust bridge archive as a plain library on both systems. On Linux, `-Wl,--exclude-libs,ALL` hides every archive's names. On macOS, only `-unexported_symbol,_sqlite3_*` (from `50edff253`) hides anything, so every global name in the Rust archive and the DuckDB archives stays exported. The Rust cdylibs pass because rustc gives the linker its own export list.
  - The Darwin reading is right. On Mach-O, `nm -gU` lists the external defined names, and the export trie confirms each is visible to `dlsym`. The Linux reading `nm -D --defined-only` reads the same thing for ELF.
  - What the leak breaks: macOS two-level namespace binds the extension's own references to its own names at link time, and none of the panic names is weak. So no other image can supply the extension's hook state. The names are exported, though, so any image can reach `HOOK`, `set_hook` and `take_hook` through `dlsym` or a flat-namespace lookup. Ticket 0374's promise reads "no other library can supply it or reach it", and ADR 0098 asks that the hook state stay the binding's own. The shipped file breaks the "reach" half. The check is right and stays.
  - The broken pipe line comes from `printf ... | grep -qi panic`: `grep -q` exits at the first match while the shell's `printf` still writes the 39,247-line export list.
- Keeps: `own_panic_hook`'s four tests and their Linux and macOS readings; the extension's entry `thinkthen_duckdb_cpp_init` and its DuckDB archives' exported names (the C++ names the host may share, left as they are); the Linux link line; the build's SQLite export guard, widened to Rust names; `strip_macos.py` and the footer.
- Changes: one link option, one build guard, one helper.
  - `databases/duckdb/cpp/CMakeLists.txt`: on Apple, link the Rust bridge archive with `LINKER:-load_hidden,<archive>`. Apple's `ld` documents it as "treats all global symbols from the static library as if they are visibility hidden. Useful when building a dynamic library that uses a static library but does not want to export anything from that static library." It covers the bundled SQLite, so `-unexported_symbol,_sqlite3_*` goes. Linux keeps its plain link and `--exclude-libs,ALL`.
  - `databases/duckdb/cpp/build.sh`: the macOS export guard counts SQLite names (`_sqlite3_`) and Rust names, and fails on any. Rust names here come in two manglings: v0 names start `__R` (the standard library's, 1,673 exported today), and legacy names end in `17h` and a 16-digit hash (2,580 today). The run's ARM Mac file matches the pattern 4,253 times; the Linux file matches none.
  - `sdlc/scripts/installed.sh`: `own_panic_names` lists up to three panic names with `awk`, which reads all its input. The import and export tests use it and print the names they found.
- Proof: an M5 build, the stock host, the two guards, and lint.
  - On the M5, `databases/duckdb/cpp/build.sh` builds the branch's extension. `nm -gU` on it lists no panic name and no Rust name in either mangling, and `own_panic_hook` passes on it. `thinkthen_duckdb_cpp_init` is still exported.
  - On the M5, the stock DuckDB v1.5.5 CLI loads that extension and answers `thinkthen_decide` from a loopback backend, and `verify_package.py` and `verify_interrupt.py` pass on it.
  - `own_panic_hook` on the run's ARM Mac extension, read on the M5, still fails and names three panic symbols, with no broken pipe line. A plant that drops `-load_hidden` fails `build.sh` with its export count.
  - On this Linux host, with Ubuntu's default `awk` (mawk), `own_panic_hook` still passes on the run's x86 Linux DuckDB extension, its C archive's `libthinkthen.so` and a release `libthinkthen_c.so`. Two tiny libraries that link libc fail with their own lines: one exporting `rust_panic_x` and `core_panicking_y` prints `exports a panic symbol: core_panicking_y rust_panic_x`, and one importing `rust_panic_q` prints `imports a panic symbol: rust_panic_q`.
  - `sdlc/scripts/lint` in full, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` and `sdlc/scripts/tickets`.
  - No workflow is dispatched. The coordinator's next rehearsal proves both Mac smoke jobs.
- Defers: the DuckDB archives' C++ names stay exported on macOS, about 39,000 of them, including weak definitions that dyld may coalesce with the host's: `llvm-nm-18 -gU -m` counts 2,154 weak externals in the ARM Mac file and 5,857 in the Intel Mac file. Hiding them could change how C++ exceptions and type identity cross between host and extension, because Apple's libc++ compares type information by address for default-visibility types. That needs its own evidence and ticket. No panic name is among them.
