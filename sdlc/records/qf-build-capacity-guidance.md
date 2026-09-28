# Build concurrency follows available capacity

Ian explicitly removed the one-heavy-build limit. Independent Linux and M5 jobs may overlap when load, memory and I/O permit. Main `12266de6` records the ruling; `46e50418` aligns AGENTS.md and sdlc/README.md with lane-specific rung locks, isolated output and retained exclusion for genuinely shared mutable installations. No runtime script or product behavior changed.

Fresh independent Sol Medium review accepted `46e50418` after inspecting the heavy-lock helper and both changed documents. The helper honors the selected lock path and skips reacquisition only when that lock is already held. Pages, tickets and diff checks passed. No full suite was appropriate for this guidance change.

The coordinator observed Linux load 1.28 across 16 logical CPUs with 22.4 GiB available and negligible memory/CPU pressure. M5 load was 0.40 across 18 logical CPUs, memory_pressure reported 95 percent free, and disk had 452 GiB free. These snapshots justified overlap then; workers inspect current capacity before substantial work rather than treating the snapshots as permanent limits. Four implementation lanes received the new instruction.

## What the build taught us

A machine-wide lock can serialize independent work despite spare capacity. Retain locks for the actual shared mutable resource and same-output exclusion. Use capacity observations to admit independent work; reduce build jobs or defer new heavy work when pressure rises. This removes an avoidable scheduling restriction, but it does not claim a measured speedup or explain away separate code-review and platform corrections.
