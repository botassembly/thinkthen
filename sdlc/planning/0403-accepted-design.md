The final release outcome remains one unsigned archive per existing DuckDB target, containing two separately compiled C++ extensions:

```text
v1.5.5/<platform>/thinkthen.duckdb_extension
v1.5.4/<platform>/thinkthen.duckdb_extension
LICENSE.thinkthen
LICENSE.duckdb
NOTICE
DEPENDENCIES.txt
LICENSES/...
```

Keep `thinkthen-duckdb-$V-$TARGET.tar.gz` and its checksum sidecar. `$V` is the ThinkThen release version, distinct from the DuckDB version.

| Release target | DuckDB repository platform |
|---|---|
| `x86_64-unknown-linux-gnu` | `linux_amd64` |
| `aarch64-unknown-linux-gnu` | `linux_arm64` |
| `aarch64-apple-darwin` | `osx_arm64` |
| `x86_64-apple-darwin` | `osx_amd64` |

Preserve every SQL function, overload, result type, setting, error sentence, removed-function refusal, bind validation, statement owner, cancellation boundary, secrecy rule, cache behavior and request limit. Preserve existing conformance exceptions and their named replacement evidence. ADR 0081’s C++ API and Rust bridge remain. This ticket introduces no core change or stable C API migration.

Stock unsigned DuckDB 1.5.4 compatibility establishes no ThinkThen load in dbt v2’s custom driver. dbt v1 with `duckdb==1.5.5` remains the documented 0.2 route. A successful stock load and a matching footer cannot establish signed dbt v2 compatibility.

## Build and compatibility contract

`DUCKDB_VERSIONS` in `databases/duckdb/tools/version.env` is the supported-version authority, newest first. Each version uses its own official source commit, CLI and static archive pins, extracted manifest, and Python requirements. Validate source identity, archive membership and bytes before admission. Missing prerequisites report exit 77; corrupt inputs fail. Preparation may download official inputs outside offline gates.

Build each selected version separately under `<CMake base>/<version>/<target>/`. Canonical output is `databases/duckdb/build/artifacts/cpp/<version>/<target>/thinkthen.duckdb_extension`. Preserve architecture checks, complete footer checks, macOS deployment checks, footer-preserving stripping and hidden exports. Start with shared unchanged C++ sources; record any demonstrated header difference in `NOTES.md`.

`cpp/build.sh` alone writes the newest convenience alias `build/thinkthen.duckdb_extension`, atomically after validation. Older builds leave it unchanged. Repository consumers use canonical files. A missing canonical file fails even when an alias or obsolete output exists.

The real matching stock CLI and Python module must load each native file. Each host must refuse the other version's actual file with exit 1 and DuckDB's sentence naming both versions. Retain patched-footer refusal checks. Both hosts must install from the local two-version repository in fresh extension directories. A missing matching member must fail without installation-cache or checkout fallback.

Keep the newest conformance and existing SQL suite sequence. The older version runs conformance, rank, and the find and portable selections already named by installed mode. Retain distinct secrecy, cancellation, invalid-input, cache, request-limit, panic-hook and removed-function checks. Backends remain loopback with fake keys; no paid calls enter gates.

## Package and consumer contract

One archive per existing target contains exactly one extension per supported version at its version/platform path. Preserve archive names, ThinkThen release version, checksum sidecars, architecture checks, footer checks, release routing and counts. Generic source/static overrides apply only to the selected version; other versions use their prepared caches.

Collect legal material from both pinned sources. Deduplicate only identical bytes. Keep root `LICENSE.duckdb`; retain differing texts at version-qualified paths and reference them in `NOTICE`. `NOTICE` names both versions. `DEPENDENCIES.txt` distinguishes each version's static inventory and source notices and lists common Rust dependencies once.

Installed mode validates repository members, then loads only unpacked bytes in the matching CLI and Python host. Both versions receive stock loads and wrong-version checks; newest keeps the complete installed sequence and older keeps its focused sequence. No build or checkout fallback occurs in installed mode.

The published-install consumer queries its actual stock host with `SELECT version(); PRAGMA platform;`, validates that identity, selects the matching regular archive member, checks its footer, loads it and queries the loaded ThinkThen version. Retain the recorded details call, cache off, boolean true and zero requests. Missing, linked, misplaced or mismatched members fail. Wrong loaded version and wrong result facts fail through the common validator.

Use existing offline packer and consumer checks. Plants protect missing older files, swapped folders, wrong platforms/footers, duplicates, extras, links, escapes and incomplete legal material. Invoke the real packer in owned scratch; rejected inputs produce neither archive nor checksum sidecar. Native consumer checks use genuine locally packed bytes and saved recordings.

The install sample uses `INSTALL thinkthen FROM './'; LOAD thinkthen;` against a fresh repository and installation directory. Other SQL samples may load a fresh scratch copy by path. Replay consumes the processed canonical artifact and preserves saved result bytes.

Public pages and the DuckDB README declare both supported versions independently; the build checks for omitted declarations against `DUCKDB_VERSIONS`. Public wording states that the archive is unsigned, stock 1.5.4 compatibility does not enable dbt v2, and dbt v1 with DuckDB 1.5.5 remains the documented route. Signing and community listing are deferred.

## Qualification and review

Source preparation alone does not qualify either native version. Run actual local builds, matching CLI/Python loads, symmetric wrong-version refusals, conformance and focused older suites, real two-version packaging and installed-file checks. Report which native platforms actually ran. Linux evidence does not establish macOS, ARM64 or native Intel behavior.

Run policy before the one fresh source review root arranges. Run focused checks after fixes, and full tests and lint on the landing commit. Replay changed docs examples. Keep release rehearsal and Ian's approvals; all four release targets still require actual runner proof before release. Add no verification framework or receipt controls.

Ian can overturn the combined archive choice, supported versions, C++ API choice and older-suite scope. Root writes one canonical 0403 record at landing.
