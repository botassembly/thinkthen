# 0434: Match SQL question inputs and descriptions

Status: COMPLETE. All 248 required cases passed through each source and installed SQL extension. Reviewed fixes and final functional, lint and specification checks passed.

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

## Family build contract (2026-10-06)

Risk: High. The privileged PostgreSQL loader must retain descriptor-based
regular-file/size/confinement/link checks and the extension-wide PUBLIC revoke;
errors must not expose file contents or credentials. Ian can overturn these
additive SQL spellings. This is one family build with 0452, awaiting one fresh
whole review; it is not a landing record or a completion claim.

- DuckDB `thinkthen_recognize(input VARCHAR, kinds VARCHAR[] | VARCHAR,
  settings VARCHAR := NULL)` retains the name list and adds existing native
  recognize JSON/question-file grammar, including described kinds. NULL input
  or kinds returns NULL; invalid grammar refuses before transport.
- SQLite retains its recognize signatures and comma-name grammar. A kinds
  argument starting with `[` after whitespace is Usage, including malformed
  arrays; it never becomes a label or sends a request.
- PostgreSQL `thinkthen_question_file(path text) RETURNS text` reads the
  explicitly named path (no `@` expansion), validates a question or question
  set with existing native parsers, and returns the authored bytes unchanged.
  NULL returns NULL; empty/invalid paths are Usage, unreadable/invalid files
  are Local. Existing 1 MiB, role and administrator confinement rules apply.
  No server evidence/image reader is added.

0456's accepted shared design/plan slice A landed at bf373969c; native
implementation is awaited. The loader delegates validation to today's
Question/QuestionSet APIs until that owner supplies the shared loader
validation API for this door. No SQL-local input-declaration parser or
`@NAME` resolution is introduced here.

- Code review: ACCEPT 2026-10-06. Fresh High whole-family review; the sole NULL-before-kind-validation defect was corrected and its resolution confirmed.

Complete family adoption is qualified under 0435. Its [single family record](../records/0435-sql-call-facts-and-prices.md) carries the source and installed results, review corrections and remaining integration gates.

Landed: e056fa21c4ec3ce048a9977385064ca5ad115aee
