# The churn probe left one panic unexplained

Status: Open. Found by ticket 0086's build and code review, 2026-09-24. Owner: ticket 0094, whose C probe carries G3 and R7-1.

## What happens

`probes/churn-0086` ran the public Rust API 152 times, and 151 runs ended cleanly. One early run ended with exit 101, a panic on the probe's main thread. It ran at a load near 60 with the machine in swap, and its output was not kept. The 104 later runs kept the output of any failed run, and all ended cleanly. So the cause is still unknown. Three causes fit:

- A scoped thread failed to start under memory pressure, and the probe's `expect` panicked. This is the likeliest.
- Another process bound the probe's refused port. The probe makes that port by binding a free one and dropping it, so the port can be claimed during a run. One call would then succeed, and the closing count would fail.
- A panic escaped the public door, which would be a defect.

The stand-in side ran 24 times of the 300 the ticket asked for. That is too few to show a crash that occurs about once in 100 runs.

Ian ruled on 2026-09-24 that the churn is a one-time measurement. It does not run on every ticket or rerun, because it overloads the machine. The remaining runs are dropped.

## Fix

Give the probe a refused address that no process can claim, such as a listener it holds with a zero backlog, or a reserved port on a closed interface. Have the probe report a thread start failure as its own exit status. Rerun only if Ian asks, and ticket 0094's C probe covers the stand-in crash.
