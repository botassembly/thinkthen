# 0035: Replace the live supervisor with one ledger

Ticket 0035 replaces three production scripts with one 309-line launcher. One shared lock protects a versioned append-only ledger. Initialization syncs a permanent marker, then copies the audited 476,000,000-token limit and 429,118-token charge once. Every job appends and syncs its full reservation before direct execution. Status reads the same ledger. A missing ledger after initialization remains disabled.

The gate, socket, pending and unresolved states, recovery, process identities, worktree scan, migration command, test switches, usage scan, and completion line are gone. The Linux proof runs from rung 2. CI no longer fetches full history, and install no longer checks supervisor-only kernel features.

Red first: missing setup returned the old generic failure instead of the `--init` instruction. The green suite covers initialization, status, exact fit, concurrent reservations, malformed rows, storage failure, exit and signal status, arguments, working directory, credential confinement, killed jobs, and migration states.

Code review reproduced two faults. Deleting a charged ledger allowed reinitialization from the stale checkpoint, and deleting the ledger's final newline still passed validation. Both new regression tests failed before the marker and terminator checks and pass after them.

Production fell from 996 raw and 869 nonblank lines to 309 raw and 267 nonblank lines. Proof fell from 1,634 raw and 1,560 nonblank Rust lines to 228 raw and 206 nonblank shell lines. The deletions are 687 raw and 602 nonblank production lines and 1,406 raw and 1,354 nonblank proof lines. The Rust ratchet fell from 17,827 to 16,267.

The second review reproduced both repairs and accepted the implementation. The four local rungs passed with the key and base address unset. Commit `6343e7a` landed on `main`, and GitHub run `35523772125` passed all four rungs.

The coordinator acquired the permanent lock and verified active idle version-one state against the checkpoint. The coordinator renamed it to `state-v1-retired.json`, synced the authority directory, and initialized the new ledger. Status then reported 476,000,000 allowed, 429,118 charged, and 475,570,882 remaining. Migration ran no job and made no paid call.
