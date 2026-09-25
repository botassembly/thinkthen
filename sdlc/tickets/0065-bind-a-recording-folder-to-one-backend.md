---
flow: build
priority: 65
opens: crates/thinkthen specification sdlc/issues sdlc/planning sdlc/ratchet.json
---

# 0065: Bind a recording folder to one backend

Status: landed

## Outcome

A cache or recording folder cannot silently become cold because the backend interface or address changed. The first write-capable use binds the folder to one canonical backend identity before the tool reads a key or sends a request. A later mismatch fails locally with an action a stranger can follow. Exact replay from an older folder remains read-only and compatible.

## Current facts

An entry name covers the adapter, resolved URL, and request bytes. The folder records none of that identity separately. A stopped run resumed with another address therefore misses every entry and pays for the whole run again. The incident in `sdlc/issues/closed/2026-09-21-a-cache-resume-under-a-different-address-silently-re-bills-everything.md` sent twenty unintended requests this way. The same issue records the unclear replay-miss sentence.

Ticket 0061 made entry installation durable, ticket 0062 put the cache on by default, and ticket 0064 exposed sends after they happen. This ticket prevents the address mistake before a send. Ian can overturn the compatibility boundary for old folders.

## Design

Every newly bound folder has one private `.thinkthen-backend.json` file. Its closed JSON shape is:

```json
{"schema":"thinkthen.backend-folder/1","backend_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
```

`backend_sha256` is the lowercase SHA-256 of the canonical adapter name, a newline, and the canonical resolved endpoint URL already used by the recording digest. The fixed-size digest keeps the marker bounded even though accepted backend URLs have no length limit. The model stays out because one folder may intentionally hold requests for several models and the request bytes already distinguish them.

Before its first entry lookup, every default-cache, `--cache`, `--record`, and paired `--record`/`--replay` run compares this identity with the request. A mismatch exits 5 before key lookup or network access and prints exactly `thinkthen: the recording folder belongs to another backend interface or address; restore its backend settings or choose another folder`. It repeats no path, address, file bytes, or evidence.

A new or empty folder gets the identity before key lookup or network access. The tool writes and syncs a private temporary file, publishes the final name without replacing a winner, compares a concurrent winner, syncs the folder after a successful publication, and attempts to remove its temporary name on every returned path. A killed process may leave only a private dot-prefixed temporary file or one complete parseable final identity. Concurrent first users with different identities produce one winner and one local refusal; they never send under both identities.

An identity read accepts at most 256 bytes. The directory entry and opened handle must name the same regular, non-symlink file before and after the bounded read. A symlink, non-regular object, oversized file, replacement during inspection, malformed JSON, foreign schema, missing field, extra field, wrong field type, or value that is not exactly 64 lowercase hexadecimal characters is a safe local storage failure. No untrusted marker byte reaches a diagnostic.

An unmarked folder is empty when it has no digest-shaped final entry. Unknown names, dot-prefixed temporary files, and the lock directory do not make it nonempty. A digest-shaped regular file or other object makes it nonempty even when its contents are damaged. A write-capable use of an unmarked nonempty folder exits 5 before key lookup or network access and prints exactly `thinkthen: the recording folder predates backend binding; replay it read-only or choose a new folder`. The tool does not scan entries to infer an address.

`--replay` never creates or changes the identity file. An exact entry in an older unmarked folder still replays. A replay miss keeps exit 5 and prints exactly `thinkthen: the replay folder holds no entry named \`NAME\`; the entry name covers the backend interface, address, and request`, with `NAME` replaced only by the derived lowercase digest filename. A marked replay checks the identity first, so a mismatch names the real condition rather than a missing digest.

Matching folders keep their current behavior. Default cache and `--cache` replay hits and send misses. Paired record/replay does the same. Record-only still sends and applies the existing conflict rule. `--no-cache` creates and checks no identity. `status` and `cache prune` remain offline maintenance commands; prune ignores and preserves the identity file.

The core owns the closed identity value and comparison. The engine owns private atomic storage and the check before entry preparation. The command owns the two fixed messages and exit code. No request, result, entry, digest, counter, key handling, output order, or cache size rule changes.

## Proof

- A two-address loopback test fills a cache under the first address, then repeats the same command under the second. The second server counts zero requests, the key is not read, no result prints, exit 5 and the exact mismatch sentence appear, and the first entries remain byte for byte unchanged.
- New default, explicit cache, record-only, and paired record/replay folders bind before their first send. Matching reuse, mixed hit and miss, record conflicts, models sharing one folder, and normalized spellings of the same address keep their current behavior.
- Concurrent first users with matching identities both proceed under one identity. Concurrent first users with different identities leave one complete marker, send under at most its identity, and give the loser the exact mismatch sentence. Injected failures at every marker-write stage leave no partial final file. A successful return has synced the marker and its directory. Killing a process at each stage leaves either no final marker or one complete parseable marker; the proof makes no durability claim for a published name before its directory sync.
- An exact `--replay` hit from an unmarked committed recording succeeds with no key, network, or write. A miss explains that interface, address, and request form the entry name. A marked mismatch fails before lookup. A nonempty unmarked write-capable folder refuses before key or network access. Empty folders and folders containing only ignored private artifacts bind successfully.
- A malformed, foreign-schema, non-regular, symlinked, oversized, replaced-during-read, unreadable, or otherwise unusable identity is a safe local storage failure. The bounded read allocates no more than 257 bytes while detecting the limit. A canonical URL longer than 256 bytes still produces the fixed-size marker and reopens successfully. Diagnostics repeat no untrusted bytes. Default-cache privacy checks still run, and a created marker and its temporary file are mode `0600` on Unix.
- Prune preserves the marker. Status counts only recording entries. Existing replay fixtures need no migration. The recording specification, build queue, roadmap, source issue, and help where needed agree. The four offline gates and `git diff --check` pass.

## Boundaries

Excluded: changing `meta.replayed`, renaming `--replay`, changing entry contents or digests, inferring an identity from old entries, moving folders, automatically copying old entries, cache eviction, token accounting, retries, Ctrl-C, public library APIs, and paid calls.

Complexity: Contract 2, state and timing 2, reach 1, proof 2, cost of error 1; total 8. Shared durable state, concurrent first use, compatibility, and the paid-call boundary set Level 3. Selected implementation model: `gpt-5.6-sol`, medium reasoning.

## Landing review

Ian transferred completion to the architect after the original owner stopped. Fresh independent code review found missing directory syncs on matching-marker reuse and concurrent-winner paths. Sol remediated them within this accepted durability contract; the same reviewer accepted the fix. Regression tests failed before the fix and passed afterward, including read-only exclusion. The coordinator's full combined ladder passed 568 Rust tests and nineteen green how-tos at an exact 33,839-line ceiling. Hosted gate run `35748913958` passed on integrated revision `ba60f04`, which the coordinator fast-forwarded to main and pushed. The matching record distinguishes original branch evidence from final integration evidence.
