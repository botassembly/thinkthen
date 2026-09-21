---
flow: build
priority: 48
opens: crates/thinkthen specification sdlc/planning/adr sdlc/planning sdlc/ratchet.json
---

# 0039: Coalesce cache misses by digest

Status: landed

## Outcome

Concurrent `--cache` runs for one request send that request once when the lock owner installs a complete entry. Every waiter then reads that entry and succeeds as a replay. If the owner fails without an entry, the next waiter may send. A stopped process releases its lock through the operating system, with no owner file, timeout, recovery command, or process scan.

## Current facts

The cache reads before it sends and installs the first complete entry atomically. Two workers or processes can both miss before either installs that entry. They then buy the same request twice, and different replies make one successful paid call fail with a recording conflict. The current two-process test pins that behavior and observes two requests. The engine plan requires equal request digests to be asked once and places this work before `find`.

## Contract

1. Lock only cache misses. `--record` without replay keeps its existing immutable-first-write and conflict behavior. `--replay` opens no lock file, needs no key, and sends no request.
2. A cache caller first reads the requested entry. On a miss it takes an exclusive operating-system file lock for that digest, then reads the entry again. An entry found on the second read is returned as a replay. A remaining miss may read the key, send, decode, and install the entry while holding the lock.
3. Put the lock implementation in an engine-ready module. It accepts paths and returns `io::Result`; it knows no clap type, standard stream, `Failure`, or exit code. The command edge maps its error under the existing local recording failure.
4. Use the standard library's blocking file lock and add no dependency. Lock files live under the cache folder's private `.locks` directory and contain no data. The directory is mode `0700` and a lock file created by the tool is mode `0600` on Unix. Keep lock files after use. Removing a lock path can split later callers across two file identities.
5. A lock is held by an open file alone. Normal return, backend failure, decode failure, recording failure, process death, and signal death close the file and release the lock. No process identifier, start time, pending state, stale test, timeout, or manual recovery enters.
6. Keep the existing entry format, digest, request bytes, output order, retry rule, secrecy rule, and first-complete-response rule. A waiter reports `replayed: true` under `--details` and carries the recorded usage, as every replay does today.
7. Amend decided ADR 0020 in place. Preserve its record-only conflict rule and record the later cache coalescing decision. Update `recording.md`, `records.md`, both active plans, and the former two-request race test. Planning claims say equal requests are sent once when they share a cache, and “one record remains one request” becomes one logical judgment per record. Mark this ticket landed in its record. Put `find` next.
8. Document the unavoidable interruption window. If a process dies after the backend accepts its request and before the cache entry is installed, a waiter sends again because no durable answer exists. Closing that window would require pending state and recovery, outside this cache contract.

## Acceptance

- A table-driven same-process test sends 16 identical records under `--jobs 1`, `4`, and `32` into a fresh cache and counts exactly one listener request per run. Every record answers in input order. Detailed output identifies one live answer and the rest as replays. An `annotate` case proves its grouped path shares the same lock behavior.
- A deterministic two-process test starts both commands before the first answer can be installed. The waiter has no key. Both succeed, their values agree, exactly one request reaches the listener, and the waiter and a later run replay the entry without a key.
- A process-death test stops the process that holds the digest lock before it can install an entry. A second process then acquires the lock, sends once, writes the entry, and succeeds without a recovery step. The test always reaps its children and releases its listener.
- Backend, decode, and recording failure cases each release the lock. The first caller fails; one waiter sends and succeeds; exactly two requests occur; no recovery step is needed. Two different digests can reach the listener together, proving the lock does not serialize a whole folder.
- A lock-setup failure with no key returns the existing safe local recording failure and sends zero requests. This pins successful lock acquisition before key reading and sending.
- Focused tests prove replay-only creates no `.locks` directory, record-only preserves the existing conflict rule, lock names contain only the digest, and Unix modes are private.
- Existing cache resume, recording conflict, closed-pipe, retry, secrecy, annotate, tag, CSV/TSV input, request-byte, digest, and replay tests stay green. All committed recordings replay.
- The four repository rungs and `git diff --check` pass. No paid call is needed.

## Excluded and following order

Excluded: cache eviction, deleting persistent lock files, a wait timeout, a recovery command, closing the accepted-before-install interruption window, cross-folder coalescing, record-only coalescing, in-memory result caching, counters, the one-crate move, and a public engine interface. A repeatedly failing stream of unique digests can leave harmless empty lock files. Next is `find`, followed by page 16 and the transforms.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: none
- Final level: 3
- Reasons: the implementation is small, but its contract spans processes, paid calls, process death, and immutable recordings. Deterministic concurrency tests must prove the second cache read and automatic release.
- Selected model: `gpt-5.6-sol` with medium reasoning.

## Review

- Design review: accepted after one remediation. The first review required a qualified one-send promise, exact backend/decode/recording failure outcomes, a keyless waiter, and proof that lock setup precedes key reading and sending. The revised ticket adds each point, and the same reviewer accepted it.
- Implementation: complete and ready for independent code review. The first rewritten process race failed because the keyless waiter finished before the owner installed an entry. The implementation adds a neutral lock module, takes the lock only for a cache miss, checks replay again under it, and keeps the guard through decode and recording. Focused compiled-binary tests cover jobs 1, 4, and 32; annotate; two processes; distinct digests; backend, decode, recording, and process-death release; lock setup before the key; private empty lock files; and replay-only behavior.
- Code review: accepted after two remediations. The first pass did not pin preservation of a pre-existing cache-folder mode, made the setup failure at the cache path instead of `.locks`, inferred lock waiting from a 50 ms silence, and could leave a child alive when the killed-owner test unwound. The first remediation fixed those points. The second review found that the process proof compared only replay flags, omitted a later keyless replay, and installed its child guard after a fallible pipe write. The second remediation compares all detailed output after normalizing only `meta.replayed`, proves a third keyless replay adds no request, and wraps the child immediately after spawn. The same reviewer accepted the final tree with no remaining finding.
