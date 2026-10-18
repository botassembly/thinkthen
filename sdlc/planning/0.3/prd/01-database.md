---
date: 2026-10-13
status: draft PRD
release: 0.3, designed for 0.4
source: Ian's rulings of 2026-10-13 in [../data.md](../data.md) and [../evals.md](../evals.md); PRD 00 and its questions; PRDs 02, 03 and 05 for their dependencies; Ian's ThinkThen notes (data-model, fixed-and-tunable, decision-knowledge, strategy) and the ThinkThen ideal state; ThinkThen 0.2's `specification/cache.md`, `recording.md`, `settings.md`, `records.md` and `result.md`, ADR 0111, and the store code in `crates/thinkthen/src/engine/store`. The 30 worked questions, a size measurement and sources are in [01-database-questions.md](01-database-questions.md)
---

# PRD 01: The database

0.2 keeps one answer per question in a cache folder, and a refresh replaces it. 0.3 replaces the cache with one database that keeps every answer, right answer and actor, and reuses answers by a rule the caller picks.

## Who and what's in it for them

- **The builder** never pays twice, across the upgrade too. Proof: a 0.2 cache imports, and its runs repeat with zero sends.
- **The builder changing models** decides with one setting whether old answers count. Proof: the same run under `--reuse any-model` sends nothing, and each result names the stored answer it reused and the model that gave it.
- **The data person** queries answers with plain SQL. Proof: documented queries over the views run in CI from sqlite3 and DuckDB.
- **The reviewer** settles a right answer once, and it grades every model and wording. Proof: one query grades five models and two wordings against one key version.
- **The operator** runs several command line jobs and notebooks on one file. Proof: the concurrency experiment below. The 0.5 proxy writes to the same schema.
- **Everyone** can afford to keep everything. Measured: one million stored answers from five models over 200,000 records of 400 bytes took 510 MB, about 330 bytes per answer beyond the text.

## Use cases

U12, U18, U21, U23 to U28, U47 to U53, and the storage half of U30 to U35. PRD 00's questions 15 to 24, 38 and 39 set the ground.

## The design

**One file.** The database is `thinkthen.sqlite` in the platform data folder: `~/.local/share/thinkthen/` on Linux, `~/Library/Application Support/thinkthen/` on macOS and `%LOCALAPPDATA%\thinkthen\` on Windows. `--database PATH`, `THINKTHEN_DATABASE` and the configuration file's `database` point elsewhere. The proxy opens the same default file. The SQL extensions open a database only when one is named, as 0.2 does with caches. `status` prints the path, size and current user.

**Tables, private.**

- What was asked: `question` (id, function, contract), `wording` (question, digest, number), `evidence` (digest, kind, text), `evidence_member` for pairs, sets and candidate lists, and `context` (digest, text).
- What came back: `observation` holds one model's raw probabilities for one wire question: 0.2's exact request key, address, model, answering model, links to wording, evidence and context, tokens, latency, reported cost when known, batch size, time, origin and call. `answer` holds one answer ID handed to a caller: its reading and threshold. `answer_observation` links an answer to every observation it used, in order, because one `rank` answer can rest on several model replies.
- Who and how: `user` (name, person or agent) and `call` (function, surface, tool and step, user, agent, proxy rule version). `call_answer` links each call to the answers it returned, so a repeated call that reuses an answer is still recorded.
- Knowledge: `right_answer`, `answer_key`, `answer_key_version`, `answer_key_member`, `outcome`, `eval` and `eval_answer`, used in 0.4 and created in 0.3 so 0.4 needs no reshaping. PRD 02 owns their columns. The schema experiment added three things they need: a right answer may replace several earlier ones (`right_answer_replaces`), a search question's right answer records the search terms it was judged against (`query_digest`), and a pair asked in both orders shares one right answer (`truth_digest` on evidence).

Content rows key on SHA-256 digests. Observations and answers keep 0.2's IDs. Other rows key on time-ordered UUIDs, so two databases merge without collisions.

**Published views.** `answers`, `right_answers`, `answer_keys`, `calls`, `outcomes` and `users`, plus the views the schema experiment showed notebooks need: `observations`, `answer_observations`, `call_answers`, `contexts`, `evidence_items`, `evidence_members`, `evals`, `eval_answers` and `picks`. Each view lists its columns by name and documents its row grain; the `answers` view has one row per answer and observation pair, so counts and money use distinct IDs. Writes go through ThinkThen. Recommended: the views ship marked preview in 0.3 and become a stable promise in 0.4, once curation and evals have used them.

**Questions and versions.** A question has a stable identifier and numbered versions of its wording. Looking up stored answers by question may name a version or leave it out. Without one, the lookup uses the latest version (Ian, 2026-10-14). Reuse still keys on the exact text sent.

**Reuse rules.** A rule names which optional parts must match. Wording text, evidence and context always match. The default rule, `exact`, is 0.2's key: address, model, answering model and the exact bytes sent. `--reuse` loosens it with `any-address`, `any-model-version`, `any-model` and `any-wording-version`, joined by `+`. `--reuse fresh` asks again and keeps both answers. `--no-store` asks and keeps nothing. Two indexes serve every rule: one on the exact key, one on evidence, wording and context. A reuse under a loose rule writes no observation, so model B never appears to have said what model A said.

**Newest.** Among accepted observations, the latest answering time wins, and insertion order breaks ties. A provider's no-store reply leaves a marker that blocks older answers. Within one call, a key keeps its first resolution.

**Nothing overwritten, nothing removed.** Refresh and a second settle add rows. A corrected right answer names the ones it replaces. The database keeps everything, including every repeated call (Ian, 2026-10-14). 0.3 has no removal command; clearing old answers is later work. `status` shows the database size.

**Users.** One `default` user exists from the start. `thinkthen login NAME` creates a user and makes it current, with no password. `THINKTHEN_AGENT` names an agent. Every write records the user, and an agent's write also records the agent, as W3C PROV's "acted on behalf of". A proxy records the names its callers claim.

**From 0.2.** The first open imports the 0.2 default cache once and leaves the old file in place. `admin database import PATH` brings in other folders, fixtures and exports, and repeats change nothing. Imported rows serve `exact` reuse at once. A 0.3 call that reuses one fills in its wording, evidence and context links, so old answers join curation as they are used. `--record` and `--replay` keep 0.2's fixture folders and `thinkthen.jsonl`. Replay never opens the database. `answers export` writes JSONL, and `--fixture DIR` writes a replayable folder.

**Splitting a question into parts.** Wording, evidence and context get their own digests where the typed inputs supply them, so curation can group answers by wording. The exact key stays the identity for reuse. The split experiment showed why: `recognize` and `relate` questions depend on earlier steps (prior spans, entity order, candidate menus), so the three parts alone cannot rebuild the question sent. The database stores the renderer version and those ordered dependencies. A part that cannot be named before sending stays empty, and that record serves only exact reuse. ThinkThen never guesses parts by parsing the text.

**Concurrency.** Write-ahead log, full sync, one transaction per reply, 0.2's 30-second wait limit, local disks only. The experiment found no lost or damaged answer with twelve writers and three readers, and readers never blocked writers. Under twelve writers running nonstop on a busy machine, one write in a hundred waited more than a second, almost all of it waiting for the write lock. 0.3 promises no write time until the product's own SQLite build is measured on a quiet machine. A reader that holds one snapshot open lets the write-ahead log grow: 2.2 GB beside a 20 MB database in the experiment. Notebook recipes close their reads, and `status` warns when the log is large. The 0.5 proxy groups its callers' writes.

**Postgres later.** SQLite strict typing, types both engines share, JSON read with `->>`, and one migration script per schema version, run on PostgreSQL in CI from the start. The schema experiment ran 39 queries, covering every 0.4 use, with the same answers on both engines. It found three rules the migrations follow: views name their columns, because PostgreSQL fixes a `SELECT *` view's columns when it is created and SQLite does not; queries normalize JSON booleans, which SQLite returns as 0 and 1 and PostgreSQL as text; and numbers are cast to NUMERIC before rounding. A later PostgreSQL store writes from its own connection, so a rolled-back query never loses a paid answer.

## What must be true

| Risk | What retires it |
| --- | --- |
| Many writers lose answers or time out | Experiment run: zero lost rows; the slowest waits exceed a second under heavy load. Remeasure with the product's SQLite on a quiet machine |
| Loose rules slow lookups | Lookup timing under every rule at ten million observations |
| Import loses or changes 0.2 answers | Every repo fixture and one real cache imported; exact reruns send nothing |
| The engine cannot split wording, evidence and context for every function | Experiment run: all ten functions split, with stored dependencies for `recognize` and `relate`; an unsplit part stays empty and serves only `exact` |
| The schema cannot serve 0.4 | Experiment run: 39 queries over every eval use pass on SQLite and PostgreSQL with the additions above. The checked-in schema test repeats them |
| Images swell the file | Size measured on the 0036 image runs |
| PostgreSQL forces a redesign | The CI run above |

## Out of 0.3

PostgreSQL as the store. Sign-in and checked identity. Removing answers, retention policies and automatic eviction. Answering from right answers. Reuse for similar evidence. Administration over the network. Merging two users.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 3 and 5. Answered by Ian on 2026-10-14: reuse keys on the exact text, and a lookup by question uses the latest version unless one is named; the database keeps everything, including repeated calls, and clearing comes later.
