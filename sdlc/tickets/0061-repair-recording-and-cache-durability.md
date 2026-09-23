---
flow: build
priority: 38
opens: crates/thinkthen specification sdlc/issues sdlc/ratchet.json sdlc/scripts/policy.py sdlc/planning/rust-standards.md Cargo.toml Cargo.lock
---

# 0061: Repair recording and cache durability

Status: landed

## Outcome

A recording writer proves it can create the private entry before it sends a request, installs only a complete synced entry, repairs a damaged entry after one successful answer, removes completed cache locks, and attempts temporary cleanup on every returned path while reporting cleanup failure. Replay alone stays read-only and refuses damage locally. The default cache remains off.

## Current facts and decisions

The engine currently opens the recording folder only after a backend answer succeeds. A read-only folder therefore costs one request before exit 5. A file-size limit can terminate the process with `SIGXFSZ` and leave a partial temporary file. A damaged entry makes record-only retries pay and fail forever, while cache mode refuses it without a request and cannot repair it. Every successful cache entry also leaves a permanent empty lock file.

The queue's newer ruling wins over the older issue wording: a damaged entry is never trusted, and the next successful answer through a writing mode replaces it. `--replay` alone still refuses a damaged entry before key lookup or network access. A valid first complete answer remains immutable. The same answer succeeds without rewriting it, and a different answer keeps the existing conflict refusal.

A writer obtains a prepared write permit before key lookup or network access. The permit creates the private folder and opens a private temporary file in the same folder. A digest lock is needed only while the final entry is missing or damaged. Record-only work that finds a valid entry pre-opens its temporary file without a digest lock, sends live, and then compares with the valid entry. Cache work replays a valid entry without a lock or temporary file. A returned error or failed request drops the permit and attempts to remove its temporary file.

| Mode and entry | Before live work | Result |
| --- | --- | --- |
| No recording option | No recording I/O | Send normally |
| Replay only | Read the final entry; no lock or temporary file | Replay a valid entry; refuse a miss or damage locally |
| Record only, valid entry | Pre-open a private temporary file; no digest lock | Send live, then accept an identical answer or refuse a conflict |
| Record only, missing or damaged entry | Lock, recheck, then pre-open a temporary file while still missing or damaged | Send live and fill or repair. If the recheck finds a valid entry, release the old lock and follow the valid-entry row |
| Cache, valid entry | No lock or temporary file | Replay it |
| Cache, missing or damaged entry | Lock, recheck, then pre-open a temporary file while still missing or damaged | Send once and fill or repair. If the recheck finds a valid entry, replay it without a temporary file |

After a reply decodes, the writer serializes the complete version-one envelope, writes and syncs the temporary file, then installs it atomically. A missing entry keeps the existing no-replacement rule. A damaged entry is replaced atomically under the digest lock. The recording directory is synced before success. A write, sync, link, rename, directory-sync, cleanup, or lock-removal failure is a local failure at exit 5. Every returned path attempts temporary cleanup. Cleanup success leaves no temporary file; cleanup failure uses the same safe failure and makes no stronger deletion claim. The common permission and path failures occur during preflight and therefore send nothing. A later disk-full or sync failure can still discard a paid answer, so this ticket mitigates that issue and does not claim to eliminate every interruption window.

On Unix, the process installs one safe `signal-hook` flag handler for `SIGXFSZ`, once, before the first recording preflight. Setup failure is the fixed local recording failure before key lookup or network access. The handler prevents the default process death, so the failed write becomes the same structured local error and cleanup path as another storage failure. The workspace keeps `unsafe_code = "forbid"`. The dependency and policy pages record why the standard library cannot install the handler safely. This ticket adds no Ctrl-C behavior.

A digest lock is removed while its owner still holds it and only after the owner proves that a valid final entry exists. This applies after a fill, repair, identical-answer success, conflict, and a directory-sync failure that leaves a valid entry. Existing waiters may still hold the unlinked inode; each rechecks, sees the valid entry, and releases the old lock before any record-only live call. Every new caller sees the valid entry before deciding that no lock is needed. A failure with no valid entry leaves the empty lock file, because unlinking it while waiters exist could allow two live owners. Successful removal syncs the `.locks` directory as well as the recording directory. Lock-removal or lock-directory-sync failure uses the fixed local error. Lock files left by older versions remain legacy debris until the later prune command; removing them without knowing whether an older process waits on their inode is unsafe.

Every storage-phase failure prints exactly `thinkthen: the recording folder could not be read or written; check its permissions and free space` and exits 5. The engine failure and its `Debug` form hold no path, entry bytes, evidence, credential, or operating-system error text.

Ian can overturn the sync durability, damaged-entry repair rule, missing-or-damaged-entry serialization, signal dependency, and failed-lock retention.

## Scope

Replace the recorder's separate replay, lock, and post-answer write calls with one engine-owned prepared recording operation that spans lookup, preflight, live work, and atomic installation. Keep exact request construction and response decoding. Add deterministic unit, process, and loopback tests for corruption, concurrency, permission failure, storage failure, cleanup, modes, secrecy, and unchanged recordings. Update the recording specification and relevant issue statuses.

Excluded: a default cache, XDG or configuration discovery, cache size caps or pruning, `status`, address-change warnings, `meta.replayed` naming, retry timing, send counters, token ledgers, Ctrl-C, public Rust APIs, and paid calls.

## Acceptance

- `--record` and a cache miss or damaged cache create and open their private temporary entry before key lookup or any request. An unwritable folder exits 5 with the exact safe recording-folder line, sends zero requests, and leaves no new file when cleanup succeeds. Replay-only performs no write preflight, and a cache hit replays without creating a temporary file.
- Replay-only missing and damaged entries fail locally with their established safe diagnostics, no key read, and zero requests. Cache and record modes never trust a damaged entry. One successful local-server answer replaces it atomically, and the next replay succeeds with zero requests.
- A valid existing entry remains immutable. An identical recorded answer succeeds. A different answer refuses with the established conflict line and leaves the first entry byte-for-byte unchanged. Concurrent processes over the same digest never install a partial or mixed entry.
- Every successful new or repaired entry is closed and synced before atomic installation, and the folder is synced before success. Injected write, file-sync, install, final-validation-read, directory-sync, cleanup, lock-removal, and lock-directory-sync failures exit 5, preserve any valid old entry, attempt temporary cleanup, and print only the exact fixed line. Tests distinguish successful cleanup from an injected cleanup failure.
- Under a child process with a file-size limit, the command exits 5 through its fixed tool-owned diagnostic instead of dying from `SIGXFSZ`; the final entry and temporary file are absent. Signal setup uses no unsafe code and changes no Ctrl-C behavior.
- After a successful cache fill or repair, its new digest lock file is absent and both affected directories were synced. A real two-process owner/waiter test handshakes after the waiter has opened and blocked on the owner's original lock inode, then lets the owner install and unlink it; the run still sends one request and both callers receive the same answer. Concurrent record-only calls against a valid entry each send once and never split missing-entry serialization. A conflict or later sync failure removes the lock when a valid final entry remains. A failed owner with no valid entry retains one empty private lock file and never a process identity or other content. Legacy lock files remain documented for the later prune command.
- Folder and entry modes remain `0700` and `0600` on Unix. Request bytes, digests, response decoding, result values, output order, exit meanings, retries, and existing committed recordings remain unchanged. No dependency beyond the reviewed signal mechanism is added.
- Tests fail first on the reproduced faults and then pass. They cover damaged cache and record repair, replay-only refusal, every successful temporary cleanup path, injected cleanup and lock-unlink failure, all eight command families, and every new failure's diagnostic and `Debug` secrecy. The recording specification replaces its stale damage, record-only lock, permanent-lock, syncing, and temporary-file rules. The four repository gates and `git diff --check` pass without a key or outside network access.

## Dependencies

Tickets 0055 and 0060, ADR 0017, the A4 build-queue entry, and the issues `a-corrupt-recording-entry-bills-every-retry-and-never-repairs`, `a-write-failure-after-a-good-exchange-discards-the-paid-answer`, `a-cache-write-hitting-a-size-limit-kills-the-process`, and `lock-files-stay-after-their-entries-land`.

## Complexity

- Contract: 1
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 2
- Total: 8
- Minimum level floor: concurrency
- Final level: 3
- Reasons: public recording behavior stays stable, but the repair changes the lifecycle shared by multiple processes and must prove atomic replacement, durable writes, signal handling, secrecy, and owner/waiter safety.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work adds default storage, public result fields, a new command, retry behavior, counters, or general cancellation.

## Review

Independent design review rejected the first draft because unlinking a lock used by every record-only writer could split serialization across two inodes, the mode rules and preflight sentence conflicted, cleanup and issue-closing claims exceeded what a filesystem can promise, lock cleanup was incomplete, the diagnostic was not exact, signal policy files were outside the declared scope, proof omitted concurrency and secrecy cases, and the score used the wrong scale. The repair limits digest locks to missing or damaged entries, adds the complete mode table and valid-entry invariant, narrows cleanup and mitigation claims, fixes the diagnostic and signal setup, expands proof, and uses the accepted score and model route.

Re-review requested four exact repairs: honest cleanup wording in the outcome, the narrower name for serialization, issue files in scope, and an owner/waiter handshake that proves the waiter opened the original inode. The repair makes each change. The same reviewer accepted the final design and its level-3 Sol Medium route.

## Implementation evidence

The engine now prepares one recording operation before key lookup or transport. That operation owns any missing-or-damaged digest lock and an already-open private temporary file. A decoded answer is written, synced, closed, and installed atomically. A damaged entry is replaced under the lock. A valid entry remains immutable. A valid final entry allows the owner to unlink its lock while existing waiters remain on the original inode. Every returned live-work failure attempts temporary cleanup.

`signal-hook` 0.3.18 supplies the safe process-once Unix `SIGXFSZ` handler. The standard library has no safe signal-installation interface. The policy table and Rust standards name the dependency and reason.

The focused proof observes the real waiter through Linux `/proc`, matches its open descriptor inode to the owner's linked lock, and observes the kernel's lock-wait state before release. The owner then installs and unlinks. Release produces one request and the same answer for both processes. Other proof covers two concurrent record-only processes, damaged record and cache repair, replay-only refusal, cache-hit and record-only modes, file and folder modes, all eight commands, `SIGXFSZ`, and injected write, file-sync, install, final-validation-read, directory-sync, cleanup, lock-removal, and lock-directory-sync failures. Every applicable injected failure snapshots and preserves a valid old entry byte for byte. A divergent duplicate answer also leaves the original bytes exactly unchanged. The fixed storage failure and its `Debug` form carry no path, entry bytes, evidence, key, or operating-system text.

With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, all four repository gates pass. The test rung passes 188 library tests, 239 backend tests, and the command, demo, transform, and live-script self-tests. The spec rung passes 27 executable page checks, 7 transform checks, every committed replay, and 19 green demos. The exact source ceiling is 29,242 non-blank Rust lines. No paid or outside request ran.

Independent code review rejected the first implementation because a failed final-entry read was swallowed, the process race did not prove the waiter opened the original inode, and injected faults did not snapshot a valid old entry. The repair propagates final-read failure, observes the waiter's descriptor inode and kernel lock-wait state in a real process test, and preserves exact old bytes across every applicable fault and a divergent conflict. The same reviewer accepted the final implementation with no remaining finding.
