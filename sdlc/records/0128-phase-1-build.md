# 0128 Phase 1 build: versions, installer, time limit, and workflow checks

Builder: Claude (Opus subagent), 2026-09-25, on `ticket/0128-release-and-install` from the accepted ticket at `b8cf0076`. The build commit is `130c30ae`. `origin/main` at `0c38769e` merged cleanly at `45253335` before the final run. Ian can overturn every decision below. Phases 2 to 4 each wait for their own go-ahead. This build took no outward step: no workflow dispatch, no tag, and no GitHub setting.

## Outcome

Phase 1 is built on Linux, and every rung passes on the merged head. A version check reads 46 places and can set them all. A download script passes 32 edge cases under dash and bash. One time-limit helper replaces GNU `timeout` in every surface check. `surfaces --release` fails a "not run". A workflow check holds both workflows to dispatch-only runs and pinned code. Every package carries its publish metadata. The R package builds outside the repository against the packed crate. The six community files exist. The dry bump to 0.1.0 passes the ladder with the held Rust test edits applied.

## Ian's rulings folded in

The coordinator relayed Ian's 2026-09-25 rulings during the build.

- `CITATION.cff` and the gemspec list Ian Maurer as the only author.
- `CODE_OF_CONDUCT.md` names Ian as its only contact, and conduct reports go to GitHub issues. The build adds no other community file or channel. `SECURITY.md` still points to GitHub's private vulnerability reporting, as item 13 says. Ian can overturn that and send security reports to issues too.
- One package per language, plus the download script. No per-platform npm packages. This matches decision 6.
- Visibility and going live are Ian's calls. The go-public recommendation and the cost estimate left the ticket.
- Ian's one-time steps now sit in the ticket's one "Ian's setup list" section.

## What each item built

1. `sdlc/scripts/versions` reads 46 places at 0.0.1. It finds binding manifests through `sdlc/surfaces.txt` and lock entries through `lock_versions` over every tracked `Cargo.lock`. `--tag` and `--set` work. Its self-test holds 9 cases. `lint` runs the self-test and the check.
2. Version-free tests: **held.** Ticket 0119 is not on main, so item 4 holds the Rust test edits. The five edits sit ready as a script and run in the dry bump below. The PostgreSQL half landed: `check.sh` reads `EXT_VERSION` from `thinkthen.control` for the shipped SQL name and the update-path plant.
3. The dry bump: see its section below.
4. The Rust test exception: held, as item 2 says.
5. Publish metadata on the crate, `pyproject.toml`, `package.json`, the gemspec, and `DESCRIPTION`. The TypeScript loader picks `thinkthen-<platform>-<arch>.node`, and any other pair gets the pinned refusal "thinkthen: no native addon for <pair>; this package ships linux-x64, linux-arm64, darwin-x64, and darwin-arm64". `check.sh` tests that refusal and the four-name pack list. `publish = false` and `"private": true` stay until Phase 4 step 3.
6. R's published shape in `tools/config.R`. `libraries/r/check.sh` gains a step that builds a copy of the package outside the repository. It uses `cargo package`'s copy of the crate through `[patch.crates-io]` in a private `CARGO_HOME` with `CARGO_NET_OFFLINE=true`. It then asserts the rewritten dependency line and answers one question through the loopback backend.
7. and 8. `install.sh` and its byte-identical site copy, with `THINKTHEN_INSTALL_BASE` and `THINKTHEN_INSTALL_API`. `sdlc/scripts/installer-test` runs 16 edge rows under dash and bash, 32 cases, against a loopback fake GitHub. `lint` runs it.
9. `sdlc/scripts/verdict.sh` and `surfaces --release`. The registry self-test pins the six lines over 0, 77, and 1 in both modes.
10. `sdlc/scripts/time-limit` replaces all sixteen `timeout` calls. The registry self-test pins a passed-through exit code and a timeout that leaves no process alive.
11. The first-run sample: see "Deviations".
12. The README: **held** until ticket 0126 lands, per the coordinator.
13. The six community files.
14. The site's lines: held until Phase 4 step 3, per the ticket.
15. `gate.yml` installs the public API tool, its nightly, and PyYAML from the install rung's pins.
16. `pages.yml` runs by dispatch alone, and every action is pinned to a commit.
17. `sdlc/scripts/workflows` with a 14-case self-test, including a planted `release.yml`. `lint` runs both.

## Deviations

- **The first-run sample reuses demo 27.** Item 11 asked for a new `examples/first-run/`. `demos/27-test-with-no-network` already holds one input, `report.txt`, and a one-entry recording that answers `thinkthen decide 'Does this report say what the person did before the problem appeared?' --replay … < report.txt` with `true`. A copy would duplicate it, so Phase 2 packs the release file from the demo. Ian can overturn this and ask for the separate folder.
- **time-limit kills a process group.** Item 10 said "kills it by process id". A kill by process id leaves the command's children alive, and those children hold the output pipe open. perl's `setpgrp` gives the command its own group, so one kill reaches everything it started. perl also restores the interrupt and quit signals, which POSIX `sh` ignores in a background command. Without that restore, the Ruby Ctrl-C test timed out in the first run.
- **gate.yml reads only `PUBLIC_API_TOOLCHAIN`.** The nightly Quick Fix landed on main before this build, so the fallback for the older pin name was dropped.
- **Two binding ratchets rose.** `libraries/typescript/ratchet.js.json` rose by 7 for the loader's platform pick and refusal. `libraries/r/ratchet.R.json` rose by 17 for the published shape. `sdlc/ratchet.json` did not move.

## Budgets

Nonblank lines.

| Budget | Limit | Measured |
|---|---|---|
| `sdlc/scripts/versions` | 140 | **157** |
| `databases/postgresql/check.sh` version lines | 6 changed | 5 |
| `loader.js` | 20 | 10 added |
| `tools/config.R` | 30 added | 18 |
| `install.sh` | 260 | 176 |
| `sdlc/scripts/installer-test` | 200 | 190 |
| `sdlc/scripts/time-limit` | 25 | 23 |
| The sixteen `timeout` calls | 24 changed | 16 call lines, plus 5 `LIMIT` lines and their 5 comments |
| `surfaces` and `verdict.sh` | 30 added | **34** (25 and 9; 28 net of the 6 lines removed) |
| `sdlc/scripts/workflows` | 170 | **202** |
| `gate.yml` | 15 added | 11 |
| `pages.yml` | 15 changed | 13 |
| The six community files | 220 | 52 |
| The `surfaces` rung | +2 minutes | Not measured on a warm build. Run3's rung took about 19 minutes, lock wait included. Run1's took about 18 minutes, before the R step's first success |

Three budgets were crossed. The coordinator accepted all three on 2026-09-25, because each carries a self-test or a planted fixture that a rung runs. `versions` carries a 9-case self-test with the full output pinned. `workflows` carries a 42-line planted `release.yml` for its 14-case self-test. The `surfaces` overage is the release-rule table and the time-limit self-test. Each file was trimmed first: headers shortened, a shared package-name helper, the three C version rows folded into one, and the pre-Quick-Fix nightly branch dropped.

## Plants

Each plant ran, went red, was restored, and had its file touched.

| Plant | Result |
|---|---|
| `install.sh` without the checksum comparison | `installer-test` 30/32: the wrong-checksum row installed under dash and bash |
| `versions` without the `version.rb` row | self-test 8/9: the `ruby` case failed |
| `workflows` accepting a tag pin | self-test 13/14: the `tag-pin` case failed |
| `verdict` passing 77 in release mode | `surfaces --registry` exit 1: "the release rule printed" |
| `time-limit` without the group kills | `surfaces --registry` exit 1: "time-limit returned 3 and 124, or left a process alive" |
| R's published shape off (`.published_shape <- FALSE`), on the scratch branch | `libraries/r/check.sh` exit 1: the outside build failed to read `/tmp/crates/thinkthen/Cargo.toml` |
| The dry bump without the `recordings.rs` edit | `meta_holds_the_url_the_model_the_usage_and_the_cached_flag` fails at `recordings.rs:234`: the row reads "thinkthen 0.1.0", and the test pins 0.0.1 on line 224 |

The two `time-limit` and `verdict` plants ran again on the trimmed files, with the same results.

## Rungs

On the merged head `45253335`, run once each through the rungs' own lock: install, lint, test, spec, and surfaces all exit 0. Lint prints "versions self-test: 9/9 cases hold", "versions: 46 places read 0.0.1", "workflows self-test: 14/14 cases hold", and "installer-test: 32/32 cases hold under dash and bash". Every landed surface passes: rust, polars, c, python, typescript, ruby, r, duckdb, sqlite, and postgresql. Every ratchet was re-measured after the merge and holds. `sdlc/ratchet.json` stays at 61764.

The first run, on the unmerged tree, failed two surfaces. Ruby's Ctrl-C test timed out, because time-limit's command inherited an ignored interrupt signal. time-limit now restores it. R's outside step built from `git archive HEAD`, which did not hold the uncommitted `config.R`. It passed once committed, and the patch moved into a private `CARGO_HOME`, as the ticket says.

## The dry bump

Local branch `scratch/0128-dry-bump` never lands and is not pushed. It ran `versions --set 0.1.0`, which rewrote 37 files, and applied the held Rust test edits. Then it ran the whole ladder.

1. `lint` failed on `cargo fmt --check`. The `exchange.rs` edit sits inside a `format!` call, and rustfmt splits it over three lines. Fix: the held edit script now writes the formatted form.
2. `lint` then failed on the ratchet: 61766 against 61764. The three-line form adds 2 Rust lines. The coordinator accepted the rise, and the ticket's deferred gaps now say so.
3. One `test` run failed a SIGINT test in `cli::interrupt`. The builder had started that run with a shell `&`, so the ladder inherited an ignored interrupt signal. The same run without `&` passed. No product fault.
4. `test`, `spec`, and `surfaces` then exit 0 at 0.1.0, with every surface passing.

No other test pinned the version.

## Held

- README edits, until ticket 0126 lands.
- The Rust test version edits, item 2's four tests and the fixture, until ticket 0119 lands. The edit script lives outside the repository. The dry bump proved it at 0.1.0.
- The site's tap, "Coming with 0.1", and uninstall lines, until Phase 4 step 3.

## Code review fixes

The code review returned one medium and three low findings. All four are fixed. `main` was not merged again.

1. **Medium: time-limit forwarded no signal.** A Ctrl-C or a TERM stopped time-limit and left the command's own process group running until the limit. time-limit now traps HUP, INT, and TERM. The trap sends TERM to the command's group, stops the watcher, removes the mark, and exits 129, 130, or 143. A nested time-limit receives the outer group's TERM and stops its own group the same way. The `surfaces --registry` table gains a row. It sends TERM to a time-limit whose command started a grandchild, and it requires exit 143 and no surviving grandchild.
2. **Low: install.sh took any version and folder.** After it resolves the version, the script requires X.Y.Z in digits and dies with "not a release version: <value>; give X.Y.Z". It refuses an install folder holding a quote or a backslash, since both reach the receipt's JSON. `installer-test` gains two rows, a `--version ../../x` and a folder named `a"b`, now 36 cases under dash and bash.
3. **Low: `versions --set` wrote a half-edited tree.** `--set` requires `\d+\.\d+\.\d+` and returns 2 otherwise. It builds every edit in memory and writes only after every pattern matched. The self-test gains `bad-set` (`--set 1.2`) and `lost-set` (a missing version line). Both require that no file changed.
4. **Low: every Pages job held write.** The top level now grants `contents: read`. The build job, which runs `npm ci` with the npm cache, holds `contents: read` and `pages: read` for `configure-pages`. Only the deploy job holds `pages: write` and `id-token: write`, and it runs no npm.

Plants, each red, then restored and touched:

| Plant | Result |
|---|---|
| time-limit without its three traps | `surfaces --registry` exit 1: "a TERM to time-limit returned 143, or left a process alive" |
| install.sh without the version check | `installer-test` 34/36: the `../../x` row fails under dash and bash |
| install.sh without the folder check | `installer-test` 34/36: the `a"b` row fails under dash and bash |
| `versions` without the X.Y.Z check | the self-test crashes with an `IndexError` on `--set 1.2`, so `lint` fails |
| `versions` writing each file as it goes | self-test 10/11: `lost-set` fails |

Growth, accepted by the coordinator for these fixes: time-limit 23 to 33 nonblank lines. `versions` 157 to 169. `install.sh` 176 to 185. `installer-test` 190 to 196. `surfaces` 13 more added lines and 1 changed. `pages.yml` 7 more changed lines.

Checks after the fixes: `lint` exit 0, with "versions self-test: 11/11 cases hold", "installer-test: 36/36 cases hold under dash and bash", "workflows self-test: 14/14 cases hold", and the `surfaces --registry` pass line. No Rust file moved, so `test` did not rerun.

## Code re-review fixes

The re-review accepted findings 2 to 4 and named two gaps in time-limit. Both are fixed.

1. **The trap race.** time-limit set its traps after it started the command and the watcher. A signal in that window ended time-limit and left the command running. The traps and an empty `child` now come before the command starts. `stop` skips the command's group while `child` is empty.
2. **A command that ignores TERM.** `stop` sent TERM only. It now starts the same escalation the watcher uses: TERM to the group, then KILL 2 seconds later. It then waits for the command. The watcher is stopped first.

The `surfaces --registry` row now runs a command that ignores TERM (`trap "" TERM`) and starts a grandchild, which inherits the ignored TERM. The row sends TERM to time-limit. It then requires the grandchild to be dead within 4 seconds, before it waits on time-limit, and it requires exit 143. Waiting on time-limit first hid the fault: a TERM-only `stop` blocked until the command's own 30 seconds ran out.

| Plant | Result |
|---|---|
| `stop` sends TERM only, with no KILL | `surfaces --registry` exit 1: "a TERM to time-limit left a process alive after 4 seconds" |
| time-limit without its three traps | `surfaces --registry` exit 1, with the same line |

`surfaces --registry` ran 20 times in a row after the fix: 20 passed, 0 failed. `lint` exit 0. time-limit grows from 33 to 39 nonblank lines, and `surfaces` changes 10 more lines.

The acceptance at `85cf78d0` asked for one more fix. If the TERM row's pid file never appeared, `kill -0 ""` failed and the row passed without testing anything. After its polling loop, the row now fails with "the TERM row's command never wrote its grandchild's pid". Plant: the command wrote to a path the row never reads, and `surfaces --registry` exits 1 with that line. After the restore, `surfaces --registry` and `lint` pass.
