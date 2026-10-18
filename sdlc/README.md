# sdlc/

Tickets and the notes that drive work on thinkthen. Tickets exist to drive the code; the code is what matters. pm reads and changes ticket status.

| Folder | What it is |
| --- | --- |
| [planning/](planning/) | The design study, the Rust standards, the plan, and the architecture decision records |
| [tickets/](tickets/) | Work a ticket authorizes. One file per ticket, numbered from 0001 |
| [records/](records/) | Optional short notes, such as review findings and measurements |
| [scripts/](scripts/) | The gate ladder: `install`, `lint`, `test`, `spec`, `surfaces` |
| surfaces.txt | The nine surfaces and their state. `sdlc/scripts/surfaces` reads it |

Run every build and check on M5 through `pm job run --machine m5 NNNN SHA --disk GIB --memory GIB -- COMMAND` at a pushed commit. The job owns its worktree and output, reserves disk and memory, and limits concurrent builds. Linux checks use the pinned arm64 container on M5. Proof specific to Linux x86_64 uses the pinned container on Yellow through `pm job run --machine yellow`; container jobs reserve the matching `linux.memory_gib` or Yellow-only `linux.yellow_memory_gib` value from `sdlc/tools.json` with pm `--memory`. Yellow runs one heavy job at a time. Hosted workflows run by hand for releases after local checks pass.

PM job admission is the only build turn-taking mechanism. Do not wrap jobs or toolchain mutations in another build lock.

A ticket needs an Outcome and a Proof. Add other notes only when they help the builder.

## Ticket preparation and completion

Follow [ticket-preparation.md](planning/ticket-preparation.md) when preparing and finishing a ticket. A landing record is optional.

## Repository map

`crates/thinkthen` holds `core`, `engine`, `cli`, and the binary. `specification/` is the contract; `spec/` holds `mustmatch` pages. `demos/` holds how-tos; each green one is an ADR 0016 test. `transforms/` holds `jq` over saved rows. The queue owner owns `site/` as well as the rest of the repo, under [ownership.md](planning/ownership.md). `probes/` holds ruled live measurements. `sdlc/` holds the records mapped above; `CONTRIBUTING.md` defines terms.
