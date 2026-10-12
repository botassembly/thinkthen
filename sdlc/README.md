# sdlc/

Tickets and the notes that drive work on thinkthen. Tickets exist to drive the code; the code is what matters. pm reads and changes ticket status.

| Folder | What it is |
| --- | --- |
| [planning/](planning/) | The design study, the Rust standards, the plan, and the architecture decision records |
| [tickets/](tickets/) | Work a ticket authorizes. One file per ticket, numbered from 0001 |
| [records/](records/) | Optional short notes, such as review findings and measurements |
| [scripts/](scripts/) | The gate ladder: `install`, `lint`, `test`, `spec`, `surfaces` |
| surfaces.txt | The nine surfaces and their state. `sdlc/scripts/surfaces` reads it |
| ratchet.json | The source size ceiling. `sdlc/scripts/lint` reads it |

The heavy rungs `install`, `test`, `spec`, and `surfaces` honor `THINKTHEN_HEAVY_LOCK`. Ian authorizes independent builds to overlap when host load, available memory and I/O permit. Give each lane its own lock, such as `/run/user/1000/thinkthen-codex-2.lock` on Linux or `/tmp/thinkthen-codex-3.lock` on M5, and keep build output in separate worktrees. The scripts' fallback `${XDG_RUNTIME_DIR:-/tmp}/thinkthen-heavy.lock` is not a machine-wide scheduling requirement. Retain exclusive access to genuinely shared mutable toolchain installations or the same build output.

Inspect capacity before substantial work; the [capacity ruling](decisions/2026-10-08-preserve-planning-rulings.md#build-capacity) gives the starting guide. Reduce compiler jobs or defer a new heavy step when the host is under pressure. A direct Linux command using a lane lock runs as `flock -o "$THINKTHEN_HEAVY_LOCK" <command>`. Without `-o`, a background `sccache` server can inherit and retain that lock. Rungs already under their own lock skip reacquiring it. M5 uses Python `fcntl` when locking is needed because `flock` is absent.

A ticket needs an Outcome and a Proof. Add other notes only when they help the builder.

## Ticket preparation and completion

Follow [ticket-preparation.md](planning/ticket-preparation.md) when preparing and finishing a ticket. A landing record is optional.

## Repository map

`crates/thinkthen` holds `core`, `engine`, `cli`, and the binary. `specification/` is the contract; `spec/` holds `mustmatch` pages. `demos/` holds how-tos; each green one is an ADR 0016 test. `transforms/` holds `jq` over saved rows. The queue owner owns `site/` as well as the rest of the repo, under [ownership.md](planning/ownership.md). `probes/` holds ruled live measurements. `sdlc/` holds the records mapped above; `CONTRIBUTING.md` defines terms.
