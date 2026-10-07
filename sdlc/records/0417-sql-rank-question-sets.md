# 0417: SQL rank question sets

Slice A adds thinkthen_rank_set across DuckDB, SQLite and PostgreSQL using the existing native turns merge. It preserves keys, member names, stable ties, LIMIT behavior and combined count facts. Existing single-question calls remain intact. Complete result metadata, identities and named/declaration adoption remain open with their native and SQL owners.

Fresh whole-family High review accepted0066b0639. Focused public set and legacy cases passed on SQLite, PostgreSQL and both DuckDB versions, including zero-send recording replay from empty caches, admission, cancellation and secrecy. Full tests and lint run on the landing commit.

## What the build taught us

Cache hits do not prove recording replay. Empty-cache tests now establish that individual member recordings supply the set without requests. Later integration updates this same record.

Complete ordered rank-member results, identities and named/declaration adoption are now qualified through all three source and installed SQL consumers. The [0435 family record](0435-sql-call-facts-and-prices.md) owns those results and final integration gates.
