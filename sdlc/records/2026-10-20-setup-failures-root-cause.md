# Root cause: most failed runs on 2026-10-20 found no product bug

Severity: medium. It repeatedly slowed the freeze and wasted M5 time; it put no wrong answer in front of anyone.

## Cases

About sixteen failed runs on 2026-10-20, sorted by cause:

- Product, four: batch-one session pacing read ahead (0731), owned C errors dropped the record position (0732), C engine construction had no owned error (0741), replayed results lacked the model (0723).
- Setup, about eight: image fragment order installed Cargo providers before Rust (0738), a C README line contract (0742), a thread-count test counted the next libtest thread (0743), hard-coded CLI and debug-command paths (0707, 0739), a retired result tag in an MCP fixture (0707), C install fixtures missing compiler output (0641), BSD `wc` spacing (0737), a Yellow bootstrap cycle (0734).
- Invocation, four: a read-only checkout where fixtures write scratch (0738), a missing C operations producer step (0741), a hard-coded image tag retired by 0738's cleanup and Docker hidden by tool isolation (0743).

## Why

1. Tests and proof commands assumed machine details: program paths, tools on PATH, image tags, thread counts.
2. Each test carries its own knowledge of the setup; nothing hands every test one setup.
3. Until 0740, no standard command ran one surface in the Linux container, so developers composed pm job, Docker and paths by hand.
4. Tickets described proofs in prose, and each developer started fresh, so each re-derived the command.
5. The coordinator used a fresh agent for every step, including repairs, which discards what the last developer learned.
6. The test setup changed while in use: the image, tool manifest, image cleanup and run commands all changed on the same day.
7. Phase 1 machinery work overlapped phase 2 by choice, to keep lanes moving, and the core cut deferred language tools.
8. Root cause: the test machinery is still large and tied to particular machines, the problem rule 5 of [the 0.2.1 ruling](../decisions/2026-10-18-0-2-1-rebuilds-the-basis.md) names.

## Fixes

Recorded as [standard proof commands](../decisions/2026-10-20-standard-proof-commands.md): three standard commands; briefs carry exact commands; the author fixes its own failed check; setup holds still after the freeze; failed runs name their cause as product, setup or invocation. 0740 provides the surface command. Phase 3 tickets were rewritten on 2026-10-20 to cite the standard commands.

## Reach

Shared with other teams through the developer skill and pm job habits. Jarvis has the companion finding from Genome: lanes now run the routine suite on their branch merged with main before review.
