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

This machine runs heavy work under one shared lock, so each batch of 8 ran as `flock LOCK timeout 1500 sh run.sh MODE ...` with `RUNS=8`. The lock was `/tmp/thinkthen-heavy.lock` for the first batches and `/run/user/1000/thinkthen-heavy.lock` after it moved. The main side was built from a `git archive` of commit e08276e6 on `ticket/0086-public-rust-api`, so edits in progress could not change the probed code.

## Results, 2026-09-24

| Side | Runs | Clean | SIGSEGV (139) | Abort (134) | Panic (101) |
| --- | --- | --- | --- | --- | --- |
| Stand-in Rust API, tag `surfaces-wave7-final` | 24 | 24 | 0 | 0 | 0 |
| Public API, commit e08276e6 | 48 | 47 | 0 | 0 | 1 |

One public run ended with status 101, a Rust panic on the probe's main thread, in the second batch at a load near 60 with the machine in swap. That batch sent each run's output to `/dev/null`, so the panic message is lost. `run.sh` now keeps the output of any run that does not end cleanly. The 32 later public runs kept output, and all ended cleanly. The public API returns an engine panic on the calling thread as `Error::Defect`, and the probe counts that as a failed call. So the panic most likely came from the probe's own code. Three causes fit. A scoped thread may have failed to start under memory pressure. Another agent's loopback server may have bound the freed port, so one call succeeded and the closing count failed. Or a panic escaped the public door, which would be a defect. The record cannot tell them apart. The next public batches with output kept will name it.

Neither side reached 300 runs. On a 16-core machine, one batch of 8 public-API runs took 4 min 55 s at 826% CPU and pushed the load to 120. A batch of 8 stand-in runs took about 3 min at 182% CPU. After each public batch, the load needed several minutes to fall back under 10. At that pace, 300 runs a side would hold the machine for about 6 hours on the public side and about 2 hours on the stand-in side.

The stand-in's Rust API showed no crash at the tag. The probe did not reproduce the C door's crash, so G3 and R7-1 stay open for ticket 0094's C probe, and ticket 0086 claims no fix for them. The public API showed no SIGSEGV or abort. Its one panic is open until a kept output names it.
