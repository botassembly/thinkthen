# Area 10: Packaging and release

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

A hand-dispatched GitHub workflow builds every release file on four runners, smokes each one from its own archive, collects a draft, and (in release mode only, behind an environment and an armed variable) publishes to crates.io, PyPI, npm, RubyGems and a Homebrew tap. A curl installer and local scripts serve the same files.

Paths are from the repo root.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, workflows | `.github/workflows/release.yml` (343, 14 jobs), `gate.yml` (83), `pages.yml` (71) |
| Code, scripts | `sdlc/scripts/release-workflow` (730), `workflows` (661), `release-pack` (402), `release-language-tools.py` (368), `versions` (311), `release-managed-pair.py` (244), `release-smoke` (237), `release-go-cpp-pair` (225), `release-container` (164), `package` (85), `publish-builds` (85), `installed.sh` (59), `release-archive-tree.py` (43), `install.sh` (185): about 3,800 |
| Tests | Self-tests: `release-archive-self-test.py` (450), `release-managed-pair-self-test.py` (331), `release-language-tools-self-test.py` (264), `release-smoke-command-test` (74), `release-smoke-source-packages-test` (59), `installer-test` (196), `workflows --self-test`, `versions --self-test` |
| Contract | ADR 0015 and 0047; cleanup ruling 10 (`sdlc/planning/cleanup-2026-09-30.md:25`); `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`; ticket 0128; `sdlc/planning/issue-priorities-2026-09-30.md`; `CHANGELOG.md` (46) |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 3,800 nonblank lines of workflow and script code, 4,300 with the workflows, tests excluded |
| States and concurrency | 3 | A job graph of 14 jobs with a four-runner matrix for `build` and `smoke`, artifact hand-offs, an environment with an armed variable, a heavy-lock file, and a container build per Linux target. No threads in the area's own code |
| Rules and refusals | 5 | `release-workflow` alone holds 120 `fail` refusals. The workflow check adds pin rules for every action, image and tool, and `versions` checks 59 places read 0.0.1 |
| Surfaces touched | 5 | All 22. `release-pack` names 21 parts (command, C, go, cpp, csharp, jvm, swift, zig, php, dart, flutter, ada, objective-c, cobol, sqlite, duckdb, postgresql, python, typescript, ruby, first-run) over 4 targets, for up to 80 native files plus the crate and the npm umbrella (whether each language part is built per target or only on x86-64 Linux was not read) |
| Settings | 1 | No row of `specification/settings.md` applies. The installer reads its own `THINKTHEN_INSTALL_*` variables |
| Contract weight | 2 | Two ADRs, one cleanup ruling, one 9 KB issue and ticket 0128. No spec page owns release. ADR 0015 section 6 is stale (see Quality) |
| Churn and debt | 5 | 110 commits on these paths since 2026-09-23, 7 of them fixes or review answers. Two dispatched rehearsals have both failed on our own bugs, 5 bugs in all. Four issues are open from the second run, plus the language package issue and the macOS symbol debt |

Mean 3.6, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | D | The contract for 0.1 is that a rehearsal passes every job on all four targets with zero "not run" (`sdlc/tickets/0128-release-and-install.md`, Phase 3b). Neither dispatch reached a finished build: run 36778953361 stopped at `resolve`, and run 36780048676 stopped on four bugs, each confirmed in the pinned code: the uv check (`sdlc/scripts/release-workflow:238-239`), the apt note refusal (`sdlc/scripts/release-language-tools.py:183-185`), the fetch list that omits `databases/duckdb/bridge/Cargo.toml` (`sdlc/scripts/release-workflow:224-227`), and five tests that start a command the library-only build never makes (for example `crates/thinkthen/tests/library/key_address.rs`, `ca_bundle.rs`). No package is proven by a rehearsal: 0 of about 80 files. `release.yml` has publish jobs for crates.io, PyPI, npm, RubyGems and the tap only (`release.yml:262-331`). NuGet, Packagist, Maven Central, pub.dev, R-universe and the Go, Zig and Swift tags have no job, though the release issue lists them as channels. ADR 0015 section 6 says "GitHub Actions runs the gate ladder on every push" (`sdlc/planning/adr/0015-*.md:43-45`), while `gate.yml:1-6` says no push starts it, per a closed issue that the ADR does not cite. `CHANGELOG.md:5` lists the command, six libraries and three extensions, and names none of the 11 C-door language packages or Polars, although ruling 10 says 0.1 waits for every surface |
| Reliability | C | The real rehearsal found bugs the self-tests missed. `release-language-tools-self-test.py` fixtures all begin with "Reading package lists", so the real apt note passed review. The host-setup path (uv, cargo fetch, maturin) has no self-test, and the workflow check does not compare the fetch list with the repo's lock files (the issue proposes one). Local runs hide fresh-runner bugs because a developer's target folder and crate registry are already warm. Safety is strong: release mode needs a tag, the `release` environment, and the armed variable as the first step of each outward job (`release.yml:262-270`), and nothing has published. Five failures in two dispatches, all in our code, is the repeated-regression case |
| Maintainability | C | `sdlc/scripts/release-workflow` is one 730-line shell script with 24 verbs; `workflows` is 661 lines. The five family gates appear twice, in `smoke` and again in `draft` (`release.yml:210-225` and `:253-257`). Five pair scripts repeat one shape (`release-go-cpp-pair`, `release-managed-pair.py`, and the three family branches inside `release-workflow`). Versions are read from 59 places by one script, which is good, and a Quick Fix can add a place only by editing that script. Every action is pinned to a commit SHA and each tool to a version, so a new person changes one pin at a time. Debt is filed and owned by ticket 0128 phase 3b |

## Strengths

- Nothing can publish by accident: release mode needs a `v*` tag, the `release` environment and `RELEASE_ARMED`, and each outward job checks the variable before it checks out or downloads (`.github/workflows/release.yml:262-270`, `:347-358`).
- Every action is pinned to a commit SHA with its tag in the comment, and a workflow check rejects unpinned installs of maturin and npm (`.github/workflows/release.yml:25`, `sdlc/scripts/workflows:23-28`).
- The installer refuses a missing or wrong checksum, a staged binary that reports another version, and a symlinked folder (`install.sh:1-12`, `:124-127`).
- The crate package is checked for unexpected files, and the unpacked source must build alone and print the transforms (`sdlc/scripts/package:40-60`).
- A rehearsal builds a draft with a commit target and no tag, so the whole build and smoke can run on any commit without an outward name (ticket 0128 item 4 of "Decisions").

## Cleanup

1. **Land the four rehearsal fixes and run the third rehearsal.** Where: `sdlc/scripts/release-workflow:224-227` (add the DuckDB bridge manifest) and `:238-239` (run `python3 -m uv --version`, print what it read), `sdlc/scripts/release-language-tools.py:183-185` (drop the `startswith` check), and `#[cfg(feature = "cli")]` on the five tests. Why: the workflow has never finished a build. These fixes are in flight under ticket 0128 phase 3b. Size: M. Blocks 0.1: yes.
2. **Add a workflow check that every `Cargo.lock` a release build reads has its manifest in the host-setup fetch list.** Where: `sdlc/scripts/workflows`, `sdlc/scripts/release-workflow:224-227`. Why: the DuckDB bridge's lock alone pins `cc` 1.5.1, and a new lock file can be missed again. Size: S. Blocks 0.1: no.
3. **Add self-test fixtures for the real apt note and a long uv version line.** Where: `sdlc/scripts/release-language-tools-self-test.py`, a new host-setup self-test. Why: the self-tests passed on output the real command never prints. Size: S. Blocks 0.1: no.
4. **Name the language-registry publish path.** Where: `.github/workflows/release.yml` (no NuGet, Packagist, Maven Central, pub.dev, R-universe job), `sdlc/issues/2026-09-25-release-and-install-for-0-1.md` "Still open" item 5. Why: the release issue lists them as channels and the "done when" line requires installs from every named channel. Either add jobs after Ian's account setup, or state which channels ship as GitHub assets only. Size: L. Blocks 0.1: yes.
5. **Correct ADR 0015 section 6 and `CHANGELOG.md:5`.** Where: `sdlc/planning/adr/0015-*.md:43-45`, `CHANGELOG.md:3-5`. Why: the ADR says a push runs the ladder, which a closed ruling overturned. The changelog omits eleven language packages and Polars. Size: S. Blocks 0.1: yes for the changelog (a published contract), no for the ADR.
6. **Write the package-by-platform table with a proof column.** Where: `sdlc/issues/2026-09-25-release-and-install-for-0-1.md`. Why: nobody can now say which of about 80 files are proven. The rehearsal record should fill it. Size: S. Blocks 0.1: no.
7. **Fold the five-family gate loop into one workflow verb.** Where: `.github/workflows/release.yml:210-225` and `:253-257`, `sdlc/scripts/release-workflow`. Why: a new family is added in two places. Size: S. Blocks 0.1: no.
8. **Split `release-workflow` by verb family.** Where: `sdlc/scripts/release-workflow` (730). Why: one shell file holds host setup, capture, gates, tap and draft, with no direct self-test for the setup path that just failed. Size: M. Blocks 0.1: no.

## Confidence: medium

What was read: `release.yml` in full, the head of `gate.yml`, `install.sh` head and target table, the failing lines of `release-workflow` and `release-language-tools.py`, the head of `release-pack`, `package`, the head of `workflows` and `versions`, ADR 0015 section 6, the release issue in full, the four open rehearsal issues, ticket 0128's rehearsal records, and the last commits.

Not checked: no script or workflow ran. I did not read `release-container`, `release-smoke`, `publish-builds`, or the pair scripts' bodies, so the package count by platform (80) is an upper bound from `release-pack`'s part list, and whether each part builds on every target is unconfirmed. The four bugs are in the pinned commit and are being fixed now, so items 1 and 2 may already be resolved on main. I did not check `pages.yml` or the site install page, which marketing owns. The macOS proofs cannot be checked from here.
