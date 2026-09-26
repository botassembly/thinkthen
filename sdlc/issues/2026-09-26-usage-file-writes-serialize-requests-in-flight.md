# Usage file writes serialize the requests in flight

Status: Open. Filed 2026-09-26 from workspace experiment 268, a timing study of `filter` over 306 short titles against `jev-latest`.

## What happens

`Counters::add` takes the shared `persistent` lock (`crates/thinkthen/src/engine/usage.rs:133`) and then calls `update()`. `update()` rewrites the usage file and forces the file and its folder to disk (`:214`, `:215`, `:237`, `:243`). Each request updates the file twice, once before sending and once when the reply arrives. Every worker waits on the same lock, so the disk sets the pace for all of them.

## Evidence

- At `--jobs 16`, 6 to 9 requests were in flight on average. At `--jobs 32`, 5 to 16 were.
- A 100-title run made 200 usage updates and 400 disk flushes. The median flush took 11.4 ms, and together they took 4.4 s, one after another.
- The same 306-title run took 7.2, 6.6 and 7.1 s with the usage file on disk, and 3.5 and 4.0 s with only the usage file moved to memory.
- About 3.5 s of a typical 7 to 8 s run goes to waiting on these writes.

## Why it matters

`specification/records.md` says `--jobs N` means N requests in flight. `specification/recording.md` calls the usage file best effort. Neither says it limits speed. The talk quotes speed, and today the command runs at about half the speed the service allows.

## What is asked

Usage counting should not limit how many requests run at once. The monthly totals should stay correct across processes, as they are now. The design belongs to this repository.
