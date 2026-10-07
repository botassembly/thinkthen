# 0452: SQL inputs and images

Date: 2026-10-06
Status: slice A landed. Complete family adoption is qualified under [0435](0435-sql-call-facts-and-prices.md); final integration gates remain.

DuckDB, SQLite and PostgreSQL now store explicit image values and ordered image collections and run decide/choose/score/details through native image admission. Ordinary scalar, NULL and text behavior remains. One native engine, reader and cache own the work. Complete result/facts and rank-set integration belong to 0435/0417; final parity remains open.

Fresh High review found one defect: constant rich kinds were resolved before NULL evidence was skipped. Resolution now follows row-level NULL checks; independent malformed-kind and unreadable-file regressions passed on both DuckDB versions. The reviewer confirmed ACCEPT. An early fixture made three remote requests with a fake key and received 401; its endpoint/backend configuration was corrected. The reviewer found no remaining remote-routing path in the new consumers.

SQLite gate, PostgreSQL 96 checks and focused loader/image/client-reader cases, both DuckDB versions’ image/cancellation checks, format/policy/ratchets and private-name checks passed. Full tests and lint run on the landing commit with affected SQL checks. Existing unexecuted conformance cases count as not checked, not passes. Later integration updates this same record.

The [0435 family record](0435-sql-call-facts-and-prices.md) supersedes this slice’s open native-format and complete-result adoption gaps. It retains the earlier review and fixture history.
