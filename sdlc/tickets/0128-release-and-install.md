---
flow: build
priority: 128
opens: install.sh site/public/install.sh site/src/data/catalog.mjs site/src/pages/install.astro .github/workflows sdlc/scripts README.md SECURITY.md CONTRIBUTING.md CODE_OF_CONDUCT.md CHANGELOG.md CITATION.cff .github/ISSUE_TEMPLATE libraries/*/check.sh databases/*/check.sh databases/postgresql/package.sh libraries/ruby libraries/r/thinkthen/tools sdlc/records sdlc/tickets
---

# 0128: Release and install for 0.1

Status: ready. Owner: Claude. Builds last, after ticket 0119 lands. Nothing in this ticket starts before the coordinator accepts it.

Review route: a fresh read-only Claude session reviews this design and each part's final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A new user installs ThinkThen 0.1.0 with one Homebrew line or one download script. They can also install it from crates.io, PyPI, npm, RubyGems, R-universe, or a release download for C, SQLite, DuckDB, and PostgreSQL. Every file a user installs was built from the tagged commit and passed the replay cases before it was published. Every surface reads 0.1.0. A user with no key runs the README's first example and sees a real answer.

The ask is `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`, all fifteen items. Ian's rulings of 2026-09-25 govern it:

1. Release publishing runs on GitHub Actions as an intentional release process, with trusted publishing on crates.io, PyPI, npm, and RubyGems. This overturns the 2026-09-22 Actions pause for release jobs only. The gate stays run by hand.
2. The Homebrew tap lives under `botassembly`.
3. R ships through R-universe.

Earlier rulings still hold. The first release is 0.1 on every surface. One crate is named `thinkthen`. One Homebrew line and one download script ship. Package names, the tap, DNS, the site, and papers are Ian's. The release checklist counts a surface check that reports "not run" as a failure (ticket 0111, and queue item 6 of `sdlc/planning/one-line-plan-2026-09-24.md`).

## The one rule on outward steps

No agent step in this ticket publishes a package, pushes a tag, creates a GitHub release that is not a draft, claims a name, creates a repository, sets a secret or a setting, or pushes a workflow with any trigger but `workflow_dispatch`. Each outward step is a manual trigger that Ian starts or approves. The list under "What only Ian can do" names every one. A builder who reaches an outward step stops and hands it to Ian.

The release workflow enforces the rule twice. It starts only from `workflow_dispatch`. Every job that uploads to a registry, pushes to the tap, or publishes the GitHub release runs in the GitHub environment `release`, and Ian is its one required reviewer. GitHub then pauses each such job until Ian approves it.

## Design

The work splits into three parts. Parts A and B land as code with no outward effect. Part C is Ian's run of the release, with the agent at his side.

### Part A: local, no Actions

1. **One version check.** New `sdlc/scripts/versions` reads the version from `crates/thinkthen/Cargo.toml`. It compares every other version-bearing file against it and names each mismatch. The files are the ten `Cargo.toml` files under `libraries/` and `databases/`, `libraries/python/pyproject.toml`, `libraries/typescript/package.json`, `libraries/r/thinkthen/DESCRIPTION`, the three `THINKTHEN_VERSION_*` lines in `libraries/c/include/thinkthen.h`, `databases/postgresql/thinkthen.control`, and each `Cargo.lock` entry for those packages. The Ruby gem reads the crate already, and the DuckDB metadata reads its own `Cargo.toml`, so neither needs a row. The check runs from `lint`. With `--tag vX.Y.Z` it also requires the tag to equal `v` plus the version. The release workflow runs it that way first.
2. **The download script.** New root `install.sh`, ported from BioMCP's `install.sh` with every behavior the issue lists. It installs to `~/.local/bin` or `THINKTHEN_INSTALL_DIR`. `site/public/install.sh` is a byte-for-byte copy, and `sdlc/scripts/versions` also compares the two files. The script is POSIX `sh`, because the site's line pipes into `sh` and Debian's `sh` is dash. It keeps the effect of `pipefail` by writing each download to a file and checking each step's exit code.
3. **The installer's source override.** `THINKTHEN_INSTALL_BASE` replaces `https://github.com/botassembly/thinkthen/releases`. The real boundary is GitHub over HTTPS, and no gate touches the network. The override lets the table test point the script at a `file://` folder of fake releases. It also lets the rehearsal install from its own built files before the repository is public. A mirror can use it too. No other test hook enters.
4. **The packer.** New `sdlc/scripts/release-pack TARGET OUT`. It builds and packs every per-platform release file for one Rust target from a clean checkout, and writes a `.sha256` file beside each. It uses `sha256sum` where present and `shasum -a 256` on macOS. The files it makes are listed under "What a release holds". The PostgreSQL half restores `databases/postgresql/package.sh` from tag `surfaces-wave7-final`. It drops GNU `sed -i` and hard-coded `.so` names, which closes the `package.sh` halves of error-index rows R3-32 and R4-10.
5. **The smoke.** New `sdlc/scripts/release-smoke DIR`. It installs each file in DIR into a fresh temporary prefix, with no repository on the path. It then runs that surface's existing examples against the conformance loopback backend. It stops on the first failure and on any "not run". Each surface's `check.sh` gains one mode: when `THINKTHEN_ARTIFACT` names a built file, it installs that file and skips its own build. The smoke also runs the README's "First run" block, taken verbatim from `README.md`, with the key unset.
6. **Not run is a failure at release.** `sdlc/scripts/surfaces --release` treats exit 77 as a failure and prints `surfaces: FAIL <surface> (not run)`. The plain rung keeps today's "not run" line and its exit code. `release-smoke` calls the release form.
7. **The rung.** The `surfaces` rung gains `release-pack` and `release-smoke` for the host's own target, so the smoke cannot rot between releases. On the Linux gate host that proves the `x86_64` Linux files.
8. **The first run with no key.** New `examples/first-run/` holds one input file and the one-entry recording that answers it. It ships as the release file `thinkthen-first-run.tar.gz`. The README's "First run" block downloads it and runs one `decide` under `--replay`. The command archive keeps the binary alone.
9. **The Mac builds.** `databases/duckdb/check.sh`, `databases/postgresql/check.sh`, and `libraries/ruby/check.sh` each exit 77 on macOS today. Each loses that guard and gains a macOS path. PostgreSQL on macOS builds against Homebrew's `postgresql@16`. `release-pack` carries the SQLite and DuckDB package halves of error-index rows R5-37 and R4-19, which tickets 0109 and 0110 moved here.
10. **The Ruby and R Mac builds.** `libraries/ruby/setup-ruby.sh` and `toolchain.env` gain the macOS Ruby 3.4 build for both chips, pinned by sha256, under `~/.cache/thinkthen-toolchains/`. `libraries/ruby/check.sh` replaces GNU `timeout` with a per-file bound written in `sh`: a background sleeper that kills the child by process id. The `exit 77` "Linux only" guard goes. `libraries/r/thinkthen/tools/config.R` gains one rule for a build outside this repository: with `crates/thinkthen` absent, the R crate depends on the published `thinkthen` at the same exact version. R-universe builds from a subfolder, so it needs that rule.
11. **The README.** An "Install" section above the first example holds the Homebrew line, the download script, and one line on `cargo install thinkthen`. One paragraph says who issues the key, the listed price with the record that measured it, and that `THINKTHEN_BASE_URL` swaps the backend. A table of the outcomes and their exit codes, 0 to 6 and 70, sits near the top and matches `specification/channels.md`. Three badges show the release workflow, the crates.io version, and the license. The Gates sentence says `gate.yml` runs only by hand. It closes stumble-register rows 1, 2, 7, and 18.
12. **Community files.** `SECURITY.md` points to GitHub's private vulnerability reporting and names no address. `CONTRIBUTING.md` points to the ladder and `sdlc/`. `CODE_OF_CONDUCT.md` is the Contributor Covenant 2.1 with the same reporting route. `CHANGELOG.md` starts at 0.1.0 and has a "Breaking changes" heading. `CITATION.cff` names the software, the repository, the license, and the version. `.github/ISSUE_TEMPLATE/bug.md` asks for the command, the output, and `thinkthen --version`.
13. **The site's lines.** `site/src/data/catalog.mjs` changes the tap line to `brew install botassembly/thinkthen/thinkthen` and drops "Coming with 0.1" from both install lines. `install.astro` gains the uninstall line: delete `~/.local/bin/thinkthen` and its receipt `~/.local/bin/thinkthen.install.json`.

### Part B: the workflows, dispatch only

14. **`pages.yml`.** Drop the `push` trigger and keep `workflow_dispatch`. Pin every action to a commit with the tag in a comment, as `gate.yml` does. The poll step stays.
15. **`release.yml`.** New. It starts only from `workflow_dispatch` with two inputs. `mode` is `rehearse` or `release`. `ref` is a commit for `rehearse` or a tag `vX.Y.Z` for `release`. Top-level permissions are `{}`. Every job checks out the one commit SHA that the first job resolves, never a branch. Every action is pinned to a commit. The jobs run in this order:
    - `resolve`: resolve `ref` to one SHA and run `sdlc/scripts/versions`. In `release` mode it adds `--tag`.
    - `build`: a matrix over the four targets runs `release-pack`. Linux command builds are static `musl`. Linux loadable libraries build inside the `manylinux_2_28` container, as BioMCP's wheels do, so they load on glibc 2.28 and later. macOS builds run on native runners for each chip.
    - `wheels`, `npm-pack`, `gems`: build the Python abi3 wheels, one npm package, and one platform gem per target.
    - `smoke`: on a native runner per target, download every built file and run `release-smoke`, which calls `surfaces --release`. The installer runs here against `file://` through `THINKTHEN_INSTALL_BASE`.
    - `draft`: upload every file and checksum to a draft GitHub release. `rehearse` names it `vX.Y.Z-rehearsal-<sha7>` with the commit as target, so no tag exists. `release` uses the tag.
    - In `release` mode only, after `draft`, each in environment `release` and each paused for Ian's approval: `crates`, `pypi`, `npm`, `rubygems`, `tap`, and `publish`. `publish` turns the draft into the release. R-universe then builds from the published release.
16. **The workflow check.** New `sdlc/scripts/workflows` runs from `lint`. For every file in `.github/workflows/` it requires `workflow_dispatch` as the only trigger and every `uses:` pinned to a 40-character SHA. For `release.yml` it requires top-level `permissions: {}`, every checkout at the resolved SHA, and `environment: release` on every job that holds `id-token: write` or reads a secret. It carries planted copies that break each rule, as `tickets --self-test` does.

### Part C: Ian's release run

The agent writes the checklist into `sdlc/records/0128-release-0-1.md` and follows it beside Ian. Every step marked Ian is his. The run happens in one cycle, and nobody edits `release.yml` inside it. A change to `release.yml` restarts the checklist at the rehearsal.

1. Ian finishes the setup items under "What only Ian can do".
2. Ian dispatches `release.yml` in `rehearse` mode on the head of main. Every job passes, and the smoke reports zero "not run". The agent records the run.
3. The agent lands one commit that sets 0.1.0 everywhere and moves `CHANGELOG.md` to 0.1.0. `versions` passes.
4. Ian dispatches `rehearse` again on that commit.
5. Ian pushes tag `v0.1.0` at that commit and dispatches `release` mode with the tag.
6. Ian approves each publish job in turn. The agent checks each registry page after it lands.
7. Ian runs `pages.yml` by hand. The agent then runs the public checks: `curl -fsSL https://thinkthen.dev/install.sh | sh` into a clean prefix on Linux, `brew install botassembly/thinkthen/thinkthen` on the M5, and one install from each registry into a clean place. Each prints `thinkthen 0.1.0` or the surface's version line, and each runs the first-run block.
8. Ian deletes the four local registry tokens and the rehearsal drafts.

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

Registries: `thinkthen` on crates.io, which covers the command through `cargo install` and the Rust library. `thinkthen` on PyPI as four abi3 wheels with no source distribution. `thinkthen` on npm as one package that carries all four native addons, with `loader.js` picking by platform and chip. `thinkthen` on RubyGems as four platform gems. `thinkthen` on R-universe under `botassembly`. The Homebrew formula lives in `botassembly/homebrew-thinkthen`.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **BioMCP's workflow, not `cargo-dist`.** The issue decided this by default. A new release tool in the shipping cycle is the risk BioMCP hit.
2. **Linux and macOS on both chips, and no Windows.** The issue decided this by default. Windows waits for demand.
3. **Dispatch only, and one approval per outward job.** A tag push starts nothing. Ian's click starts the workflow, and his approval releases each publish job. BioMCP starts its release on a tag push. This ticket does not, because the rule on outward steps forbids a workflow that runs on push.
4. **A rehearsal with no tag.** A draft release with a commit target creates no tag until it is published. The rehearsal therefore proves the whole build and smoke on any commit with no outward name. It never publishes.
5. **Static `musl` for the command, glibc 2.28 for loadable libraries.** One static file runs on any Linux. A library loads into a host process, and a glibc host cannot load a `musl` library. The `manylinux_2_28` floor matches BioMCP's.
6. **One npm package with four addons.** The usual pattern adds one npm package per platform. Each of those is a new name, and names are Ian's. One package claims nothing new. It costs a larger download.
7. **Wheels only on PyPI.** A source distribution would compile Rust on the user's machine and needs the engine crate beside it. Four abi3 wheels cover every shipped platform.
8. **The first-run sample is its own release file.** Item 1 asks for the binary alone in the archive, and Homebrew wants that shape. The README fetches the small sample and runs it with no key.
9. **The installer stays POSIX `sh`.** The site's line pipes into `sh`. Changing the line to `bash` would change Ian's site copy.
10. **R builds against the published crate outside this repository.** R-universe builds the package from its subfolder, where the path to `crates/thinkthen` does not exist. Inside the repository nothing changes. Part C publishes crates.io before R-universe builds.
11. **The tap update uses one deploy key.** Trusted publishing covers registries, not a push to another repository. A write deploy key scoped to the tap repository alone lives as a secret of the `release` environment, so only an approved job can read it.
12. **The rung smokes the host's files.** A script that checks something runs from a rung, or it rots. The other three targets run in the rehearsal.
13. **The Rust Polars door is not in this ticket.** Ian ruled that it moves into the `thinkthen` crate behind a `polars` feature. That is a code change to the engine crate and ADR 0047, and it needs its own ticket. This ticket publishes `thinkthen` alone. `libraries/polars` stays `publish = false` either way. If the feature lands first, the published crate carries it.
14. **Mac proof of record comes from the runners, plus one M5 run.** GitHub's macOS runners build and smoke both chips in every rehearsal. Item 9 also asks for one recorded local Mac run of the Ruby check. That run happens once on the M5, the only Mac in the fleet, as a light one-time check that cleans up after itself.

## Edge cases

### The download script

| Input | Result |
| --- | --- |
| Linux `x86_64` or `aarch64`, macOS `x86_64` or `arm64` | Installs the matching archive, prints the `PATH` line when needed, exits 0 |
| Any other system or chip | Names the system and chip, lists the four supported ones, exits 1, writes nothing |
| Checksum file missing | Refuses, exits 1, leaves the old binary |
| Checksum mismatch | Refuses, exits 1, leaves the old binary |
| Staged binary prints another version | Refuses, exits 1, leaves the old binary |
| `latest`, newest release lacks this platform's archive | Installs the newest release that has it |
| No `jq`, or the API fails | Resolves `latest` through the redirect address |
| Install folder is a symlink | Refuses, exits 1 |
| Installed binary is a symlink | Refuses, exits 1 |
| Run killed after staging | The receipt reads `pending`. The next run replaces the binary and writes `installed` |
| Run under dash | Same results as under bash |
| Folder already on `PATH` | Prints no `PATH` line |
| Any run | Edits no shell profile |

### The version check

| Input | Result |
| --- | --- |
| Every file agrees | Exit 0, one summary line |
| One file differs | Exit 1, one line naming the file, its version, and the crate's |
| A file lost its version line | Exit 1, one line naming the file |
| `--tag v0.1.0` at 0.1.0 | Exit 0 |
| `--tag v0.1.0` at 0.0.1, or `--tag 0.1.0` | Exit 1, one line naming the tag and the version |
| `site/public/install.sh` differs from `install.sh` by one byte | Exit 1, one line naming both files |

### The workflow check

| Input | Result |
| --- | --- |
| A workflow with `push`, `pull_request`, `schedule`, or `workflow_run` | Exit 1 naming the file and trigger |
| A `uses:` pinned to a tag or a branch | Exit 1 naming the file and line |
| A job with `id-token: write` or a secret and no `environment: release` | Exit 1 naming the job |
| A checkout of `main` or of `ref` in place of the resolved SHA | Exit 1 naming the job |
| Top-level permissions other than `{}` in `release.yml` | Exit 1 |

### Release surfaces

| Surface check exit | Plain `surfaces` | `surfaces --release` |
| --- | --- | --- |
| 0 | `pass` | `pass` |
| 77 | `not run`, rung exit unchanged | `FAIL <surface> (not run)`, exit 1 |
| Other | `FAIL`, exit 1 | `FAIL`, exit 1 |

## Acceptance

Each test answers the four questions of `CLAUDE.md`. Each row names the behavior it protects and the planted fault that turns it red. No test calls the network, reads a key, or publishes. The builder runs every plant, records it red, and restores and touches the file.

| Test | Behavior | Planted fault that turns it red | Why no existing test catches it | Test-only hook |
| --- | --- | --- | --- | --- |
| `versions --self-test` from `lint` | Every edge row of the version check, whole output pinned | Skip `DESCRIPTION` in the file list. The planted R mismatch passes | No script compares versions today | None |
| `sdlc/scripts/installer-test` from `lint` | Every edge row of the download script, run under both `sh` and `bash` against fake `file://` releases whose fake binary prints a version | Drop the checksum comparison. The mismatch row installs | No installer exists | `THINKTHEN_INSTALL_BASE`, named in Design item 3. The real boundary is GitHub over HTTPS |
| `workflows --self-test` from `lint` | Every edge row of the workflow check against planted copies | Accept tag pins. The tag-pinned copy passes | No check reads workflows | None |
| `surfaces --registry` self-test from `lint` | The release form counts 77 as a failure with the exact line | Map 77 to pass in the release form. The planted 77 surface passes | Today's rung reports 77 as "not run" by design | None |
| `release-smoke` for the host target from `surfaces` | Each packed file installs into a clean prefix with no repository and passes its surface's examples. The README's first-run block prints the recorded answer with the key unset and zero loopback requests | Pack the C archive without `thinkthen.pc`. The C smoke fails. Separately, point the first-run block at another input. It exits 5 | `surfaces` builds from source and never runs a packed file | `THINKTHEN_ARTIFACT` in each `check.sh`. The real boundary is an installed file, and each check only builds from source today |
| Rehearsal run, Part C step 2, recorded | Every file for all four targets builds from one SHA and passes the smoke on native runners with zero "not run" | Not planted in Actions. The same smoke carries the host plants above | Nothing builds macOS or `aarch64` today | None |
| M5 run of `libraries/ruby/check.sh` and `release-smoke`, recorded | The Mac toolchain, `dynamic_lookup`, and the `sh` per-file bound work on a real Mac | Restore the GNU `timeout` call. The M5 run exits 127 on `timeout` | No Mac run exists. R5-37 says the recipe cannot run as written | None |

The README's exit-code table has no test of its own. `specification/channels.md` is the contract, and review checks the copy. A test of a copied table is the copied-inventory junk that `CLAUDE.md` rejects.

## Budgets

Nonblank lines. Stop and re-score before crossing any of them.

- `install.sh`: at most 260. `site/public/install.sh` is the same file.
- `sdlc/scripts/versions`: at most 110, self-test included.
- `sdlc/scripts/installer-test`: at most 170.
- `sdlc/scripts/workflows`: at most 130, self-test included.
- `sdlc/scripts/release-pack`: at most 180. `databases/postgresql/package.sh`: at most 70.
- `sdlc/scripts/release-smoke`: at most 150.
- `sdlc/scripts/surfaces`: at most 20 added.
- The macOS paths in the DuckDB and PostgreSQL checks: at most 40 added together.
- The `THINKTHEN_ARTIFACT` mode: at most 15 added to each `check.sh`, 150 in total.
- Ruby Mac toolchain and the `sh` bound: at most 45 added across `setup-ruby.sh`, `toolchain.env`, and `check.sh`.
- `libraries/r/thinkthen/tools/config.R`: at most 20 added.
- `.github/workflows/release.yml`: at most 420. `pages.yml`: at most 15 changed.
- `README.md`: at most 50 added. The six community files: at most 220 together. `examples/first-run/`: one input and one recording entry.
- No Rust source changes and no new Cargo, Python, npm, or Ruby dependency. The ratchet does not rise. The workflow uses only actions BioMCP already pins, plus the registries' own publish actions, each pinned to a commit.
- The `surfaces` rung grows by at most five minutes on a warm build on the gate host.

## Stop rules

Stop and hand back before any of these:

- Any outward step: a publish, a tag, a non-draft release, a name, a repository, a secret, a setting, or a workflow trigger other than `workflow_dispatch`.
- Any paid or network call from a gate, or any read of a key.
- A registry that does not offer trusted publishing from GitHub Actions when the builder reads its current documentation. The builder then brings Ian the options: a token held as a `release` environment secret, or a hand publish for that registry.
- R-universe cannot build the package from its subfolder against the published crate. The builder then brings the options: a vendored crate on a release branch, or CRAN-style vendoring in the package.
- A surface that will not build or smoke on a macOS runner or on `aarch64` Linux. No surface drops from the release silently.
- A change to `release.yml` after the Part C rehearsal passes. The checklist restarts.
- Any Rust source change, since ticket 0119's mutation audit covers the engine tests.
- A budget above, or the `surfaces` rung time limit.

## Scope and exclusions

Excluded: Windows builds. `cargo-dist`. The Rust Polars feature move, per decision 13. CRAN. DuckDB's community extension repository and signed DuckDB extensions. PostgreSQL through apt, PGXN, or any major version but 16. npm packages per platform. A PyPI source distribution. A container image. Homebrew core. Signing or notarizing macOS binaries. Build provenance attestations beyond what npm adds on its own. VHS recordings, the published skill file, and the other site items that moved to the docs issue. Making the repository public, which is Ian's.

## Dependencies and order

Builds after ticket 0119 lands, and after every surface ticket then in flight. Parts A and B each land through this worktree on `ticket/0128-release-and-install` and close nothing until Part C ends. `site/src/data/catalog.mjs` and `install.astro` are also named by `2026-09-25-site-samples-and-pages-after-the-surfaces-land.md`. If a ticket for that issue is in flight when Part A starts, the builder merges its site changes first and touches only the two install lines. Part C needs the repository public before step 7, because the download script and Homebrew fetch release files anonymously.

## What only Ian can do

Numbered in the order Part C needs them.

1. Decide when the repository goes public. The rehearsal can run while it is private. GitHub bills private-repository runner minutes, and macOS minutes bill at ten times the Linux rate. Recommendation: go public before the first rehearsal, so every run is free, and accept that the draft releases are then visible to collaborators only until published.
2. Re-enable Actions for `release.yml` alone. `gate.yml` and `pages.yml` stay disabled until he runs them by hand.
3. Create the GitHub environment `release` with himself as the one required reviewer, limited to the `main` branch and `v*` tags.
4. Add `botassembly/thinkthen`, workflow `release.yml`, and environment `release` as a trusted publisher on crates.io, PyPI, npm, and RubyGems. Each registry's owner page holds this setting.
5. Create the public repository `botassembly/homebrew-thinkthen`. Add a write deploy key to it and store the private half as the `release` environment secret `TAP_DEPLOY_KEY`.
6. Create `botassembly/botassembly.r-universe.dev` with a `packages.json` that names `thinkthen` at subfolder `libraries/r/thinkthen`, tracking the latest release. Install the R-universe GitHub app on it.
7. Turn on private vulnerability reporting in the repository settings, since `SECURITY.md` points there.
8. Approve one light M5 run of the Ruby check and the smoke under the two-worlds rule.
9. Dispatch each rehearsal and the release, push tag `v0.1.0`, and approve each publish job.
10. Run `pages.yml` by hand after the release publishes.
11. Delete the four local registry tokens once trusted publishing works, and delete the rehearsal drafts.
12. Fill the About box, the topics, and the social preview image the same day.

## What needs a Mac

- The macOS builds and smokes for both chips run on GitHub's macOS runners in every rehearsal and release. They need no Mac of ours.
- One recorded local run of `libraries/ruby/check.sh`, `libraries/r/check.sh`, and `release-smoke` on the M5, per item 9. That closes error-index row R5-37 with the record.
- One `brew install botassembly/thinkthen/thinkthen` on the M5 in Part C step 7. The `x86_64` macOS install is proved on a GitHub runner, since the fleet has no Intel Mac running macOS.

## Complexity

Contract 2; state and timing 2; reach 3; proof 2; cost of error 3; total 12. Final level: 3. The risk is a published file that a user cannot install or that fails where the gate passed. The smoke of every packed file on its native runner is the guard. A second risk is an outward step taken without Ian. The dispatch-only trigger, the `release` environment, and the workflow check are the guard.

## Evidence

- Starts from: The issue `sdlc/issues/2026-09-25-release-and-install-for-0-1.md` and Ian's rulings in `sdlc/planning/issue-backlog-2026-09-25.md`. The BioMCP lessons, filed at `81c5714e` and merged into that issue at `c67f97e8`. BioMCP's `release.yml`, `install.sh`, and `scripts/check-version-sync.sh`, which are MIT and Ian's. Its 0.9.0 wheel crash, its wrong checksums, its late pin to the tagged commit, and its 119 silent docs failures. Ticket 0111's "not run" ruling and its `package.sh` at tag `surfaces-wave7-final`. Ticket 0108's R deferral and ticket 0112's Linux-only Ruby port with error-index row R5-37. `demos/27-test-with-no-network`, which shows `--replay` reads no key. `sdlc/scripts/surfaces`, which maps exit 77 to "not run". `gate.yml`, which pins every action and runs only by hand.
- Keeps: The gate run by hand and `gate.yml` dispatch only. Every surface's current check and its source build. The plain `surfaces` rung's "not run" line. The version 0.0.1 on main until Part C step 3. Every Rust source file. The key rules: no job reads `THINKTHEN_API_KEY` and no gate calls the network. `publish = false` on every binding crate.
- Changes: One version check, one download script, one packer, one smoke, one workflow check, and one dispatch-only release workflow. `pages.yml` loses its push trigger and pins its actions. `surfaces --release` counts "not run" as a failure. Each `check.sh` can test an installed file. Ruby builds on macOS without GNU `timeout`. R builds against the published crate outside the repository. The README gains install, key, exit-code, and badge rows, and six community files appear. The tap moves to `botassembly`. `crates/thinkthen` drops `publish = false` in the 0.1.0 commit.
- Proof: The seven rows of "Acceptance". Four self-tests and the host smoke run from rungs, each with a planted fault. The recorded rehearsal on all four targets and the recorded M5 run. The public install checks of Part C step 7.
- Defers: Windows. `cargo-dist`. The Rust Polars feature move to its own ticket. CRAN, DuckDB community extensions, PostgreSQL packages beyond 16, per-platform npm packages, a PyPI source distribution, macOS signing, and provenance attestations. The docs-issue items: VHS, the skill file, `llms.txt`, and the broken-link check.

## Closes

On landing Part C: `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`. Stumble-register rows 1, 2, 7, and 18 close with the README commit. Error-index rows R3-32, R4-10, R4-19, and R5-37 close with Part A and the M5 record.

## What Ian can overturn

Every decision above. The ones most worth his look: dispatch-only in place of BioMCP's tag trigger (3), one fat npm package in place of new per-platform names (6), no PyPI source distribution (7), the sample as its own release file (8), R against the published crate (10), and leaving the Rust Polars move to its own ticket (13). The recommendation to go public before the first rehearsal is his call on money and visibility.
