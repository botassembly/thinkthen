# 0376: Prepare the release candidate cut

Status: in progress. Plan: `sdlc/planning/cleanup-2026-09-30.md`, lane claude-4.

Milestone: 0.1

## Outcome

The 0.1 cut is one checked command plus the README, CHANGELOG and site text.

1. The release workflow's rehearse mode runs from `refs/heads/main` or from a release branch `refs/heads/release/X.Y`, as ADR 0116 item 7 asks. It refuses every other ref. Release mode still runs only from a `v*` tag. ADR 0116 item 7 placed this change at the cut and named `release/*`. The coordinator's brief moves it before the cut, so the cut does not wait on a ticket. Until `release/0.1` exists, the new rule allows nothing new. This ticket amends ADR 0116 item 7 and its Context to record the earlier timing and the stricter `release/X.Y` pattern. Ian can overturn both.
2. `versions --set 0.1.0` writes every version copy that the product or a check compares. It also drops the two publish holds: `publish = false` in `crates/thinkthen/Cargo.toml` and `"private": true` in `libraries/typescript/package.json`. The version check holds the rule both ways. At 0.0.1, the unreleased placeholder, both holds must be present. At any other version, both must be gone.
3. No check or test pins 0.0.1 where the product's output or a built file carries the version. Each reads the version from its source or is a place `versions` writes.
4. `sdlc/planning/release-process.md` holds the cut as a short checklist. It links ticket 0128 for details.

## Evidence

- Starts from: ADR 0116, ticket 0128 and this ticket's dry run.
  - ADR 0116 item 7: rehearse mode must accept `refs/heads/release/*` at the cut. `sdlc/scripts/release-workflow resolve` refuses any rehearse ref but `refs/heads/main`. Release mode has no branch check; it requires `refs/tags/v*`.
  - Ticket 0128 Phase 4 step 3: the release commit runs `versions --set 0.1.0`, drops the two publish holds, dates `CHANGELOG.md`, and adds the README "Install" section and the site's held lines.
  - Ticket 0128 Phase 1 item 2 and its build record's dry bump (`sdlc/records/0128-phase-1-build.md`). In 2026-09 the dry bump passed the whole ladder at 0.1.0 with five held Rust test edits applied. Those edits waited for ticket 0119, because 0119's audit read those tests. They never landed. 0119 is still in progress but now Milestone `later`. Its `opens:` list names `crates/thinkthen/src/engine/deadline_tests*`, its issue file and `sdlc/ratchet.json`. Its current slice leaves the other engine and CLI tests untouched, so it no longer reads the held tests. Both tickets re-measure `sdlc/ratchet.json`, so whichever lands second re-measures it again.
  - This ticket's dry run, 2026-10-01, on a scratch branch from main `2be1777e4` that was then deleted:
    - `versions --set 0.1.0` changed 41 files, 56 lines each way: 19 Cargo manifests and locks, the C header (4 lines), Zig's `build.zig` (3 lines), and the Python, TypeScript, Ruby, R, PostgreSQL, C#, JVM, Dart, C++, Objective-C, COBOL and `CITATION.cff` copies. Dropping the two holds changed 2 more lines in files already on the list. `versions` then read 59 places at 0.1.0.
    - `versions --self-test` failed 2 of 24 cases. `zig-abi` and `zig-diagnostic` plant edits against the literal 0.0.1, so at 0.1.0 the plant changes nothing.
    - `policy.py` failed 9 rules. `thinkthen` must set `publish = false` while the repository is private. The C#, JVM, Dart, Flutter, Flutter example, Objective-C, COBOL, C++ and Zig manifest rules expect 0.0.1.
    - A grep for `0.0.1` (and `0, 0, 1`, `v0.0.1`) outside records, tickets, issues, locks, recorded `.jsonl` fixtures and probes found these misses, each of which fails at 0.1.0:
      - `libraries/go/thinkthen.go` line 11 refuses at compile time any header but 0.0.1. `versions` does not write it, so the Go build fails.
      - Four Dart `pubspec.lock` files hold five entries that record the path package `thinkthen_dart` (and `thinkthen_flutter` in the example) at 0.0.1: `libraries/dart/flutter`, `libraries/dart/flutter/example`, and `libraries/dart/checks/consumers/{alpha,bravo}`. `libraries/dart/pubspec.lock` has no such entry. `pub get` rewrites them, and the Dart installed check's `--enforce-lockfile` comparison fails.
      - Rust tests pin `"tool":"thinkthen 0.0.1"`: `crates/thinkthen/tests/backend/choosing.rs`, `exchange.rs`, `find.rs`, `recordings.rs`, `annotate/record_failure.rs` (three rows, newer than 0128), the fixture `tests/fixtures/recognize-detailed.json`, and `libraries/c/tests/door/golden.rs`.
      - `libraries/go/thinkthen_test.go` pins the same tool string.
      - C#: `check.sh` names `thinkthen-c-0.0.1-...` and `Botassembly.ThinkThen.0.0.1.nupkg`. `tests/package_check.py` names both, pins the header's `0, 0, 1` macros, and plants `PATCH 1` to `PATCH 2`, which plants nothing at 0.1.0. `tests/isolated_consumer.py` names the nupkg and checks the nuspec version. `tests/Installed.csproj` and `tests/Smoke.csproj` reference the package at version 0.0.1.
      - JVM: `tests/installed.py` and `tests/package_check.py` pin the POM version.
      - `sdlc/scripts/release-managed-pair-self-test.py` builds its fixture at 0.0.1, but its `managed-gate` step reads the version from `crates/thinkthen/Cargo.toml`.
      - The site's Rust install example `site/examples/install/rust/files/Cargo.toml` asks for `thinkthen = "0.0.1"`. `site/scripts/smoke-bindings.mjs` patches that requirement to the working tree, so at 0.1.0 the site smoke fails. `site/examples/bindings-proof.json` pins the file's hash. Marketing owns `site/`, so this ticket does not edit it. The cut checklist names the edit and the proof re-run as part of the site text step.
    - The grep also found copies that do not fail at 0.1.0, which this ticket leaves alone:
      - Names a check writes and reads back itself: the Swift and Zig local archive names, the `.pc` files in `libraries/go/check.sh`, `libraries/go/fixtures/*.py` and `sdlc/scripts/installed.sh`, and the Go fixtures' `require ... v0.0.1`, which a `replace` line overrides.
      - Planted or fixture inputs: the planted crates in `sdlc/scripts/lint`, `libraries/ruby/check.sh` and `libraries/typescript/check.sh`; `policy.py`'s second-ureq plant; `release-registry-self-test.py`'s composer plant; `catalog.py`'s planted archive; `transforms/trials/test.sh`; `crates/thinkthen/src/core/mod.rs`'s version-format table; the recorded `.jsonl` fixtures, which tests read and never compare as tool strings (the 0128 dry bump proved this).
      - Crates and packages that never ship: `conformance/*`, `databases/duckdb/bridge`, the Dart consumers' own `version:` lines, and `site/package.json`, which `versions` skips on purpose.
      - Text, which the cut's text step updates by hand: the READMEs of `libraries/{ada,cobol,csharp,go,jvm,objective-c}`, `libraries/dart/CHANGELOG.md`, and `databases/postgresql/NOTES.md`. Planning pages and ADR 0112 record history and stay.
- Keeps: every guard and rule this ticket does not name.
  - Every other `resolve` guard: the exact two arguments, the dispatch SHA check, release mode only from a `v*` tag, `versions --tag` in release mode, and the outputs' three lines.
  - Every other `policy.py` rule, including `publish = false` on every other workspace member and every binding crate.
  - `versions`' all-or-nothing write, its refusals, and its skipped files.
  - Each changed test's assertions, apart from the version it reads.
- Changes: one workflow rule, two scripts' version handling, and the tests that pin 0.0.1.
  - `sdlc/scripts/release-workflow`: rehearse accepts `refs/heads/main` or a ref matching `^refs/heads/release/[0-9]+\.[0-9]+$`. The refusal reads `rehearse must run from main or a release/X.Y branch, got REF`.
  - `sdlc/scripts/release-archive-self-test.py`, which `workflows --self-test` runs: rehearse succeeds from `refs/heads/release/0.1` with the same three outputs, and is refused from `refs/heads/feature`, `refs/heads/release/next`, `refs/heads/release/0.1/fix` and `refs/tags/v0.1.0`. Release mode is refused from `refs/heads/release/0.1`.
  - `sdlc/scripts/versions`:
    - New places: Go's three header macros in `thinkthen.go`; the tool string in `thinkthen_test.go`; the package reference in the two C# test projects; the five path-package entries in the four Dart locks.
    - The publish holds. At 0.0.1 each hold must be present; at any other version each must be gone. A wrong hold prints one line naming the file. `--set` to a version other than 0.0.1 drops both holds in the same all-or-nothing write.
    - `--set` never writes the holds back, so `--set 0.0.1` on a released tree fails its own check. No step sets 0.0.1 again.
    - The self-test's Zig plants and new rows read the current version, so they hold at any version. New rows: a hold kept past 0.0.1, a hold missing at 0.0.1, `--set` dropping both holds, and a Dart lock that differs.
  - `sdlc/scripts/policy.py`: the manifest rules read the version from `crates/thinkthen/Cargo.toml`. The `publish = false` member rule skips `thinkthen`, whose hold `versions` now checks.
  - Rust tests: the five backend tests and `golden.rs` build the tool string from `env!("CARGO_PKG_VERSION")`. `recognize-detailed.json` gains a `$VERSION` placeholder that `stores.rs` fills. `sdlc/ratchet.json` takes the re-measured total.
  - C# and JVM Python tests and `libraries/csharp/check.sh` read the version from their own manifest. `package_check.py` builds the header macros and its plant from that version.
  - `release-managed-pair-self-test.py` reads the version from `crates/thinkthen/Cargo.toml` and builds its fixture from it.
  - `sdlc/planning/release-process.md`: section 4's rehearsal line names the release branch. Section 5 names the branch `release/0.1`, the X.Y form, and becomes the cut checklist, in this order, each step naming its branch:
    1. The release candidate conditions of ADR 0116 item 3 hold on main.
    2. On a ticket branch from main, the agent runs `sdlc/scripts/versions --set 0.1.0` and writes the text: `CHANGELOG.md`'s date, the README "Install" section, the binding READMEs and Dart CHANGELOG, and the site's held lines and Rust example with its proof re-run. The coordinator lands that commit on main.
    3. The coordinator runs the checkpoint on that main commit, and release QA runs its round on it.
    4. The coordinator tags `rc/0.1.0-rc.1` and cuts `release/0.1` from that commit.
    5. Ian dispatches `gh workflow run release.yml --ref release/0.1 -f mode=rehearse`. It passes.
    6. On Ian's go, Ian tags `v0.1.0` on the head of `release/0.1` and dispatches release mode from the tag, as ticket 0128 Phase 4 continues.
    After the cut, main carries 0.1.0 and takes 0.2 work. The 0.2 cut sets the next version. Fixes follow ADR 0116 item 5.
  - `sdlc/planning/adr/0116-release-branches-cut-at-the-release-candidate.md`: item 7 and the Context sentence, as Outcome 1 says.
  - `sdlc/tickets/0128-release-and-install.md`: the status line drops the hold on the Rust test edits, and Phase 4 step 3 says `versions --set 0.1.0` drops the publish holds and links the checklist.
- Proof: the focused checks and a second dry run.
  - `python3 sdlc/scripts/versions --self-test` and `versions`; `python3 sdlc/scripts/workflows --self-test` and `workflows`; `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`; `sdlc/scripts/lint` once the load allows.
  - `python3 -m py_compile` on the changed C# and JVM Python tests, whose toolchains this host may lack.
  - A second dry run on a scratch branch, deleted afterwards: `versions --set 0.1.0` alone gives a tree where `versions`, `versions --self-test`, `workflows --self-test`, `policy.py` and the grep above leave only the copies listed as not failing. The focused tests that read the version run at 0.1.0: the six Rust test files, `release-managed-pair-self-test.py`, and the Go `go vet` compile of `thinkthen.go` where the toolchain is present. The ticket records the result.
- Defers: three gaps stay.
  - The full ladder and every surface at 0.1.0. The coordinator's next checkpoint or the cut's own checkpoint runs them. The C#, JVM, Go and Dart surface checks need their toolchains and the coordinator's checkpoint.
  - The cut's text: README, CHANGELOG, binding READMEs, the Dart CHANGELOG and the site lines. Those are words for the cut commit.
  - Dispatching any workflow run.

## What the build taught us

Added before landing.
