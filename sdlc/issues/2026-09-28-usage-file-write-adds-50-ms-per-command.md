# The usage-file write adds about 50 ms to each command

Filed 2026-09-28 by the marketing lead, from a read-only timing investigation.

## What happens

Beatles Bench runs one `thinkthen` process per question. Its median time is 0.215 s per question over 1,501 questions on build 02dc0b96, with four questions running at once (beatles-bench `results/tables/cost.tsv`).

Jev's server takes about 0.06 s of that, per a proxy measurement in local experiment 268. ThinkThen's own work takes about 0.05 s.

A local measurement against a fake backend, with no paid calls, gave these medians:

| Build | Usage file | One at a time | Four at a time |
| --- | --- | --- | --- |
| 02dc0b96 | on disk | 0.018 s | 0.057 s |
| 02dc0b96 | in memory | 0.004 s | |
| 3b96e720 (main) | on disk | | 0.050 s |

Nearly all of ThinkThen's time goes to the usage-file write that each process makes. Ticket 0141 stopped usage writes from holding back requests inside one process. The cost for one process per call remains.

## Why it matters

The shell and every script that calls the command once per item pay this cost on each call. It is about a quarter of a typical answer's time, and it makes ThinkThen look slower than Jev.

## Asked

Measure the per-process usage write, and decide whether one process can skip or defer it without losing counts.
