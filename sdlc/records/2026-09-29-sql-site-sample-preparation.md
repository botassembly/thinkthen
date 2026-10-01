# SQLite site SQL sample preparation at `101a42d3`

This is a two-example read-only check against current `origin/main`, not a site edit or a new SQL API decision. The installed ticket-0259 SQLite library at `databases/sqlite/target/release/libthinkthen0.so` has SHA-256 `f5443f6fdb34ee3a691fdc6f46cdbf171f1d66c61514b7f825874aa24c325bbe`, identical to the library inside the existing 0259 release archive. The host was the pinned SQLite 3.50.0 CLI from `~/.cache/thinkthen-toolchains/sqlite-3500000-host`; its amalgamation hashes passed `databases/sqlite/amalgamation.sha256`. I piped each exact site file to `sqlite3 -batch -bail -cmd '.load …/libthinkthen0.so' :memory:` with one fresh local conformance backend and its loopback-only key. The exact tiny reproduction command is retained locally at `target/codex-builds/qf-sql-site-sample-preparation/reproduce.py`; its `reproduce.jsonl` sibling holds file hashes, exits, streams and send counts. The isolated bad-call log uses the site's setup and failed `SELECT`, so its line number is 8 rather than the full file's 16. There was no package rebuild, provider call, site mutation, or broader sample run.

| Exact current site file | Observed installed-host result | Source boundary |
| --- | --- | --- |
| [`recognize/sqlite.sql`](../../site/examples/functions/recognize/sqlite.sql) | Exit **1**. Standard output `1\|Chicago.\|person\n` came from its first recognize query. Standard error was `Parse error near line 16: no such table: thinkthen_relations\n`. The complete file sent **2** requests before the second query failed. The second query alone, after creating its one-row `tickets` table, exited **1** with the same missing-table diagnosis and **0** sends. | SQLite registers `thinkthen_recognize` and `thinkthen_relate`, but no `thinkthen_relations` table function (`databases/sqlite/src/ffi.rs:127-130`). Its supported one-text relation form is `thinkthen_recognize_document(text, spec)`, returning JSON (`src/scalars.rs:381-389`; `src/recognize_document.rs:11-28`; `README.md:47`). The separate `thinkthen_relate` table function reads caller-owned entity rows. |
| [`relate/sqlite.sql`](../../site/examples/functions/relate/sqlite.sql) | Exit **1**, empty standard output, standard error `Runtime error near line 14: thinkthen usage: thinkthen_relate needs one to four rules (19)\n`, and **0** sends. | The file supplies four arguments: `rules`, `id`, `body`, and `either:contradicts`. SQLite reads argument four as the required **kind-column name**, then sees no rule (`databases/sqlite/src/tables.rs:343-370`; `README.md:32,49`). Its `rules` table also lacks a kind column. |

For the recognize page, the smallest correction is to retain its working first `SELECT` and remove the unsupported second `SELECT`; link to the relate page for row relationships. If that page needs a one-text relation result, this complete SQL form uses the supported JSON result instead:

```sql
SELECT t.id,
       json_extract(thinkthen_recognize_document(
         t.body,
         '{"version":1,"recognize":{"kinds":{"person":"A person.","organization":"An organization.","place":"A place."},"relations":[{"name":"associated_with","source":"person","target":"organization"}]}}'
       ), '$.relations') AS relations
FROM tickets AS t;
```

A local one-row check of that exact form exited **0**, returned `1|[]`, and sent **2** requests on the site's present text. An empty relation list is the canned answer for that fixture, so it is not a useful positive relation demonstration without a separately chosen recorded example. Do not add a fake `thinkthen_relations` alias merely to make the slide run.

For the relate page, give the table a real kind column, store a kind such as `policy` on each row, and pass the column name before the rule:

```sql
CREATE TABLE rules(id INTEGER, body TEXT, kind TEXT);
INSERT INTO rules VALUES
  (1, 'Book economy class for flights under six hours.', 'policy'),
  (2, 'Book business class for every flight.', 'policy');
SELECT relation, source, target, probability
FROM thinkthen_relate('rules', 'id', 'body', 'kind', 'either:contradicts');
```

That two-row form exited **0**, returned `contradicts|1|2|0.9`, and sent **1** loopback request. Marketing can retain the site's eight policy sentences by adding `kind` to every inserted row and the fifth function argument. The eight-row output was not run here; it needs its own fixed expected output before publication.

The next site-owned batch is these two SQLite SQL corrections and a focused installed-host replay for their published output and send counts. The **recognize** correction advances criterion 11's named wrong SQLite recognize sample in the [original SQL usability issue](../issues/closed/2026-09-28-sql-interface-usability-before-0-1.md). The **relate** correction belongs to the existing [site-samples issue](../issues/closed/2026-09-25-site-samples-and-pages-after-the-surfaces-land.md)'s unexecuted host/SQL sample work; original criterion 11 does not name it. Both issues remain open. This batch does not complete criterion 11's five pages or six surfaces, make SQLite's per-text relations a row source, or satisfy any remaining common-syntax, frame, Abbey Road, usage, error, or ADR criterion. Marketing owns `site/`; this note changes no issue status, acceptance count, or original criterion.

## Independent preparation review

Fresh independent Sol Medium review accepted corrected candidate `8b41940a`. Its first pass caught the issue attribution: only recognize belongs to SQL usability criterion11; relate belongs to the existing site-samples issue. The correction retains both open outcomes. The reviewer checked the stored reproduction script and log against current site hashes, installed library identity and exact exits and send counts. No website edit, package rebuild or issue closure follows from these notes.
