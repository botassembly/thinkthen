# sdlc/

The system of record for thinkthen. Nothing outside this repository is authoritative.

| Folder | What it is |
| --- | --- |
| [planning/](planning/) | The design study, the Rust standards, the plan, and the architecture decision records |
| [issues/](issues/) | Problems found and filed. An issue is not a ticket and authorizes no work |
| [tickets/](tickets/) | Work a ticket authorizes. One file per ticket, numbered from 0001 |
| [records/](records/) | Sealed records of work that ran |
| [scripts/](scripts/) | The gate ladder: `install`, `lint`, `test`, `spec`, `surfaces` |
| surfaces.txt | The nine surfaces and their state. `sdlc/scripts/surfaces` reads it |
| ratchet.json | The source size ceiling. `sdlc/scripts/lint` reads it |

The heavy rungs `install`, `test`, `spec`, and `surfaces` honor `THINKTHEN_HEAVY_LOCK`. Ian authorizes independent builds to overlap when host load, available memory and I/O permit. Give each lane its own lock, such as `/run/user/1000/thinkthen-codex-2.lock` on Linux or `/tmp/thinkthen-codex-3.lock` on M5, and keep build output in separate worktrees. The scripts' fallback `${XDG_RUNTIME_DIR:-/tmp}/thinkthen-heavy.lock` is not a machine-wide scheduling requirement. Retain exclusive access to genuinely shared mutable toolchain installations or the same build output.

Inspect capacity before substantial work; the [work plan](planning/work-plan-2026-09-27.md#build-capacity) gives the starting guide. Reduce compiler jobs or defer a new heavy step when the host is under pressure. A direct Linux command using a lane lock runs as `flock -o "$THINKTHEN_HEAVY_LOCK" <command>`. Without `-o`, a background `sccache` server can inherit and retain that lock. Rungs already under their own lock skip reacquiring it. M5 uses Python `fcntl` when locking is needed because `flock` is absent.

A ticket numbered 0120 or higher carries a `## Evidence` section of five `-` list items labeled exactly `Starts from:`, `Keeps:`, `Changes:`, `Proof:`, and `Defers:`, each followed by text. They name the evidence it starts from, the behavior it keeps, what it deliberately changes, what proves it, and the gaps it defers. Workspace decision `2026-09-24-experiments-reduce-risk.md` sets the rule. `sdlc/scripts/tickets` enforces it from the lint rung. Earlier tickets are exempt by number. Every numbered ticket here counts as a product ticket.

The destination this repository serves lives outside it, in Ian's workspace. The five factory scripts under `project/` are absent until this repository registers with the factory.

## Ticket preparation and completion

Follow [ticket-preparation.md](planning/ticket-preparation.md) when preparing and finishing a ticket. A Quick Fix keeps its lessons in its build record.

## Repository map

`crates/thinkthen` holds `core`, `engine`, `cli`, and the binary. `specification/` is the contract; `spec/` holds `mustmatch` pages. `demos/` holds how-tos; each green one is an ADR 0016 test. `transforms/` holds `jq` over saved rows. The queue owner owns `site/` as well as the rest of the repo, under [ownership.md](planning/ownership.md). `probes/` holds ruled live measurements. `sdlc/` holds the records mapped above; `CONTRIBUTING.md` defines terms.
