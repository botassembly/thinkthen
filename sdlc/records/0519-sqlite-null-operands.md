# Consistent SQL values and function descriptions

DuckDB, SQLite and PostgreSQL now short-circuit required NULL operands before inspecting partners, reading files or sending requests. Optional NULL settings use defaults. Complete calls accept native BLOB or bytea images, DuckDB exposes native JSON values, and every registered public function has an in-database description. Supported names and input forms remain available; backend failures remain failures.

The implementation slices have fresh accepted code reviews recorded in ticket 0519. Review findings corrected SQLite's usage description and DuckDB's required-file NULL handling. Final qualification adds no product code or separate verification machinery.

Current installed DuckDB and PostgreSQL packages were rebuilt from `41d983f79418711ad5f025ef8bd90c1574e6ea7e`. Eight focused DuckDB cases and three PostgreSQL groups pass, covering required and optional NULLs, native binary complete images, strict replay, backend failures, registration descriptions and JSON operations. SQLite's closure checks cover the same outcomes through its installed extension, including all ten complete calls. Root routine and lint evidence is reused where the relevant code is unchanged; release-only and platform qualification remain candidate work.

The DuckDB extension SHA-256 is `6f406e9ad61caefe67ffb1d02b4b4b6d35e93de71ecefb047cb35895bab929cd`. PostgreSQL's installed library is `47c81bab14e838b48fc419cf045b788fb4978b29d944fc00d1b23ebf5d6dd384`; its generated SQL is `0d213f70ec3c315569c9e601ba6833d9157e6886b128cdf7ab341e9428f68755`. SQLite's installed library is `fcf8a53560839788f47504e073c9d8e4a65cd6d00d1f6dfbcc845d5639223e93`. Qualification logs are `target/0519-close/{duck-build,pg-build,duck-focused,pg-focused}.log` in the qualification lane. These artifacts establish local installed behavior, not publishing readiness.

A documentation-only correction also gives the Python README's refund result a purpose-specific name, resolving the existing named-answer lint failure. The existing checker and its self-test pass.

## What the build taught us

NULL admission must precede parsing other arguments. Native registration metadata prevents a separate function catalog from drifting. Binary images and actual JSON types must be checked through the installed database, because source builds alone cannot establish their SQL representation. The final check reused each database's existing consumer tests instead of adding another proof layer.
