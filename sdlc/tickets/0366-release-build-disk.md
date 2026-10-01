# 0366: The x86 Linux release build keeps at least 3 GB of disk spare

Status: landed. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-2. Parent: ticket 0128, phase 3b.

## Outcome

The x86 Linux `build` job writes less, so its fullest point leaves at least 3 GB free on the hosted runner instead of 95 MB. It gets there only by writing fewer files and by removing folders our own steps made with `scratch_dir` in the same run (`worktrees.md` rule 11). It deletes nothing of the runner image. The job logs the free space and the size of our big folders at each stage, so the next rehearsal shows the real peak.

## Evidence

- Starts from: ticket 0128's seventh to ninth rehearsals. Run 36820491315's `df` showed 8.4 GB free on x86 Linux before the container started, and `/mnt` on the same disk. Ticket 0128 records that run 36812401623's x86 job warned of 95 MB free before `managed-build`, just after the container step. That job's log holds no `df` line, and its free space at the start was not logged. Run 36817514201 failed earlier, on an HTTP 503, and logged no `df`. The 8.4 GB was measured after `host-setup` and the language tools, so the host Ruby's 850 MB had already been taken out of it.

  A local baseline on main `56b2dbefe` ran `release-pack x86_64-unknown-linux-gnu OUT c sqlite duckdb postgresql python typescript ruby` in a fresh clone, offline, from the existing crate registry and toolchain caches. These are the container's x86 parts that build with cargo or CMake. The source wrappers copy files and build nothing. A sampler ran `du` on the clone and its `TMPDIR` every 5 s. The peak was 3.13 GB, reached at the end, in 419 s. Each part keeps its own build folder until the run ends:

  | Folder | Size |
  | --- | ---: |
  | `databases/postgresql/target` | 1.2 GB |
  | `libraries/ruby/target` | 311 MB |
  | `libraries/typescript/target` | 258 MB |
  | `databases/duckdb/build` (CMake) | 238 MB |
  | `libraries/python/target` | 230 MB |
  | `libraries/c/target` | 219 MB |
  | `databases/duckdb/bridge/target` | 212 MB |
  | `databases/sqlite/target` | 186 MB |

  Other writes in the job, measured from the matching local caches: the Ruby build folder that `libraries/ruby/setup-ruby.sh` keeps holds 771 MB, of which 72 KB are logs. The container builds that Ruby once and keeps the folder through every cargo build. `host-setup` builds a second Ruby on the Linux build host, about 850 MB with its install, and no step of the Linux build job runs it. DuckDB's source and static libraries take about 550 MB, Node 204 MB, and the PostgreSQL RPMs about 60 MB. The pinned image is 1.73 GB. The container's minimal Rust toolchain is about 600 MB, and the 53 MB source archive sits in three copies (the provenance tar, the run copy and its extracted tree).

  These add up to about 7.3 GB of the roughly 8.3 GB the container step consumed. About 1 GB is not measured. The likely places are the image pull's compressed layers, the container's writable layer and the cargo-pgrx install. The new `disk` lines and the per-part lines close that gap on the next rehearsal.

  Two ideas were measured and dropped. The PostgreSQL part alone, built cold with `CARGO_INCREMENTAL=0`, still filled 1.2 GB. Adding `CARGO_PROFILE_DEV_DEBUG=0` for pgrx's debug-mode SQL generator saved 19 MB. A shared build folder for all parts cannot peak below the largest part's 1.2 GB, so it cannot beat removing each part's folder once packed. The parts also build with different features, so a shared folder would share little.
- Keeps: every release file's name, archive members and checksum layout, with no local path inside; `release-pack --reuse`, which packs the lanes' surface builds and builds nothing new; a caller's `CARGO_TARGET_DIR`, which `release-pack` keeps using and never removes (the archive self-test sets one); the container's offline build and its run folder; `setup-ruby.sh`'s log folder, which it keeps after success and after failure; the Linux smoke job's host Ruby; the macOS build jobs' host Ruby.
- Changes: four scripts write less or keep their writes in the run folder, and the workflow logs disk use.
  - `sdlc/scripts/release-pack`: in build mode, when the caller sets no `CARGO_TARGET_DIR`, each part builds in its own `scratch_dir` folder. `release-pack` points `CARGO_TARGET_DIR` and DuckDB's `THINKTHEN_DUCKDB_CPP_BUILD` at it and removes it with `scratch_remove` once the part is packed. It adds a remap of that folder to `/build/target` in `RUSTFLAGS`, `CFLAGS` and `CXXFLAGS`, after its other remaps, since the folder no longer sits under the remapped checkout. Each build script appends its own `HOME` remap after it, so a `TMPDIR` under `HOME` would show the scratch name under `/build/home`; no release job has one. After each part is packed, and before its folder is removed, it prints one line: the part, its build folder's size, and the free space where the output goes.
  - `sdlc/scripts/release-container`: the container gets `TMPDIR=/work/tmp`, a folder inside its run folder. The part build folders then stay on the disk `linux-work` chose, and out of `HOME`.
  - `libraries/ruby/setup-ruby.sh`: the source trees unpack and build in a `scratch_dir` folder, which the run removes at exit. The configure, make and install logs go to the kept `build.XXXX` folder as now. Ruby's own `config.log` and each `ext/*/mkmf.log` are copied there too, after success and after a failed step, since an extension that configure skips leaves its reason only in `mkmf.log`.
  - `sdlc/scripts/release-workflow host-setup`: on Linux, Ruby is set up only for the smoke job (`RELEASE_SMOKE=1`). The x86 and ARM Linux build jobs both stop building the host Ruby. The `workflows` self-test's host-setup table drops `setup-ruby.sh` from the Linux build rows.
  - `sdlc/scripts/release-workflow disk LABEL [PATH...]`, a new step that prints `df -h /` and the size of each named path that exists. `release.yml` calls it four times in the build step: at its start, after the host setup and language tools steps; after the container; after the managed build; and after the final pack. It replaces the bare `df -h / /mnt` line.
- Proof: local measurements before and after, and the release self-tests.
  - The same local measurement on the branch: the peak falls from 3.13 GB to the largest part plus the packed files, with the same release files out. The build record lists both peaks and each part's line.
  - A run of `setup-ruby.sh` into a scratch `HOME` from the existing Ruby archives, offline: the toolchain checks pass, the kept folder holds the three logs, `config.log` and the `mkmf.log` files, and the scratch build folder is gone. A planted failing `make` keeps the same logs and removes the scratch folder.
  - `release-archive-self-test.py` gains one row: with no `CARGO_TARGET_DIR`, a fake cargo records the folder it was given, and that folder is gone after the run. The existing rows keep a caller's folder.
  - The `workflows` self-test's host-setup table, `release-language-tools-self-test.py`, `release-managed-pair-self-test.py`, `release-registry-self-test.py`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/tickets` and `lint` in a clean clone. The `lint` plant already refuses `scratch_remove` on the checkout.
  - Expected runner headroom, summed from the measurements: the build folders sum to 2.85 GB of the 3.13 GB peak, and the largest, PostgreSQL's, stays at 1.2 GB, so they save about 1.65 GB. The container's Ruby build folder saves 0.77 GB and the host Ruby 0.85 GB, about 3.3 GB in all. That takes the 95 MB low point to about 3.4 GB free, 0.4 GB over the 3 GB aim. The local branch measurement replaces the first figure. The next rehearsal's `disk` lines confirm it. No GitHub Actions run is dispatched by this ticket.
- Defers: the ARM Linux and macOS jobs, which had 33 GB or more. They gain the build folder change, and ARM Linux also drops the host Ruby, with nothing more to do. The small duplicates of downloaded archives in the container (PostgreSQL and OpenSSL RPM copies, under 100 MB). The pinned image and the Rust toolchain, which the build needs.

## What the build taught us

- Moving a build folder out of the checkout moves it out of the path remaps too. Scan the packed files for local paths after any change to where a build writes.
- The local measurement on the branch, at `b335461a5`, built the same seven parts offline from the same caches. The peak fell from 3.13 GB to 1.52 GB, a saving of 1.61 GB, against the 1.65 GB estimated. The run took 418 s, against 419 s on main, at a similar load; a second run took 614 s at a higher load. The per-part lines read: C 219 MB, SQLite 186 MB, DuckDB 305 MB (bridge and CMake), PostgreSQL 1.2 GB, Python 227 MB, TypeScript 255 MB and Ruby 311 MB. The 144 MB left in `databases/duckdb/build` is two copies of the packed extension, as on main.
- The first branch run put the scratch folder's path into the files: the SQLite library named `.../tmp.XXXX/release/build/libsqlite3-sys-.../out/bindgen.rs`, where main named `/build/source/databases/sqlite/target/...`. Main's build folders sat under the checkout, which the remap already covered. A per-part remap fixed it. A scan of all 205 files from the seven outputs then found no local path, as on main. The binaries still differ from main's in that path string and in Rust's symbol hashes, which follow the flags.
- `release-smoke` passed all seven files from the branch run: Python, Ruby, TypeScript, C, DuckDB, PostgreSQL and SQLite. The Ruby gem was built with the Ruby the new `setup-ruby.sh` made, and smoked against the pinned Ruby the check requires.
- `setup-ruby.sh` built Ruby offline from the existing archives into a scratch `HOME` in 139 s. Its own checks passed, and the kept folder holds 2.5 MB of logs, against 771 MB before. A planted `make` that fails only the main build kept `configure.log`, `config.log` and `make.log`, removed the scratch tree and exited 1. A `make` that fails every call stopped in configure and kept `config.log` the same way.
- The new `release-archive-self-test.py` row failed on main's `release-pack`, whose fake cargo got no build folder, and passes on the branch.
- Expected runner low point: 1.61 GB from the build folders, 0.77 GB from the container's Ruby tree and 0.85 GB from the host Ruby, about 3.2 GB in all. The 95 MB warning becomes about 3.3 GB free, just over the 3 GB aim. The next rehearsal's `disk` and per-part lines replace this estimate.
