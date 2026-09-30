# SQLite and PostgreSQL build their plan JSON by hand

Status: open. Found while building ticket 0314 slice 4a on main at `6c29039b9`. Owner: none yet; the SQL hosts are outside ticket 0314's port pass (ADR 0112 section 6).

Kind: debt

Pay when: the next change to either host's `thinkthen_plan`, or before 0.1.

Keeping it leaves two hand copies of the plan shape beside the one the crate now serializes, so a new plan member can reach the C door and miss SQL.

## The problem

Slice 4a gave `PlanEstimate` a `Serialize` impl, and the generated result schema now holds its `plan` definition. The C door writes it with `serde_json::to_string`. SQLite (`databases/sqlite/src/scalars/plan.rs:77-91`) and PostgreSQL (`databases/postgresql/src/keyed.rs:184-199`) still build the same six members with `json!`. Both already print `first_body_utf8`, the member name the crate took, so switching each to `serde_json::to_string(&estimate)` changes no byte. DuckDB returns a native `STRUCT` with `first_body` and stays as it is.

Lane 1 was editing the SQL hosts' limits (0304 slice 3e) when this was found, so slice 4a left them alone.
