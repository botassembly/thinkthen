# 0434: SQL inputs and images

Date: 2026-10-06
Status: slice A landed. Complete family adoption is qualified under [0435](0435-sql-call-facts-and-prices.md); final integration gates remain.

Rich DuckDB recognize kinds, SQLite strict kind grammar and duplicate refusals, and the privileged bounded PostgreSQL question-file loader now work. Ordinary scalar, NULL and text behavior remains. One native engine, reader and cache own the work. Native 0456 additions and final complete SQL adoption remain open.

Fresh High review found one defect: constant rich kinds were resolved before NULL evidence was skipped. Resolution now follows row-level NULL checks; independent malformed-kind and unreadable-file regressions passed on both DuckDB versions. The reviewer confirmed ACCEPT.

SQLite gate, PostgreSQL 96 checks and focused loader/image/client-reader cases, both DuckDB versions’ image/cancellation checks, format/policy/ratchets and private-name checks passed. Full tests and lint run on the landing commit with affected SQL checks. Existing unexecuted conformance cases count as not checked, not passes. Later integration updates this same record.

The initial full run sampled seven active requests in the existing shared-cap test where its fixture requires eight. The same case passed alone. The full run uses four test processes to reduce competing load; its own eight-request behavior and all cases remain unchanged.

Landing lint found the PostgreSQL client-reader example inherited its parent environment. It now uses the existing child helper to keep credentials out of psql.

The [0435 family record](0435-sql-call-facts-and-prices.md) supersedes this slice’s open native-format and complete-result adoption gaps. It retains the earlier review and fixture history.
