# Ticket 0148 owes old recording folders and the "retired layout" sentence

Status: closed by reviewed ticket 0148 on 2026-09-27. Filed 2026-09-26 by the queue owner from local experiment 273, report 09, finding 2. An amendment to ticket 0148, which is accepted on its branch and not landed. Blocks 0.1 with 0148.

## What happens

A library cannot open a recording folder made before folder binding. The report opened a copy of `demos/27-test-with-no-network/recording` from R. Both a hit and a miss failed with `the recording folder uses a retired layout`. The report counted 74 of 97 recording folders in this repository with entries and no `.thinkthen-backend.json`. Demo 27 is one of them.

The command handles the same folder. Under `--cache` it says ``the recording folder predates backend binding; replay it read-only or choose a new folder``, and under `--replay` it replays read-only. `specification/recording.md` line 40 promises that exact replay from an older unmarked folder stays read-only and creates no marker.

So the shipped offline examples cannot be reused from a library. The library message suggests the recording format was retired, which is false.

## What 0148 says today

0148 decision 3 says "the builder applies the command's folder rules", and its folder section lists the cache, record and replay pairs. It names neither unmarked folders under `replay` nor the library's error sentence.

## Checked on main

Verified: `crates/thinkthen/src/public/error.rs:198` reads "the recording folder uses a retired layout". `recording.md:40` reads as described. The R run and the folder count come from the report.

## What to amend in 0148

1. The builder's `replay` accepts an unmarked folder read-only and writes no marker, as `--replay` does.
2. The library error for an unmarked folder under a write-capable setting uses the command's sentence, in library words, and never says "retired layout".
3. A test replays demo 27's folder from a library.

## Done when

0148 lands with these three items, or the coordinator records why one moved elsewhere.

## Candidate proof, 2026-09-27

The 0148 ticket branch now maps library `replay` to a read-only recorder. A Rust public-boundary regression copies demo 27's unmarked recording, replays its hit with no key, checks an exact local miss, and verifies that the entry and marker state do not change. A write-capable use of the copy gets the command-equivalent backend-binding advice. The shared settings replay case counts zero extra loopback requests after a miss on each of the seven tested surfaces. Focused Rust and C settings tests pass. The reviewed ticket is landed; the three required behaviors pass.
