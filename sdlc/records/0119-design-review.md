ACCEPT

# Design review of ticket 0119

Reviewer: a fresh read-only Opus session that did not write the ticket. This page restates its two replies.

## First pass, commit d2198507

Not accepted. Two blocking and three non-blocking findings.

1. Blocking. The CLI sat outside the mutant scope, yet its tests could be deleted. A test that catches only CLI bugs moves no core or engine mutant, so it could go with no evidence.
2. Blocking. `cargo-mutants` 27.1.0 cannot pick mutants by which test reaches them. The ticket had to name a selection by file and the comparison files.
3. Non-blocking. The test command was not pinned, and the claim about the default command was unproven. Output should go outside the repository.
4. Non-blocking. Each run sets its timeout from its own baseline, so load can move a mutant between missed and timeout. Pin one timeout and check for flaky mutants first.
5. Non-blocking. Secrecy tests check today's commands too. The ticket said they guard code not yet written.

## Second pass, commit 7d9b35c6

ACCEPT. All five findings are fixed. Two non-blocking notes followed, and the author folded both in:

1. The final-run rule said "unchanged" while the batch rule lets a mutant move between caught and timeout. The final rule now matches the batch rule.
2. A `-f` rerun can miss a lost catch in another module. The final full run finds it, and the builder now finds the batch by rerunning the moved mutant with `-F` at each accepted batch commit, newest first.

The reviewer confirmed the tool pin under `~/.cache/thinkthen-toolchains/`, the Beelink load rule, the deletion rule, the ratchet effect, the stop rule, the report metrics, the timing, and the code-review plan. `~/.cache/thinkthen-toolchains/` does not exist on this machine yet. The Beelink has 16 cores.
