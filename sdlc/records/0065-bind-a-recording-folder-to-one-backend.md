# 0065: Bind a recording folder to one backend

Date: 2026-09-22

Status: Implementation and remediation independently accepted; integrated verification and landing in progress.

## Landing continuation

Ian transferred ownership after the original agent stopped. The original `landed` status described completed branch work: `6618694` had a passing hosted gate but was not an ancestor of main. The coordinator merged main `8336a2a` without discarding either branch's changes. The only textual conflict was the exact source ceiling; the combined pre-remediation measurement was 33,772.

Fresh independent code review found a missing directory sync on matching existing-marker and concurrent-winner paths. Sol remediated the level-3 durability invariant with observed failing tests, a shared sync helper, and deterministic existing-marker, forced-winner, and read-only exclusion cases. The same reviewer accepted the repair. Focused identity tests pass ten cases with one intentional child-harness ignore; nine backend identity tests pass. The exact source ceiling is now 33,839, including 67 remediation lines. The coordinator ran all four gates on the integrated, repaired tree with the provider key empty and Cargo offline mode. All passed: 568 Rust tests, one intentional subprocess-harness ignore, doctests, replay checks, 27 specification checks, seven transform-page checks, and nineteen green how-tos. Policy, package checks, audit, formatting, clippy, docs, and both staged/unstaged diff checks passed. Hosted verification and landing remain pending.

The original review and test evidence below describes the earlier branch, not the combined tree.

## Result

The first write-capable use of a recording folder now stores a private fixed-size identity for the canonical backend interface and address before key lookup or network access. Reusing that folder with another address exits 5 with a fixed action and sends nothing. The identity excludes the model, so several models may share a folder.

Exact read-only replay from an older unmarked folder remains compatible. A write-capable use of an older nonempty folder refuses locally and tells the user to replay it read-only or choose a new folder. Replay misses now say that the backend interface, address, and request form the entry name.

The engine writes and syncs a private temporary marker, publishes it without replacing a concurrent winner, and syncs the folder. Its bounded reader follows no symlink and rejects malformed, oversized, non-regular, unreadable, or replaced marker files without printing their bytes. Status and prune ignore the marker, and prune preserves it.

The Rust ceiling rose from 32,713 to 33,629 nonblank lines. The fixed identity value, atomic storage, six storage-stage cases, two interruption points, concurrent first-use cases, hostile-file matrix, cache-mode integration cases, and existing-test isolation account for the increase. The implementation reused the recording digest writer, directory gate, directory sync, storage-fault harness, loopback listener, cache status and prune paths, and shared test helpers. No dependency, request, result, recording entry, counter, or command option changed.

## Review and proof

Independent design review rejected three drafts. It required a bounded race-aware marker reader, one exact replay-miss sentence, honest durability wording, a fixed-size identity that accepts unbounded URL lengths, and precise digest validation. The final design scored Level 3 and was accepted for Sol Medium implementation.

Independent code review rejected the first implementation. The complete test rung exposed a secrecy test whose shared home crossed several loopback addresses. The marker writer lacked its promised interruption proof, one stale temporary name could be removed by a process that did not create it, and several accepted modes and hostile-file cases lacked focused tests.

Remediation isolated that test's homes, made temporary-name retries preserve files owned by another process, extended the existing fault harness across every marker stage, killed a real child before and after publication, and added the missing mode, concurrency, file-trust, permissions, long-address, prune, and status proofs. The same reviewer reran the full test rung and accepted the repaired implementation with no material finding.

The final install, lint, test, and specification rungs passed with no paid or outside request. The test rung passed 212 active library tests with one intentionally ignored child harness, 272 backend tests, every edge suite, doctest, transform, probe, demo self-test, and local live-fixture test. The specification rung reproduced every committed recording and passed all 19 green how-tos. The lint rung measured the exact 33,629-line ratchet. `git diff --check` passed.
