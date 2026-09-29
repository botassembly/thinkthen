---
flow: build
priority: 275
opens: sdlc/issues/2026-09-29-lint-runs-the-full-package-validation.md
---

# 0275: Separate package validation from routine lint

Status: design candidate for fresh independent review. Source baseline: main `1b1f09d70`. The coordinator may approve this gate routing within Ian's accepted light routine-check outcome.

## Outcome and scope

Keep `sdlc/scripts/lint` as the bounded policy, formatting, dependency, documentation and static-guard rung. Remove only its 4 MiB stale-crate plant and `sh sdlc/scripts/package` call. Keep `release-smoke-source-packages-test` in lint: its bounded source-package refusal fixture is separate from the full Cargo package campaign. Do not move or delete other lint checks, functional/stress tests, policy tables or package guarantees.

Keep `sdlc/scripts/package` as the single complete, explicit local package checkpoint. Move the exact stale-crate plant immediately before its existing `rm -f -- target/package/thinkthen-*.crate`, so a package invocation still exercises the 0104 trailing-bytes regression. Its library-only build and dependency graph/target checks, no-default all-target tests, internal doctests, two **distinct** private-export probes (default and no-default feature profiles), `cargo package`, package-member and catalog checks, isolated unpacked library and CLI builds, transform byte comparisons, and release panic-mode checks remain in their current order. The isolated unpacked build must stay fresh for that trust claim; ordinary lint no longer incurs it.

Route the manually dispatched release's `crate` job in `.github/workflows/release.yml` through this same script, after checkout of `needs.resolve.outputs.sha`, pinned toolchain setup and `cargo fetch --locked` for the script's offline Cargo commands, and before upload of `target/package/*.crate`. Replace its lone `cargo package --locked --package thinkthen`; do not package twice. The existing `crate -> smoke -> draft -> publication` needs chain then makes every package check a release prerequisite. A failed script must stop the artifact upload. Keep manual dispatch, resolved-SHA checkout and publication guards. This is a required release checkpoint, not a new routine gate or permission to publish.

## Small routing proof and checks

Extend the existing `sdlc/scripts/workflows` checker and `--self-test`, rather than adding a wrapper. Assert the release `crate` job's ordered route: resolved-SHA checkout, dependency fetch, `sdlc/scripts/package`, then `crate-package` artifact upload; reject a removed/replaced package call or upload placed before it. Its compact `RELEASE` fixture currently abbreviates `crate` to a no-step job, so give that fixture the minimal valid steps and plant one missing/replaced package call and one reversed order. Add one bounded text check of the actual lint script that rejects a direct package invocation and the old stale-archive plant; plant either line in the check's self-test. This checks command routing without compiling, and the existing workflow graph check retains `crate` as a prerequisite of smoke and draft. Avoid pretending text routing proves that Cargo commands pass.

Implementation claim after design acceptance: `sdlc/scripts/lint`, `sdlc/scripts/package`, `.github/workflows/release.yml`, `sdlc/scripts/workflows`, `sdlc/scripts/README.md`, `sdlc/planning/rust-standards.md`, this ticket and one build record. `sdlc/scripts/scratch.sh` is owned by another cleanup and stays read-only. No product source, dependencies, test selection, SQL/DataFrame, site or M5 changes. Coordinate workflow-file ownership before implementation if another release builder has it.

For the focused builder check, run shell/Python syntax and `workflows --self-test` plus its normal static check, the relevant package-route and lint-route plants, offline policy, pages/tickets and diff checks. Then the coordinator names one integrated **routine lint** checkpoint after the cleanup merge. Run `sdlc/scripts/package` as a separately named local package or release checkpoint when source/toolchain inputs require fresh package evidence; do not run it as part of this preparation or by default for this routing edit. The 0104 prior successful package proof can be retained only for matching source, toolchain, lockfile and package inputs. Static routing evidence alone does not qualify the current package or actual Actions runner.

## Evidence

- Starts from: [open lint/package issue](../issues/2026-09-29-lint-runs-the-full-package-validation.md), current `lint` and `package` at main `1b1f09d70`, [0104 stale-crate proof](../records/0104-quick-fix-fresh-package-before-check.md), and the [preparation trace](../records/0275-lint-package-preparation.md).
- Keeps: All present package trust checks, the separate default/no-default private probes, stale-archive refusal, bounded lint guards, offline execution, and the release graph's manual/resolved-source/publication controls.
- Changes: Removes the full package campaign from routine lint, relocates its adversarial stale crate into the explicit package script, and makes that script a required release `crate` step before upload.
- Proof: Static route fixture plants and workflow graph/order check without compilation; focused syntax/policy checks; one named integrated routine lint checkpoint after cleanup merge; separate package execution only at an explicit matching-input checkpoint.
- Defers: Current-source package execution, actual Actions runner qualification, release publication, all SQL/DataFrame held work, and any broader test-tier or stress audit.

## What the build taught us

- Preparation found that the release `crate` job currently calls bare `cargo package`, so removing the lint call alone would strand the full trust proof. The accepted builder must report the implemented route, focused results and any corrected assumptions here before landing.
