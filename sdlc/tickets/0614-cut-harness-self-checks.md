# 0614: Delete workflow-text assertions and planted lint self-tests

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision ba9fd2c, accept

Reviews: revision 5262ed6, accept

## Outcome

- The workflow checker keeps only rules that protect a public release: pinned action SHAs, no unpinned installs, no expanded untrusted inputs, and publishing only through the approved dispatch. The step-by-step text assertions on `release.yml` and their self-tests are deleted.
- The planted self-checks in `lint` are deleted: the planted `rm`, the planted test run, the fake git dependency for `cargo deny`, and the planted environment names. The real checks they guarded stay.
- Library gates that grep their own scripts for banned commands, such as `libraries/ruby/check.sh:40-58`, drop that step. Shared lint keeps any rule that still guards something real.
- Self-test scripts that only cover deleted rules are deleted.
- These checks must stay: the archive clean-tree guard (bfdc8d1ce) and the retention of failed parity logs (a334b3bf2).

### Proof

- On the M5 through pm job, `sdlc/scripts/lint` and `sdlc/scripts/test` pass.
- Done once by hand and not committed: adding an unpinned action to a workflow fails `lint`, and reordering two steps in `release.yml` without changing what they do passes `lint`.
- The archive clean-tree guard and the failed-parity log retention still run in `test`.

## Evidence

A large share of the harness tests the harness itself, not the product. `sdlc/scripts/workflows` is 1,403 lines. Its `release()` rules, about lines 173 to 484, assert the text of `release.yml` step by step, so every workflow edit also needs a matching checker edit. The file has 81 commits since 2026-09-15. `sdlc/scripts/lint` plants a forbidden `rm` and a forbidden test run to check its own checkers (around its first dozen lines). It builds a fake git dependency to check that `cargo deny` refuses it (its last block). It also runs an environment-name planting check. `sdlc/scripts` and `conformance` together hold about 70,000 lines, against about 116,000 lines in `crates/thinkthen/src`.

## Progress

- 2026-10-18 started
