---
flow: build
priority: 128
opens: install.sh site/public/install.sh site/src/data/catalog.mjs site/src/pages/install.astro .github sdlc/scripts README.md SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md CHANGELOG.md CITATION.cff examples/first-run crates/thinkthen/Cargo.toml crates/thinkthen/tests/backend crates/thinkthen/tests/fixtures/recognize-detailed.json libraries databases sdlc/records sdlc/tickets
---

# 0128: Release and install for 0.1

Status: done. 0.1.2 published on every registry from tag `v0.1.2` in run 37130570517, GitHub release v0.1.2 is public and Latest, and Pages deployed main's 0.1.2 install text. Phase 4 steps 1 to 9 are done. The public install checks passed on every live channel except the macOS gem, which passed with 0.1.2, and R-universe, which built 0.1.2 after the release. Ian left the retained history step to the agent, and the agent dropped it. Ian confirmed that no local registry tokens exist. The Linux R install line and the `ruby` placeholder gem are tracked in the release record `sdlc/records/0128-release-0-1.md`. Phase 1 landed 2026-09-25 (`sdlc/records/0128-phase-1-build.md`). Phase 2 landed 2026-09-26 (`sdlc/records/0128-phase-2-build.md`). Phase3a landed after independent High ACCEPT at `3b540d19`. Phase 3b rehearsal runs 36945370940, 36998358908 and 37010060315 passed. Owner: the queue owner.

Milestone: 0.1

Current review route: a fresh read-only Codex High release-safety reviewer accepted Phase3a's final diff at `3b540d19`. The earlier Claude design and Phase 1/2 reviews remain the historical review record. Ian explicitly handed the queue and review route to Codex on 2026-09-28.

## Outcome and authority

A new user installs ThinkThen 0.1.0 with one Homebrew line or one download script. They can also install it from crates.io, PyPI, npm, RubyGems, R-universe, or a release download for C, SQLite, DuckDB, and PostgreSQL. Every file a user installs was built from the tagged commit and passed the replay cases on its own platform before it was published. Every surface reads 0.1.0. A user with no key runs the README's first example and sees a real answer.

The ask is `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`, all fifteen items. Ian's rulings of 2026-09-25 govern it:

1. Release publishing runs on GitHub Actions as an intentional release process, with trusted publishing on crates.io, PyPI, npm, and RubyGems. This overturns the 2026-09-22 Actions pause for release jobs only. The gate stays run by hand.
2. The Homebrew tap lives under `botassembly`.
3. R ships through R-universe.

Earlier rulings still hold. The first release is 0.1 on every surface. One crate is named `thinkthen`. One Homebrew line and one download script ship. Package names, the tap, DNS, the site, and papers are Ian's. The release checklist counts a surface check that reports "not run" as a failure (ticket 0111, and queue item 6 of `sdlc/planning/one-line-plan-2026-09-24.md`).

The coordinator's rulings of 2026-09-25 on the first review also govern it. The work runs as four phases inside this ticket. GitHub's macOS runners give the macOS proof of record. Ian's later 2026-09-28 authorization allows native M5 build checks; it supersedes the former no-M5 restriction without replacing the runner proof of record. Release mode is dispatched from the tag.

## Retained history step

The tracked-tree private-name cleanup passed fresh review at `dfd6130c`; the external 30-entry guard found no tracked-path or tracked-file hit. The original issue is closed for that cleanup. Ian's 2026-09-26 ruling asked for a fresh one-commit history before public release. The repository went public with its full history. On 2026-10-03 Ian left the step to the agent, and the agent dropped it, because the tags, the Go module proxy and the registries' provenance now name its commits. The release record's "Closed decisions" holds the reasoning.

## The one rule on outward steps

No agent step in this ticket publishes a package, pushes a tag, creates a GitHub release that is not a draft, claims a name, creates a repository, sets a secret or a setting, or pushes a workflow with any trigger but `workflow_dispatch`. Each outward step is a manual trigger that Ian starts or approves. "Ian's setup list" names every one. A builder who reaches an outward step stops and hands it to Ian.

The release workflow enforces the rule three ways. It starts only from `workflow_dispatch`. Every job that uploads to a registry, pushes to the tap, or publishes the GitHub release runs in the GitHub environment `release`. That environment has Ian as its one required reviewer and allows `v*` tags only. A tag ruleset lets only Ian create a `v*` tag.

## Phase 1: Linux, no Actions

Everything here runs on the Linux gate host through the ladder. Nothing runs on GitHub.

1. **One version check.** New `sdlc/scripts/versions` reads the version from `crates/thinkthen/Cargo.toml`. It compares every other version-bearing place against it and names each mismatch. The places are:
    - the `Cargo.toml` of each binding that `sdlc/surfaces.txt` names, found through the same manifest rule `sdlc/scripts/surfaces` uses. Ticket 0130 then changes the list with no edit here;
    - every `thinkthen` and `thinkthen-*` package entry in every tracked `Cargo.lock`, read with `policy.py`'s `lock_versions`. That covers root `Cargo.lock` line 603, each binding lock's own entry and its `thinkthen` path entry, and `conformance/consumer/Cargo.lock`, whose pins `check_consumer` already holds to the root's;
    - `libraries/python/pyproject.toml`;
    - `libraries/typescript/package.json`, and both version fields of `libraries/typescript/package-lock.json`, lines 3 and 9;
    - `libraries/ruby/lib/thinkthen/version.rb`, which holds its own literal. The gemspec reads the crate, but the loaded module reads this file;
    - `libraries/r/thinkthen/DESCRIPTION`;
    - `libraries/c/include/thinkthen.h`: the three `THINKTHEN_VERSION_*` lines and the comments on lines 2 and 56;
    - `databases/postgresql/thinkthen.control`;
    - `CITATION.cff`, once Phase 1 item 13 adds it.

   The DuckDB metadata reads its own `Cargo.toml` at build time, so it needs no row. The check skips these on purpose, and says so in its header: the `conformance/` crates, which never ship; `site/package.json`, which versions the site app; the planted crates in the Ruby and TypeScript checks; `probes/churn-0086/Cargo.lock`, a sealed probe; and the history in `libraries/c/DESIGN.md`. It also compares root `install.sh` with `site/public/install.sh` byte for byte. It runs from `lint`. With `--tag vX.Y.Z` it also requires the tag to equal `v` plus the version. With `--set X.Y.Z` it writes the version into every place above, every lock entry included, and then checks.
2. **Version-free tests.** A product test that pins the version reads it from the crate. Four tests in `crates/thinkthen/tests/backend` pin `"tool":"thinkthen 0.0.1"`: `choosing.rs` line 165, `exchange.rs` line 156, `recordings.rs` line 224, and `find.rs` line 69. Each builds the string from `env!("CARGO_PKG_VERSION")`. `tests/fixtures/recognize-detailed.json` gains a `$VERSION` placeholder, which `recognize.rs` fills beside `$URL`. `databases/postgresql/check.sh` line 82 names `thinkthen--0.0.1.sql`, and line 570 plants `thinkthen--0.0.1--0.0.2.sql` and updates to `0.0.2`. Both read the version from `thinkthen.control`, and the plant updates to `VERSION-probe`. Input fixtures that the code reads and never compares keep their literal. The dry bump in item 3 proves which those are.
3. **The dry bump.** On a scratch commit that never lands, the builder runs `versions --set 0.1.0` and then the whole ladder, `surfaces` included. Every failure is either a test fixed by item 2 or a stop. The record lists each failure and its fix. The scratch commit is then dropped.
4. **The Rust test exception.** Items 2 and 3 change Rust test files in `crates/thinkthen/tests`. That is the one exception to "no Rust source change". If ticket 0119 is still open when Phase 1 builds, this change waits for 0119 to land, since 0119's mutation audit reads these tests. If 0119 has landed, the change lands with Phase 1.
5. **Publish metadata.** Every package gets the fields its registry shows: description, license, repository `https://github.com/botassembly/thinkthen`, homepage `https://thinkthen.dev`, readme, and keywords.
    - `crates/thinkthen/Cargo.toml` gains `repository`, `homepage`, `readme`, `keywords`, and `categories`. It keeps `publish = false` until Phase 4 step 3.
    - `libraries/python/pyproject.toml` gains `description`, `readme`, `authors`, `classifiers`, and `[project.urls]`.
    - `libraries/typescript/package.json` drops "Not published." from its description and gains `repository`, `homepage`, `keywords`, and `os` and `cpu` lists for the four platforms. It keeps `"private": true` until Phase 4 step 3. `libraries/typescript/build-addon.sh` copies the built library, `.so` on Linux and `.dylib` on macOS, to `thinkthen-<platform>-<arch>.node`, with the two names from `node -p process.platform` and `node -p process.arch`. `files` drops `thinkthen.node` and lists `thinkthen-linux-x64.node`, `thinkthen-linux-arm64.node`, `thinkthen-darwin-x64.node`, and `thinkthen-darwin-arm64.node`. `loader.js` loads the one that matches `process.platform` and `process.arch`. On any other pair it throws exactly `thinkthen: no native addon for <platform>-<arch>; this package ships linux-x64, linux-arm64, darwin-x64, and darwin-arm64`. The TypeScript check pins that sentence by setting `process.platform` to `win32` before it loads the package.
    - `libraries/ruby/thinkthen.gemspec` replaces the `example.invalid` homepage and gains `metadata` source and changelog links. Its `authors` list Ian Maurer alone, by Ian's ruling of 2026-09-25.
    - `libraries/r/thinkthen/DESCRIPTION` gains `URL`, `BugReports`, and `OS_type: unix`. It keeps its `SystemRequirements: Cargo, rustc` line.
6. **R outside the repository.** R-universe builds the package from its subfolder, where `crates/thinkthen` does not exist. `libraries/r/thinkthen/tools/config.R` gains a third shape beside the repository shape and the tarball shape. When `src/rust/vendor/registry` is absent and the path to `crates/thinkthen` does not exist, it does three things. It rewrites `src/rust/Cargo.toml` so the `thinkthen` dependency reads `=VERSION` from crates.io with no path. It runs `cargo update --package thinkthen`, so the lock gains the registry entry and keeps every other pin. It builds with `--locked` and without `--offline`, since R-universe fetches crates. The repository and tarball shapes keep `--locked --offline`.
7. **The download script.** New root `install.sh`, ported from BioMCP's `install.sh`, with every behavior the issue lists. It installs to `~/.local/bin` or `THINKTHEN_INSTALL_DIR`. `site/public/install.sh` is the same bytes. The script is POSIX `sh`, because the site's line pipes into `sh` and Debian's `sh` is dash. It keeps the effect of `pipefail` by writing each download to a file and checking each step's exit code. It checks a checksum with `sha256sum` where present and `shasum -a 256` otherwise.
8. **The installer's two source overrides.** `THINKTHEN_INSTALL_BASE` replaces `https://github.com` and `THINKTHEN_INSTALL_API` replaces `https://api.github.com`. The real boundary is GitHub over HTTPS, and no gate touches the network. The overrides let the installer test point the script at a loopback server. They let the rehearsal install from its own built files before publish. A mirror can use them too.
9. **Not run is a failure at release.** New `sdlc/scripts/verdict.sh` holds one function, `verdict MODE CODE`, that maps a check's exit code to `pass`, `not run`, or `FAIL`. `sdlc/scripts/surfaces` sources it. `surfaces --release` treats exit 77 as a failure and prints `surfaces: FAIL <surface> (not run)`. The plain rung keeps today's "not run" line and its exit code. Phase 2's smoke sources the same function.
10. **One time limit for every check.** New `sdlc/scripts/time-limit SECONDS COMMAND...` in POSIX `sh`. It runs the command in the background, starts a sleeper that kills it by process id when time runs out, and returns the command's exit code or 124 on a kill. Every `timeout` call in a surface check moves to it. Every `timeout` call in a script a surface check runs moves to it too. They are in `databases/duckdb/check.sh` lines 70, 78, and 80, `databases/postgresql/check.sh` line 110, `databases/postgresql/runtime.sh` lines 61, 83, 90, and 98, `databases/sqlite/check.sh` line 57, `libraries/r/tests/with-backend.sh` line 36, `libraries/ruby/check.sh` lines 110, 112, 113, and 124, and `libraries/typescript/check.sh` lines 63 and 74. The two PostgreSQL lines that match "statement timeout" are server messages and stay.
11. **The first run with no key.** New `examples/first-run/` holds one input file and the one-entry recording that answers it. It ships as the release file `thinkthen-first-run.tar.gz`. The README's "First run" block downloads it and runs one `decide` under `--replay`. The command archive keeps the binary alone.
12. **The README.** The "Install" section waits for the Phase 4 step 3 commit, for the same reason as item 14. It goes above the first example and holds the Homebrew line, the download script, and one line on `cargo install thinkthen`. Phase 1 adds the rest. A "First run" block downloads the sample and runs it. One paragraph says who issues the key, the listed price with the record that measured it, and that `THINKTHEN_BASE_URL` swaps the backend. A table of the outcomes and their exit codes, 0 to 6 and 70, sits near the top and matches `specification/channels.md`. Three badges show the release workflow, the crates.io version, and the license. The Gates sentence says `gate.yml` runs only by hand. Phase 1 closes stumble-register rows 2 and 7. `README.md` is shared with ticket 0126. Whichever lands second merges the other's changes.
13. **Community files.** `SECURITY.md` points to GitHub's private vulnerability reporting. `CONTRIBUTING.md` points to the ladder and `sdlc/`. `CODE_OF_CONDUCT.md` is the Contributor Covenant 2.1. It names Ian as its only contact, and conduct reports go to GitHub issues. `CHANGELOG.md` starts at 0.1.0 and has a "Breaking changes" heading. `CITATION.cff` names the software, Ian Maurer as its only author, the repository, the license, and the version, and `versions` checks its version. `.github/ISSUE_TEMPLATE/bug.md` asks for the command, the output, and `thinkthen --version`.
14. **The site's lines, held.** Phase 1 does not touch the site. The tap line, the "Coming with 0.1" markers, and the uninstall line change in the Phase 4 step 3 commit, so no page names an install that does not exist yet. That commit changes the tap line in `site/src/data/catalog.mjs` to `brew install botassembly/thinkthen/thinkthen`, drops "Coming with 0.1" from both install lines, and gives `install.astro` the uninstall line: delete `~/.local/bin/thinkthen` and its receipt `~/.local/bin/thinkthen.install.json`.
15. **The gate workflow.** `gate.yml` installs the two tools the install rung requires and never installed. It reads the pins from `sdlc/scripts/install` with `sed`, so the two files cannot drift. It runs `cargo install --locked cargo-public-api@$PUBLIC_API_VERSION`. The Quick Fix on branch `ticket/qf-install-nightly-and-root-test`, not landed on 2026-09-25, renames the nightly pin. The step works whichever lands first. When the install script names `PUBLIC_API_TOOLCHAIN`, the step runs `rustup toolchain install "$PUBLIC_API_TOOLCHAIN" --profile minimal`. When it names only the older `PUBLIC_API_NIGHTLY`, the step installs the dated toolchain one day after that date, since a nightly's `rustc` reports the previous day, and links it as `nightly`, which the older check reads. It also runs `pip install PyYAML==6.0.1` for item 17. The install rung checks PyYAML by running `python3 -c 'import yaml; print(yaml.__version__)'` and requiring exactly `6.0.1`, the version on the gate host and in Ubuntu 24.04. `sdlc/scripts/lint` gains the calls to `versions`, `installer-test`, and `workflows`.
16. **`pages.yml`.** Drop the `push` trigger and keep `workflow_dispatch`. Pin every action to a commit with the tag in a comment, as `gate.yml` does. The poll step stays.
17. **The workflow check.** New `sdlc/scripts/workflows`, in Python, runs from `lint`. It parses each file in `.github/workflows/` with PyYAML's safe loader. The install rung names PyYAML at a pinned version, as it names `jq`. For every workflow it requires:
    - `workflow_dispatch` as the only trigger;
    - every `uses:` pinned to a 40-character SHA;
    - every container image and every `docker run` image pinned by `@sha256:` digest;
    - no `${{ inputs.* }}` and no `${{ github.event.* }}` inside a `run:` block. Values pass through `env:`;
    - in `gate.yml`, a step that installs `cargo-public-api` and the dated nightly from the pins in `sdlc/scripts/install`.

   For `release.yml`, once it exists, it also requires:
    - top-level `permissions: {}`;
    - no `actions/cache` and no other cache;
    - every checkout at the SHA the `resolve` job outputs;
    - `contents: write` in the `draft` and `publish` jobs alone;
    - `environment: release` on every job that holds `id-token: write` or reads a secret;
    - pinned versions for `maturin`, `npm`, and `gem` wherever a step installs or runs them.

   It carries planted copies that break each rule, as `tickets --self-test` does. Phase 1 plants a `release.yml` copy, so every rule is proved before Phase 3 writes the real one.

## Phase 2: pack and smoke on Linux

Everything here runs on the Linux gate host through the ladder, plus one local proof in a container. Nothing runs on GitHub.

1. **The packer.** New `sdlc/scripts/release-pack TARGET OUT`. It packs every per-platform release file for one Rust target and writes a `.sha256` file beside each. "What a release holds" lists the files. In the release workflow it builds each file from a clean checkout. With `--reuse` it packs the files each surface check already built in its own build folder, and builds nothing. The `surfaces` rung calls it that way. The PostgreSQL half restores `databases/postgresql/package.sh` from tag `surfaces-wave7-final`. It drops GNU `sed -i` and hard-coded `.so` names, which closes the `package.sh` halves of error-index rows R3-32 and R4-10. It also carries the SQLite and DuckDB package halves of rows R5-37 and R4-19, which tickets 0109 and 0110 moved here. `databases/postgresql/check.sh` is shared with ticket 0129. Whichever lands second merges the other's changes.
2. **The smoke.** New `sdlc/scripts/release-smoke DIR`. It starts the conformance loopback backend. Then, for each file in DIR, it installs the file into a fresh temporary prefix and runs that surface's check with `THINKTHEN_ARTIFACT` naming the file. It applies the release rule to each file's check exit code itself: 0 passes, 77 fails as "not run", and any other code fails. That rule is one shell function, `verdict`, in `sdlc/scripts/verdict.sh`. `surfaces` and `release-smoke` both source it, and neither calls the other. The smoke also runs the README's "First run" block, taken from `README.md`, with the key unset. It runs every line of the block except its one `curl` line, and unpacks the sample from DIR in that line's place.
3. **The installed-file mode.** When `THINKTHEN_ARTIFACT` names a built file, a surface's `check.sh` installs that file and skips its own build. The real boundary is an installed file, and each check only builds from source today. Each check keeps the repository's own copy off the import path in the same way. It copies its `tests/` folder into a temporary folder and runs from there, with the repository absent from `PYTHONPATH`, `NODE_PATH`, `RUBYLIB`, and `R_LIBS`. It then asserts that the loaded code came from the install prefix:

    | Surface | Installs with | Asserts the loaded path |
    | --- | --- | --- |
    | Python | `pip install` of the wheel into a fresh venv | `thinkthen.__file__` is under the venv |
    | TypeScript | `npm install` of the tarball into a fresh project | `require.resolve('thinkthen')` is under that project |
    | Ruby | `gem install --install-dir` of the platform gem | `$LOADED_FEATURES` names the gem folder |
    | C | Unpack the archive | `pkg-config --variable=libdir thinkthen` is in the archive folder |
    | SQLite, DuckDB, PostgreSQL | Unpack the archive | The check loads the extension by the unpacked path |
    | Command | Unpack the archive | The check runs the unpacked binary by path |

   R ships no release file, so the smoke has no R row. R-universe builds and checks the package itself. Phase 1's out-of-repository R build is the local proof, and Phase 4 step 8 installs from R-universe on Linux.
4. **The rung.** The `surfaces` rung ends with `release-pack --reuse` and `release-smoke` for the host's own target, so the smoke cannot rot between releases. On the Linux gate host that proves the `x86_64` Linux files.
5. **The container toolchains, proved first.** The release workflow builds Linux loadable libraries inside `quay.io/pypa/manylinux_2_28_x86_64`, pinned by digest. Before Phase 3, the builder packs every Linux loadable file inside that container on the gate host and runs the smoke on the host against the result. Each toolchain inside the container has one named source:

    | Toolchain | Source inside the container |
    | --- | --- |
    | Rust | `rustup-init` at a pinned version, checked by sha256, with the channel `rust-toolchain.toml` names |
    | Python | The image's `/opt/python/cp310-cp310`, with `maturin` at a pinned version |
    | Node headers | The `nodejs.org` tarball for Node 22 at a pinned version, checked by sha256 |
    | C, SQLite, DuckDB | The image's `gcc`. The SQLite amalgamation from `databases/sqlite/amalgamation.sha256`. The DuckDB C API headers from `databases/duckdb/vendor` |
    | PostgreSQL 16 | The PGDG EL8 `postgresql16`, `postgresql16-libs`, `postgresql16-devel`, and `postgresql16-server` RPMs, pinned by sha256 and unpacked with `rpm2archive` as the calling user. `pg_config` sits in `postgresql16`. `cargo-pgrx` 0.17.0 with `--locked`. bindgen's libclang from PyPI's `libclang` 18.1.1 wheel, pinned by sha256, with the image gcc's own headers |
    | Ruby | Phase 3a correction after inspecting `rbsys/x86_64-linux:0.9.130`: that official image has Ruby 3.1.7 and 4.0.2 but no 3.4, and glibc 2.31. The Linux gems instead build from `toolchain.env`'s SHA-pinned Ruby 3.4.11 sources inside each pinned `manylinux_2_28` target image. Those images have OpenSSL runtime libraries but no headers, so `release-container` unpacks the matching SHA-pinned AlmaLinux `openssl-devel` RPM for the build only. The native runner still installs and smokes each platform gem. The earlier `rb-sys-dock` plan remains in the Phase 2 record as history. |

   R needs no container, because R-universe builds it. The `aarch64` half of the container proof runs first in the Phase 3 rehearsal.

## Phase 3: macOS and the release workflow

1. **The macOS paths.** `databases/duckdb/check.sh`, `databases/postgresql/check.sh`, and `libraries/ruby/check.sh` each exit 77 on macOS today. Each loses that guard and gains a macOS path. The PostgreSQL self-test `darwin_reports_not_run`, at lines 103 to 116 of `check.sh`, becomes `darwin_runs`. It pins the new behavior: with `uname` reporting Darwin and no toolchain folder, the check reports "not run" for the missing toolchain, not for the kernel. PostgreSQL on macOS builds against Homebrew's `postgresql@16`, pinned by its bottle's sha256. `libraries/ruby/setup-ruby.sh` and `toolchain.env` gain the macOS Ruby 3.4 build for both chips, pinned by sha256, under `~/.cache/thinkthen-toolchains/`.
2. **`release.yml`.** New. GitHub dispatches only a workflow that is on the default branch, so Phase 3 lands in two steps. Phase 3a lands `release.yml` on main, dispatch only, through this worktree. Phase 3b is Ian's first dispatch of `rehearse`, in item 4. Between the two, nothing can publish. Release mode needs a `v*` tag, which only Ian can create. Each outward job requires release mode, enters environment `release`, and checks that environment's `RELEASE_ARMED` variable as its first step before checkout, download, or publish. Ian sets the variable only after his setup items. An environment variable is unavailable to a job-level `if` before the job starts, so the first-step check must stay inside the environment job. GitHub creates a referenced environment with no protection, so this guard holds even before Ian configures `release`. The workflow check pins the placement on every outward job. The workflow starts only from `workflow_dispatch` with one input, `mode`, set to `rehearse` or `release`. `rehearse` builds the commit it is dispatched from. `release` must be dispatched from a `v*` tag, and the `resolve` job fails on any other ref. There is no `ref` input. Top-level permissions are `{}`, and no job uses a cache. The runners are `ubuntu-24.04` for `x86_64` Linux, `ubuntu-24.04-arm` for `aarch64` Linux, `macos-15` for `arm64` macOS, and `macos-15-intel` for `x86_64` macOS. The jobs run in this order:
    - `resolve`: output the one commit SHA and run `sdlc/scripts/versions`. In `release` mode it adds `--tag` with the tag name.
    - `build`: one job per target runs `release-pack`. Linux command builds are static `musl`. Linux loadable libraries build in the containers of Phase 2 item 5. macOS builds run on the native runner for each chip.
    - `wheels`, `npm-pack`, `gems`, `crate`: build the Python abi3 wheels, one npm package, one platform gem per target, and `cargo package --package thinkthen`.
    - `smoke`: on the native runner for each target, download every built file and run `release-smoke`. The runner installs each host the smoke needs at a pinned version: Python 3.10, Node 22, Ruby 3.4 from the pinned toolchain, R, the DuckDB CLI through `databases/duckdb/tools/setup.sh`, the SQLite amalgamation through `databases/sqlite/setup.sh`, PostgreSQL 16, `pkg-config`, and `jq`. It also installs the packed crate with `cargo install --locked --path` on the unpacked copy and runs the first-run block with it. The installer runs against a loopback server that serves the built files, through the two overrides.
    - `draft`: fail if a release for this name already exists, draft or not. Then upload every file and checksum to a new draft. `rehearse` names it `vX.Y.Z-rehearsal-<sha7>` with the commit as target, so no tag exists. `release` uses the tag.
    - In `release` mode only, after `draft`, each in environment `release` and each paused for Ian's approval: `crates`, `pypi`, `npm`, `rubygems`, `tap`, and `publish`. `publish` turns the draft into the release. R-universe then builds from the published release.
3. **Partial publishes.** A publish job fails without change when its registry already holds this version. After a partial failure, Ian re-runs only the failed jobs from the same run. Nobody dispatches a second release run for a tag whose draft exists, and the `draft` job enforces it.
4. **The first rehearsal, Phase 3b.** After Phase 3a lands, Ian dispatches `rehearse` on main. Every job passes on all four targets, and the smoke reports zero "not run". This run is the macOS proof of record for both chips. It closes error-index row R5-37 with the record `sdlc/records/0128-rehearsal-1.md`.

    **First rehearsal attempt, 2026-09-30.** Ian authorized test runs on 2026-09-30, after ticket 0351's M5 proof landed at `e4136b764`. The coordinator's agent dispatched `gh workflow run release.yml --ref main -f mode=rehearse`. The run is [36778953361](https://github.com/botassembly/thinkthen/actions/runs/36778953361) at main `8bb32e59a`. The workflow's `resolve` refuses any ref but `refs/heads/main` in rehearse mode, so the run could not be dispatched from the tag `checkpoint/surfaces/2026-09-30-1`. Main `8bb32e59a` contains that checkpoint commit `145630045`. The agent watched the run with a guard that would cancel it when `draft` started, because the dispatch allowed no GitHub release, draft or not. The guard never fired.

    | Job | Result |
    | --- | --- |
    | `resolve` | Failure after 8 seconds. Cause: our code. |
    | `build`, `wheels`, `gems`, `npm-pack`, `crate`, `smoke`, `draft` | Skipped, since `resolve` failed |
    | `crates`, `pypi`, `npm`, `rubygems`, `tap`, `publish` | Skipped, as rehearse mode requires |

    `release-workflow resolve` lets `sdlc/scripts/versions` print its success line into `$GITHUB_OUTPUT`, and GitHub refuses the line `versions: 59 places read 0.0.1`. The bug is filed as `sdlc/issues/closed/2026-09-30-release-resolve-writes-the-version-line-into-its-outputs.md`. No runner built or smoked anything, so this run proves no target. It does not close R5-37. It does not check Debt 026, the macOS DuckDB extension's SQLite names, because the macOS jobs never ran. The run published nothing, created no release or tag, and created no environment. No secret or account was needed to reach `resolve`. Environment `release` does not exist yet. GitHub refuses tag rulesets on this private repository without GitHub Pro, so Ian's setup item 3 needs a plan upgrade or a public repository. Rehearse mode references neither. Phase 3b stays open. After the fix lands, the next dispatch repeats this step.

    **Second rehearsal attempt, 2026-09-30.** The quick fix for `resolve` landed at `a7ee565ab`. Under the same test approval, the coordinator's agent dispatched `gh workflow run release.yml --ref main -f mode=rehearse` again. The run is [36780048676](https://github.com/botassembly/thinkthen/actions/runs/36780048676) at main `a7ee565ab`. This dispatch allowed a draft release, which would stay a draft. Phase 4 step 9 has Ian delete rehearsal drafts, so the agent would leave any draft in place. The run never reached `draft`.

    | Job | Result | Cause |
    | --- | --- | --- |
    | `resolve` | Success in 6 seconds | The fix holds |
    | `build` (`aarch64-apple-darwin`, `macos-15`) | Failure in `host-setup` after 2 minutes | Ours: `sdlc/issues/closed/2026-09-30-release-host-setup-uv-check-fails-on-macos.md` |
    | `build` (`x86_64-apple-darwin`, `macos-15-intel`) | Failure in `host-setup` after 7 minutes | Ours: the same uv issue |
    | `build` (`x86_64-unknown-linux-gnu`) | Failure in the managed language tools step after 10 minutes | Ours: `sdlc/issues/closed/2026-09-30-release-language-tools-refuses-the-apt-simulation-note.md` |
    | `build` (`aarch64-unknown-linux-gnu`) | Failure in the container build after 17 minutes | Ours: `sdlc/issues/closed/2026-09-30-release-host-setup-skips-the-duckdb-bridge-crates.md` |
    | `crate` | Failure in `sdlc/scripts/package` after 2 minutes | Ours: `sdlc/issues/closed/2026-09-30-crate-job-library-tests-start-a-command-never-built.md` |
    | `wheels`, `gems`, `npm-pack`, `smoke`, `draft` | Skipped, since `build` or `crate` failed | |
    | `crates`, `pypi`, `npm`, `rubygems`, `tap`, `publish` | Skipped, as rehearse mode requires | |

    All four failures are ours, and each issue names its fix. No failure came from GitHub, a runner, or an upstream package. No runner finished a build, so this run proves no target and does not close R5-37. It does not check Debt 026, because neither macOS job reached the DuckDB build. The run published nothing, created no release, draft, or tag, and changed no setting or secret. No secret was read. Phase 3b stays open. The next dispatch waits until all four fixes land.

    **Third rehearsal attempt, 2026-09-30.** The quick fix for the four build failures landed at `f080fd50c`. Its closed issues name each fix and its proof. Under the same test approval, the agent dispatched `gh workflow run release.yml --ref main -f mode=rehearse` again. The run is [36791693601](https://github.com/botassembly/thinkthen/actions/runs/36791693601) at main `f080fd50c`.

    | Job | Result | Cause |
    | --- | --- | --- |
    | `resolve` | Failure; the job never started | GitHub billing: "The job was not started because recent account payments have failed or your spending limit needs to be increased." |
    | `build`, `wheels`, `gems`, `npm-pack`, `crate`, `smoke`, `draft` | Skipped, since `resolve` failed | |
    | `crates`, `pypi`, `npm`, `rubygems`, `tap`, `publish` | Skipped, as rehearse mode requires | |

    No runner ran a step, so this run tests none of the four fixes, proves no target, and does not close R5-37. It does not check Debt 026. The failure is not ours, so no issue was filed. The agent did not dispatch again, because every run fails the same way until the account's billing is fixed. The run published nothing, created no release, draft, or tag, and changed no setting or secret. No secret was read. Phase 3b stays open. The next dispatch waits for Ian to fix the account's payment method or spending limit under "Billing & plans" in the account settings.

    **Fourth rehearsal attempt, 2026-09-30.** Ian approved rehearse-mode dispatches from `refs/heads/main` on 2026-09-30, with use of the M5 for macOS reproduction. Billing works again. The agent dispatched `gh workflow run release.yml --ref main -f mode=rehearse`. The run is [36804404259](https://github.com/botassembly/thinkthen/actions/runs/36804404259) at main `91878c233`.

    | Job | Result | Cause |
    | --- | --- | --- |
    | `resolve` | Success | |
    | `crate` | Success | The earlier fix holds |
    | `build` (`x86_64-unknown-linux-gnu`) | Failure in the managed language tools step | Ours: `selected_plan` refused apt's `Inst` line for a version that two archives hold, written `noble-updates, noble-security` |
    | `build` (`x86_64-apple-darwin`, `macos-15-intel`) | Failure in `host-setup` | Ours: the runner's Homebrew offered OpenSSL 3.6.3, and the pin named the patch 3.6.4 |
    | `build` (`aarch64-apple-darwin`, `macos-15`) | Failure after the DuckDB link | Ours: `strip_macos.py` matched the runner home `/Users/runner` inside DuckDB's own prebuilt path `/Users/runner/work/duckdb/duckdb/...` |
    | `build` (`aarch64-unknown-linux-gnu`) | Failure packing the command | Ours: no step installs the musl Rust target, and clang compiles the bundled SQLite against glibc's `*64` names, which musl no longer exports |
    | `registries`, `wheels`, `gems`, `npm-pack`, `smoke`, `draft` | Skipped, since `build` failed | |
    | Release-only jobs | Skipped, as rehearse mode requires | |

    The quick fix `Land quick fix: the fourth rehearsal's build failures and the macOS DuckDB SQLite exports` answers all four. `selected_plan` accepts apt's archive list and broken-package tail and names any line it refuses. The OpenSSL check matches the 3.6 series, because Homebrew offers only its newest patch and OpenSSL 3 keeps one ABI within a series. `strip_macos.py` and `release-smoke` allow only DuckDB's upstream prefix under the home folder. `host-setup` adds the musl target on Linux, and `release-pack` compiles SQLite for the static command with `SQLITE_DISABLE_LFS`, using clang when no musl gcc exists. Ticket 0319 said a GitHub runner was unaffected by the musl compiler gap; this run disproves it. On this x86 host, the static command built that way passed `release-smoke --command` and a cache round trip through `thinkthen.sqlite`. Review checked the glibc-header build against musl: `struct stat`, `errno`, the file flags and the mutex sizes are safe on x86-64 and aarch64, and a mismatch such as a fortify or `long double` helper fails at link time. Whether the ARM runner has clang, and whether the aarch64 link resolves, waits for the next run.

    The M5 reproduced the DuckDB failure at `91878c233`: the old check fails with `HOME=/Users/runner`, the new one passes, and a home path outside DuckDB's prefix still fails. It also settled Debt 026: the macOS ARM extension exported 285 `_sqlite3_` names. The fix hides them, and `build.sh` now refuses any; see `sdlc/issues/closed/2026-09-30-duckdb-macos-extension-may-export-sqlite-names.md`. On the M5 the patched extension passed every DuckDB suite except four tests that fail the same way without the fix, filed as `sdlc/issues/closed/2026-09-30-duckdb-check-fails-four-tests-on-macos.md`. The release smoke runs none of the four.

    No runner finished a build, so this run proves no target and does not close R5-37. It published nothing, created no release, draft or tag, and read no secret. Phase 3b stays open.

    **Fifth rehearsal attempt, 2026-09-30.** Under the same approval, the agent dispatched again. The run is [36809341385](https://github.com/botassembly/thinkthen/actions/runs/36809341385) at main `ab4cf0459`.

    | Job | Result | Cause |
    | --- | --- | --- |
    | `resolve` | Success | |
    | `crate` | Failure in the public stop test | Ours: the test's second half reused a backend whose `release` frees every later held reply. The first engine's answer could reach the cache before the second engine looked, so the second engine hit the cache and was never stopped |
    | `build` (`x86_64-unknown-linux-gnu`) | Failure in the managed language tools step | Ours: `check_jdk` searched PATH for `javac` and found the runner image's own JDK |
    | `build` (`x86_64-apple-darwin`) | Failure in `cargo pgrx package` | Ours: host setup wrote no pgrx home, so pgrx stopped with "`$PGRX_HOME` does not exist" |
    | `build` (`aarch64-apple-darwin`) | Failure in `cargo pgrx package` | Ours: the same missing pgrx home. The DuckDB extension built and passed its strip check |
    | `build` (`aarch64-unknown-linux-gnu`) | Success | Clang built the static musl command on the ARM runner |
    | Later jobs | Skipped | |

    The quick fix `Land quick fix: the fifth rehearsal's stop test race, JDK pick and macOS pgrx home` answers all four. The stop test's second half holds its own reply on its own backend. With a 200 ms pause before the second engine asks, the old test fails and the new one passes. `check_jdk` uses `THINKTHEN_JDK_HOME` or the pinned package's tree and never searches PATH. Host setup on macOS writes `~/.pgrx/config.toml` when it is missing, as the Linux container does.

    **Sixth rehearsal attempt, 2026-09-30.** The agent dispatched the last run the approval allowed. The run is [36812401623](https://github.com/botassembly/thinkthen/actions/runs/36812401623) at main `9b3f33b76`.

    | Job | Result | Cause |
    | --- | --- | --- |
    | `resolve` | Success | |
    | `crate` | Success | |
    | `build` (`x86_64-apple-darwin`) | Success | |
    | `build` (`aarch64-apple-darwin`) | Success | `build.sh` found no exported SQLite name in the DuckDB extension |
    | `build` (`aarch64-unknown-linux-gnu`) | Success | |
    | `build` (`x86_64-unknown-linux-gnu`) | Failure in `managed-build` | Ours: the managed tools check demanded `/usr/bin/bwrap`. The build job installs only the JDK packages and runs no bubblewrap consumer |
    | `registries`, `wheels`, `gems`, `npm-pack`, `smoke`, `draft` | Skipped, since `build` failed | |
    | Release-only jobs | Skipped, as rehearse mode requires | |

    The JDK fix held: the runner selected `/usr/lib/jvm/java-21-openjdk-amd64` at 21.0.12.1. The quick fix `Land quick fix: the sixth rehearsal's managed tools check drops bubblewrap` removes the bubblewrap line. The smoke job's language tools step still pins bubblewrap before its installed consumers, and the workflows self-test holds that order. No runner has run the fix yet. The same x86 job warned that 95 MB of disk remained before `managed-build`, so the next run may need to free runner disk before the managed packages.

    Three of four targets built in rehearse mode, including both Macs. The registry, wheel, gem, npm, smoke and draft jobs have not run. No failure so far needs a setup step, account or secret from the documentation team. The runs published nothing, created no release, draft or tag, and read no secret. R5-37 needs the smoke job, so it stays open, and Phase 3b stays open.

    **Seventh to ninth rehearsal attempts, 2026-10-01.** The coordinator allowed three more dispatches under Ian's approval of rehearsals until they pass. Before them, the quick fix `Land quick fix: build the Linux release container on the runner's /mnt disk` moved the container folder to `/mnt` when that disk is writable and roomier, and it logs `df` before and after. The coordinator first asked to delete the image's unused toolchains. Lint refuses that under `worktrees.md` rule 11, and the workspace rule lets a script delete only what it made.

    | Run | Commit | Result |
    | --- | --- | --- |
    | [36817514201](https://github.com/botassembly/thinkthen/actions/runs/36817514201) | `aec47ecab` | `resolve`, `crate`, macOS ARM and ARM Linux passed. x86 Linux failed when the language tools met an HTTP 503 from an outside server. macOS Intel failed when Ruby setup lost DNS for `cache.ruby-lang.org` |
    | [36820491315](https://github.com/botassembly/thinkthen/actions/runs/36820491315) | `728a9a74a` | `resolve`, `crate` and both Macs passed. Both Linux builds failed in the release container: its EL8 curl 7.61 lacks `--retry-all-errors`, which the retry fix had added |
    | [36825140435](https://github.com/botassembly/thinkthen/actions/runs/36825140435) | `7ab8ec3e3` | GitHub did not start `resolve`: "recent account payments have failed or your spending limit needs to be increased" |

    The quick fix `Land quick fix: retry the release downloads on a server error or a lost connection` makes the language tools retry a request twice on a server error, a timeout or a lost connection. The quick fix `Land quick fix: retry the Ruby fetch in a loop the release container's curl supports` replaces the curl flag with a three-try loop. No runner has run either fix.

    The `/mnt` change frees nothing on these runners. Run 36820491315's `df` showed `/` and `/mnt` on the same `/dev/root`: 8.4 GB free on x86 Linux and 33 GB on ARM Linux. Run 36812401623's x86 build passed the container peak with 95 MB left, because the container's scratch folder goes at exit. A slightly larger build would fill the disk. Ian's options: allow the release workflow to delete the runner image's unused toolchains (Android, GHC, the image's .NET, CodeQL, about 20 GB) as a written exception to rule 11 for hosted runners; pay for a larger runner; or have a ticket shrink the container build's own output. The agent recommends the first: it costs nothing and changes no shipped file.

    Both Macs built in each of runs 36820491315 and 36812401623, and ARM Linux built in runs 36812401623 and 36817514201. No run has reached `registries`, `smoke` or `draft`. No failure needs a setup step, account or secret from the documentation team. The runs published nothing, created no release, draft or tag, and read no secret. Phase 3b stays open.

    **Rehearsal run [36864015235](https://github.com/botassembly/thinkthen/actions/runs/36864015235), 2026-10-01**, at main `1437ae845`, after ticket 0369 answered run 36853575250's four smoke failures. Every build and package job passed for the first time, and the ARM Linux smoke passed entirely. Three smoke jobs failed. x86 Linux failed only `libraries/csharp`: the release workflow's managed pair packs `LICENSE` and `README.md` beside the nupkg, and the installed check expected the checkpoint packer's two-file folder. Both Macs failed `databases/postgresql` at `line 97: / 1000: syntax error`: `release-smoke` runs `sh check.sh`, macOS's sh is bash 3.2 in POSIX mode, and the check's guard accepted any bash. Intel Mac also failed one Python shared case, `47-recognize-C18-relations`, on one transport close that no other job saw. Ticket 0375 answers the first two: `release-pack csharp` packs the same four files and the check holds exactly those; the PostgreSQL check re-runs itself under bash 5 outside POSIX mode, the macOS smoke installs Homebrew's bash and checks it, and the check reads macOS's space-padded `wc -l` counts as numbers. A scratch M5 run with a PostgreSQL 16.15 source build found that last failure past the bash fix, and then passed the macOS ARM archive 12 of 12. The run published nothing and created no release, draft or tag. Phase 3b stays open.

    **Rehearsal run [36889643043](https://github.com/botassembly/thinkthen/actions/runs/36889643043), 2026-10-01**, at main `39f3b99fa`, after ticket 0375. Every build and package job passed, and both Linux smoke jobs passed entirely. Both Macs failed one PostgreSQL step, `find_proxy_cases`, with no message after about 36 s. Its loopback proxy is a Python `ThreadingHTTPServer`, whose bind calls `socket.getfqdn`. That lookup stalls about 35 s on a macOS runner, so the proxy printed no port inside the step's 5 s wait. Intel Mac also failed DuckDB's `verify_interrupt.py`: the held call's request never reached the backend's count in its 30 s bound. Run 36864015235's Intel Mac Python case 47 failure did not recur. Ticket 0386 answers the PostgreSQL failure: the shared proxy names itself from its bound address, and the two PostgreSQL proxy steps fail with a reason. The two Intel Mac failures share one cause, which ticket 0373 slice B already fixed on main in `c0abe6a8c`: macOS gave each accepted socket the listener's nonblocking mode, and the backend dropped a connection whose request had not yet arrived. The run published nothing and created no release, draft or tag. Phase 3b stays open.

    **Rehearsal runs 36903959951 to 36937157758, 2026-10-01.** Four more runs each failed one late step. Each failure has its own record.

    | Run | Commit | Result and answer |
    | --- | --- | --- |
    | [36903959951](https://github.com/botassembly/thinkthen/actions/runs/36903959951) | `aaa856664` | Both Mac smoke jobs failed `thinkthen.duckdb_extension exports a panic symbol`. Ticket 0388 hides the Rust symbols in the macOS DuckDB extension |
    | [36916614435](https://github.com/botassembly/thinkthen/actions/runs/36916614435) | `f48fd7403` | Both Mac smoke jobs passed every surface, then failed `installer-test`, which knew only Linux machine names. Record `qf-installer-test-macos-targets.md` |
    | [36926183166](https://github.com/botassembly/thinkthen/actions/runs/36926183166) | `4d4200e0c` | Both Mac smoke jobs failed `installer-smoke`: the loopback install server stalled in `socket.getfqdn`. Record `qf-mac-loopback-servers.md` |
    | [36937157758](https://github.com/botassembly/thinkthen/actions/runs/36937157758) | `997924847` | Every build and all four smoke jobs passed for the first time. `draft` stopped in `collect` because the four first-run archives differed. Record `qf-first-run-reproducible.md` |

    **Clean rehearsals, 2026-10-02. Phase 3b is done.** Three runs passed every job through `draft` on all four targets: `resolve`, `crate`, four `build` jobs, `wheels`, `npm-pack`, `gems`, `registries`, four `smoke` jobs and `draft`. The release-only jobs were skipped, as rehearse mode requires. Each run created only a rehearsal draft. None published, tagged or read a secret.

    | Run | Ref and commit | Version |
    | --- | --- | --- |
    | [36945370940](https://github.com/botassembly/thinkthen/actions/runs/36945370940) | main `94d0500c0` | 0.0.1 |
    | [36998358908](https://github.com/botassembly/thinkthen/actions/runs/36998358908) | main `f65faea4e`, after the 0.1.0 release commit `ff7120f89` (ticket 0387) | 0.1.0 |
    | [37010060315](https://github.com/botassembly/thinkthen/actions/runs/37010060315) | `release/0.1` at `4e880cdf6`, tagged `rc/0.1.0-rc.1` and `checkpoint/surfaces/2026-10-02-1` | 0.1.0 |

    In run 37010060315 the four smoke jobs are 110863135525 (x86 Linux, `ubuntu-24.04`), 110863135686 (ARM Linux, `ubuntu-24.04-arm`), 110863135603 (Apple Silicon, `macos-15`) and 110863135461 (Intel Mac, `macos-15-intel`). Each job passed the command, C, SQLite, DuckDB, PostgreSQL, Python, TypeScript and Ruby files, `installer-test` (50 of 50), `installer-smoke` and `crate-smoke`. No surface reported "not run". The only "not run" lines are documented conformance case skips, such as `25-defect-fault`. Each job also ran ticket 0374's cases: `keeps its own panic hook` on every shipped native library, and the token cap refusal on the command and each installed library. The x86 Linux job also passed the eleven language packages: Go, C++, Swift, Zig, PHP, Dart, Ada, Objective-C, COBOL, C# and the JVM. The `draft` job 110874926335 passed the five language gates and the managed package check, then created the rehearsal draft.

    These runs are the macOS proof of record for both chips. The Ruby gem built and passed its installed check on both Macs, so error-index row R5-37 closes. Release QA round 5 on `checkpoint/surfaces/2026-10-02-1` (`4e880cdf6`) was clean. Phase 4 steps 2, 3 and 4 are done: run 36998358908 is the rehearsal on main, ticket 0387 landed the release commit at `ff7120f89`, and run 37010060315 is the rehearsal from `release/0.1`.

    Ticket 0389 answered the two docs team asks of 2026-10-01. The `nuget` job logs in through NuGet trusted publishing and reads no secret. The `pub` job takes its pub.dev token from the run's OIDC token through `dart-lang/setup-dart` and uses no Google Cloud. pub.dev accepts the release-mode dispatch from the `v*` tag once its `workflow_dispatch` box is ticked, so no tag-push job was needed. A change to `release.yml` restarts the Phase 4 checklist at step 2, so a new rehearsal from `release/0.1` follows that change.

## Phase 4: Ian's release run

The agent writes this checklist into `sdlc/records/0128-release-0-1.md` and follows it beside Ian. Every step marked Ian is his. The run happens in one cycle, and nobody edits `release.yml` inside it. A change to `release.yml` restarts the checklist at step 2.

1. Ian finishes every item in "Ian's setup list".
2. Ian dispatches `rehearse` on the head of main. It passes.
3. The agent runs `versions --set 0.1.0`, which also drops `publish = false` from `crates/thinkthen/Cargo.toml` and `"private": true` from `package.json` (ticket 0376), and dates `CHANGELOG.md`. [release-process.md](../planning/release-process.md) section 5 orders this step with the checkpoint and the `release/0.1` cut, and step 4's rehearsal runs from `release/0.1`. The same commit adds the README's "Install" section and the site's held lines from Phase 1 items 12 and 14. The coordinator lands that commit.
4. Ian dispatches `rehearse` on that commit. It passes.
5. Ian creates tag `v0.1.0` at that commit and dispatches `release` from the tag.
6. Ian approves each publish job in turn. The agent checks each registry page after it lands.
7. Ian runs `pages.yml` by hand.
8. The agent runs the public checks, the first use of the real URLs. Before publish, the draft release is not reachable without a login, so the installer and Homebrew are proved against built files in step 4 and against the real URLs only here. The checks are: `curl -fsSL https://thinkthen.dev/install.sh | sh` into a clean prefix on Linux; one install from each registry into a clean place on Linux; and, on the M5, `brew install botassembly/thinkthen/thinkthen`, `thinkthen --version`, and the first-run block under `--replay`. Each prints `thinkthen 0.1.0` or its surface's version line, and each runs the first-run block. The M5 starts no backend and runs no cargo.
9. Ian deletes the four local registry tokens and the rehearsal drafts.

## What a release holds

For each of the four targets, `x86_64` and `aarch64` Linux and `x86_64` and `arm64` macOS, each with a `.sha256` beside it:

| File | Holds | Channel |
| --- | --- | --- |
| `thinkthen-<ver>-<target>.tar.gz` | The command binary alone | Download script, Homebrew, release page |
| `thinkthen-c-<ver>-<target>.tar.gz` | `thinkthen.h`, the static and shared libraries, and `thinkthen.pc` | Release page |
| `thinkthen-sqlite-<ver>-<target>.tar.gz` | The loadable SQLite extension | Release page |
| `thinkthen-duckdb-<ver>-<target>.tar.gz` | The unsigned DuckDB extension for DuckDB v1.5.5 | Release page |
| `thinkthen-postgresql16-<ver>-<target>.tar.gz` | The PostgreSQL 16 extension, control file, and SQL | Release page |

Once per release: `thinkthen-first-run.tar.gz` and its checksum.

Registries: `thinkthen` on crates.io, which covers the command through `cargo install` and the Rust library. `thinkthen` on PyPI as four abi3 wheels with no source distribution. `thinkthen` on npm as one package that carries all four native addons. `thinkthen` on RubyGems as four platform gems. `thinkthen` on R-universe under `botassembly`. The Homebrew formula lives in `botassembly/homebrew-thinkthen`.

Ticket 0355 adds `Botassembly.ThinkThen` on NuGet, `io.github.botassembly:thinkthen-jvm` on Maven Central, `thinkthen_dart` on pub.dev, `botassembly/thinkthen` on Packagist from the root `composer.json`, and the Go module tag `libraries/go/v<version>`. Its "Needs from the release setup" table lists what those jobs read.

## Decisions

Each is the agent's decision unless marked the coordinator's. Ian can overturn any of them.

1. **BioMCP's workflow, not `cargo-dist`.** The issue decided this by default. A new release tool in the shipping cycle is the risk BioMCP hit.
2. **Linux and macOS on both chips, and no Windows.** The issue decided this by default. Windows waits for demand.
3. **Dispatch only, from the tag, with one approval per outward job.** The coordinator's ruling. A tag creates nothing on its own. Ian's click starts the workflow, and his approval releases each publish job. BioMCP starts its release on a tag push. This ticket does not, because the rule on outward steps forbids a workflow that runs on push.
4. **A rehearsal with no tag.** A draft release with a commit target creates no tag until it is published. The rehearsal therefore proves the whole build and smoke on any commit with no outward name. It never publishes.
5. **Static `musl` for the command, glibc 2.28 for loadable libraries.** One static file runs on any Linux. A library loads into a host process, and a glibc host cannot load a `musl` library. The `manylinux_2_28` floor matches BioMCP's. The Ruby gem's floor is its build image's, and the record states it.
6. **One npm package with four addons.** Ian ruled on 2026-09-25: one package per language, on crates.io, npm, PyPI, RubyGems, and R-universe, plus the download script for the command. The usual pattern adds one npm package per platform, and this ticket adds none. One package claims nothing new. It costs a larger download.
7. **Wheels only on PyPI.** A source distribution would compile Rust on the user's machine and needs the engine crate beside it. Four abi3 wheels cover every shipped platform.
8. **The first-run sample is its own release file.** Item 1 of the issue asks for the binary alone in the archive, and Homebrew wants that shape. The README fetches the small sample and runs it with no key.
9. **The installer stays POSIX `sh`.** The site's line pipes into `sh`. Changing the line to `bash` would change Ian's site copy.
10. **R builds against the published crate outside this repository.** Inside the repository nothing changes. Phase 4 publishes crates.io before R-universe builds.
11. **The tap update uses one deploy key.** Trusted publishing covers registries, not a push to another repository. A write deploy key scoped to the tap repository lives as a secret of the `release` environment, so only an approved job can read it.
12. **The rung packs from its own builds.** A script that checks something runs from a rung, or it rots. Packing the files the checks already built keeps the rung inside its time budget. The other three targets run in the rehearsal.
13. **The macOS proof of record is the runner.** The coordinator's ruling. `macos-15` and `macos-15-intel` build and smoke both chips in every rehearsal. The M5 runs only `brew install`, `thinkthen --version`, and the first-run block, after publish.
14. **PyYAML parses the workflows.** A line-based reader misses YAML's other forms. PyYAML is already on the gate host and on GitHub's Ubuntu images. The install rung pins it.
15. **The Rust Polars door is ticket 0130's.** Ian ruled that it moves into the `thinkthen` crate behind a `polars` feature. Ticket 0130 does that. This ticket publishes `thinkthen` alone. `libraries/polars` stays `publish = false` either way. If the feature lands first, the published crate carries it.

## Edge cases

### The download script

| Input | Result |
| --- | --- |
| Linux `x86_64` or `aarch64`, macOS `x86_64` or `arm64` | Installs the matching archive, prints the `PATH` line when needed, exits 0 |
| Any other system or chip | Names the system and chip, lists the four supported ones, exits 1, writes nothing |
| Checksum file missing | Refuses, exits 1, leaves the old binary |
| Checksum mismatch | Refuses, exits 1, leaves the old binary |
| No `sha256sum`, only `shasum` | Checks with `shasum -a 256` and installs |
| Staged binary prints another version | Refuses, exits 1, leaves the old binary |
| `latest`, newest release lacks this platform's archive | The API lists releases, and it installs the newest one that has it |
| No `jq`, or the API answers an error | Resolves `latest` through the redirect from `/releases/latest` |
| Install folder is a symlink | Refuses, exits 1 |
| Installed binary is a symlink | Refuses, exits 1 |
| Run killed after staging | The receipt reads `pending`. The next run replaces the binary and writes `installed` |
| Run under dash | Same results as under bash |
| Folder already on `PATH` | Prints no `PATH` line |
| Any run | Edits no shell profile |

### The version check

| Input | Result |
| --- | --- |
| Every place agrees | Exit 0, one summary line |
| One place differs | Exit 1, one line naming the file, its version, and the crate's |
| A file lost its version line | Exit 1, one line naming the file |
| `--tag v0.1.0` at 0.1.0 | Exit 0 |
| `--tag v0.1.0` at 0.0.1, or `--tag 0.1.0` | Exit 1, one line naming the tag and the version |
| `--set 0.1.0` | Every place reads 0.1.0, and the check passes |
| `site/public/install.sh` differs from `install.sh` by one byte | Exit 1, one line naming both files |

### The workflow check

| Input | Result |
| --- | --- |
| A workflow with `push`, `pull_request`, `schedule`, or `workflow_run` | Exit 1 naming the file and the trigger |
| A `uses:` pinned to a tag or a branch | Exit 1 naming the file and the step |
| A container image without a digest | Exit 1 naming the file and the image |
| `${{ inputs.mode }}` inside a `run:` block | Exit 1 naming the step |
| `gate.yml` without the public API tool or the dated nightly | Exit 1 naming the missing tool |
| `release.yml` with a cache step | Exit 1 naming the step |
| `contents: write` on a job other than `draft` or `publish` | Exit 1 naming the job |
| A job with `id-token: write` or a secret and no `environment: release` | Exit 1 naming the job |
| An outward job without `environment: release` or a first-step check of its environment `RELEASE_ARMED` variable before outward work | Exit 1 naming the job |
| A checkout of a branch or a ref in place of the resolved SHA | Exit 1 naming the job |
| Top-level permissions other than `{}` in `release.yml` | Exit 1 |
| An unpinned `pip install maturin`, `npm install -g npm`, or `gem update --system` | Exit 1 naming the step |
| A `registries` job that skips `registry-pack`, rehearsal signing, the pub dry run or the `registry-packages` upload, or a `maven` job that uploads before release signing (ticket 0355) | Exit 1 naming the job |
| A trusted-publishing job (`crates`, `pypi`, `npm`, `rubygems`, `nuget`, `pub`) without `id-token: write`, or one that reads a secret (ticket 0389) | Exit 1 naming the job |

### Release surfaces

| Surface check exit | Plain `surfaces` | `surfaces --release` and `release-smoke` |
| --- | --- | --- |
| 0 | `pass` | `pass` |
| 77 | `not run`, rung exit unchanged | `FAIL <surface> (not run)`, exit 1 |
| Other | `FAIL`, exit 1 | `FAIL`, exit 1 |

## Acceptance

Each test answers the four questions of `CLAUDE.md`. Each row names the behavior it protects and the planted fault that turns it red. No test calls the network, reads a key, or publishes. The builder runs every plant, records it red, and restores and touches the file.

| Phase | Test | Behavior | Planted fault that turns it red | Why no existing test catches it | Test-only hook |
| --- | --- | --- | --- | --- | --- |
| 1 | `versions --self-test` from `lint` | Every edge row of the version check, with the whole output pinned | Skip `version.rb` in the list. The planted Ruby mismatch passes | No script compares versions today | None |
| 1 | The dry bump, recorded | The whole ladder passes at 0.1.0 | Revert item 2 in `recordings.rs`. The dry bump fails at line 224 | Nothing has run the ladder at another version | None |
| 1 | `sdlc/scripts/installer-test` from `lint` | Every edge row of the download script, under both `sh` and `bash`. A loopback HTTP server in Python's standard library serves the redirect, the API list, the archives, and the checksums. A fake binary prints its version. The `shasum` row runs with a `PATH` that holds `shasum` and no `sha256sum` | Drop the checksum comparison. The mismatch row installs | No installer exists | `THINKTHEN_INSTALL_BASE` and `THINKTHEN_INSTALL_API`, named in Phase 1 item 8 |
| 1 | R out-of-repo build, from `surfaces` | A copy of `libraries/r/thinkthen` outside the repository installs against an unpacked `cargo package` copy of the crate. Cargo reaches that copy through `[patch.crates-io]` in a private `CARGO_HOME`, and the build runs with `CARGO_NET_OFFLINE=true`, so Cargo refuses any fetch while the command itself carries no `--offline` | Keep the path dependency in the rewrite. The build fails on the missing path | The R check builds inside the repository only | None. `[patch.crates-io]` is Cargo's own mechanism |
| 1 | `workflows --self-test` from `lint` | Every edge row of the workflow check against planted copies | Accept tag pins. The tag-pinned copy passes | No check reads workflows | None |
| 1 | `verdict` table in the rung's self-test | Both modes of `verdict` over 0, 77, and 1, with the exact line each prints | Map 77 to pass in the release mode. The planted 77 row passes | Today's rung reports 77 as "not run" by design | None |
| 1 | `time-limit` table in the rung's self-test | A command that ends in time returns its code, and one that overruns returns 124 and is gone by process id | Drop the kill. The overrun row waits and exits 0 | GNU `timeout` did this, and macOS lacks it | None |
| 2 | `release-smoke` for the host target, from `surfaces` | Each packed file installs into a clean prefix and passes its surface's check from a copied `tests/` folder, loaded from the prefix. The README's first-run block prints the recorded answer with the key unset and zero loopback requests | Pack the C archive without `thinkthen.pc`. The C smoke fails. Separately, add the repository to `PYTHONPATH`. The path assertion fails | `surfaces` builds from source and never runs a packed file | `THINKTHEN_ARTIFACT`, named in Phase 2 item 3 |
| 2 | The container proof, recorded | Every Linux loadable file built in `manylinux_2_28` passes the smoke on the host | Not planted. The host smoke's plants cover the check | Nothing builds in the container today | None |
| 3 | The first rehearsal, recorded | Every file for all four targets builds from one SHA. The packed crate installs. Every file passes the smoke on its native runner with zero "not run" | Not planted in Actions. The same smoke carries the Phase 2 plants | Nothing builds macOS or `aarch64` today | None |

The README's exit-code table has no test of its own. `specification/channels.md` is the contract, and review checks the copy. A test of a copied table is the copied-inventory junk that `CLAUDE.md` rejects.

## Budgets

Nonblank lines. Stop and re-score before crossing any of them.

Phase 1:
- `sdlc/scripts/versions`: at most 140, self-test included.
- The Rust test changes: at most 15 lines changed across the four test files and the fixture.
- `databases/postgresql/check.sh` version lines: at most 6 changed.
- Publish metadata: at most 60 added across the package files. `loader.js`: at most 20.
- `libraries/r/thinkthen/tools/config.R`: at most 30 added.
- `install.sh`: at most 260. `site/public/install.sh` is the same file. `sdlc/scripts/installer-test`: at most 200.
- `sdlc/scripts/time-limit`: at most 25. The sixteen calls that move to it: at most 24 lines changed.
- `sdlc/scripts/surfaces` and `verdict.sh`: at most 30 added together.
- `sdlc/scripts/workflows`: at most 170, self-test included. `gate.yml`: at most 15 added. `pages.yml`: at most 15 changed.
- `README.md`: at most 50 added. The six community files: at most 220 together. `examples/first-run/`: one input and one recording entry.
- The `surfaces` rung grows by at most two minutes on a warm build on the gate host.

Phase 2:
- `sdlc/scripts/release-pack`: at most 180. `databases/postgresql/package.sh`: at most 70.
- `sdlc/scripts/release-smoke`: at most 150.
- The installed-file mode: at most 20 added to each `check.sh`, 180 in total.
- The container build scripts: at most 80.
- The `surfaces` rung grows by at most three more minutes on a warm build.

Phase 3:
- The macOS paths in the DuckDB, PostgreSQL, and Ruby checks: at most 60 added together. Ruby's macOS toolchain: at most 30 added across `setup-ruby.sh` and `toolchain.env`.
- `.github/workflows/release.yml`: at most 450.

Every phase: no Rust source change except Phase 1 item 4, and no new Cargo, npm, or Ruby dependency. The ratchet does not rise. PyYAML is the one new tool, pinned by the install rung. The workflow uses only actions BioMCP already pins, plus the registries' own publish actions, each pinned to a commit.

## Stop rules

Every phase stops and hands back before any of these:

- Any outward step: a publish, a tag, a non-draft release, a name, a repository, a secret, a setting, or a workflow trigger other than `workflow_dispatch`.
- Any paid or network call from a gate, or any read of a key.
- A budget in its phase, or its rung time limit.
- Any Rust source change beyond Phase 1 item 4.

Phase 1 also stops when:
- The dry bump fails outside a test that pins the version. A cache key, recording name, or result field that changes with the version is a product question.
- The R copy cannot build against the packed crate with no network.

Phase 2 also stops when:
- A toolchain has no source inside `manylinux_2_28` that the table above names. The builder brings the options, such as another image pinned by digest or a higher glibc floor for that surface.
- A surface's check cannot keep the repository's copy off its import path.

Phase 3 also stops when:
- A registry does not offer trusted publishing from GitHub Actions when the builder reads its current documentation. The builder brings the options: a token held as a `release` environment secret, or a hand publish for that registry.
- A surface will not build or smoke on a macOS runner or on `ubuntu-24.04-arm`. No surface drops from the release silently.
- `ubuntu-24.04-arm` is not available to this repository when Ian turns runs on. The builder records it and waits for his word.
- A change to `release.yml` after the rehearsal passes. The rehearsal runs again.

Phase 4 stops at every step that is Ian's, and when any job fails. A failed publish job follows Phase 3 item 3.

## Scope and exclusions

Excluded: Windows builds. `cargo-dist`. The Rust Polars feature move, which ticket 0130 owns. CRAN. DuckDB's community extension repository and signed DuckDB extensions. PostgreSQL through apt, PGXN, or any major version but 16. npm packages per platform. A PyPI source distribution. A container image. Homebrew core. Signing or notarizing macOS binaries. Build provenance attestations beyond what npm adds on its own. VHS recordings, the published skill file, and the other site items that moved to the docs issue. Making the repository public, which is Ian's.

## Dependencies and order

### Current Phase 3 preparation, 2026-09-28

The [release preparation record](../records/0128-release-preparation.md) compares its then-current main `69296bba` with this accepted phase plan and the active 0231/0232 branches. It is a factual build handoff, not a change to the four-phase outcome. Phase 1 and Linux x86-64 Phase 2 are landed. Phase 3a now implements the dispatch-only `.github/workflows/release.yml`, Linux ARM container inputs, and macOS PostgreSQL/Ruby host routes in a separate lane. The [Phase 3a build record](../records/0128-phase-3a-build.md) states its exact proof and gaps. Ian's repository/environment/tag setup and manual dispatch are prerequisites for Phase 3b execution, not for writing and reviewing Phase 3a. Native M5 checks are authorized, while the four release runners remain the proof of record and the Phase 4 public checks remain his release run.

The native Ruby rehearsal found that `build.sh`'s exported path-remapping `RUSTFLAGS` replaced Cargo's Darwin target linker flags. Phase 3a restores dynamic Ruby symbol lookup while retaining source-path remapping, then gives the copied Mach-O extension a relative install name. A source compile alone did not prove the gem: the focused M5 check installed the new platform gem into a fresh folder, loaded its native extension there, inspected its minimum OS and dependencies, and scanned its raw bytes for builder paths. That current-OS check does not close the macOS 15 runner or four-target release smoke.

Ticket 0231 owns conversion of the retained DuckDB C API package targets to the accepted C++ path and their distinct native installed checks. Its reviewed Apple Silicon slice landed at `f21e3451`; Linux ARM64 and macOS Intel still select the old C API fallback. Put the remaining 0201/0149/0157 and registers 51/72 **platform** criteria in that owner's package proof; do not discard their SQL acceptance. Phase 3a's `smoke-bundle` refuses an old C API archive before the release smoke, using the distinct C++ package license/notice/dependency files. Ticket 0232 owns the SQLite macOS ARM64 checker and genuine 3.49.0/3.50.0 load boundary; its selected M5 installed checks and independent code review passed at landing `1d6b71f4`; a macOS15 runner result remains. The 0226 C/SQLite and 0227 language-package proof from Linux or the M5 does not transfer to another target or to final release artifacts. Keep all four-target smoke, version, checksum, import-path and zero-not-run checks in 0128. An old C API DuckDB archive must not count as the new C++ package.

Fresh 0232 code review found its first macOS hash correction required `shasum` on Linux, regressing hosts that previously needed only `sha256sum`. Its owner is correcting the existing SQLite setup/check and release-smoke routes. Before integrating, select a working checksum-verification capability on each host and retain one small Linux `sha256sum`-only and macOS `shasum` witness. No new checker framework is needed.

Build Phase 3a's workflow and the independent PostgreSQL/Ruby macOS routes while 0231/0232 finish their claimed files. Have the workflow select one resolved SHA, exact native target/toolchain, a changed installed archive and `release-smoke` on each runner. Use the existing `sdlc/scripts/workflows --self-test` and focused shell/static checks before landing; only a later Ian-dispatched rehearsal can establish native runner results. The macOS build must distinguish SDK headers, `MACOSX_DEPLOYMENT_TARGET`, CMake deployment setting and the Rust standard-library sysroot. The active 0231 investigation found Homebrew Rust 1.95's standard library advertises macOS 26 while official isolated rustup 1.95's advertises macOS 11; verify the selected toolchain and final artifact on the intended macOS 15 runner. An empty quiet-build log establishes neither command nor exit code; retain the invocation and status separately, with hashes for artifact identity. Compilation, a Mach-O metadata check, and load on a newer M5 each prove different boundaries.

The two older DuckDB `cpp/verify_decide.py` and `cpp/verify_tables.py` scripts are not invoked by current `check.sh`. Their fixed row-to-send counts and warm-ignores-total assertion conflict with maximal batching and the landed warm total. The preparation record identifies the retained row, vector-boundary and prepared-error assertions that an owner must move to an invoked suite before retiring those scripts and their README advice. This is a test disposition for 0222/0231, not permission to weaken release smoke.

Phase 1 builds after ticket 0119 lands, or beside it with item 4 held back, as item 4 says. Each phase lands through this worktree on `ticket/0128-release-and-install` before the next starts. `site/src/data/catalog.mjs` and `install.astro` are also named by `2026-09-30-site-replay-folders-have-no-fixture.md`. If a ticket for that issue is in flight, the builder merges its site changes first and touches only the two install lines. These files overlap other tickets. Whichever lands second merges the other's changes, and 0128's phase always merges `origin/main` before its final run:

| File or folder | Other tickets | Order |
| --- | --- | --- |
| `crates/thinkthen/tests/backend/exchange.rs` | 0123, building, and 0127 | Both land before Phase 1 item 2, which then edits one line |
| `crates/thinkthen/tests` | 0126, building, and 0119 | Phase 1 item 4 governs 0119. 0126 lands first |
| `README.md` | 0126 | 0126 lands first |
| `sdlc/scripts/lint` | 0127 | Whichever lands second adds its calls beside the other's |
| `databases/postgresql/check.sh` | 0129 | Whichever lands second |
| The crate list and every lock | 0130, new, which moves Rust Polars into `thinkthen` behind a `polars` feature | The version check finds manifests and locks rather than listing them, so either order works. The published crate carries the `polars` feature if 0130 lands before Phase 4 step 3 | Phase 3b's rehearsal needs Ian's setup items 1 to 3. Phase 4 step 8 needs the release files readable with no login, because the download script and Homebrew fetch them that way. That rests on Ian's visibility call.

## Ian's setup list

Ian ruled on 2026-09-25 that his one-time setup steps sit in this one list. He tracks them in his own launch to-do and gets this list when the release job is ready. The builder asks him nothing more about publishing. Repository visibility, and when Actions runs go live, are his calls. Public Actions runs are off until he says to go live.

One-time setup, in the order the phases need it:

1. Turn Actions on for `release.yml` when he says runs go live. He disabled both workflows by hand on 2026-09-22. `gate.yml` and `pages.yml` stay disabled until he runs them by hand.
2. Create the GitHub environment `release` with himself as the one required reviewer, allowing `v*` tags only.
3. The tag rule: add a tag ruleset on `v*` so only he can create, move, or delete a release tag.
4. Turn on private vulnerability reporting in the repository settings, since `SECURITY.md` points there.
5. The trusted publishers: add `botassembly/thinkthen`, workflow `release.yml`, and environment `release` as a trusted publisher on crates.io, PyPI, npm, and RubyGems. Each registry's owner page holds this setting. The [official crates.io API](https://crates.io/api/v1/crates/thinkthen) and [sparse index entry](https://index.crates.io/th/in/thinkthen) showed an existing `thinkthen` 0.0.1 release on 2026-09-28. Verify Ian's ownership and attach the publisher to that existing crate; no first-publish bootstrap is needed. The local `publish = false` flag does not establish registry state. The [crates.io trusted-publishing RFC](https://github.com/rust-lang/rfcs/blob/master/text/3691-trusted-publishing-cratesio.md) explains why a genuinely uncreated crate would require an initial owner publish before this publisher could be configured.
6. The tap key: create the public repository `botassembly/homebrew-thinkthen`. Add a write deploy key to it and store the private half as the `release` environment secret `TAP_DEPLOY_KEY`.
7. R-universe: create `botassembly/botassembly.r-universe.dev` with a `packages.json` that names `thinkthen` at subfolder `libraries/r/thinkthen`, tracking the latest release. Install the R-universe GitHub app on it. Done on 2026-10-02: the repo exists with that `packages.json`, and Ian installed the app on `botassembly`. Packagist also lists `botassembly/thinkthen` as of the same day, and its GitHub webhook works. The release workflow has no job for either one. Packagist reads the `v0.1.0` tag through its webhook. R-universe builds from the published GitHub release. Phase 4 step 8 installs from both.
8. NuGet trusted publishing (ticket 0389). The `nuget` job reads no key.
   1. On nuget.org, open the menu under your name and choose Trusted Publishing. Check that one policy is active with owner `botassembly`, repository owner `botassembly`, repository `thinkthen`, workflow file `release.yml` and environment `release`. You made it on 2026-10-01. If it shows as temporarily active, restart its seven-day window on that page shortly before the release run.
   2. In GitHub, open Settings, Environments, `release`, and add the environment variable `NUGET_USER` with the value `imaurer`. It must be the nuget.org profile name of the person who made the policy, not an email address. It is a variable, not a secret.
9. pub.dev trusted publishing (ticket 0389). The `pub` job reads no key and uses no Google Cloud.
   1. Upload a placeholder `thinkthen_dart` by hand, because pub.dev offers automated publishing only for a package that exists. The docs team prepares it from `libraries/dart` with the pubspec version set to `0.0.1`; main reads `0.1.0`, and the `pub` job refuses a version pub.dev already holds. Run `dart pub publish` in that folder, signed in with your Google account.
   2. On the package's Admin tab, move it to the verified publisher the docs team set up.
   3. On the same Admin tab, under Automated publishing, choose "Enable publishing from GitHub Actions". Enter repository `botassembly/thinkthen` and tag pattern `v{{version}}`. Tick "Enable publishing from `workflow_dispatch` events" and leave "push events" unticked: the release run is a dispatch from tag `v0.1.0`. Tick "Require GitHub Actions environment" and enter `release`. Save.
10. The arming switch: set the `release` environment variable `RELEASE_ARMED` to `true`, last, after items 5 to 9.

His steps during the release run, in Phase 4:

1. Dispatch each rehearsal, create tag `v0.1.0`, dispatch the release from it, and approve each publish job.
2. Run `pages.yml` by hand after the release publishes.
3. Approve the M5's light checks in Phase 4 step 8 under the two-worlds rule.
4. Delete the four local registry tokens once trusted publishing works, and delete the rehearsal drafts.
5. Fill the About box, the topics, and the social preview image the same day.

## What needs a Mac

- Ian authorized focused M5 native builds on 2026-09-28. They expose host-toolchain and artifact errors before dispatch. `macos-15` and `macos-15-intel` still build and smoke both chips in every rehearsal and release, and the first rehearsal is the proof of record.
- The M5 runs three light commands after publish, in Phase 4 step 8: `brew install botassembly/thinkthen/thinkthen`, `thinkthen --version`, and the README's first-run block under `--replay`. It starts no backend and runs no cargo. It uninstalls afterward.

## Complexity

Contract 2; state and timing 2; reach 3; proof 2; cost of error 3; total 12. Final level: 3. The risk is a published file that a user cannot install or that fails where the gate passed. The smoke of every packed file on its native runner, loaded from its install prefix, is the guard. A second risk is an outward step taken without Ian. The dispatch-only trigger, the `release` environment, the tag ruleset, and the workflow check are the guard.

## Evidence

- Starts from: The issue `sdlc/issues/2026-09-25-release-and-install-for-0-1.md` and Ian's rulings in `sdlc/planning/issue-backlog-2026-09-25.md`. The BioMCP lessons, filed at `81c5714e` and merged into that issue at `c67f97e8`. BioMCP's `release.yml`, `install.sh`, and `scripts/check-version-sync.sh`, which are MIT and Ian's. Its 0.9.0 wheel crash, its wrong checksums, its late pin to the tagged commit, and its 119 silent docs failures. Ticket 0111's "not run" ruling and its `package.sh` at tag `surfaces-wave7-final`. Ticket 0108's R deferral and its tarball shape in `tools/config.R`. Ticket 0112's Linux-only Ruby port with error-index row R5-37. `demos/27-test-with-no-network`, which shows `--replay` reads no key. `sdlc/scripts/surfaces`, which maps exit 77 to "not run". `gate.yml`, which pins every action and runs only by hand, and `sdlc/scripts/install`, which pins `cargo-public-api` 0.52.0 and its nightly. A search of main on 2026-09-25 for the literal version, which found the places Phase 1 items 1 and 2 name.
- Keeps: The gate run by hand and `gate.yml` dispatch only. Every surface's current check and its source build. The plain `surfaces` rung's "not run" line. The version 0.0.1 on main until Phase 4 step 3. Every Rust source file outside the four tests Phase 1 item 2 names. The key rules: no job reads `THINKTHEN_API_KEY` and no gate calls the network. `publish = false` on every binding crate. The R repository and tarball shapes.
- Changes: One version check with `--set`, one download script, one time-limit helper, one packer, one smoke, one workflow check, and one dispatch-only release workflow. `gate.yml` installs the public API tool and its nightly. `pages.yml` loses its push trigger and pins its actions. `surfaces --release` counts "not run" as a failure. Each `check.sh` can test an installed file. DuckDB, PostgreSQL, and Ruby build on macOS. R builds against the published crate outside the repository. Every package gains its publish metadata. The README gains install, key, exit-code, and badge rows, and six community files appear. The tap moves to `botassembly`. `crates/thinkthen` drops `publish = false` in the 0.1.0 commit.
- Proof: The ten rows of "Acceptance". Six self-tests and the host smoke run from rungs, each with a planted fault. The recorded dry bump, container proof, and first rehearsal. The public install checks of Phase 4 step 8.
- Defers: Windows. `cargo-dist`. The Rust Polars feature move, which ticket 0130 owns. CRAN, DuckDB community extensions, PostgreSQL packages beyond 16, per-platform npm packages, a PyPI source distribution, macOS signing, and provenance attestations. The docs-issue items: VHS, the skill file, `llms.txt`, and the broken-link check. The held Rust test edits of Phase 1 item 2 raise the Rust ceiling by 2 when they land after ticket 0119. Whoever lands them re-measures with `sdlc/scripts/ratchet.mjs`.

## Closes

On landing Phase 4: `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`. Stumble-register rows 2 and 7 close with Phase 1's README commit, and rows 1 and 18 with the Phase 4 step 3 commit. Error-index rows R3-32, R4-10, and R4-19 close with Phase 2. Row R5-37 closes with the Phase 3 rehearsal record.

## What Ian can overturn

Every decision above. The ones most worth his look: no PyPI source distribution (7), the sample as its own release file (8), R against the published crate (10), and leaving the Rust Polars move to ticket 0130 (15). `SECURITY.md` keeps GitHub's private vulnerability reporting, which stays on GitHub.

## What the build taught us

The Phase 3a Linux jobs build library packages in pinned containers but run installed smoke on separate native runners. Their shared `host-setup` originally fetched pinned DuckDB tools only on Darwin, leaving both Linux smoke runners dependent on incidental host state. The [Linux DuckDB setup Quick Fix](../records/qf-linux-duckdb-release-setup.md) places that fetch only in native Linux smoke. Its first fake-command witness counted the call but silently supplied `uv` and Python 3.13 before the code did; fresh review caught the missing transitive prerequisites. The corrected witness enforces their order, and the workflow establishes Python 3.13 while retaining the active 3.10 build and 3.12 smoke interpreters. This is local proof of routing and provisioning, not an actual runner result; the first four-target rehearsal and separate ARM64 package integration remain open.

## Review

- Design review 1, 2026-09-25: findings, not ACCEPT. The coordinator ruled the four phases, the runner as the macOS proof of record, release dispatched from the tag, the tag ruleset, and three more items for Ian. This rewrite answers every finding. It adds the dry bump, `versions --set`, and the version-free tests; the full list of version places; the R rewrite, lock, and `OS_type`; one time-limit helper; the CI guards; publish metadata; the packed-crate smoke and the partial-publish rule; the loopback installer test and the `shasum` row; packing from the rung's builds; each container toolchain's source; the smoke runners and the import-path rule; and `gate.yml`'s public API tool and nightly.
- Design review 2, 2026-09-25: the release-safety design is sound. This version answers the eleven items. The smoke and `surfaces` share `verdict` and neither calls the other. The M5 runs only `brew install`, `--version`, and the first-run block. Phase 3 splits at landing `release.yml` on main, with the `RELEASE_ARMED` guard. The version check finds every lock entry through `lock_versions`. It adds the five time-limit calls, the overlap table with 0130, the rehearsal's cost, the npm addon names and refusal sentence, the Quick Fix either-order rule and the PyYAML check, and holds the tap and site lines until Phase 4 step 3.
- Ian's rulings, 2026-09-25: Ian Maurer is the only author in `CITATION.cff` and the gemspec. The Code of Conduct names him as its only contact, and conduct reports go to GitHub issues. No other community files or channels. One package per language, plus the download script, and no per-platform npm packages. Visibility and going live are his calls, so the go-public recommendation and the cost estimate left his list. His one-time setup steps moved into "Ian's setup list".
- The queue owner's rulings on Phase 2, 2026-09-26. Ian can overturn each one.
  1. Phase 2 edits `databases/postgresql/check.sh`, which ticket 0138 also opens. The two changes sit in separate parts of the file. Whichever ticket lands second merges the other.
  2. The manylinux_2_28 image holds no libclang, which pgrx needs. PyPI's `libclang` wheel supplies it, pinned by sha256 and installed as the calling user. It keeps the glibc 2.28 floor, and nothing runs as root. The pinned-source table names it.
  3. The Ruby gem's `rb-sys-dock` build waits for Phase 3. Phase 2 pulls no image but manylinux_2_28.
