# Quick Fix qf-request-cost: drop the lock-folder sync from each cache entry

Status: landed. Ian can overturn the dropped sync.

## Evidence it starts from

The PostgreSQL surface port, ticket 0111 on `ticket/0111-port-postgresql-surface`, recorded the engine's request rate in `sdlc/records/0111-build-postgresql-surface.md` and `databases/postgresql/NOTES.md`. After main's loopback fix at `365fc938`, the backend answered in under 0.1 ms on a reused connection. The engine still spent about 12 ms on each request at throttle 1: 200 records took 2,455 ms. A 2,000-record batch at throttle 32 ran at about 1,140 requests a second, and the 20,000-row warm ran at about 670. The warm step sits at its 30 s bound.

## How it was measured

A scratch program outside the repository called `decide_many` through the public Rust API. It sent records to the conformance backend's generic arm, one new record each, so every record missed the cache. It ran with no cache, with a cache on tmpfs (`/dev/shm`), where a sync costs nothing, and with a cache on the workstation's ext4 NVMe disk. `strace -f -c` counted system calls, and `strace -T` timed the syncs. No hook entered the repository.

## Breakdown at throttle 1, 200 records

| Setting | Per request |
| --- | --- |
| No cache | 1.09 to 1.19 ms |
| Cache on tmpfs | 1.19 to 1.47 ms |
| Cache on ext4, before | 11.8 to 12.6 ms |

- Syncs cost about 10.8 ms of the 12. Each new entry ran three: the entry file, the cache folder, and the `.locks` folder after the lock file's removal. The 200-record run made 602 syncs, three per entry and two for the folder's backend marker. Under `strace -T` each took about 3.8 ms.
- Other cache work costs 0.05 to 0.3 ms: the lock file, the temporary file, the hard link, and the final read. A request scans no folder and runs no prune.
- About 1 ms of the remaining 1.1 ms sits in the loopback backend. The client writes the request head and body in two writes, and it sets `TCP_NODELAY`, so neither write waits for an acknowledgement. The backend peeks, sees only the head, and sleeps 1 ms before it peeks again. A real server reads the stream and does not sleep.
- Building the request, sending it, and parsing the reply take about 0.1 to 0.2 ms. One connection served all 200 requests. The address is numeric, so the client made no DNS lookup, and a plain `http://` loopback address sets up no TLS.

## Cause

Each new cache entry paid for three disk syncs. `specification/recording.md` requires two of them: the tool syncs the complete entry and then the recording folder before success. The third synced `.locks` after the completed lock file's removal. Ticket 0061 added it, and neither the specification nor ADR 0017 requires it. If a power loss undoes the removal, an empty lock file comes back. It holds no answer, a later caller reads the valid entry and needs no lock, and prune removes the file.

## Change

- `remove_lock` in `crates/thinkthen/src/engine/recorder.rs` unlinks the lock file and no longer syncs `.locks`. A comment says why.
- The `LockDirectorySync` fault stage and its two table rows leave `recorder/fault.rs`, because the step they injected into is gone.
- `specification/recording.md` says the tool does not sync `.locks` after the removal, so a power loss can bring back an empty lock file until a later prune.
- `cache prune` keeps its own `.locks` sync. It is not on the request path.
- The ratchet falls from 61,721 to 61,719.

The entry sync, the folder sync, the lock protocol, the file modes, and secrecy do not change.

## Before and after

The before and after binaries ran in alternating pairs on fresh cache folders on the same disk. The one-minute load stayed between 2 and 9.

| Run | Before | After |
| --- | --- | --- |
| Throttle 1, 200 records | 11.96 to 12.57 ms a request, about 82 a second | 8.29 to 8.96 ms a request, about 116 a second |
| Throttle 32, 2,000 records | 1,307 to 1,330 requests a second | 1,963 to 2,057 requests a second |
| Throttle 32, 20,000 records | 14.4 s and 17.3 s, 1,385 and 1,157 a second | 9.2 s and 12.9 s, 2,171 and 1,550 a second |

Outliers taken under load spikes: one before run at throttle 1 took 20.1 ms a request, and one after pair took 11.1 ms and 1,099 a second. The 20,000-record runs overlapped a load climb to 23.

## Checks

With `THINKTHEN_API_KEY` unset:

- `install`: exit 0.
- `lint`: exit 0, with the ratchet at 61,719.
- `test`: exit 0. The storage fault tables pass without the dropped stage. The cache locking and durability suites pass unchanged.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

## Deferred

- The two required syncs still cost about 7 ms for each new entry at throttle 1. Two options remain. A shared folder sync lets workers that install entries at the same time wait for one sync that starts after all their links. That keeps the specification's guarantee and needs a small coordinator, and it helps only at throttle above 1. One sync at the end of a call would change the guarantee that each answer is durable before it returns. That choice is Ian's.
- The loopback backend's 1 ms peek poll adds about 1 ms to every request in tests. The conformance backend could read the rest of a request with a blocking read. It serves tests only.
- `sdlc/issues/2026-09-25-a-fresh-cache-can-refuse-itself-as-a-retired-layout.md` files a race found during the measurement. It predates this change.
- The PostgreSQL warm was not rerun here. Its binding lives on `ticket/0111-port-postgresql-surface`.
