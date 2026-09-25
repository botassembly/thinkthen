# Engine churn, ticket 0086

This probe carries G3 and R7-1 from `sdlc/issues/2026-09-23-surfaces-branch-error-index.md`. Through the C door, 32 threads over 70 engines, each engine with its own timeout, against a refused port, crashed the stand-in in 1 of 83 runs and in 2 of 104 runs. Both crashes were a SIGSEGV in a new thread's start (`sdlc/issues/2026-09-23-the-c-door-churn-still-crashes-in-a-new-threads-start.md` at tag `surfaces-wave7-final`).

`src/main.rs` runs that load through Rust: 32 threads share 70 engines and each thread makes 20,000 calls to a refused loopback port, 640,000 calls a run. One source drives two APIs. With the `standin` feature it drives the stand-in's Rust API at tag `surfaces-wave7-final`, where each engine gets its own transport timeout from 1,000 to 1,976 ms and no retry. Without it, it drives the public API of this checkout. The public API has no transport timeout, so each engine carries its own call deadline over the same 70 values instead. The run fails unless every call fails.

`run.sh` builds one side, runs it `RUNS` times (300 by default), 8 at a time at nice 19, and prints how the runs ended by exit status. A status of 139 is a SIGSEGV, and 134 is an abort. Before each batch it waits while the one-minute load is 10 or more. It needs no key and no network. It unsets `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`, and the public side sends a made-up key to the refused loopback port alone.

```sh
mkdir -p /tmp/churn-tag
git archive surfaces-wave7-final | tar -x -C /tmp/churn-tag
sh probes/churn-0086/run.sh tag /tmp/churn-tag
sh probes/churn-0086/run.sh main
```

This machine runs heavy work under one shared lock. So each batch ran as `flock LOCK timeout 1500 sh run.sh MODE ...` with a small `RUNS`: 8 per batch at first, then 4 with `PARALLEL=4`. The lock was `/tmp/thinkthen-heavy.lock` for the first batches and `/run/user/1000/thinkthen-heavy.lock` after it moved. The main side was built from a `git archive` of a named commit on `ticket/0086-public-rust-api`, so edits in progress could not change the probed code.

## Results, 2026-09-24

These counts are final. Ian ruled on 2026-09-24 that this churn is a one-time measurement. It does not run on every ticket or rerun, because it overloads the machine.

| Side | Runs | Clean | SIGSEGV (139) | Abort (134) | Panic (101) |
| --- | --- | --- | --- | --- | --- |
| Stand-in Rust API, tag `surfaces-wave7-final` | 24 | 24 | 0 | 0 | 0 |
| Public API, commits e08276e6 and 13445804 | 152 | 151 | 0 | 0 | 1 |

The first 48 public runs were of commit e08276e6, 8 at a time. One batch of 8 took 4 min 55 s at 826% CPU and pushed the load to 120. One run in that batch ended with status 101, a panic on the probe's main thread, at a load near 60 with the machine in swap. Its output went to `/dev/null`. The 104 later runs were of commit 13445804, 4 at a time with `PARALLEL=4`. Each batch held the heavy lock and started at load 10 or below. All 104 ended cleanly, so no kept output names the panic. `sdlc/issues/2026-09-24-the-churn-probe-left-one-panic-unexplained.md` lists the causes that fit it. One cause is this probe's refused port: the probe binds a free port and drops it, so another process can claim it during a run.

The stand-in showed no crash in 24 runs, too few to rule out a crash that occurs about once in 100 runs. G3 and R7-1 stay open for ticket 0094's C probe, and ticket 0086 claims no fix for them.
