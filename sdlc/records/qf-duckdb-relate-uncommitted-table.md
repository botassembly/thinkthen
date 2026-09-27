# DuckDB relate identifies a caller's uncommitted table

Quick Fix `qf/duckdb-relate-uncommitted-table`, 2026-09-27. Status: code candidate awaiting fresh review and landing.

- Starts from: Register 77 in local experiment 284 and main at `22f36d00`. A real DuckDB relate over `t` created in the caller's open transaction returned `thinkthen usage: the relate query failed: Catalog Error: Table with name t does not exist!`. The outside-in case failed with this error before the fix.
- Keeps: A genuinely missing name retains DuckDB's original catalog error. A temporary table retains its existing ADR 0038 sentence. Every refusal occurs before an engine request.
- Changes: On another missing-table error, relate appends the separate-connection rule and conditional commit advice. The original catalog error stays visible. This covers an uncommitted table in any catalog or schema without claiming that a typo is uncommitted. The DuckDB page now states the rule.
- Proof: The first outside-in DuckDB case failed on main with `Catalog Error: Table with name t does not exist!`. The final cases cover default, attached active catalog, explicit active schema, and qualified table transactions, plus a missing typo and temporary table. Each new refusal asserts zero loopback backend sends. Source checks and both DuckDB source ratchets pass. Native rebuild and final DuckDB suite are pending behind the shared heavy lock.
- Defers: Relate still cannot run SQL on the caller's connection under DuckDB's stable C API. The caller must commit before relating that table. No paid or live backend was used. The coordinator runs the full repository ladder after fresh review.

The broad catalog lookup was considered and rejected. DuckDB's missing-table text omits the catalog and schema in an ordinary qualified query. A lookup of the active search path would miss that case, while searching every catalog could mislabel a typo as an uncommitted table. The neutral advice covers both without a new parser or test hook. Ian can overturn this wording choice.

The Rust source grows by 5 nonblank lines and the Python suite by 29. These lines earn the universal, qualified-table message and outside-in cases that prove the original error and zero sends. I checked the existing temporary-table boundary for reuse and added the advice there. The correction needs no new catalog lookup, SQL parser, or test hook.
