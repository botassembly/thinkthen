# Drive 0.2 with maximum parallelism

Date: 2026-10-13

Ian ruled that library quality is a core part of 0.2, that all of it ships in 0.2, and that the team should finish faster by running work in parallel. The libraries do not collide with each other, so they run side by side. This is how the coordinator drives the team until the 0.2 candidate passes. It works with [the standing build rulings](2026-10-13-standing-build-rulings.md). The tickets' "Remaining work" amendments say what to build. This file says how to run it. Ian can overturn it.

## The coordinator's job

The coordinator keeps every lane busy and moves work onto `ticket/0543-final`. It does not write product code, and it does not wait on one lane before starting another. Its loop:

1. Read `pm next`, the lane list and its PM pane messages.
2. Start a developer on every lane that is ready and has no developer, up to the limits below.
3. When a developer hands back, start a fresh reviewer on that lane's head.
4. When the reviewer accepts, the lane merges itself into `ticket/0543-final` (see Merging).
5. Answer questions from the standing rulings and ticket amendments. Send the PM only what neither covers, and keep the other lanes moving while it waits.
6. Record one line of progress per landed lane with `pm ticket progress`.
7. Repeat.

## Lanes

Each lane has one developer, one branch named `ticket/NNNN-<lane>`, one owned set of paths, one proof command and one handwritten budget. The lane lists are in the "Remaining work" amendments of 0562, 0564, 0568, 0569, 0573, 0574, 0575, 0576, 0577, 0578 and 0579.

Start these lanes now, all at once:

- **G1, generator** (0562 and 0576 stage 1). Restore Zig generation first. Lint stops at Zig, so every other language's stale-generation check is blind until it lands. Then do the stable names, the plain rank and filter lists, typed error facts and settings containers. Land each of these as its own small merge so language lanes can rebase onto it early.
- **G2, Rust core** (0576 stage 1, 0565 remainder).
- **G3, tooling** (0568 and 0573 tooling items).
- **MCP** (0576).
- **DuckDB, SQLite, PostgreSQL**: one lane each (0576 ruling 3).
- **Rust tests** (0564).
- **CLI fixes** (0569).
- **Backend errors** (0579).
- **Docs** (0575): the website branch merge, rebuild and replay.
- **Public texts** (0578): the replay check and the root README fix.
- **Every language lane** from the 0576 table: Python, R, TypeScript, Ruby, PHP, JVM, C#, Go, Dart and Flutter, Swift, Objective-C, Zig, C++, Ada and COBOL.

Language lanes do not wait for G1. A language lane starts with the items that do not depend on renamed generated code: examples, `plan`, docs, RBS, threads and waits, native views, privacy, ratchet scope and option order. When a G1 merge lands, every language lane rebases onto `ticket/0543-final`, regenerates its own outputs and continues. A lane never hand-edits a generated file.

## Limits

- Developers and reviewers are cheap; builds are the limit. Run as many developer lanes as are ready.
- On the Beelink, at most four heavy builds run at once. The one heavy-build lock that every `check.sh` uses holds four shared permits, not one exclusive permit, and the existing load cap still applies. G1 and G2 take the next free permit before other lanes. A developer edits and reasons while its build waits.
- A checkpoint or candidate routine run takes all four permits, so no other heavy build writes to disk while its timed tests run. Lanes keep editing and reviewing meanwhile. The checkpoint records `/proc/pressure/io` before and after. A timeout during high disk pressure reruns once under the same exclusive hold before anyone treats it as a failure.
- The load cap applies only on the Beelink. Hosted runners, such as the GitHub Mac runners, use the four-permit lock alone, because each runner is a dedicated machine whose reported load does not reflect other lanes.
- Hosted Linux runners are free for this public repo. A language lane runs its installed-package checks and its language matrix on hosted runners whenever the existing workflows can carry them. The Beelink keeps Rust builds, generator checks and candidate runs.
- Before asking for a fresh review, a developer runs its lane's full check and the reviewer's checklist itself, collects every failure in one run and fixes them all.
- After each merge, the lander deletes that lane's worktree, its `/tmp/thinkthen-target-<lane>` folder and its scratch under `/tmp/thinkthen-*`. A cleanup sweep removes any such folder untouched for a day whose branch is merged or gone.
- Each lane builds into its own `CARGO_TARGET_DIR=/tmp/thinkthen-target-<lane>` and deletes it when it lands.
- When machine load stays above the core count, start no new heavy build until it falls.
- The Swift and Objective-C lanes, and any lane that needs Windows or macOS proof, dispatch the nonpublishing GitHub workflows on their own lane branch. Use the M5 only for light, one-time checks.

## Merging

- A lane merges only after a fresh reviewer accepts its head and its own proof command passes.
- The lane's developer rebases onto the current `ticket/0543-final`, reruns its proof command, merges with `--no-ff`, and pushes. If someone else's merge lands first, it rebases again. The coordinator does not do each merge by hand.
- A conflict outside the lane's own paths means two lanes shared a file. The developer stops and asks the coordinator, who moves that file to one owner.
- After every third lane merge, the coordinator runs the checkpoint gate on `ticket/0543-final`: routine `test`, `lint` and every library build, collecting every failure. A failure goes back to the lane that caused it, and other lanes keep merging.

## Questions

- The coordinator answers from the standing rulings and the ticket amendments.
- It sends only what they leave open to the PM's pane with one "From coordinator:" line naming the ticket. It never asks in Ian's pane or its own.
- While a question waits, the asking lane works on its other items and every other lane continues.

## Records

- Each landed lane gets one `pm ticket progress` line naming its merge.
- Each wave gets at most one short record: what ran, the result and the counts.
- No raw data, no lists of what evidence does not prove, and no records written only to hold status.

## Recovery

- After a capacity error, a crash or a compaction, the coordinator rereads the build-coordinator skill, `pm daily`, `pm next` and this file. It then resumes every lane from its last pushed commit.
- A developer that dies is replaced by a fresh developer on the same branch. Its pushed work is kept.
- A developer pushes after every fix, so a crash loses at most one fix.

## Reaching the candidate

- When every lane in the tickets above has merged, the coordinator runs the 0543 candidate checklist.
- Public texts and the DuckDB listing reach Ian for approval through the PM as soon as they pass the fresh-reader test. They never wait for the candidate.
- Publishing, the registry pushes, the DuckDB PR push and the thinkthen.dev deploy wait for Ian's one go.

## Agent slots

- When the harness caps native developer agents, keep those slots on the critical path: G1 first, then G2 and the hardest language lanes. Hand self-contained grunt work to `pi-job` and verify each report before merging: R help pages, Ruby RBS, ratchet scope changes, test-folder records, README fixes and example rewrites.
