# SQLite `thinkthen_find` drops its `model` setting

Status: open. Found 2026-10-01 while building ticket 0378. Owner: the queue owner.
Milestone: 0.2

SQLite's `thinkthen_find` accepts `{"model":"NAME"}` in its settings and then sends the engine's own model. DuckDB and PostgreSQL send the named model.

`databases/sqlite/src/scalars/find.rs` checks the settings with `Settings::check(For::Find)`, which allows `model`, and then builds `Question::find(&argument)` without it. The SQLite engine is built once per process, so the model cannot travel on the engine.

The fix follows `thinkthen_rank` on SQLite: read `Settings::model()` and pass it to `Question::with_model`, which ticket 0378 added. A loopback test captures the request body and pins its `model`, as `databases/sqlite/tests/test_rank.py` does for rank.
