# 0584: Experiment: the answer schema covers every 0.4 use

Status: COMPLETE.

Milestone: 0.3

Reviews: revision 0e46dc2, reject
Reviews: revision a37ca83, reject
Reviews: revision a37ca83, accept

## Outcome

Learn whether PRD 01's tables and published views answer every curation, eval and notebook use planned for 0.4 as plain SQL on SQLite and PostgreSQL, before the database ticket fixes the schema.

## Evidence

- Starts from: the marketing repo's `products/thinkthen/0.3/prd/01-database.md`, `prd/02-evals-and-curation.md`, `evals.md` (the table of eval uses and Ian's curation rulings) and `data.md`; ThinkThen's `specification/cache.md` and `crates/thinkthen/src/engine/store/`.
- Keeps: No product code changes, no paid model calls, no publishing, and no agent or machine time taken from 0.2 work.
- Changes: `sdlc/records/0584*` only. Owned numbered experiment folder with `OWNER` reading `thinkthen/0584`; PostgreSQL runs as `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0584`.
- Method: write the tables and the six views as SQL. Fill them from a synthetic fixture: two tools, three models, two wording versions, 200 evidence items, right answers settled by a person and by an agent, a disputed item, two answer key versions, outcomes, imported labels and a bulk-picked batch. Write one query per use: grade a setup against a key, compare two setups, score a judge, the three queue orders (lowest confidence, two models disagreeing, a model disagreeing with imported labels) with the random share, freeze a key, join outcomes, export training rows, a notebook read of answers with evidence text, and a lookup under each reuse rule.
- Proof: every query returns the rows computed by hand, on both engines. The record lists each table or view change the uses forced.
- Stop rule: every use passes, or the record names the uses that cannot be served and the change each needs.
- Decision it changes: the 0.3 database schema, under Ian's ruling that it is designed for curation from the start.
- Machine: the Beelink.
- Defers: the lasting CI test and the schema files move into the 0.3 database ticket.

## Progress

- 2026-10-14 started

Ian ruling, 2026-10-14. A question has a stable identifier and a version number. A saved-answer lookup may name the version or leave it out. Without a version, the lookup matches the identifier and uses the latest version by default. The answer database keeps every answer, including the history of repeated calls. Clearing old answers is later work.
