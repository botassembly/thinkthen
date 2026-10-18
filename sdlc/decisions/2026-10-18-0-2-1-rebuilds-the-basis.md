# 0.2.1 rebuilds the basis before any release

Ian, 2026-10-18: the current code will not be published. The [2026-10-18 quality review](../records/2026-10-18-0-2-quality-review.md) found the causes of the 0.2 thrashing in the code itself, so fixing them is the release. Stop 0.2 qualification. Let the coordinator land the reviewed repair in hand, then cancel the running candidate runs. Plan the fixes as 0.2.1. Every issue the review found becomes one or more tickets.

- 0.2.0 is never published. The next published release is 0.2.1. Existing 0.2.0 candidate tags stay as they are.
- Order: the test machinery, release workflows, Rust core, host crate and Rust library first. Then C, the C interface, the command line, the installers, the generator, the binding contract and conformance. Then every other language and the three database extensions.
- Open 0.2 tickets that still apply move to 0.2.1. The docs and site update that was 0.2.1 becomes 0.2.2, still a docs change after the release, with no new release of its own.
- 0.3 still plans only and starts nothing until 0.2.1 is published.
- This supersedes the [one-release ruling of 2026-10-17](2026-10-17-one-0-2-release-coverage-does-not-gate.md) on which code ships; its rule that coverage does not gate the release stays.

## Root causes

Each technical cause in the review traces to a decision or process choice.

1. **Each language computes answers itself.** The answer, probability, pick and resolution rules live in a Python template and in each language's projection code, about 15 to 18 copies, and they already disagree. The 2026-10-13 parallelism ruling let the generator lane write those rules while the core lane worked separately, and no rule said Rust computes final answers. The 2026-10-11 review found the copies, but its fix generated the copies instead of deleting them.
2. **Bindings were built while the contract changed.** The binding design changed four times, on 10-08, 10-09, 10-11 and 10-13, and the header changed 51 times during binding work. Review findings kept joining 0.2, and no commit was named as the core freeze.
3. **There is no single native path.** Each design change added a path: the C session, fixed layouts, a JSON session and four direct Rust bridges. The old paths stayed until each language migrated. Every contract change therefore lands three or four ways.
4. **Each language runs its own session loop.** The C interface offered only a non-blocking read and a per-thread last error, and nobody revisited it until a review.
5. **The test machinery tests the environment.** Fixtures copy the scripts they test, PATHs are written by hand, and a missing tool reports "not run" instead of failing. Builds moved between machines in the middle of the freeze. Release-only checks ran for the first time at each candidate, so each candidate found one new setup fault.
6. **Line ceilings.** Lint enforced 56 ratchet files. 178 commits changed only ceilings, even after 2026-10-14, when ceilings stopped gating the candidate. This goes against the binding guide's own rule to set no line target.
7. **A long-lived release branch.** The integration branch ran for five days while main changed its gate machine. The 2026-10-17 merge mostly reconciled conflicts in the test machinery.

## Rules for 0.2.1

1. **Freeze the core contract before language work.** A named commit fixes the C interface and the final result packet. No language ticket starts before that commit. Changing the contract after it needs a ruling, and every language then moves in one batch.
2. **Rust computes answers.** The final packet carries each call's answer, probability, pick and resolution. A check refuses answer rules in templates or language code.
3. **One environment.** One pinned Linux container serves the local gate, the hosted Linux job and release qualification. It runs as linux arm64 on the M5 and as linux x86_64 on Yellow; Ian approved containers on Yellow and left the use of both machines to the PM (2026-10-18). One declared tool list replaces hand-written PATHs. A missing tool fails the check. No machine moves during a freeze.
4. **No line ceilings.** Delete the ratchet files. Code review judges size.
5. **A smaller test machinery.** Fixtures run on the real checkout and never copy scripts. During a release, a new check replaces an old one.
6. **Candidates are main revisions.** Lanes land on main through the gate. The release suite runs on main at each checkpoint, so no release-only check meets a candidate first.
7. **Scope closes at the freeze.** Fixes found after the freeze go to the next release unless they block shipping.
