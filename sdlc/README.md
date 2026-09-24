# sdlc/

The system of record for thinkthen. Nothing outside this repository is authoritative.

| Folder | What it is |
| --- | --- |
| [planning/](planning/) | The design study, the Rust standards, the plan, and the architecture decision records |
| [issues/](issues/) | Problems found and filed. An issue is not a ticket and authorizes no work |
| [tickets/](tickets/) | Work a ticket authorizes. One file per ticket, numbered from 0001 |
| [records/](records/) | Sealed records of work that ran |
| [scripts/](scripts/) | The gate ladder: `install`, `lint`, `test`, `spec` |
| ratchet.json | The source size ceiling. `sdlc/scripts/lint` reads it |

The heavy rungs `install`, `test`, and `spec` each hold an exclusive `flock` on `THINKTHEN_HEAVY_LOCK`, default `${XDG_RUNTIME_DIR:-/tmp}/thinkthen-heavy.lock`, and a rung called under a held lock skips it. Another heavy command, such as a cold `cargo` build outside a rung, runs as `flock -o "$THINKTHEN_HEAVY_LOCK" <command>`. Without `-o`, a background `sccache` server that the command starts inherits the lock and holds it after the command exits, and every later rung waits forever.

A ticket numbered 0120 or higher carries a `## Evidence` section of five `-` list items labeled exactly `Starts from:`, `Keeps:`, `Changes:`, `Proof:`, and `Defers:`, each followed by text. They name the evidence it starts from, the behavior it keeps, what it deliberately changes, what proves it, and the gaps it defers. Workspace decision `2026-09-24-experiments-reduce-risk.md` sets the rule. `sdlc/scripts/tickets` enforces it from the lint rung. Earlier tickets are exempt by number. Every numbered ticket here counts as a product ticket.

The destination this repository serves lives outside it, at `notes/ideal-state/thinkthen.md` in the workspace. The five factory scripts under `project/` are absent until this repository registers with Factory 2.
