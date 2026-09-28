---
opens: sdlc/tickets/0232-macos-sqlite-checks.md sdlc/records/0226-macos-arm64-package-proof.md databases/sqlite/setup.sh databases/sqlite/check.sh databases/sqlite/build.rs databases/sqlite/tests/host_sqlite.sh databases/sqlite/tests/helper.py databases/sqlite/tests/test_schema.py databases/sqlite/tests/load_probe.c databases/sqlite/amalgamation-3490000.sha256 sdlc/scripts/release-smoke
---

# 0232: Check installed SQLite packages on macOS ARM64

Status: proposed checker design for fresh review. Owner: Codex. This ticket serves the macOS ARM64 SQLite package remainder of ticket 0226. Its product source is read-only; it changes only setup and verification code after design review. The isolated M5 checkout is based on pushed `10f85fa7` and visibly reports the unrelated `site/examples/beatles/BENCH` case collision.

## Outcome and retained behavior

The installed macOS ARM64 `libthinkthen0.dylib` must load on a real SQLite 3.50.0 host, return zero local sends from `thinkthen_usage()`, and allow a later host operation. A real SQLite 3.49.0 host must refuse the same archive with the exact safety-floor diagnostic and allow a later host operation. Record the library's dyld residency after both paths. Run the existing installed examples and shared conformance on a supported Python runtime, report its actual SQLite version, and keep their no-network assertions. The already accepted caught-panic child tests remain separate source proof. Do not add a public fault switch, product dependency, release publish, or broad surfaces run.

Linux's existing pinned 3.50.0 Python host, stock below-floor test, `.so` package, and installed route remain. The macOS adaptation must not label the installed Python SQLite as pinned 3.50.0. On M5, `/usr/bin/python3` is 3.9.6 and reports SQLite 3.51.0; a managed Python 3.13 reports SQLite 3.50.4 with `_sqlite3` built in. Neither proves the exact 3.50.0 or below-floor load. `DYLD_LIBRARY_PATH` cannot replace that built-in module. The supported managed Python runs functional examples and conformance; native amalgamation-linked hosts prove the two exact version boundaries.

## Small checker change

Keep the existing `test_schema.py` functions `test_pinned_host_keeps_a_successful_registration_available` and `test_a_host_below_the_floor_refuses_the_load` as the installed assertions. On macOS they invoke one small native `load_probe.c` built twice, once with each checked amalgamation's `sqlite3.c`; on Linux they keep their present Python route. The probe uses real `sqlite3_open`, `sqlite3_load_extension`, `sqlite3_prepare_v2` and `sqlite3_step` calls. It returns the SQLite version, load result and diagnostic, zero-send usage on success, a later `SELECT 7`, and dyld residency after closing the first connection. Resolve image paths before comparing them because dyld reports `/private/tmp` for an archive unpacked under `/tmp`. The Python entry points assert the exact result; the probe itself uses no backend, real key, or synthetic version. Build output stays in the ignored toolchain cache, outside Git.

`setup.sh` obtains the already pinned SQLite 3.50.0 source and one official SQLite 3.49.0 amalgamation for macOS. Add a checksum file for 3.49.0 and verify both sources before compiling. The official 3.49.0 zip has SHA-256 `cb6851ebad74913672014c20f642bbd7883552c4747780583a54ee1cd493f13b`; its `sqlite3.c` and `shell.c` have SHA-256 `032f545fd56206903bf25309714acb924504ab599f80b51002b89d08579b0485` and `5220eebd7b176e8acae8a6b5f1b22333df401079741588e08cd1e05d524a286f`. Keep cached source as the one-time setup input. `host_sqlite.sh` compiles the native probe beside the existing pinned CLI on macOS and keys its build on both source hashes. It keeps its Linux shared-library path. Use `shasum -a 256` after checking that capability, or a Python standard-library hash helper; M5's `/sbin/sha256sum` rejects GNU `--check --quiet` despite hashing files.

`check.sh` selects the `.dylib` archive member and the configured Python 3.10+ executable on macOS. It passes the two probe paths into the selected `test_schema.py` assertions. `helper.py` chooses the `.dylib` default in source mode; installed mode continues to use the explicit unpacked archive path. Do not depend on `LD_LIBRARY_PATH` or `DYLD_LIBRARY_PATH` to change the managed Python's built-in SQLite. Avoid writing a second test harness or duplicating the ordinary examples and conformance.

The built macOS SQLite dylib currently has an absolute `LC_ID_DYLIB` under the builder's home. The packaged bytes contain that path once, even though macOS `/usr/bin/strings` does not show load-command strings. Give the cdylib a relative install name through a narrow `databases/sqlite/build.rs`, following the C binding's existing build script, and inspect the packaged Mach-O ID plus raw bytes. `release-smoke` already scans unpacked files with byte-oriented `grep -rlaF`, so no new scan is needed. Its checksum helper selects M5's `/sbin/sha256sum` and then calls unsupported `-c`; choose a hash verifier that actually supports checking on each host. Keep the Linux archive verification working.

## Evidence

- Starts from: ticket 0226's accepted guard source, M5 archive and native exploratory results, and the Linux installed checker at `10f85fa7`.
- Keeps: the 3.50.0 floor, ordinary SQL and error behavior, source child tests, existing Linux proof, and no paid or live backend call.
- Changes: native macOS host checks, portable source and archive hashes, the SQLite dylib install name, and installed `.dylib` selection.
- Proof: exact 3.50.0 load and zero-send call, exact 3.49.0 refusal, later `SELECT 7` and observed dyld residency in both paths, installed functional examples and conformance, and raw package path scan.
- Defers: macOS Intel, Linux ARM64, retained DuckDB C API packages, and other binding panic issues.

The [0226 M5 record](../records/0226-macos-arm64-package-proof.md) holds the pushed archive hashes and focused source outcomes. An exploratory native C probe at `/tmp/thinkthen-m5-sqlite-load-probe.c`, compiled directly with checked 3.50.0 and 3.49.0 `sqlite3.c`, loaded the packaged dylib. At 3.50.0 it returned success, zero requests, later `7`, and residency `1`. At 3.49.0 it returned the exact floor refusal, later `7`, and residency `1`. The installed CLI built from each source independently showed the same load and later-call results. The M5's observed failed-load residency does not mean the extension explicitly pins a failed load; no pin is proposed. The packaged dylib has a local `std::panicking::HOOK` symbol and ordinary ARM64 Mach-O dylib flags. Recheck the exact package through the maintained driver after the checker lands; do not infer universal macOS loader behavior from one host.

The smallest reviewable validation is: hash-check both source trees; compile the two native probes; run the selected installed `test_schema.py` cases; run installed `examples.py` and `conformance.py`; inspect the repacked Mach-O ID and raw home-path scan; run focused formatting, script checks and affected ratchets. Exercise the release checksum helper on the two archives without a broad release smoke. Run the accepted SQLite worker child tests only if implementation changes that source, which this ticket does not propose. Preserve exact archive SHA and host versions in the build record. macOS Intel, Linux ARM64, retained DuckDB C API packages, and other binding panic issues remain outside this ticket.

## What the build taught us

Pending implementation and review.
