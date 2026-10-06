# 0434: SQL inputs and images

Date: 2026-10-06
Status: in progress. Slice A implements the existing native input/image contract; final native format adoption remains open.

Rich DuckDB recognize kinds, SQLite strict kind grammar and duplicate refusals, and the privileged bounded PostgreSQL question-file loader now work. Ordinary scalar, NULL and text behavior remains. One native engine, reader and cache own the work. Native 0456 additions and final complete SQL adoption remain open.

Fresh High review found one defect: constant rich kinds were resolved before NULL evidence was skipped. Resolution now follows row-level NULL checks; independent malformed-kind and unreadable-file regressions passed on both DuckDB versions. The reviewer confirmed ACCEPT.

SQLite gate, PostgreSQL 96 checks and focused loader/image/client-reader cases, both DuckDB versions’ image/cancellation checks, format/policy/ratchets and private-name checks passed. Full tests and lint run on the landing commit with affected SQL checks. Existing unexecuted conformance cases count as not checked, not passes. Later integration updates this same record.
