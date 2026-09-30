# 0322: Every surface adds to the usage totals

Status: waits for ADR 0111 slice 3. Design: ADR 0113 (proposed). Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Issue: `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`, option 1.

## Outcome

`thinkthen status` shows what every surface spent. An engine built by `EngineBuilder::from_env()` adds its requests, retries, live tokens and cache answers to the same count-only monthly files the command writes. That covers every port, the C door and the three SQL extensions. An engine built by `EngineBuilder::new()` writes nothing. `specification/recording.md` stops listing the SQL leaks that current source already closed.

## Evidence

- Starts from: ADR 0113; ADR 0111 slice 3, which puts every surface on one send stage and one lookup stage; tickets 0308 and 0311, where one `from_env` read reached every surface with no port code; `engine/usage.rs::Counters`, which already persists given a folder; the fork rule in `engine/process.rs`; record `2026-09-28-accounting-after-c-facts.md`.
- Keeps: the usage files, lock, modes, `thinkthen.usage/1` shape and retry sidecar; best-effort persistence with ADR 0097's one-second finish deadline; the count points of ADR 0111 section 7; the `status` reader and schema; every port constructor; the per-process request total and token cap, per backend in PostgreSQL.
- Changes: `from_env` seeds the counters with `config::usage_path()`. Library and SQL write failures stay silent. Test entry points point the usage folder at scratch and fail if the real one changed. `recording.md`, the SQL READMEs and the changelog say every surface adds to the totals and name PostgreSQL's per-backend cap.
- Proof: offline, against the counted loopback backend, under a scratch usage folder. The command, a Rust `from_env` engine with one retried 503, the C door and the SQLite extension send five attempts; `status --json` then reports 5 requests, 1 retry and the tokens the replies reported. A cached rerun reaches the listener zero times and adds cache answers. `EngineBuilder::new()` leaves the folder absent. A `0755` usage folder changes no result and no arrival count. Two child processes sending three each total six; a forked child and its parent sending one each total two. PostgreSQL: two connections sending one each give the server user's `status` two.
- Defers: a cluster-wide PostgreSQL cap through shared memory; an opt-out setting such as `THINKTHEN_USAGE=off`; a per-surface split in `status`; reporting library write failures to the caller; the other ports over C beyond the C door case, which start from the same `from_env`.
