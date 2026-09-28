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

## Later listener proposal, pending fresh review

The historical mutation review above remains accepted history; Ian's later functional-gate ruling supersedes its campaign requirement. The accepted functional amendment is recorded in [the ticket](../tickets/0119-mutation-audit-of-the-engine-tests.md) and [its preflight](0119-functional-audit-preflight.md). The new [listener preflight](0119-listener-preflight.md) proposes only the scripted reply helper and two callers in `engine/width_tests.rs`. A fresh reviewer should compare the 503 status, 600 ms header, JSON body, one-response-per-connection order, after-write announcement, final owner cleanup and cache replay with the current raw helper. Its clients do not inspect the reason phrase, but that difference must be checked against the HTTP parser. The note rejects replacing synchronous quiet-socket `accept()` with an asynchronously counted zero. This listener proposal has no design acceptance yet.
