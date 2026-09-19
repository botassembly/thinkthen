# sdlc/

The system of record for thinkn. Nothing outside this repository is authoritative.

| Folder | What it is |
| --- | --- |
| [planning/](planning/) | The design study, the Rust standards, the plan, and the architecture decision records |
| [issues/](issues/) | Problems found and filed. An issue is not a ticket and authorizes no work |
| [tickets/](tickets/) | Work a ticket authorizes. One folder or file per ticket |
| [records/](records/) | Sealed records of work that ran |
| [scripts/](scripts/) | The gate ladder: `install`, `lint`, `test`, `spec` |
| ratchet.json | The source size ceiling. `sdlc/scripts/lint` reads it |

The destination this repository serves lives outside it, at `notes/ideal-state/thinkn.md` in the workspace. The five factory scripts under `project/` are absent until this repository registers with Factory 2.
