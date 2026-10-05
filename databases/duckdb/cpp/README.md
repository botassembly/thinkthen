# DuckDB C++ extension

The Linux x86-64, Linux ARM64, Apple Silicon and Intel macOS release packages use this extension. It calls the public `thinkthen` engine through `bridge/` and registers its scalars, `thinkthen_rank`, the `_many` tables, `thinkthen_usage()`, and `thinkthen_relate`. `thinkthen_warm` and `thinkthen_probability` stay registered only to refuse with their replacements. The Intel package has focused installed proof under Rosetta on macOS 26. Native Intel hardware, macOS 15 and release-runner qualification remain open.

The default source is DuckDB v1.5.5 at `d8cdaa33fda8df955cc76ef58a280f68f4cd43fa`. The official [v1.5.5 Linux static release ZIP](https://github.com/duckdb/duckdb/releases/download/v1.5.5/static-libs-linux-amd64.zip) has SHA-256 `deb47c5300f3c99725e84cdb14d214c3b12bbd748b613b1698b938c894cb68eb`. The four target-specific archive manifests fix every extracted `.a`; CMake rejects a changed source commit, missing archive, added archive, or hash mismatch. The source and archives are DuckDB MIT inputs. The Rust bridge depends on the repository's public engine and its pinned Cargo lock. The C++ file follows the separately accepted 0201 proof; no template code is copied into it.

Ticket 0403 slice A prepares separate C++ builds for DuckDB v1.5.5 and v1.5.4. `tools/version.env` supplies the supported list and the per-version source, CLI and archive pins. `tools/setup.sh --fetch` prepares both toolchains outside offline gates. The first listed version supplies the default. Select one build with `THINKTHEN_DUCKDB_VERSION`; unknown selectors fail.

Provide absolute `THINKTHEN_DUCKDB_CPP_SOURCE` and `THINKTHEN_DUCKDB_CPP_STATIC_DIR` paths only for the selected version. They must match its raw source bytes, modes, links and archive manifest. `THINKTHEN_DUCKDB_CPP_BUILD` names a CMake base; each build uses `<base>/<version>/<target>/`. Canonical processed files live at `build/artifacts/cpp/<version>/<target>/thinkthen.duckdb_extension`. Repository consumers read those files. Only a successful default build atomically refreshes the convenience copy `build/thinkthen.duckdb_extension`.

Build offline from the repository root under the lane's heavy lock:

```sh
export THINKTHEN_HEAVY_LOCK=/tmp/thinkthen-claude-0-heavy.lock
flock -o "$THINKTHEN_HEAVY_LOCK" env THINKTHEN_DUCKDB_VERSION=v1.5.4 sh databases/duckdb/cpp/build.sh
```

Preparation is partial. The new pins require fresh High review before native builds or loads. No 1.5.4 ABI or header compatibility proof is recorded yet. Slice A retains the existing single-version release archive with the default extension at its root. Slice B will change the archive to the combined repository layout. Public installation stays at 0.1.2. dbt v1 with DuckDB 1.5.5 remains the documented 0.2 route. A stock unsigned 1.5.4 load cannot qualify dbt v2's custom driver or signature requirements.

`verify_decide.py --extension PATH` uses the surface's loopback backend and isolated child process. Set `THINKTHEN_BACKEND_BIN` to the local `conformance-backend` binary, and use the pinned v1.5.5 Python environment. It asserts bad foldable and row-dependent questions send zero requests, NULL sends none, a valid question sends one, and SQL exposes `BOOLEAN`. Repeated evidence in each decision chunk sends once, a two-record rank sends one request, the removed `thinkthen_probability` sends nothing, and details returns parseable JSON text as `VARCHAR`. It never uses a real key or paid service.

The extension shares one bind and grouped execution path for ordinary scalars, and keeps try-details row failures separate. It registers scalar, nested, rank, usage and relate SQL forms with their retained types, and the two refusing removed forms. The focused stock-host verifiers check foldable and later-row validation, NULL propagation, complete nested values, first-seen deduplication, statement budgets, file and cache permissions, and real loopback sends. Ticket 0149 adds caller-session model, timeout, retries, inline profile, record and replay settings, plus a shared send budget.

Relate reads capped, committed rows through a separate connection to the caller's database. A per-database gate lasts through its SQL query and engine answer. Focused checks cover edge mapping, JSON and file rules, no-send refusals, the plan guard, queue deadline, closed-file cleanup, and held and running-SQL SIGINT. The temporary-table Local error now explains the separate connection. Fresh review accepted read-only database support within ticket 0201, with committed-only visibility and caller permission checks.

The four target packages include the landed 0167 pair planner. Their separate build records identify selected stock-host, relation, find, prepared-file, warm/settings, request-budget and interruption checks. `release-pack` validates target architecture and the DuckDB footer's platform, DuckDB version, extension version and ABI before it archives an extension, including under `--reuse`. The macOS Intel installed checks ran under Rosetta; they do not establish native Intel or macOS 15 release-runner support.

The release archive contains both version/platform members. Linux x86-64 native builds against the pinned v1.5.5 and v1.5.4 headers compile the same C++ sources without compatibility guards. Other native targets still require their release rehearsal.
