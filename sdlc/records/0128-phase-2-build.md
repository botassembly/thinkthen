# 0128 Phase 2 build: pack and smoke on Linux

Builder: Claude (Opus subagent), 2026-09-26, on `ticket/0128-phase-2-pack-and-smoke` in lane `worktrees/thinkthen-lane-1`, from `origin/main` at `f43bfe91`. `origin/main` at `d410ef4a`, which holds ticket 0137 and the design fixes, merged cleanly at `13712cdd`. `origin/main` at `0ec897e1`, which holds tickets 0134 and 0139, merged cleanly at `98499261` before the final run. This build took no outward step: no publish, no tag, no release, and no workflow dispatch. Ian can overturn every decision below.

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
    - DuckDB's conformance reports its one internal-invariant case as not run when the check sets `THINKTHEN_DUCKDB_HOOKS` to empty. A release file carries no test hook.
    - PostgreSQL's `runtime_install` takes a module folder and an extension folder. The normal check keeps the shipped build at `target/release/thinkthen-pg16-shipped`, which `package.sh --reuse` stages. Installed mode runs `examples`, `slide_sample`, `recognize_and_relate_as_drawn`, `conformance`, and `the_fake_key_stays_in_the_environment`.
    - `build-wheel.sh` keeps its wheel in `target/wheels`. The TypeScript check writes its real `npm pack` to `target/pack` in place of the dry run.
4. **The rung.** `surfaces` counts a "not run" surface. When every surface passed, it runs `release-pack --reuse` for the host's target into `target/release-pack` and then `release-smoke`, and prints `surfaces: pass release smoke`. When a surface did not run, it prints the smoke as "not run", which `--release` fails. `surfaces` and `release-smoke` both source `verdict.sh`, and neither calls the other.
5. **The container proof.** See its section.

The README gains its "First run" block, the one README item this phase needs. Phase 1's other held README items stay held.

## The container proof

`sdlc/scripts/release-container WORK OUT` copies `git archive HEAD` into WORK and runs `quay.io/pypa/manylinux_2_28_x86_64@sha256:407f771c51a2c3e83ebe5a7970b4289ead3a6db21d9b9c089168775cad11d328` with `--user 1000:1000` on that copy. It never mounts the worktree. It then copies the files to OUT, here the lane's ignored `target/release-container`. The run mounted this user's crate registry and built offline, with one exception: cargo-pgrx fetched itself.

| Toolchain | Source used |
| --- | --- |
| Rust | `rustup-init` 1.28.2, sha256 `20a06e64…cdd43c`, and 1.95.0 from `rust-toolchain.toml` |
| Python | `/opt/python/cp310-cp310` with maturin 1.15.0, sha256 `653020a6…29b99e` |
| Node | `node-v22.22.3-linux-x64.tar.xz`, checked against `libraries/typescript/node.sha256`. napi-rs needs no Node headers |
| C, SQLite, DuckDB | The image's gcc. Neither extension build reads the SQLite amalgamation or the DuckDB headers. The amalgamation serves only the host test CLI |
| PostgreSQL 16 | PGDG EL8 16.15 RPMs `postgresql16`, `-libs`, `-devel`, and `-server`, each pinned by sha256 and unpacked with `rpm2archive` as the user. `pg_config` sits in `postgresql16` and runs from the unpacked tree. `cargo-pgrx` 0.17.0 with `--locked --no-default-features`. Its default features reach OpenSSL, which the image lacks |
| libclang | PyPI `libclang` 18.1.1 manylinux2010 wheel, sha256 `c533091d…aa2a0b`, per the queue owner's ruling. bindgen reads `gcc -print-file-name=include` for its built-in headers |

Results:
- The container built C, SQLite, DuckDB, PostgreSQL 16, the Python wheel (tag `manylinux_2_28_x86_64`), and the npm tarball.
- The newest glibc symbol in each library is `GLIBC_2.28`: `libthinkthen.so`, `libthinkthen0.so`, `thinkthen.duckdb_extension`, `thinkthen-linux-x64.node`, `_thinkthen.abi3.so`, and PostgreSQL's `thinkthen.so`.
- `release-smoke` on the host passed all nine files: the six container-built files, plus the host-packed command, first-run sample, and Ruby gem. The gem waits for `rb-sys-dock` in Phase 3.
- After every container run, `find <lane> ! -user ian`, the same search in the scratchpad work folder, and the same search in `~/.cargo/registry` each found 0 files.

## Plants

Each plant ran, went red, was restored, and had its file touched.

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

Before each commit, a search of the diff found no plant text.

## Rungs

The final run went over the merged head `98499261`, once each, with `THINKTHEN_API_KEY` unset.

| Rung | Result | Time |
| --- | --- | --- |
| `install` | exit 0 | 107 s |
| `lint` | exit 0 | 166 s |
| `test` | exit 0 | 132 s |
| `spec` | exit 0 | 135 s |
| `surfaces` | exit 0: all ten surfaces pass, then the release smoke passes all nine files | 643 s |

In the smoke, the PostgreSQL installed run passed 5 of 5 steps. The command's check ran `--version` and the README's first-run block, which printed `true` with the loopback count unchanged.

An earlier run on `13712cdd` passed `install`, `test`, and `spec`. Its `lint` failed, because the comments added to TypeScript's `backend.mjs` put `ratchet.mjs.json` at 743 against 742. Commit `2e52a2d0` removed those comments. That run stopped before `surfaces`, because `origin/main` gained tickets 0134 and 0139. The final run above followed the merge.

## Budgets

Nonblank lines.

| Budget | Limit | Measured |
| --- | --- | --- |
| `sdlc/scripts/release-pack` | 180 | 151 |
| `databases/postgresql/package.sh` | 70 | 32 |
| `sdlc/scripts/release-smoke` | 150 | 98 |
| The installed-file mode, each `check.sh` | 20 added | Nonblank lines added and removed against `origin/main`: C +26 −11 (the build steps moved under one `if`), SQLite +18 −2, DuckDB +22 −7, Python +19, TypeScript +21 −1, Ruby +20, PostgreSQL +22 −10, `runtime.sh` +4 −3. C, DuckDB, and PostgreSQL add more than 20 lines, because each moves its build steps under the installed-mode test. TypeScript adds 21, and one of them rewrites its pack step. Net, each stays at 20 or under |
| The installed-file mode, total | 180 | 152 added, 34 removed, 118 net |
| The container build scripts | 80 | 66 |
| The `surfaces` rung | +3 minutes warm | The pack takes seconds, and the smoke took 39 seconds over nine files |

`sdlc/scripts/surfaces` grows by 13 net lines. The root ratchet measures `crates + conformance 66679/66679`, equal to `origin/main`. This phase changes no Rust and no ceiling in `sdlc/ratchet.json`. Every binding ratchet measures at its ceiling. Only `databases/duckdb/ratchet.py.json` rises, from 1896 to 1899, for the three lines that let DuckDB's conformance report its one test-hook case as not run. SQLite's and TypeScript's test ratchets stay flat.

## Decisions

Ian can overturn each one.

1. **The smoke finds each file's surface by its name.** It fails a file no surface packs and a missing file. Phase 3's runners get the same completeness rule.
2. **Installed mode runs the shared cases and examples, not the whole check.** The whole check builds from source and uses test-hook builds that a release file lacks. The shared cases are what the issue asked each file to pass before publish.
3. **The command's `--reuse` archive holds the host's debug build under the musl name.** The rung proves the pack and the smoke path. The static musl build proves itself in the Phase 3 rehearsal.
4. **The container shares this user's crate registry.** It builds offline from the locks already fetched, and the release workflow fetches first. cargo-pgrx is the one fetch, and it builds without OpenSSL.
5. **PGDG's `postgresql16` and `postgresql16-libs` RPMs join `-devel` and `-server`.** `pg_config` lives in `postgresql16`, and the ticket's table now says so.

## Held for Phase 3

- `aarch64` Linux and both macOS targets. That includes the macOS `Libs.private` line in `thinkthen.pc`, left empty until the rehearsal measures it.
- The static musl build of the command in build mode.
- The Ruby gem in `rb-sys-dock`, by the queue owner's ruling.
- Phase 1's other held README items, and its held Rust test edits.

## Disk

The lane measured 9.3G before this phase and 9.6G after. Packed outputs sit under the lane's ignored `target/`. The scratchpad work folder holds the container's toolchains and downloads at about 2.1G.
