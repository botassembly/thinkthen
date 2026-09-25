# Two unmaintained crates fail cargo-deny in the surface workspaces

Status: Closed on 2026-09-25 after a check against main. Option 1 taken in c10f84c9: ignores in the PostgreSQL and R deny.toml files, checked by BINDING_DENY.

Date: 2026-09-22. Source: the second surfaces review's dependency audit, re-run by command during the gate-coverage wave. Owner: build team (dependency policy), with the library team's evidence attached.

## What

`cargo deny --offline check advisories` (the repo's own `deny.toml`, whose `ignore` list is empty) fails in two surface workspaces:

- `databases/postgresql`: `serde_cbor 0.11.2` — `error[unmaintained]`, RUSTSEC-2021-0127, "No safe upgrade is available!"; the chain is `serde_cbor ← pgrx 0.17.0 ← thinkthen` (`cargo tree --locked -i serde_cbor`). The advisory names ciborium and minicbor as the author's alternatives.
- `libraries/r/thinkthen/src/rust`: `paste 1.0.15` — `error[unmaintained]`, RUSTSEC-2024-0436, "No safe upgrade is available!"; the chain is `paste ← extendr-api 0.8.2`. The advisory names pastey and with_builtin_macros as alternatives.

Both crates are transitive: the pinned parents (`pgrx = "=0.17.0"`, `extendr-api = "0.8"`) own the dependency, so no swap is available from this repo. The root workspace's deny check passes today because `crates/` carries neither crate; the failure appears the moment deny coverage extends over the surface workspaces, which the second review asks for at merge.

## The decision

When deny coverage extends over the workspaces, either:

1. add explicit `ignore` entries for RUSTSEC-2021-0127 and RUSTSEC-2024-0436 in `deny.toml`, each with this record as the argument (the file's own rule: an exception goes in with the record that argued it, never as a blanket ignore); or
2. block the merge prep on upstream bumps (pgrx dropping serde_cbor; extendr-api replacing paste).

Recommendation: (1) now, with the ignore entries pointing here, plus a standing note in each surface's NOTES.md; revisit at the first upstream release that drops the crate. The library team recorded the replacement plans in `databases/postgresql/NOTES.md` and `libraries/r/NOTES.md` on the `surfaces` branch.
