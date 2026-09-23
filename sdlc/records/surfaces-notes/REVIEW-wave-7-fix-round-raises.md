# Review of the wave-7 fix-round ceiling raises

Second-agent review of the thirteen commits on `w7/integrate` that move `sdlc/surfaces-ratchet.json` after `f6b8814` and that the ratchet refused. The reviewer wrote none of them. Reviewed at `e1d5576`, on 2026-09-23. The ceiling rose from 36,038 to 36,416. `5e26852` also moves the file, but it lowers the ceiling to 36,026 and the ratchet already passes it on its body.

| Commit | Ceiling | Verdict |
| --- | --- | --- |
| `e1d55762aaa56d1f04e62d1d9e8b220258925a75` | 36,243 and 36,211 to 36,416 (merge) | ACCEPT |
| `f88e0080d45a9b1eedb4cf2661b8bd157064c40a` | 36,255 and 36,026 to 36,243 (merge) | ACCEPT |
| `9767a5464c30200e4a50269de52533c53d7e8ce7` | 36,254 and 36,039 to 36,255 (merge) | ACCEPT |
| `0bd4175afc9f0ba1aefd446376977782a359e7e7` | 36,134 and 36,158 to 36,254 (merge) | ACCEPT |
| `debfb204926c8f8571e1a1f782d2352b963d0272` | 36,038 to 36,041 | ACCEPT |
| `20131863b50ee1633adcc1773b2531fe826c2690` | 36,041 to 36,103 | ACCEPT |
| `d54bf76e757dacb36eb255d5833c5bd3e88f7e87` | 36,103 to 36,158 | ACCEPT WITH FOLLOW-UP |
| `1e5f85eee5af062e7c4d27ca95a0bdcf8755196b` | 36,038 to 36,135 | ACCEPT WITH FOLLOW-UP |
| `42a7532af9e03b88c65c77a7be68c492d9a8b748` | 36,135 to 36,149 | ACCEPT |
| `ea93201ed53534ebd93a7dd61a3a9007fc153d26` | 36,149 to 36,211 | ACCEPT |
| `29955a6dc72e37739767c35b2aeaec60b993a238` | 36,038 to 36,039 | ACCEPT |
| `ce59e9ce9d8928e702a0e445ad79aa2db32132eb` | 36,038 to 36,125 | ACCEPT WITH FOLLOW-UP |
| `dd3d781943ee2d9cedad3b8aae90bbca0a585826` | 36,125 to 36,134 | ACCEPT |

## What I checked

- I recounted every commit's tree with the ratchet's rule from `git ls-tree` and `git show` of each file: non-blank `.rs` lines under the five listed directories, build folders excluded. Each of the thirteen ceilings equals its own measured count.
- For each merge I rebuilt the automatic merge with `git merge-tree --write-tree` of its two parents and diffed it against the merge's tree. All four differ only in `sdlc/surfaces-ratchet.json`, where the merge wrote the measured count in place of the conflict markers. No merge adds code of its own.
- For each lane commit I read the diff of every counted file against its parent and checked the body's growth claim.
- I searched the five directories for code the new lines could reuse: another `/proc/self/maps` reader, another memory-range check, a timed lock wait, a child-process test helper, and another `pthread_detach` counter. I found none outside the files these commits touch.

## `debfb20`: memory-map snapshot for the Arrow reader

The verifier showed that a seccomp filter makes `process_vm_readv` fail and that `mincore` then passes a protected page. Reading `/proc/self/maps` makes no system call a sandbox answers with a kill, and it sees protection bits. The parser keeps only ranges with read access and merges touching ranges, so an extent across two mappings still checks. The file lists mappings in address order, so `partition_point` holds. A map read that races a change in another thread can at worst drop or repeat a line. Each range it keeps was readable when read, so a race can cause a false refusal but no unsafe pass. The commit deletes the iovec binding and the page-stride probe. It is net 3 lines. Nothing to cut.

## `2013186`: check only the bytes the rows read

This closes the follow-up on `54b307c` in `REVIEW-wave-7-integration-raises.md`. The code change is 11 lines: a per-buffer low and high mark for views, and a first and last offset for Utf8. The `Guarded` helper gains a page before the readable one, and `ending_with` now calls `new` and `place`, so the old guard tests and the new one share one builder. The new test makes every byte outside the rows unreadable, so a check that starts at byte 0 fails it. Nothing to cut.

## `d54bf76`: bounded metadata walk

This closes the `45c634e` follow-up. The walk checks each length word before it reads it, caps the blob at 16 MiB, and checks the whole blob before the copy. The cap bounds the pair count, so the loop cannot run 2^31 times. The walk reuses the column check's snapshot. The template copy passes `None` and never reads the map. `hand_schema` now returns EINVAL on a failure the template cannot produce. That costs four lines and keeps the unsafe copy fallible in one place.

Three things fall short.

1. `build_table` takes a fresh `/proc/self/maps` snapshot only to hand `branch` a null metadata pointer. That builds from Python values with no producer memory, yet a process that cannot read its map now fails there. Make `branch` take `Option<&Readable>` and pass `None` from `build_table`.
2. `crafted_metadata_is_refused_before_it_is_read` takes its snapshot before it maps the three guard regions. Those regions are not in the snapshot, so the first word check refuses them and the test cannot tell a per-word check from a whole-blob check. Take the snapshot after each `Guarded::ending_with`.
3. `SchemaTree::text` still reads a producer's format and name to their NUL with `CStr::from_ptr`, unchecked. A name that runs into a guard page crashes the host the way the metadata did. Check those strings against the same snapshot, a page at a time up to a cap.

Follow-up: one commit that makes these three changes, with a test for a name that ends at a guard page.

## `1e5f85e`: SIGINT install retry

The old set-once cell could not record a second host action, so a raced install either chained to a stale action or left ours out for good. An `AtomicPtr` to a boxed `sigaction` is the smallest cell that a signal handler reads in one load and a later install can replace. The deliberate leak is sound. A handler that loaded the old pointer may still be copying from it, and no lock-free way exists to know when it finished. The leak is one `sigaction` for each install that records an action, which is bounded by the number of LOAD calls. Slots in a fixed static array would avoid the leak but need their own index protocol, and I would not trade that for a few hundred bytes. The flag now goes down on the failed and ignored paths, which closes the `dab4db6` follow-up. The two new tests fail on the old code with the sentences the body quotes. `in_a_fresh_child` replaces the window test's own copy of the child launch, so three tests share it.

Two comments no longer match the code. The handler comment at `databases/duckdb/src/lib.rs` line 404 says "A OnceLock read locks nothing", and the cell is now an atomic pointer. The `HOST_ACTION` comment says a record leaks once per raced install, but a failed install also records one first.

Follow-up: correct both comments in the next DuckDB commit.

## `42a7532`: evict when the budget drops

The eviction loop moves out of `insert` into `evict`, and `set_budget_kb` calls it. The only new code is the empty-table shrink. The SQLite copy has no budget setting, so it needs no matching change. The duplicate `SavedAnswers` stays under the ticket already asked for in the integration review. Nothing to cut.

## `ea93201`: timed gate wait and interrupt first

`std::sync::Mutex` has no timed lock, so a `try_lock` loop with a 5 ms sleep is the simplest wait that can stop on a deadline or a SIGINT. The deadline is computed once and shared by the wait and the query, so the caller's limit covers both. `time_limit_message` and `stop_outcome` pull out the refusal so the wait, the query, and the unit test share it. The signal count reuses the handler's existing sequence number. Nothing to cut.

## `29955a6`: refuse a classed R deadline

One `plain` guard serves both arms. Nothing to cut.

## `ce59e9c`: join the batch threads and check the scheme

Keeping each scoped handle and joining it is the smallest fix for the detach on an exiting thread. `resume_unwind` keeps the scope's panic behavior. The scheme check copies ureq's rule: http, https, and the proxy schemes. That list could drift from ureq's, but the default resolver still refuses anything it does not know, so a drift fails closed. The detach counter follows the file's existing `pthread_create` interposer.

The two stray `;` lines at `standin/src/lib.rs` lines 327 and 362 are still there. The integration review named them on `5f36536`, and this commit edits the statements they end.

Follow-up: run `cargo fmt` in `standin/` in the next stand-in commit.

## `dd3d781`: ceiling after formatting

Its parent `66de11e` only reflows four lines of `standin/tests/thread_starts.rs` into thirteen. This commit moves the ceiling by those nine lines. Nothing to cut.

## Who can overturn this

Ian can overturn any verdict here and any follow-up.
