---
date: 2026-10-13
status: draft PRD
source: Ian's rulings of 2026-10-13 in [../data.md](../data.md) and [../evals.md](../evals.md); [00-use-cases-and-surfaces.md](00-use-cases-and-surfaces.md) and its questions; PRDs 02, 03 and 05; Ian's ThinkThen notes (data-model, fixed-and-tunable, decision-knowledge, strategy, later/enterprise) and his ideal state for ThinkThen; ThinkThen 0.2's `specification/cache.md`, `recording.md`, `settings.md`, `records.md`, `result.md` and `audit.md`; ADR 0033 and ADR 0111 with its amendments; `crates/thinkthen/src/engine/store.rs` and `store/migration.rs`; `thinkthen help cache`
---

# PRD 01 working file: the database's hard questions

This file backs [01-database.md](01-database.md). Each question gives the options, the reasoning and a recommendation. **Ian** marks the two that need his direction. The rest are recommendations the PM can carry into tickets, and Ian can overturn any of them. Claims about 0.2 cite the page in [specification/](../../../../specification/) or the code.

## What 0.2 actually stores

The facts every answer below leans on:

- **One table of answers, one row per key.** `answers.key` is `UNIQUE`. The schema is at `user_version` 2 with columns `url`, `model`, `state`, `question`, `answer`, `answered_by`, token shares, `taken_at` in Unix seconds, `origin`, `key_version`, `adapter`, `observation_id` and `batch_size` (`crates/thinkthen/src/engine/store.rs`). States are stored once in a `states` table.
- **The v2 key** hashes the adapter, the canonical address, the requested model, the answering model, the state and the wire question as sent ([cache.md](../../../../specification/cache.md)). The v1 key hashed the adapter, the stored URL, the requested model, the state and the question, joined by line feeds. Migration validates v1 before rekeying.
- **Evidence and wording are fused in the bytes.** A record run quotes each record inside its question: `The text is `, the record's compact JSON, `. `, then the question. Shared context travels as the state ([records.md](../../../../specification/records.md)). So a 0.2 row has no separate wording, evidence or context.
- **A refresh replaces.** `--refresh-cache`, `--record` and the `jev-latest` model replace the held answer. A provider's `Cache-Control: no-store` on a refresh evicts the prior answers ([cache.md](../../../../specification/cache.md), "Response Cache-Control").
- **Where it lives.** The default cache is `$XDG_CACHE_HOME/thinkthen` or `~/.cache/thinkthen` on Linux and `~/Library/Caches/thinkthen` on macOS. The usage counts already moved to the state folder, because "the system may purge `Library/Caches`" ([recording.md](../../../../specification/recording.md)).
- **Locks and privacy.** Rollback journal, `synchronous=FULL`, `secure_delete=ON`, `BEGIN IMMEDIATE` for writes, a 30-second busy limit, file mode `0600`, folder `0700`, no network file systems (ADR 0111 section 3).
- **Growth.** Nothing trims the cache on its own. `cache_bytes` defaults to 100,000,000 and is only the target `cache prune` trims to ([recording.md](../../../../specification/recording.md)).
- **Two IDs already exist.** An `ObservationId` names one raw answer from a model. An `AnswerId` hashes the observations, the scope and the reading, so the same reading over the same observations restores the same ID ([result.md](../../../../specification/result.md)).
- **SQL.** The extensions never use the platform cache. They cache only in a folder `THINKTHEN_CACHE` or their own setting names, and they refuse a folder another user owns or can write. The SQLite extension runs the store on the host's SQLite, version 3.50.0 or later (ADR 0111, 2026-09-30 amendment).

## A size measurement

On 2026-10-13 a scratch script built the draft `evidence`, `request`, `observation` and `answer` tables in SQLite with the write-ahead log. It loaded 200,000 records of 400 bytes, each answered by five models: one million observations with random 32-byte IDs and two indexes. The file was 510 MB. With one answer row per observation it was 662 MB. Text accounted for about 176 MB, because each record sat once as evidence and once inside its wire question. That leaves about 330 bytes per observation and about 150 more per stored answer. Images, long documents and `jev-latest` refreshes are not in this number. The script was deleted afterwards.

## Location and lifetime

**Q1. Where does the database file live?**
Options: the platform cache folder, as 0.2; the state folder, beside usage; the data folder; a file in each project. PRD 00's question 22 found the cache folder unsafe: Apple's file system guide says the system may delete `Library/Caches` contents to free space. Right answers and answer keys are owned work, and a purge would lose them for good. The XDG specification gives `XDG_DATA_HOME` to user data files and `XDG_STATE_HOME` to state that "is not important or portable enough" for data. Answers are both important and portable. Recommendation: the data folder. Linux uses `$XDG_DATA_HOME/thinkthen/thinkthen.sqlite`, then `~/.local/share/thinkthen/`. macOS uses `~/Library/Application Support/thinkthen/`. Windows uses `%LOCALAPPDATA%\thinkthen\`. An XDG home counts only when absolute, as 0.2 rules for its paths.

**Q2. One database or one per project?**
0.2 users made per-project caches with `--cache DIR`. Ian ruled one database by default. Separate files still serve isolation: a client's data apart from another's, a throwaway experiment, a test. Recommendation: one default file. `--database PATH`, `THINKTHEN_DATABASE` and the configuration file's `database` name another. The value is a file path. A folder is refused with a sentence that names `admin database import`, because 0.2 folders and fixture folders look alike. The configuration file may name the path. It already decides where the key and the evidence go, so naming the database adds no new trust, and the 0.2 ownership warning covers it. This changes 0.2's rule that the configuration file never names a cache folder ([settings.md](../../../../specification/settings.md)), as PRD 00's question 22 already proposed.

**Q3. Does the proxy share the command line's file?**
Ian ruled that the SDK and the proxy each connect to one database by default. On one machine under one account, the same default path gives one store, which is what a single builder wants. PRD 00's principle 6 still holds: a caller pointed at the proxy uses the proxy's store, and the engine turns local reuse off. Recommendation: the same default path. `thinkthen proxy --database PATH` separates them. The two then share a file safely only on a local disk (Q19).

## Tables and keys

**Q4. What is a stored "answer"?**
`data.md` drafts an `answer` table of raw probabilities. 0.2 already uses "answer" for something else: the `AnswerId` names a reading, the value a caller received after a threshold. Outcomes attach to that ID, because the proxy "accepts what happened downstream" of the answer the code acted on ([proxy.md](../proxy.md)). One word for two things would break PRD 00's principle 2. Recommendation: two tables. `observation` holds one model's raw probabilities for one wire question, keyed by 0.2's `ObservationId`. `answer` holds one answer ID handed back, with its reading, threshold, call and the observations behind it. The public `answers` view joins them, so a person still sees "answers". The draft's "answer" row becomes "observation" inside the tables only.

**Q5. Which parts get their own table, and what keys them?**
`data-model.md` names ten parts. The tables need the ones a key or a join uses: question, wording, evidence, context, observation, answer, call, user, right answer, answer key with its versions and members, outcome and eval. A setup (wording, context recipe, model and reading) needs no table in 0.3. It is a combination of columns, and an eval record lists the setups it compared. Keys have to survive a merge, because U53 moves answers from a laptop into a proxy. Local integer IDs collide on merge. Recommendation: content rows (question contract, wording, evidence, context, request bytes) key on SHA-256 digests. Observations and answers keep 0.2's 64-character IDs. Every other row keys on a UUID version 7 from RFC 9562, which sorts by creation time and needs no coordination. Integer row IDs may serve joins inside one file and never appear in a view or an export.

**Q6. How does a wording number survive two databases?**
Ian ruled numbered wordings for tracing. A database that numbers wordings as it first sees them gives the laptop's version 3 and the proxy's version 3 different texts, and a merge then lies. Recommendation: the wording's identity is the digest of its text, option descriptions and order, as PRD 00's question 15 says. The number is the author's, from the tool file, when given. Otherwise the database assigns a local display number that never leaves the file. Exports and views carry the digest, and the number only as a label.

**Q7. What makes two pieces of evidence the same?**
`decision-knowledge.md` warns that a raw hash is too strict and a vector too loose. A canonical form per evidence type sounds better, but a wrong merge is the worst failure (`data-model.md`): a right answer would grade the wrong text. 0.2 keys on the selected bytes as encoded, and its result spec keeps file paths and line numbers out of every identity. Recommendation: the evidence digest covers the exact selected text or image bytes as sent. The source location rides on the answer, never on the evidence. Whitespace variants stay distinct. A canonical form waits for a measured need and a per-type rule.

**Q8. How do pairwise comparisons, `find` sets and candidate lists fit?**
PRD 03 makes a comparison a `find` of two and stores each pair answer "its evidence the ordered set of two items (PRD 01)". `fixed-and-tunable.md` makes the whole set the item for `find`, and the candidate list part of the item for `choose` with options per item. Position bias is real: Zheng and others found that model judges favor one position, and Qin and others ask both orders for that reason. Recommendation: one mechanism. An `evidence` row has a kind: single, pair, set or candidates. A composite row's content is the ordered list of its members' digests, kept in `evidence_member`. A-then-B and B-then-A are two pieces of evidence, so each order gets its own observation. A right answer about a pair keys on the pair with its members sorted by digest, and its value names the winning member's digest, so one right answer grades both orders. PRD 02 keeps pairwise right answers out of 0.3. The table holds them without change when they arrive.

## Reuse rules

**Q9. Can a stored answer be reused across wordings? (Ian, with Q10)**
PRD 00's question 19 found a conflict: `data.md` says wording text always matches and also lists the wording version as optional. It recommended dropping the version as an option. Ian's own answer on 2026-10-13 says "a caching rule can include or exclude the version", so dropping it overrules a ruling. Both statements hold once text and number are kept apart. The text always matches: reuse across different texts would score a new GEPA wording with old answers. The number is a label. Including it means version 7 never reuses version 6's answer even when their texts are identical, which matters to someone tracing an optimizer's runs. Excluding it lets two tools, or two numbers of one tool, share answers for identical text. Recommendation: wording text always matches. `any-wording-version` is one of four optional parts. PRD 00's question 19 changes to match.

**Q10. What is the default rule? (Ian)**
Ian ruled least surprise: everything must match, as 0.2. 0.2's v2 key holds the adapter, the address, the requested model, the answering model and the exact bytes of state and question. It holds no question ID and no wording number; a question file's `name` and `wording_version` "enter no identity" ([question-file.md](../../../../specification/question-file.md)). "Everything" could also mean adding the question ID and number to 0.2's key. That stricter default would make two tools that send identical text pay twice, which 0.2 never did. Recommendation: the default rule, `exact`, is 0.2's key and nothing more. It needs Ian because it reads his words narrowly.

**Q11. How does a rule become a lookup, and which indexes serve every rule?**
Four optional parts give sixteen rules. Ian ruled that the indexes must support every key the rules allow. An index per rule is waste. The set of observations for one wording, evidence and context is tiny: one per model and refresh. Recommendation: two indexes. `exact` looks up the 0.2 key, which stays a column. Every loose rule looks up evidence, wording digest and context, then filters address, model, answering model and wording number in memory over a handful of rows, newest first. A loose rule needs the split parts, so rows imported from 0.2 without them serve only `exact` until Q22's backfill. Loose rules also work across adapters, because they compare the split parts.

**Q12. What does a reuse under a loose rule write?**
If a run on model B reused model A's answer and stored it as B's, every later benchmark would credit B with A's work. Recommendation: a loose reuse writes no observation. The answer row records the observation it used and the rule. Each result names the observation, the model that answered and the rule. The threshold that reads it is the cut for the answering model, when a tool states cuts per model (PRD 00's question 36), because a threshold belongs to the model that produced the probability.

**Q13. Where do reuse rules live, and how are they spelled?**
PRD 00's question 18 sets the precedence: typed per call, the environment, the tool file, the configuration file, then the built-in `exact`. 0.2 has three switches: `--cache DIR`, `--refresh-cache` and `--no-cache`. Ian ruled the cache a database with caching rules, so a flag that names a folder and a flag that names a rule should not share a word. Recommendation: `--database PATH` says where. `--reuse RULE` says what counts, with `exact`, `fresh` or optional parts joined by `+`, such as `any-model+any-address`. `fresh` replaces `--refresh-cache`: it asks again and keeps both answers. `--no-store` replaces `--no-cache`: it asks and keeps nothing, the word HTTP uses for the same promise. Libraries take `database=` and `reuse=`, and the environment takes `THINKTHEN_DATABASE` and `THINKTHEN_REUSE`. `eval bench` reuses only `exact` matches, because a benchmark measures the setup it names, and PRD 02's `--fresh` asks again.

## History and newest

**Q14. What does "the newest answer that the rule accepts" mean under concurrency?**
Three clocks compete: when the model answered, when this file stored the row, and when the reading ran. Insertion order alone breaks on import: a laptop's month-old answers merged into a proxy today would become newest. Answering time alone ties often at 0.2's one-second resolution. Two processes that miss together both send, and 0.2 already accepts that ("Across processes there is no coalescing", ADR 0111). A long call could also see a newer answer arrive halfway and answer the same key two ways. Recommendation: newest means the latest answering time, in milliseconds, with local insertion order breaking ties. Imported rows keep their answering times; 0.2 seconds become milliseconds. A call resolves each key once and holds that resolution for the rest of the call, as 0.2's in-call coalescing does. The proxy coalesces across its callers. The result names the observation, so two runs that read different newest answers can tell why.

**Q15. How do refresh, `jev-latest` and a provider's no-store work when nothing is overwritten?**
Refresh and `jev-latest` become simple: each adds an observation that is now newest. No-store is harder. 0.2 evicts the old answers on a no-store refresh, so a stale answer is never served after the provider refused storage. Keeping the old rows and serving them breaks that promise. Deleting them breaks Ian's rule. Recommendation: a no-store reply writes a marker observation with its time, model and usage and no probabilities. A marker that is newest blocks reuse of older rows under every rule, so the next call asks again. History keeps the older rows. `--no-store` writes nothing at all.

**Q16. Does every answer handed back get a row?**
Outcomes need the answer ID to resolve to its reading and observations, and the ID is a hash. A row per answer makes every call write, even a full reuse, and a hot loop writes all day. The answer ID is deterministic, so a repeat with the same reading produces the same ID. Recommendation: insert answer rows only when the ID is new. A call row is written only when the call writes something new. A pure repeat writes nothing, as in 0.2, and the 0.2 usage counters keep counting reuse. The measurement above puts an answer row at about 150 bytes.

## Growth and privacy

**Q17. How big does the file get with no eviction? (Ian, as part of question 2)**
Ian ruled that nothing is thrown away. His ideal state still says answers are saved "under a size limit", and 0.2 keeps a 100 MB prune target. The measured cost is about 330 bytes per observation plus the text, which suits a builder: ten million observations come to about 3.3 GB before text. Three things grow faster. Images store their bytes. `jev-latest` adds a row on every call. A long document stored as evidence costs its full size once. SQLite's file limit is far beyond any of this. Recommendation: no automatic eviction and no size target. `status` shows the size and the count by evidence kind. `admin prune` removes only by named selectors: age, answering model and evidence kind. It previews unless told `--apply`, which reverses 0.2's default, because the file now holds owned work. The ideal-state line needs Ian's update.

**Q18. The store holds the judged text. What protects it?**
Every evidence row may hold a patient note or a contract. 0.2 already keeps the text, makes the file owner-only and zeroes deleted rows. Ian ruled on 2026-10-06 that the hosted service holds evidence text with no fingerprint-only mode, and that customer data is never pooled. Locally, reuse needs only digests: the exact key and the loose lookup both compare hashes. Curation, export and fine-tuning need the text. Recommendation: keep the text by default, as 0.2 does. Keep mode `0600`, folder `0700` and secure delete. `admin prune --drop-text` removes the text of selected evidence and keeps its digest, so answers and stats survive and reuse still works, while export and curation skip that evidence. Prune refuses to drop text a right answer needs unless told. `--no-store` keeps nothing for a sensitive call. The docs say plainly that the database holds what was judged.

## Concurrency

**Q19. How do several processes share one SQLite file?**
SQLite allows one writer at a time. The rollback journal 0.2 uses blocks readers during a write. The write-ahead log lets readers and one writer run together, and SQLite's documentation lists its costs: two side files, no network file systems, and a read-only open that needs the shared-memory file. 0.2 chose the rollback journal so a read-only replay writes nothing. In 0.3, replay reads fixture folders and never opens the database (Q23), so that reason is gone. The proxy reads constantly while command lines write, which favors the log. Recommendation: the write-ahead log with `synchronous=FULL`, `BEGIN IMMEDIATE` for writes, one write transaction per reply, 0.2's 30-second busy limit with cancellation, and a refusal on network file systems. The proxy groups its callers' writes into shared transactions. The prototype in the PRD measures lost rows and waits with the proxy, eight shells and four PostgreSQL backends on one file. PRD 05 depends on the same result.

**Q20. How do the SQL extensions reach the database?**
In PostgreSQL every role runs as the server's one operating-system user, so every role reaching a named file shares its answers and bypasses row-level security ([recording.md](../../../../specification/recording.md), ticket 0318). Each PostgreSQL connection is its own process, so a pool of fifty is fifty writers. The SQLite extension runs on the host's SQLite. Version 3.50.0, its floor, has the write-ahead log, strict tables and `->>`. Recommendation: 0.2's rule carries over. The extensions write to a database only when `THINKTHEN_DATABASE` or their own setting names one, and they refuse a file another user owns or others can write. Reading needs no ThinkThen code: SQLite reads the views after `ATTACH`, and DuckDB reads them through its `sqlite` extension with `TYPE sqlite`. PostgreSQL reads through `answers export` in 0.3, and through its own tables once PostgreSQL is a store.

**Q21. How do the 24 surfaces reach the database?**
Every library already calls the Rust core, which owns the store. 0.2 spells the cache setting differently per language: a folder or `True` in Python, a folder or `false` in TypeScript and C ([settings.md](../../../../specification/settings.md)). Recommendation: the store stays in the Rust core, and no binding touches SQLite. Every surface gains the same two settings, `database` and `reuse`, in its idiom, replacing `cache`. The data jobs themselves reach Rust, Python, R and TypeScript only, per PRD 00's question 10. A sandboxed host, such as a phone app, passes a path, because the platform data folder may not exist there.

## Migration, fixtures and tests

**Q22. How do 0.2 caches come in?**
A 0.2 row holds the wire bytes and no split parts. Splitting `The text is …. question` back apart is guesswork: [recording.md](../../../../specification/recording.md) itself lists cases where quoted text and a question in the user's words cannot be told apart. Leaving the default cache behind makes every upgrading user pay again, silently. Recommendation: the first open of a new default database imports the 0.2 default cache once, prints one line with the count, records the import, and leaves the old file in place. `admin database import PATH` brings in a `thinkthen.sqlite`, a `thinkthen.jsonl` fixture, a folder of either, or an `answers export`. It validates v1 rows before rekeying, as cache.md requires, keeps each observation ID and origin, and ignores rows it already holds. Imported rows have empty wording, evidence and context links and serve `exact` at once. When a 0.3 call reuses one, the engine knows the split parts and fills the empty links. Filling an empty link overwrites nothing. A set `THINKTHEN_CACHE` draws a one-line notice naming `THINKTHEN_DATABASE` and the import command.

**Q23. Do fixture folders and offline replay survive?**
Every library's tests, the demos and user projects replay committed `thinkthen.jsonl` folders with no network. The database is private, grows and holds history. PRD 00's question 23 keeps them apart. Recommendation: `--record DIR` and `--replay DIR` keep 0.2's folders, `thinkthen.jsonl` shape and exit codes. Replay never opens or creates the database. Record writes the folder only. `answers export --fixture DIR` writes a replayable folder from the database, which replaces `cache convert`. Test suites that call live code set `THINKTHEN_DATABASE` to a scratch path or pass `--no-store`.

**Q24. How does the schema change between releases?**
0.2 adds no backup and supports no downgrade; it says to keep an unmigrated copy. That was fair for a cache. It is not fair for right answers. Recommendation: the schema version lives in `PRAGMA user_version`. An older ThinkThen refuses a newer file and names the version it needs. Before a migration, ThinkThen writes a copy with SQLite's `VACUUM INTO`, which produces a consistent copy while others read, and keeps the newest copy. `admin database backup PATH` uses the same call.

## People

**Q25. How do users, login and agents work with no sign-in?**
Ian ruled one default user, a command that creates a user record and makes it current with no password, and agents that record themselves and the person they act for. Where "current" lives matters: inside the database, two people sharing a proxy would switch each other. The 0.2 configuration file is read-only to the tool. Recommendation: the `user` table starts with one row named `default`. `thinkthen login NAME` creates the user if missing and writes the name to a small file in the state folder beside the usage counts. `THINKTHEN_USER` overrides it for one process. `THINKTHEN_AGENT` names the agent, as PRD 00's question 39 says, and an agent is a `user` row of kind `agent`. Every write records the user and, for an agent, the agent too. This is W3C PROV's `actedOnBehalfOf`. Names are the identity in 0.3, so the same name on a laptop and a proxy is the same person after a merge. Through a proxy the client sends both names, and the proxy records the claim, which `status` and the docs call a claim.

## Evals and knowledge

**Q26. Do the tables answer every use in `evals.md`?**
Ian ruled that evals shape the database. Each row of the uses table, checked:

| Use | How the tables answer it |
| --- | --- |
| Save money | `observation` under a reuse rule |
| Pick a threshold | Raw probabilities in `observation`, joined to `right_answer` on question and evidence |
| Compare two runs | `answer` rows by call or by setup columns |
| Benchmark | `answer_key_member` to `right_answer`, `eval_answer` pins the observations, tokens and latency on each |
| Curate disagreements | Observations sharing question and evidence under two setups |
| Optimize the wording | Several `wording` rows under one question, each graded against the same right answers |
| Order quality | Right answers over a set evidence, with a grade per member (PRD 02) |
| Pairwise | Pair evidence, Q8 |
| Watch a model over time | Every observation kept with its answering model and time |
| Real outcomes | `outcome` keyed by answer ID |
| Fine-tune | Right answers joined to wording and evidence text, exported as JSONL |

Two gaps close only by rule. Drift needs history, so `jev-latest` rows stay. Fine-tuning needs text, so `--drop-text` evidence is skipped from training exports, and the export says how many.

**Q27. Is there a run table?**
`data.md` drafts one. PRD 00's question 2 removes it: a benchmark is an eval record, a disagreement needs no group, and provenance belongs to the call. PRD 02 makes an eval pin the answers it graded. Recommendation: no run table. `call` records who and how. `eval` and `eval_answer` record what was measured and against which key version.

**Q28. How do right answers and answer keys stay frozen?**
`fixed-and-tunable.md` says a check never changes; a correction is a new check that replaces it. Ian ruled frozen, numbered sets with a separate membership table. Recommendation: a `right_answer` row is never updated. A correction is a new row that names the row it replaces, and PRD 02's disputed state comes from the rows themselves. An `answer_key_version` row is never updated, and each `answer_key_member` row pins one `right_answer` row. Adding or correcting a member makes a new version. An eval pins its key version, so a reported number never moves.

## Published views and PostgreSQL

**Q29. What do the published views promise?**
PRD 00 asks Ian whether views are public and tables private. The cost of a public layout is a migration for every user's queries on every change. Recommendation, if Ian agrees there: `answers`, `right_answers`, `answer_keys`, `calls`, `outcomes` and `users` are the contract, with global IDs as text and times in ISO 8601. A view never loses or changes a column within a major version. A breaking change adds a new view, such as `answers_2`, and keeps the old one for a release. Tables carry no promise.

**Q30. How does PostgreSQL come later without a redesign?**
`decision-knowledge.md` warns that the open and hosted stores drift unless core tables stay portable. The traps are SQLite's loose typing, its rowid habits, JSON functions that differ, and time stored as integers in one engine and timestamps in the other. A deeper trap sits in the extension. A PostgreSQL function runs inside the caller's transaction, so answers written through SPI vanish when the caller rolls back, and a read-only replica refuses the write. Recommendation: declare every table `STRICT`, so SQLite refuses a wrong type as PostgreSQL does. Use integers, real numbers, text and blobs only. Keep JSON in text and read it with `->` and `->>`, which both engines accept for simple paths. Store times as integer milliseconds and expose ISO text in views. Keep UUIDs as 16-byte blobs that map to PostgreSQL's `uuid`. Write one numbered migration script per version and run the migrations and views on PostgreSQL in CI from the first ticket. A tenant is a file in SQLite and a schema in PostgreSQL, matching the "nothing is pooled" ruling. When PostgreSQL becomes a store, the extension writes paid answers from its own connection, outside the caller's transaction.

## Contradictions found

- PRD 00's question 19 drops the wording version as an optional part. Ian's 2026-10-13 answer keeps it. Q9 reconciles them.
- Ian's ideal state for ThinkThen saves answers "under a size limit". `data.md` says nothing is thrown away. Q17 asks Ian.
- `data.md` drafts `answer` as raw probabilities. 0.2's `AnswerId` names a reading. Q4 renames the raw row.
- `data.md` drafts a run table. PRD 00 removes it. Q27 follows PRD 00.
- 0.2 evicts answers on a no-store refresh. "Nothing is overwritten" forbids eviction. Q15 uses a marker.
- 0.2's configuration file never names a cache folder. PRD 00's question 22 lets it name the database. Q2 accepts the change and gives the reason.

## Sources

- SQLite, "Write-Ahead Logging": https://www.sqlite.org/wal.html
- SQLite, "File Locking And Concurrency in SQLite Version 3": https://www.sqlite.org/lockingv3.html
- SQLite, "SQLite Over a Network, Caveats and Considerations": https://www.sqlite.org/useovernet.html
- SQLite, "STRICT Tables": https://www.sqlite.org/stricttables.html
- SQLite, JSON functions and the `->` and `->>` operators: https://www.sqlite.org/json1.html
- SQLite, `VACUUM INTO`: https://www.sqlite.org/lang_vacuum.html
- SQLite, `PRAGMA secure_delete`: https://www.sqlite.org/pragma.html#pragma_secure_delete
- PostgreSQL, JSON operators: https://www.postgresql.org/docs/current/functions-json.html
- PostgreSQL, Server Programming Interface and transactions: https://www.postgresql.org/docs/current/spi.html
- DuckDB, SQLite extension: https://duckdb.org/docs/extensions/sqlite
- XDG Base Directory Specification: https://specifications.freedesktop.org/basedir-spec/latest/
- Apple, File System Programming Guide, the Library directory and Caches: https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html
- RFC 9562, UUID version 7: https://www.rfc-editor.org/rfc/rfc9562
- RFC 9111, HTTP caching and `no-store`: https://www.rfc-editor.org/rfc/rfc9111
- W3C, PROV-O, `actedOnBehalfOf`: https://www.w3.org/TR/prov-o/
- Zheng and others, "Judging LLM-as-a-Judge with MT-Bench and Chatbot Arena", 2023, on position bias: https://arxiv.org/abs/2306.05685
- Qin and others, "Large Language Models are Effective Text Rankers with Pairwise Ranking Prompting", 2023: https://arxiv.org/abs/2306.17563
