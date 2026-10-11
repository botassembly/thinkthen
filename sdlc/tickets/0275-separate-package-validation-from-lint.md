---
flow: build
priority: 275
opens: sdlc/issues/2026-09-29-lint-runs-the-full-package-validation.md
---

# 0275: Separate package validation from routine lint

Status: COMPLETE.

Opened as: 2026-10-11. Independent High design review accepted `ecf93511b`; fresh High code review accepted `e20119731`. The coordinator ran the integrated routine lint checkpoint; the existing private-name scan stopped it before later checks. That separate source-hygiene failure is filed, not a package-routing failure. Current package and Actions execution remain unqualified.

## Outcome and scope

Keep `sdlc/scripts/lint` as the bounded policy, formatting, dependency, documentation and static-guard rung. Remove only its 4 MiB stale-crate plant and `sh sdlc/scripts/package` call. Keep `release-smoke-source-packages-test` in lint: its bounded source-package refusal fixture is separate from the full Cargo package campaign. Do not move or delete other lint checks, functional/stress tests, policy tables or package guarantees.

Keep `sdlc/scripts/package` as the single complete, explicit local package checkpoint. Move the exact stale-crate plant immediately before its existing `rm -f -- target/package/thinkthen-*.crate`, so a package invocation still exercises the 0104 trailing-bytes regression. Its library-only build and dependency graph/target checks, no-default all-target tests, internal doctests, two **distinct** private-export probes (default and no-default feature profiles), `cargo package`, package-member and catalog checks, isolated unpacked library and CLI builds, transform byte comparisons, and release panic-mode checks remain in their current order. The isolated unpacked build must stay fresh for that trust claim; ordinary lint no longer incurs it.

Route the manually dispatched release's `crate` job in `.github/workflows/release.yml` through this same script. After checkout of `needs.resolve.outputs.sha`, use three separate, unconditional steps with exact run commands, in order: `rustup toolchain install 1.95.0 --profile minimal`, `cargo fetch --locked` for the script's offline Cargo commands, and `sh sdlc/scripts/package`. Only then upload `target/package/*.crate`. Replace its lone `cargo package --locked --package thinkthen`; do not package twice. The package step must propagate a nonzero exit and stop artifact upload: no `if`, `continue-on-error`, shell error suppression or command suffix such as `|| true` on this required route. The existing `crate -> smoke -> draft -> publication` needs chain then makes every package check a release prerequisite. Keep manual dispatch, resolved-SHA checkout and publication guards. This is a required release checkpoint, not a new routine gate or permission to publish.

## Small routing proof and checks

Extend the existing `sdlc/scripts/workflows` checker and `--self-test`, rather than adding a wrapper. Inspect the parsed `crate` job and require resolved-SHA checkout, then separate steps with exactly the three run commands above, then the `crate-package` artifact upload. Require these steps and the job to be unconditional; reject step `if` or `continue-on-error`, a conditional job, and any command text beyond the exact run command. This checks normal failure propagation, including rejection of `sh sdlc/scripts/package || true`, not merely string presence and order. Its compact `RELEASE` fixture currently abbreviates `crate` to a no-step job, so give it only the minimal valid steps. Plant a missing/replaced package call, reversed upload order, and bounded bypasses using `|| true`, `continue-on-error: true`, or `if` on the package step; avoid a new parser or combinatorial matrix. Add one bounded text check of the actual lint script that rejects a direct package invocation and the old stale-archive plant; plant either line in the check's self-test. The existing workflow graph check retains `crate` as a prerequisite of smoke and draft. Static routing evidence does not prove that Cargo commands pass.

Implementation claim after design acceptance: `sdlc/scripts/lint`, `sdlc/scripts/package`, `.github/workflows/release.yml`, `sdlc/scripts/workflows`, `sdlc/scripts/README.md`, `sdlc/planning/rust-standards.md`, this ticket and one build record. `sdlc/scripts/scratch.sh` is owned by another cleanup and stays read-only. No product source, dependencies, test selection, SQL/DataFrame, site or M5 changes. Coordinate workflow-file ownership before implementation if another release builder has it.

For the focused builder check, run shell/Python syntax and `workflows --self-test` plus its normal static check, the relevant package-route and lint-route plants, offline policy, pages/tickets and diff checks. Then the coordinator names one integrated **routine lint** checkpoint after the cleanup merge. Run `sdlc/scripts/package` as a separately named local package or release checkpoint when source/toolchain inputs require fresh package evidence; do not run it as part of this preparation or by default for this routing edit. The 0104 prior successful package proof can be retained only for matching source, toolchain, lockfile and package inputs. Static routing evidence alone does not qualify the current package or actual Actions runner.

## Evidence

- Starts from: [open lint/package issue](../issues/closed/2026-09-29-lint-runs-the-full-package-validation.md), current `lint` and `package` at main `1b1f09d70`, [0104 stale-crate proof](../records/0104-quick-fix-fresh-package-before-check.md), and the [preparation trace](../records/0275-lint-package-preparation.md).
- Keeps: All present package trust checks, the separate default/no-default private probes, stale-archive refusal, bounded lint guards, offline execution, and the release graph's manual/resolved-source/publication controls.
- Changes: Removes the full package campaign from routine lint, relocates its adversarial stale crate into the explicit package script, and makes that script a required release `crate` step before upload.
- Proof: Static fixture plants for missing, reordered and bypassed package steps, exact unconditional command/order and workflow graph checks without compilation; focused syntax/policy checks; one named integrated routine lint checkpoint after cleanup merge; separate package execution only at an explicit matching-input checkpoint.
- Defers: Current-source package execution, actual Actions runner qualification, release publication, all SQL/DataFrame held work, and any broader test-tier or stress audit.

## What the build taught us

- Preparation found that the release `crate` job called bare `cargo package`, so removing the lint call alone would strand the full trust proof. Fresh design review found that string/order checks alone miss a bypassed package failure. The implementation now uses exact separate commands and checks unconditional execution, failure propagation and artifact order in the existing workflow checker. Its compact fixture needed the actual five-step crate route; the prior generic checkout plant also needed to change only the build checkout so it kept one intended failure. The [build record](../records/0275-lint-package-build.md) gives the focused results. Root still owns the integrated routine lint run; current-source package execution and an actual Actions runner remain unqualified.
