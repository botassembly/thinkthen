# 0519: Make the SQL surfaces consistent and self-described

Status: OPEN.

Milestone: 0.2

Depends on: 0511

## Outcome

DuckDB, SQLite and PostgreSQL treat NULL one way, take images as binary values, return native JSON types where results are JSON, and describe every function inside the database.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). SQLite raises an error for a NULL question in rank sets but returns NULL in the complete calls. Complete calls take images as JSON arrays of byte numbers. DuckDB returns VARCHAR for annotate and details. No surface registers function descriptions.
- Keeps: Function names and current accepted inputs.
- Changes: One NULL rule in the guide applied to all three. BLOB and bytea images. DuckDB JSON type. Function comments in each extension. Claim `databases/**` per slice.
- Proof: Shared SQL cases cover NULL inputs, a BLOB image and the description query on each database.
- Defers: Extension signing and registry packaging to 0517.
