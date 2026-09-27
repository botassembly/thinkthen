# Accept the SQL and DuckDB designs

Status: design accepted, 2026-09-27. Owner: Codex. Implementation remains open.

A fresh read-only Codex reviewer checked tickets 0200 and 0201 as a paired design batch. The reviewer authored neither design. The final verdict accepted 0200 and ADR 0080 at `e982bfec`, and 0201 and ADR 0081 at `c7000a4a`. The queue owner accepts both within Ian's direction to fix the SQL findings and move DuckDB to its C++ API. Ian can overturn their recorded choices.

The first review required three corrections. PostgreSQL's try form now carries typed failures from argument reading and parsing through the worker, checking SQL NULL first. New failure values and the engine-cap refusal use fixed safe advice with no caller text or paths. DuckDB's migration includes the new try-details scalar and exempts its recoverable row failures from ordinary fatal bind and chunk validation. The final review accepted those corrections and their proof cases. It also checked the tagged DuckDB query lifecycle source and the stated first-use timing limit.

The ticket evidence checker and diff check passed in both design worktrees. No runtime proof is claimed. Ticket 0200 implements SQL row values, SQLite's explicit budget, idle engine retirement with retained totals, and PostgreSQL throttle refusal. Ticket 0201 follows in the DuckDB files and supplies the query hook and full C++ port. Register 72 stays to do until both hosts' promised budget behavior is proved. The current SQL settings and the later 0149/0155 retry accounting must survive each merge.

The two design-only lane claims did not reserve runtime files. The implementation uses a new claim before touching them. Other lanes take independent work while these database files are held.
