# 0128 Phase 2 build: pack and smoke on Linux

Builder: Claude (Opus subagent), 2026-09-26, on `ticket/0128-phase-2-pack-and-smoke` in lane `worktrees/thinkthen-lane-1`, from `origin/main` at `f43bfe91`. `origin/main` at `d410ef4a`, which holds ticket 0137 and the design fixes, merged cleanly at `13712cdd`. `origin/main` at `0ec897e1`, which holds tickets 0134 and 0139, merged cleanly at `98499261`. After the code review, `origin/main` merged cleanly four more times: `b43fcd8a` (tickets 0135 and 0142) at `a70d806c`, `851f96fc` at `d989b866`, `7850db3f` (ticket 0141) at `145e298a`, and `7443d69d` at `98f6fa44`. The final run went over `7cd8acda`, with `origin/main` still at `7443d69d`. This build took no outward step: no publish, no tag, no release, and no workflow dispatch. Ian can overturn every decision below.

## Outcome

Every Linux `x86_64` release file packs, installs into a fresh prefix, and passes its surface's check from there. The `surfaces` rung now ends with that pack and smoke, so it runs on every gate. Every loadable file also builds inside `manylinux_2_28` as a normal user, needs glibc 2.28 at most, and passes the same smoke on the host.

## What each item built

1. **The packer.** `sdlc/scripts/release-pack [--reuse] TARGET OUT [PART...]` packs the command, C, SQLite, DuckDB, PostgreSQL 16, the Python wheel, the npm tarball, the Ruby gem, and the first-run sample. Each file gets a `.sha256` beside it. TARGET must be the host's target, since each release target builds on its own runner. Without `--reuse` it builds each file. With `--reuse` it packs what the checks built.
    - The C archive holds `include/thinkthen.h`, `lib/libthinkthen.a`, `lib/libthinkthen.so` with the `libthinkthen.so.0` soname link, and `lib/pkgconfig/thinkthen.pc`. The `.pc` file finds its folder through `${pcfiledir}`, so the archive works wherever it is unpacked. A static link through `pkg-config --static` was run once by hand on Linux, and it linked and ran.
    - `databases/postgresql/package.sh` restores the tag's packager as a stager. It stages `lib/thinkthen.<suffix>` and `extension/thinkthen.control` and `thinkthen--<version>.sql`. It reads the folders from `pg_config` and names no host path or library suffix. That closes the `package.sh` halves of error-index rows R3-32 and R4-10.
    - The first-run sample packs demo 27's `report.txt` and its one recording, as the Phase 1 record decided.
2. **The smoke.** `sdlc/scripts/release-smoke DIR` starts the loopback backend. For each file it checks the `.sha256`, then runs the surface's check with `THINKTHEN_ARTIFACT` naming the file, and applies `verdict release`. The command's check runs the unpacked binary's `--version` by path. It then runs the README's "First run" block with the key and address unset, the binary first on `PATH`, and the sample copied from DIR in place of the curl line. The block must print `true`, and the loopback backend's count must not move. The smoke fails when DIR lacks any of the nine files.
3. **The installed-file mode.** Seven checks read `THINKTHEN_ARTIFACT`. Each skips its build and lint and runs its shared cases and examples against the installed file.

    | Surface | Installs with | Asserts |
    | --- | --- | --- |
    | Python | `uv pip install` of the wheel into a fresh venv, offline | `thinkthen.__file__` is under that venv |
    | TypeScript | `npm install --offline` of the tarball into a fresh project | `import.meta.resolve("thinkthen")` from the copied `tests/` is under that project |
    | Ruby | `gem install --local --install-dir` | every `lib/thinkthen` feature loaded is under the gem folder |
    | C | Unpack | `pkg-config --variable=libdir thinkthen` is inside the unpacked archive, and the slide links through it |
    | SQLite, DuckDB | Unpack | the tests load the extension by the unpacked path |
    | PostgreSQL | Unpack | the server's tree takes the module and extension files from the unpacked archive |
    | Command | Unpack, in the smoke | the smoke runs the unpacked binary by path |

   Python, TypeScript, and Ruby copy `tests/` and `examples.json` into a temporary tree with `conformance/cases.json` and `conformance/children` beside it, and run from there. Python unsets `PYTHONPATH`, and Ruby unsets `RUBYLIB`. The test-side changes are small:
    - TypeScript's `backend.mjs` imports the package by its name. In the checkout that resolves through the package's own `exports`. In the fresh project it resolves the installed copy.
    - Ruby's test children keep `GEM_PATH`.
    - SQLite's `helper.py` reads `THINKTHEN_SQLITE_EXTENSION`, as DuckDB's harness already reads `THINKTHEN_DUCKDB_EXTENSION`.
    - DuckDB's installed mode points `THINKTHEN_CONFORMANCE_CASES` at a copy of the shared cases without the one internal-invariant case. A release file carries no test hook, so the full check alone runs that case. `conformance.py` and its ratchet stay as on `main`.
    - PostgreSQL's `runtime_install` takes a module folder and an extension folder. `check.sh` and `package.sh` share one rule: `PG_CONFIG`, else `/usr/bin/pg_config`, and `${CARGO_TARGET_DIR:-target}/release/thinkthen-pg16`. Both read the module and extension folders from `pg_config`. The normal check copies the shipped build to that folder's `-shipped` twin, which `package.sh --reuse` stages. Installed mode runs `examples`, `slide_sample`, `recognize_and_relate_as_drawn`, `conformance`, and `the_fake_key_stays_in_the_environment`.
    - `build-wheel.sh` builds in a mktemp folder, as on `main`, and copies the wheel to `target/wheels`. The TypeScript check writes its real `npm pack` to `target/pack` in place of the dry run.
   `sdlc/scripts/installed.sh` holds the shared lines. It sits beside `verdict.sh`. `installed_tests` makes the test copy, `installed_unpack` unpacks the file, and `backend_start` starts the loopback backend for `surfaces` and `release-smoke`. Each folder it deletes is one it made with `mktemp` in the same run, and `installed_remove` refuses any other path. `installed_folder` stops the script when `mktemp` fails. It refuses a folder whose path holds whitespace or a glob character, the current folder, or the root, so the list of made folders holds only safe paths. It takes over the script's EXIT trap, and an interrupt or TERM exits so that trap runs. Ruby's copy sits inside the check's own plant folder, which the check's existing cleanup removes.
4. **The rung.** `surfaces` records each surface that did not run. When every surface passed, it runs `release-pack --reuse` for the host's target into a mktemp folder and then `release-smoke`, and prints `surfaces: pass release smoke`. When a surface did not run, it prints `surfaces: not run release smoke (<surface> did not run)`. `--release` has already failed that surface. `surfaces` and `release-smoke` both source `verdict.sh`, and neither calls the other. `release-smoke` builds the loopback backend online, as `surfaces` always has.
5. **The shipped-file scan.** `release-smoke` unpacks every library file and fails it when any file inside holds the builder's `$HOME` or `thinkthen_panic_probe`. The checks' own `shipped_lacks_probe` and `no_home_in_library` read their builds, and this scan reads what a user installs. The rung packs the command's debug build, so the scan skips the command archive. Its first run found three real leaks, each fixed in the build:
    - The Python wheel's CycloneDX SBOM named each path crate by its absolute folder. `pyproject.toml` turns the Rust SBOM off, and `build-wheel.sh` now checks every file in the wheel for the home path.
    - The Ruby gem's extension held 25 home paths. `build.sh` now remaps `$HOME`, as the other bindings do.
    - The C library held `ring`'s C and assembly source paths under `~/.cargo`. The C check and `release-pack`'s build mode pass `-ffile-prefix-map=$HOME=/build` to the C compiler beside the Rust remap.
6. **The container proof.** See its section.

The README gains its "First run" block, the one README item this phase needs. Phase 1's other held README items stay held.

## The container proof

`sdlc/scripts/release-container WORK OUT` copies `git archive HEAD` into a mktemp folder inside WORK and mounts that folder as `/work`, with `WORK/cache` beside it for the pinned downloads. It runs `quay.io/pypa/manylinux_2_28_x86_64@sha256:407f771c51a2c3e83ebe5a7970b4289ead3a6db21d9b9c089168775cad11d328` with `--user 1000:1000` on that copy, and never mounts the worktree. It copies the files to OUT, here the lane's ignored `target/release-container`, and removes its run folder at exit. The run mounted the current user's crate registry and built offline, with one exception: cargo-pgrx fetched itself.

| Toolchain | Source used |
| --- | --- |
| Rust | `rustup-init` 1.28.2, sha256 `20a06e64…cdd43c`, and 1.95.0 from `rust-toolchain.toml` |
| Python | `/opt/python/cp310-cp310` with maturin 1.15.0, sha256 `653020a6…29b99e` |
| Node | `node-v22.22.3-linux-x64.tar.xz`, checked against `libraries/typescript/node.sha256`. napi-rs needs no Node headers |
| C, SQLite, DuckDB | The image's gcc. Neither extension build reads the SQLite amalgamation or the DuckDB headers. The amalgamation serves only the host test CLI |
| PostgreSQL 16 | PGDG EL8 16.15 RPMs `postgresql16`, `-libs`, `-devel`, and `-server`, each pinned by sha256 and unpacked with `rpm2archive` as the user. `pg_config` sits in `postgresql16` and runs from the unpacked tree. `cargo-pgrx` 0.17.0 with `--locked --no-default-features`. Its default features reach OpenSSL, which the image lacks |
| libclang | PyPI `libclang` 18.1.1 manylinux2010 wheel, sha256 `c533091d…aa2a0b`, per the queue owner's ruling. bindgen reads `gcc -print-file-name=include` for its built-in headers |

Results of the last run, on `145e298a`, which took 295 seconds:
- The container built C, SQLite, DuckDB, PostgreSQL 16, the Python wheel (tag `manylinux_2_28_x86_64`), and the npm tarball.
- The newest glibc symbol in each library is `GLIBC_2.28`: `libthinkthen.so`, `libthinkthen0.so`, `thinkthen.duckdb_extension`, `thinkthen-linux-x64.node`, `_thinkthen.abi3.so`, and PostgreSQL's `thinkthen.so`.
- No file holds the container's home, `/work/home`. The run before the C remap found `/work/home/.cargo` paths in `libthinkthen.a`.
- `release-smoke` on the host passed all nine files in 42 seconds: the six container-built files, plus the host-packed command, first-run sample, and Ruby gem. The gem waits for `rb-sys-dock` in Phase 3.
- After every container run, `find <lane> ! -user "$(id -un)"`, the same search in the scratchpad work folder, and the same search in `~/.cargo/registry` each found 0 files. The same search over the lane after the final ladder found 0.

## Plants

Each plant ran, went red, and was restored.

| Plant | Result |
| --- | --- |
| `release-pack` packs the C archive without `thinkthen.pc` | C installed check exits 1: "the archive holds no pkg-config file for thinkthen" |
| Python installed mode exports `PYTHONPATH` as the repository's `libraries/python` | exit 1: "thinkthen loaded from outside the fresh venv" |
| TypeScript installed mode links the checkout into `node_modules` in place of `npm install` | exit 1: "thinkthen resolved to file://…/libraries/typescript/index.mjs, outside the fresh project" |
| Ruby installed mode exports `RUBYLIB` as the repository's `lib` | exit 1: "thinkthen loaded from outside the gem folder" |
| `package.sh` stages no SQL file | PostgreSQL installed check exits 1 at `CREATE EXTENSION` |
| A zeroed `.sha256` for the SQLite archive | smoke exit 1: "FAIL thinkthen-sqlite-… (its .sha256 is missing or does not match)" |
| DIR without the npm tarball | smoke exit 1: "FAIL libraries/typescript (DIR holds no file for it)" |
| The first-run sample without its recording | smoke exit 1: "the first-run block printed '', not true" |
| The README block without `--replay`, run with a fake key and the loopback address | smoke exit 1: "the first-run block reached the loopback backend" |
| `SQLITE_AMALGAMATION` names a missing folder, so the SQLite check exits 77 | smoke exit 1: "FAIL databases/sqlite … (not run)" |
| `installed_remove` and `installed_tests` get the lane path, with `rm` replaced by a recorder, against `8f735b81` and again against `0228f809` | The guard refused the lane and a path under it with exit 1. `installed_tests "$lane" libraries/typescript` removed only its own mktemp folder at exit. `scratch` pointed at the lane was refused |
| `installed_folder` with `TMPDIR=/nonexistent`, run from the lane with `rm` replaced by a recorder | Before `0228f809`: `folder` became the lane, and the exit trap recorded `rm -rf` of the lane. After: exit 1 at `mktemp`, and no removal recorded. `installed_tests` gave the same result |
| `installed_folder` with `TMPDIR` a folder whose name holds a space | exit 1: "installed.sh: refusing …/with space/tmp.…", and no removal recorded |
| A `thinkthen_panic_probe` function appended to the packed PostgreSQL SQL file | smoke exit 1: "FAIL thinkthen-postgresql16-… (the builder's home or the test probe is in extension/thinkthen--0.0.1.sql)" |
| The builder's home appended to the packed C `libthinkthen.so` | smoke exit 1: "FAIL thinkthen-c-… (the builder's home or the test probe is in lib/libthinkthen.so)" |
| The wheel built with the Rust SBOM on | `build-wheel.sh` exit 1: "thinkthen-0.0.1.dist-info/sboms/thinkthen-python.cyclonedx.json name the builder's home" |
| The gem built by `build.sh` without the remap | 25 home paths in `lib/thinkthen/thinkthen.so`, and 0 with it |

The guard plants ran before any script called the helper. The `TMPDIR` plants passed before the next `surfaces` or `release-smoke` run. Before each commit, a search of the diff found no plant text.

## Rungs

The final run went over `7cd8acda`, once each, with `THINKTHEN_API_KEY` unset. Each rung took the heavy lock itself.

| Rung | Result | Time |
| --- | --- | --- |
| `install` | exit 0, most of it waiting for another lane's heavy lock | 291 s |
| `lint` | exit 0 | 94 s |
| `test` | exit 0 | 102 s |
| `spec` | exit 0 | 22 s |
| `surfaces` | exit 0: all ten surfaces pass, then the release smoke passes all nine files | 501 s |

In the smoke, DuckDB's installed conformance ran 53 cases, and the PostgreSQL installed run passed 5 of 5 steps. The command's check ran `--version` and the README's first-run block, which printed `true` with the loopback count unchanged.

Earlier runs:
- `13712cdd`: `lint` failed, because comments in TypeScript's `backend.mjs` put `ratchet.mjs.json` at 743 against 742. Commit `2e52a2d0` removed them.
- `98499261`: all five passed. This was the hand-back head `ade15093` before the first code review.
- `a70d806c`: `surfaces` failed. The new scan found the builder's home in the C archive's `libthinkthen.a` and `libthinkthen.so`, which led to the C remap.
- `d989b866` and `145e298a`: all five passed. `145e298a` was the second hand-back. Its re-review found the failed-`mktemp` hole in `installed_folder`.

## Budgets

Nonblank lines.

| Budget | Limit | Measured |
| --- | --- | --- |
| `sdlc/scripts/release-pack` | 180 | 152 |
| `databases/postgresql/package.sh` | 70 | 33 |
| `sdlc/scripts/release-smoke` | 150 | 94 |
| The installed-file mode, each `check.sh` | 20 added | Against `origin/main`: C +28 −11, SQLite +17 −2, DuckDB +23 −7, Python +17, TypeScript +20 −1, Ruby +19, PostgreSQL +23 −11, `runtime.sh` +4 −3. C, DuckDB, and PostgreSQL are over; see decision 7 |
| The installed-file mode, total | 180 | 151 added, 35 removed, 116 net. The shared helper `installed.sh` adds 79 |
| The container build scripts | 80 | 68 |
| The `surfaces` rung | +3 minutes warm | The pack takes seconds, and the smoke took 42 seconds over nine files |

`sdlc/scripts/surfaces` adds 17 lines and removes 15. The root ratchet measures `crates + conformance 67758/67758`, equal to `origin/main`. This phase changes no Rust, no ceiling in `sdlc/ratchet.json`, and no binding ratchet. Every binding ratchet measures at its ceiling, and DuckDB's Python ratchet stays at 1896.

## Decisions

Ian can overturn each one.

1. **The smoke finds each file's surface by its name.** It fails a file no surface packs and a missing file. Phase 3's runners get the same completeness rule.
2. **Installed mode runs the shared cases and examples, not the whole check.** The whole check builds from source and uses test-hook builds that a release file lacks. The shared cases are what the issue asked each file to pass before publish.
3. **The command's `--reuse` archive holds the host's debug build under the musl name.** The rung proves the pack and the smoke path. The static musl build proves itself in the Phase 3 rehearsal. The debug build holds the builder's home, so the shipped-file scan skips the command archive.
4. **The container shares the current user's crate registry.** It builds offline from the locks already fetched, and the release workflow fetches first. cargo-pgrx is the one fetch, and it builds without OpenSSL.
5. **PGDG's `postgresql16` and `postgresql16-libs` RPMs join `-devel` and `-server`.** `pg_config` lives in `postgresql16`, and the ticket's table now says so.
6. **The wheel ships no Rust SBOM.** maturin's CycloneDX SBOM names each path crate by its absolute folder, and this build found no maturin 1.15.0 setting that remaps it. An SBOM without build paths can return in a later ticket.
7. **Three checks add more than 20 lines.** DuckDB adds 23, because its stock-CLI step moves into a function that both modes call. Copying the step would add more. C adds 28 and PostgreSQL 23, because their build lines sit indented inside `[ -n "${THINKTHEN_ARTIFACT:-}" ] || { … }` blocks, as the re-review asked. Net, each adds 17, 16, and 12 lines.
8. **Build outputs refresh without a delete.** Every script this phase adds deletes only a folder it made with `mktemp`, per rule 11 of `sdlc/planning/worktrees.md`. `build-wheel.sh`, the TypeScript pack, and PostgreSQL's shipped tree copy over their `target/` outputs in place.

## Lane 1 deleted during the review fixes

The first try at the review fixes deleted lane 1. `installed_tests` passed its REPO argument on to the cleanup, and the TypeScript check's exit removed the lane. The uncommitted fixes and the lane's build folders were lost, and nothing committed was. `sdlc/records/2026-09-26-lane-1-deleted-by-a-cleanup-trap.md` records the cause and the new rules. Ian restored the lane at `ade15093`. The fixes above were redone in order. The guarded helper was committed and pushed first. Its plant ran before any script called it. Each fix after that was committed and pushed on its own.

## Held for Phase 3

- `aarch64` Linux and both macOS targets. That includes the macOS `Libs.private` line in `thinkthen.pc`, left empty until the rehearsal measures it.
- The static musl build of the command in build mode. Once it runs with the remap, the scan can read the command archive too.
- The Ruby gem in `rb-sys-dock`, by the queue owner's ruling.
- The build folder in shipped files. Container-built libraries name source files under `/work/src`, the build copy's folder. That path names no user. The release runners' folders can be remapped in the rehearsal.
- Phase 1's other held README items, and its held Rust test edits.

## Disk

The lane measured 9.3G before this phase and 9.6G after the first hand-back. Restored cold at 792M, it measured 9.8G after the final ladder. Packed outputs sit in mktemp folders or under the lane's ignored `target/`. The scratchpad work folder holds the container's toolchains, downloads, and plant copies at about 4.5G.
