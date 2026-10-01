# 0366: The x86 Linux release build keeps at least 3 GB of disk spare

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-2. Parent: ticket 0128, phase 3b.

## Outcome

The x86 Linux `build` job writes less, so its fullest point leaves at least 3 GB free on the hosted runner instead of 95 MB. It gets there only by writing fewer files and by removing folders our own steps made with `scratch_dir` in the same run (`worktrees.md` rule 11). It deletes nothing of the runner image. The job logs the free space and the size of our big folders at each stage, so the next rehearsal shows the real peak.

## Evidence

- Starts from: ticket 0128's seventh to ninth rehearsals. Run 36820491315's `df` showed 8.4 GB free on x86 Linux before the container started, and `/mnt` on the same disk. Run 36812401623's x86 build passed the container peak with 95 MB left. Run 36817514201 failed earlier, on an HTTP 503, and logged no `df`. The 8.4 GB was measured after `host-setup` and the language tools, so it already excludes the host Ruby that `host-setup` builds.

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

  Other writes in the job, measured from the matching local caches: the Ruby build folder that `libraries/ruby/setup-ruby.sh` keeps holds 771 MB, of which 72 KB are logs. The container builds that Ruby once and keeps the folder through every cargo build. `host-setup` builds a second Ruby on the Linux build host, about 850 MB with its install, and no step of the Linux build job runs it. DuckDB's source and static libraries take about 550 MB, Node 204 MB, and the PostgreSQL RPMs about 60 MB. The pinned image is 1.73 GB.

  Two ideas were measured and dropped. The PostgreSQL part alone, built cold with `CARGO_INCREMENTAL=0`, still filled 1.2 GB. Adding `CARGO_PROFILE_DEV_DEBUG=0` for pgrx's debug-mode SQL generator saved 19 MB. A shared build folder for all parts cannot peak below the largest part's 1.2 GB, so it cannot beat removing each part's folder once packed. The parts also build with different features, so a shared folder would share little.
- Keeps: every release file's name, contents and checksum layout; `release-pack --reuse`, which packs the lanes' surface builds and builds nothing new; a caller's `CARGO_TARGET_DIR`, which `release-pack` keeps using and never removes (the archive self-test sets one); the container's offline build and its run folder; `setup-ruby.sh`'s log folder, which it keeps after success and after failure; the Linux smoke job's host Ruby; the macOS build jobs' host Ruby.
- Changes: three scripts write less, and the workflow logs disk use.
  - `sdlc/scripts/release-pack`: in build mode, when the caller sets no `CARGO_TARGET_DIR`, each part builds in its own `scratch_dir` folder. `release-pack` points `CARGO_TARGET_DIR` and DuckDB's `THINKTHEN_DUCKDB_CPP_BUILD` at it and removes it with `scratch_remove` once the part is packed. After each part it prints one line: the part, its build folder's size, and the free space where the output goes.
  - `libraries/ruby/setup-ruby.sh`: the source trees unpack and build in a `scratch_dir` folder, which the run removes at exit. The configure, make and install logs go to the kept `build.XXXX` folder as now.
  - `sdlc/scripts/release-workflow host-setup`: on Linux, Ruby is set up only for the smoke job (`RELEASE_SMOKE=1`). The `workflows` self-test's host-setup table drops `setup-ruby.sh` from the Linux build rows.
  - `sdlc/scripts/release-workflow disk LABEL [PATH...]`, a new step that prints `df -h /` and the size of each named path that exists. `release.yml` calls it after host setup, after the language tools, after the container, after the managed build and after the final pack. It replaces the bare `df -h / /mnt` line.
- Proof: local measurements before and after, and the release self-tests.
  - The same local measurement on the branch: the peak falls from 3.13 GB to the largest part plus the packed files, with the same release files out. The build record lists both peaks and each part's line.
  - A run of `setup-ruby.sh` into a scratch `HOME` from the existing Ruby archives, offline: the toolchain checks pass, the kept folder holds only the three logs, and the scratch build folder is gone. A planted failing `make` keeps the logs and removes the scratch folder.
  - `release-archive-self-test.py` gains one row: with no `CARGO_TARGET_DIR`, a fake cargo records the folder it was given, and that folder is gone after the run. The existing rows keep a caller's folder.
  - The `workflows` self-test's host-setup table, `release-language-tools-self-test.py`, `release-managed-pair-self-test.py`, `release-registry-self-test.py`, `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `sdlc/scripts/tickets` and `lint` in a clean clone. The `lint` plant already refuses `scratch_remove` on the checkout.
  - Expected runner headroom, summed from the measurements: about 1.9 GB from the build folders, 0.77 GB from the container's Ruby build folder, and 0.85 GB from the host Ruby, about 3.5 GB in all. That takes the 95 MB peak to about 3.6 GB. The next rehearsal's `disk` lines confirm it. No GitHub Actions run is dispatched by this ticket.
- Defers: the ARM Linux and macOS jobs, which had 33 GB or more and gain the same build folder change for free. The small duplicates of downloaded archives in the container (PostgreSQL and OpenSSL RPM copies, under 100 MB). The pinned image and the Rust toolchain, which the build needs.

## What the build taught us
