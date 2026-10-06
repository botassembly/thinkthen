# 0434: Match SQL question inputs and descriptions

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

DuckDB, SQLite and PostgreSQL agree on rich question/option inputs and explicit file references. PostgreSQL’s evidence-file limitation has a reviewed workaround.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 5 and 6.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: DuckDB recognize preserves kind descriptions using the existing rich grammar. SQLite recognize refuses JSON-array-looking kind text before sends instead of treating it as comma labels. PostgreSQL adds an explicit bounded question-file loader using its existing privilege and file-safety rules.
- Proof: Compare named/described kind bytes to the reference; retain valid list names. Invalid/mixed/duplicate descriptions and malformed arrays are Usage with zero sends. Exercise PostgreSQL loader grants, confinement, symlink/hardlink/nonregular refusals, size cap, valid question/set contents and literal-path ambiguity. Test the documented client-reader/table workaround with located rank/find/spans/edges.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0417 owns SQL rank sets; 0407/0413/0414 own new semantic context/options. This ticket owns residual SQL input grammar, loader and documentation, not another judgment engine.

## Design notes

Proposed thinkthen_question_file(path text) returns validated original JSON, preserving member order and duplicate detection; do not convert to jsonb. Revoke PUBLIC by default and use existing privileged loader restrictions. No general PostgreSQL server evidence-file/folder reader in 0.2: client SDK read_files inserts keyed text plus file/line/ordinal columns; SQL judges text, then the client joins/maps retained identities and spans. No evidence metadata enters wire/cache identity. Review this explicit ruling before implementation.
