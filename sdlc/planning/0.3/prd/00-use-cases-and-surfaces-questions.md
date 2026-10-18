---
date: 2026-10-13
status: draft PRD
source: Ian's ruling of 2026-10-13 that the command line and the library functions follow use cases; the 0.3 design files in this folder; Ian's ThinkThen notes (data-model, fixed-and-tunable, decision-knowledge, strategy, later/interface, later/enterprise) and his ideal state for ThinkThen; ThinkThen 0.2's `specification/` and `thinkthen --help`; the library export lists under `libraries/` and the SQL functions under `databases/`
---

# PRD 00 working file: every use case and the hard questions

This file backs [00-use-cases-and-surfaces.md](00-use-cases-and-surfaces.md). Part one lists every use case. Part two works through the questions where a wrong answer would cost a rename, a data migration or a confused user. Claims about 0.2 cite the specification page in [specification/](../../../../specification/).

## Part one: the use cases

Term: **0.3** means the release should serve it. **0.3 if** means it joins 0.3 only if its experiment wins. **Later** means after 0.3. The job column names where the use case lands in the surface.

| # | Use case | Term | Job |
| --- | --- | --- | --- |
| U1 | A builder wants to ask a yes or no, pick-one, scale or label question about a text from a shell script so that the script branches on the answer | 0.3 (in 0.2) | Ask |
| U2 | A builder wants to call the same functions from a program in their own language so that judgment sits inside code with no model API to learn | 0.3 (in 0.2) | Ask |
| U3 | A builder wants to judge every record in a stream, keep the ones that pass or put them in order, so that a pipeline reads only what matters | 0.3 (in 0.2) | Ask |
| U4 | A builder wants to judge folders by line, window or whole file, with each answer naming its file and line, so that every result points back to its source | 0.3 (in 0.2) | Ask |
| U5 | An agent wants to search files by meaning beside keyword and vector search so that it reads less and lands on the right passage | 0.3 | Ask |
| U6 | An agent wants to learn every function, tool and command from help alone so that it needs no outside documents | 0.3 | Help |
| U7 | An agent wants one MCP tool that takes a command line so that its context stays small | 0.3 | Serve |
| U8 | An agent working on a local Excel or CSV file wants to fill new columns with answers so that the person gets a file back | 0.3 | Ask |
| U9 | A spreadsheet user wants a dry run first and the results in a new file by default so that nothing is lost | 0.3 | Ask |
| U10 | A Google Sheets user wants answers inside the sheet so that no download is needed | Later (needs a hosted proxy) | Serve |
| U11 | A data person wants a column of answers in DuckDB, SQLite, PostgreSQL, pandas or Polars so that a judgment is one clause in a query | 0.3 (in 0.2) | Ask |
| U12 | A data person wants to query their stored answers with SQL or a data frame so that they own and analyze their own data | 0.3 | Answers |
| U13 | A builder wants to save a question with its wording, options and threshold as a named tool so that every caller asks it the same way | 0.3 | Toolbox, Ask |
| U14 | A builder wants a tool to chain steps, such as a database search, then `filter`, then `rank`, then keep five, so that one name runs a short pipeline | 0.3 | Ask (`run`) |
| U15 | A builder wants to call a tool by name from the command line, any library, MCP or SQL so that one definition serves every surface | 0.3 | Ask (`run`) |
| U16 | A builder wants a tool to declare its inputs and what its shell steps need so that a run fails clearly before the first step | 0.3 | Toolbox |
| U17 | A builder wants a tool to carry its own cases and report how well it does so that each tool proves itself | 0.3 if (evals experiment) | Eval |
| U18 | A builder wants never to pay twice for the same question so that a rerun costs nothing | 0.3 (in 0.2) | Ask, reuse rule |
| U19 | A builder wants to see the requests and their cost before anything is sent so that a large run brings no surprise bill | 0.3 (in 0.2 for functions) | Every sending command |
| U20 | A builder wants caps on requests and tokens so that an agent in a loop cannot run away | 0.3 (in 0.2) | Every sending command |
| U21 | A builder moving to a cheaper model wants to choose whether old answers still count so that they trade cost against freshness | 0.3 | Reuse rule |
| U22 | A builder wants to check that a new backend works with every function so that a switch is safe | 0.3 (in 0.2) | Admin |
| U23 | A builder wants to ask the same questions of several models and see agreement, cost and speed so that they pick a model on their own data | 0.3 | Eval |
| U24 | A builder wants to see whether a model changed its answers over time so that a silent vendor change shows up | 0.3 (history kept); later for a drift report | Answers |
| U25 | A builder or agent wants a list of the answers that most need a look, such as disagreements between two models or unsure answers, so that review time goes to the edges | 0.3 if (evals experiment) | Curate |
| U26 | A person or agent wants to record the right answer, with who decided and why, so that it grades every model and wording tried later | 0.3 if | Curate |
| U27 | A builder wants a named, numbered answer key so that a reported score never moves when new right answers arrive | 0.3 if | Curate |
| U28 | A builder wants to record what happened downstream after an answer so that questions improve from real outcomes | 0.3 (ruled for the proxy) | Curate |
| U29 | A team wants a review screen with keyboard grading and queues so that reviewers work fast | Later | Screens |
| U30 | A builder wants to grade answers against right answers and get a suggested threshold so that the cut is earned for this model | 0.3 (in 0.2 as `runs audit`) | Eval |
| U31 | A builder wants to compare two runs or two cuts so that they see what changed and whether the change was right | 0.3 (in 0.2 as `runs diff`) | Eval |
| U32 | A builder wants to benchmark several setups on one answer key, with cost and speed, so that a model or wording change becomes a measurement | 0.3 if | Eval |
| U33 | A builder wants to improve a tool's wording with GEPA against an answer key so that the tool gets better without guesswork | 0.3 as a recipe; later built in | Eval |
| U34 | A builder wants ordering measures, such as the right item's place in the top k, for `rank` and `find` tools so that retrieval quality is measured | 0.3 if | Eval |
| U35 | A builder wants to export settled answers, with a random share of easy cases, as training JSONL so that they fine-tune their own model | 0.3 if | Curate |
| U36 | A team wants a small model tuned for each tool and routed by the proxy so that each question gets its own cheap model | Later | Serve |
| U37 | A builder wants one setting that trades cost against accuracy in ranking so that five items get every comparison and five thousand get one cheap pass | 0.3 if (ranking experiment) | Ask (`rank`, `find`) |
| U38 | A builder wants `find` to pick from more than 255 items so that a large set needs no cutting by hand | 0.3 if (ranking experiment) | Ask (`find`) |
| U39 | A builder or agent wants a long Markdown document turned into a map of sections, paragraphs, lists and tables so that code walks the map instead of reading everything | 0.3 if (segment experiment) | Ask |
| U40 | An agent wants to read a document in chunks cut where the meaning changes, each traced to its place, so that a chunk never splits a table from its caption | 0.3 if | Ask (reader) |
| U41 | An agent wants a shorter version of a document that keeps what matters for a question so that it fits in its context | 0.3 if | Ask (`rank` recipe) |
| U42 | A team wants every caller pointed at one server so that they share one store, one set of keys and one cache | 0.3 | Serve |
| U43 | An operator wants to override a threshold or route a tool to another model in one place so that a rollout needs no code change | 0.3 | Serve (proxy configuration) |
| U44 | An operator wants other people's code that speaks the System One wire to go through the proxy so that it gets the shared cache and traces | 0.3 | Serve |
| U45 | An operator wants retries, failures and latency by backend and model so that a failing provider shows up | 0.3 | Answers |
| U46 | An operator wants the proxy to serve small non-chat models so that cheap local models answer some tools | Open: the docs may promise it; whether such models can answer these questions is unknown | Serve |
| U47 | A person wants to say who they are once so that every change they make names them | 0.3 | Setup (`login`) |
| U48 | An agent wants every change it makes to name itself and the person it acts for so that a review can trace who did what | 0.3 | Setup |
| U49 | A builder or agent wants one command that shows the version, backend, store, user, toolbox and usage so that both know the setup | 0.3 (in 0.2 as `status`, smaller) | Setup |
| U50 | A builder wants to know where the database lives, point at another, back it up and bring old caches into it so that their data is safe and portable | 0.3 | Admin |
| U51 | A builder wants to remove old or sensitive answers on purpose so that the store stays small and private | 0.3 (in 0.2 as `cache prune`) | Admin |
| U52 | A test writer wants to record answers and replay them with no network so that tests and demos run offline | 0.3 (in 0.2) | Ask (`--record`, `--replay`) |
| U53 | A builder wants to move answers between machines, or from a laptop into a team proxy, so that nothing is stranded | 0.3 | Answers, Admin |
| U54 | A team admin wants keys, roles, budgets and real sign-in so that only the right people act | Later | Admin |
| U55 | A company wants a hosted or on-premises service with single sign-on, audit export and retention plans so that it buys ease and control | Later | All jobs, through the proxy |
| U56 | A team wants a settled right answer to answer the same question again with no model call so that settled work costs nothing | Later | Ask, through the store |

## Part two: the hard questions

Each question gives the reasoning and a recommendation. **Ian** marks the few that need his product direction. The rest are recommendations the PM can carry into tickets, and Ian can overturn any of them.

### Scope and words

**Q1. Does 0.3 ship curation and evaluation commands, or only a database shaped for them? (Ian)**
The README's 2026-10-13 notes say "0.3 itself ships proxying and caching, not curation". `data.md` of the same day says both strategies (benchmark five models; curate two models' disagreements) live in the data model from day one and the command line comes first, built for agents. `evals.md` says evals start in 0.3 and may not finish. These can both hold if the shapes ship and the curation screen waits. The cheapest useful slice is small: `eval grade` and `eval compare` already exist as `runs audit` and `runs diff`; `curate settle` and an answer key add two tables an agent can drive. Recommendation: ship the database with all its tables, `answers`, `eval grade`, `eval compare`, and the reuse rule. Ship `curate` and `eval bench` when the evals experiment admits them. Market only caching and the proxy, as the README says.

**Q2. Is a "run" a concept the user sees?**
The word has three meanings today. `data.md` drafts a run table ("a named group of answers with a purpose"). 0.2's `runs audit` and `runs diff` treat a run as a saved output file ([audit.md](../../../../specification/audit.md), [diff.md](../../../../specification/diff.md)). 0.3 makes `run` the eleventh function, which runs a tool. Each use of a group already has a better home. A benchmark is an answer key version times a list of setups, so the eval itself is the record. Disagreements are answers that share a question and evidence under different setups, so they need no group. Drift is a question about time. Provenance belongs to the call, which the glossary already defines as one use of one function on any surface. Recommendation: "run" means only "run a tool". The store keeps a call record (actor, surface, tool, time) and an eval record (answer key version, setups, results). No run table and no run flag.

**Q3. What happens to 0.2's `runs` noun?**
`thinkthen run support/triage` beside `thinkthen runs diff` differ by one letter and mean unrelated things. The 2026-10-12 decision keeps no backward compatibility before 1.0, so 0.3 is the cheap moment to fix it. Recommendation: `runs audit` becomes `eval grade`, `runs diff` becomes `eval compare`, and the hidden aliases `audit`, `diff` and `check` go.

**Q4. Should the top level hold job words or resource nouns?**
ADR 0118 reserves nine resource nouns: questions, items, answers, checks, datasets, setups, findings, people and search ([specification/README.md](../../../../specification/README.md)). `data.md` floats one `db` command for the database, caching rules, administration, evals, audit and diff. Resource nouns make a user learn the data model before acting. One `db` word mixes four jobs under the name of a storage medium. A job word matches what a person sat down to do, and the resources sit under it, as in `curate key` or `admin users`. The jobs also match the four roles in Ian's later interface notes: the reviewer curates, the data scientist evaluates, the admin administers and the manager reads answers and cost. Recommendation: job words at the top, actions beneath (`thinkthen JOB ACTION`). Amend ADR 0118: keep `answers`, release the other reservations, and reserve only the later words this map names (`optimize` under `eval`).

**Q5. Is the stored thing called an answer or a decision? (Ian)**
"Rent your models, own your decisions" uses decision, and the strategy's terms table makes "decision" the counting word: one question answered about one piece of evidence. The 0.2 result calls the model's output `answer`, `data.md` names the table `answer`, and Ian speaks of "right answers" and "settled answers". A command named `decisions` may read as "results of `decide`" only. Recommendation: `answers` as the noun on every surface, and "decision" stays the counting word in docs and pricing.

**Q6. What do we call a frozen set of right answers?**
The notes use gold set, benchmark, dataset and dataset version. 0.2 already says "answer key" (`runs audit RESULTS KEY`, `runs diff --key`). "Set" already means a question set in `annotate` and `rank`. Recommendation: "answer key", with a name and numbered versions. "Benchmark" names the act (`eval bench`). "Right answer" is the plain name of one row, and "settle" is the verb for recording one, which is Ian's own word in `data.md`.

**Q7. Do the job words collide with words a language reserves?**
JavaScript modules run in strict mode, and strict mode forbids binding the name `eval`, so `import { eval } from 'thinkthen'` is a syntax error. Python already shadows its built-in `filter` and would shadow `eval`. Recommendation: job words are namespaces, never bare names: `client.eval.grade`, `thinkthen.eval.grade`, `tt_eval_grade`, `thinkthen_eval_grade`. The binding generator checks every public name against each language's reserved words.

**Q8. Proxy and MCP: one serving word or two?**
A shared `serve` word would group them, but agent configurations copy `{"command":"thinkthen","args":["mcp"]}` from the 0.2 docs, and each extra level costs every agent a token. Recommendation: two words. `mcp` keeps its 0.2 spelling. `proxy` starts the server. Whether the proxy ships in the same executable is a packaging question for the proxy PRD.

**Q9. Does `transform` survive?**
0.2 ships ten built-in jq transforms: band, calibration, compare, cost, counts, monitor, score, sweep, triage and trials ([transform.md](../../../../specification/transform.md)). With a database, each becomes a query the tool can run itself: counts and cost become `answers stats`, calibration, sweep, band and score become `eval grade` output, compare becomes `eval compare`, and triage becomes `curate queue`. `toolbox.md` moves jq into optional tool recipes. Recommendation: retire `transform` and give each analysis its new home.

### Libraries and data contracts

**Q10. Does every library expose curation, evaluation and administration? (Ian)**
The 2026-10-09 binding ruling makes every function first-class in every language, with typed results, errors and cancellation. About twelve more functions across 24 languages means about three hundred more bindings to keep in parity. Evaluation happens in notebooks (Python, R). Outcomes come from application code (TypeScript, Python). Rust carries the command line. Nobody curates from COBOL. Recommendation: three tiers with one name per concept. Tier one, the ten functions and `run`, goes everywhere. Tier two, the data jobs (read answers, settle, report an outcome, grade, compare, bench, export), goes to Rust, Python, R and TypeScript, and SQL reads the published views. Tier three, administration, stays on the command line. The command line is always the superset.

**Q11. Is the database layout a public contract? (Ian)**
"Your answers are in an open SQLite file you can query" is a strong sovereignty claim. A published layout also turns every later table change into a migration for every user's queries. Recommendation: publish named, versioned read views (answers, right answers, answer keys, calls, evals, users) and keep the tables private. Writes go through ThinkThen so the actor and never-overwrite rules hold.

**Q12. Can SQL run tools that have shell steps?**
`toolbox.md` says every library runs shell steps through the Rust core. PostgreSQL runs every role as the server's one operating-system user ([cache.md](../../../../specification/cache.md)), so a shell step from SQL runs code on the database server. Recommendation: `thinkthen_run` refuses a tool with a shell step on every SQL surface. Tools made only of function steps run.

**Q13. How does a user compare models when an engine holds one route?**
ADR 0119 gives each engine one endpoint, one key and one provider API type, and refuses model groups ([sdk-boundary.md](../../../../specification/sdk-boundary.md)). It also says a new route needs an explicitly constructed engine. Recommendation: the functions keep one route per call. `eval compare` and `eval bench` build one engine per named setup, which ADR 0119 already permits. The curation queue reads answers from any setups in the store.

**Q14. Does the SDK boundary need an amendment?**
Yes. `sdk-boundary.md` names curation as proxy business policy. The roadmap holds `state` because "the tool holds no state", holds history across runs, holds `--output FILE` and declines a `serve` command ([roadmap.md](../../../../specification/roadmap.md)). Ian's 0.3 rulings move the store, comparison and right answers into the SDK, add spreadsheet output files and add the proxy. Recommendation: one ADR amends ADR 0119's keep-and-refuse table and the roadmap at hand-off. Routing, model groups, overrides and A/B tests stay in the proxy.

### Identity

**Q15. What is the identity of a question across the command line, a tool and a SQL call?**
0.2 already keys stored answers on the exact bytes sent, so the same wording from any surface shares answers ([cache.md](../../../../specification/cache.md)). A right answer needs a key that survives rewording, and 0.2 has none. Its question digest includes the threshold, and its optional `name` and `wording_version` enter no identity ([question-file.md](../../../../specification/question-file.md)). `fixed-and-tunable.md` splits the contract (function and labels), the wording (text, descriptions, order) and the reading (threshold). Recommendation: the question id is the tool's id, or the tool's id plus a step name, when the question comes from a tool. Otherwise ThinkThen derives it from the contract and the wording. The wording version is a digest of the wording, with an optional author number. The threshold leaves identity. Every result carries the question id, the wording digest, and the tool's id and digest. ThinkThen never merges two ids by itself.

**Q16. What grammar does an id use?**
A tool name such as `support/triage` holds a slash. 0.2's question `name` allows `[a-z][a-z0-9_-]{0,63}` and the reserved wire field `question_id` allows `[A-Za-z0-9._-]`; neither admits a slash ([result.md](../../../../specification/result.md)). Recommendation: one grammar everywhere: lowercase segments joined by `/`, exactly what a person types after `run`. Widen the reserved wire field now, while no proxy uses it.

**Q17. Does a tool keep its identity when its file moves or changes?**
If the path is the id, moving a folder orphans its history. If the hash is the id, fixing a typo does. Recommendation: the id lives in the tool file and the path only finds it. The tool's digest changes with any edit and rides in every answer. A wording edit keeps the id and adds a wording version. A label change is a new question, so `toolbox check` warns when an id with stored answers changes its labels.

### The store

**Q18. Where do reuse rules live, and who sets them?**
`data.md` lets a rule choose which parts of a stored answer must match: the address, the model and the model version. Recommendation: one reuse setting with 0.2's precedence ([settings.md](../../../../specification/settings.md)): typed per call, then the environment, then the tool file, then the configuration file, then the built-in rule, which is 0.2's exact match. ThinkThen applies the rule when it looks up and never changes stored rows, so a new rule needs no migration. Each result says which stored answer it reused and under which rule. `eval bench` always asks fresh under an exact rule, because a benchmark measures the setup it names.

**Q19. Can a stored answer be reused across wordings?**
`data.md` says wording text always matches and also lists the wording version as an optional part. The two conflict. Reuse across wordings would score a new GEPA wording with old answers. Recommendation: wording text always matches. The optional parts are the address, the model and the model version.

**Q20. Does "nothing is overwritten" survive refresh, pruning and privacy?**
In 0.2 a refresh or a `--record` run replaces the held answer, `cache prune` is the one removal command, and `--no-cache` keeps nothing ([recording.md](../../../../specification/recording.md)). Recommendation: a refresh adds a new answer, and "the answer" is the newest one the rule accepts. `admin prune` stays the one deliberate removal, with a dry run, and refuses to remove evidence that a right answer needs unless told. A call may still decline to keep anything, for sensitive text.

**Q21. Is 0.2's cache folder the database, and how do old caches move in?**
0.2 keeps one `thinkthen.sqlite` per cache folder, chosen by `--cache DIR` or `THINKTHEN_CACHE`. Ian ruled one database. Recommendation: one database file replaces the default cache folder, and `--database PATH` replaces `--cache DIR`. The first open imports the 0.2 default cache once and leaves the old file in place. `admin database import` brings in other folders.

**Q22. Where does the database live?**
0.2 keeps answers in the platform cache folder, which macOS may purge ([recording.md](../../../../specification/recording.md)). Right answers and answer keys are owned work and must survive a purge. Recommendation: the platform data folder, such as `~/.local/share/thinkthen/` on Linux and `~/Library/Application Support/thinkthen/` on macOS. `THINKTHEN_DATABASE`, the configuration file's `database` and `--database` point elsewhere. `status` shows the path.

**Q23. Do `--record` and `--replay` survive beside the database?**
Tests and demos replay committed fixture folders with no network. The database is private and grows. Recommendation: keep both options for fixture folders, apart from the database. A replay writes nothing to the database. `cache convert` folds into `answers export`, which can write a replayable fixture.

### Same behavior everywhere

**Q24. How does one command behave against the local database and against a proxy?**
Ian ruled that the SDK turns its cache off in proxy mode and that the data strategy works with no proxy. Recommendation: one store per caller. When the configured backend is a ThinkThen proxy, the functions and the data commands use the proxy's store. Otherwise they use the local database. The words, the JSON and the exit codes match. `status` names the store in use.

**Q25. Can administration reach a proxy over the network in 0.3?**
The 0.3 proxy has no access control and accepts every caller. A remote prune or restore by any caller loses data. Settling and outcomes are the curation use itself, and each write records the actor the caller claims. Recommendation: `admin` acts only on a database file on the same machine. The proxy serves the functions, `answers`, `curate` and `eval` over HTTP. Overrides and routing come from the proxy's configuration file. Sign-in later opens administration under permissions.

**Q26. What is the default output of commands that are not functions?**
0.2 disagrees with itself: `status` prints name and value lines with `--json` as the option, and `runs audit` and `runs diff` print JSON lines with `--table` for a person. Recommendation: every command that is not a function prints JSON lines by default and takes `--table` for a person, `status` included. Agents are the first user.

**Q27. How does an agent discover all of this from help?**
Recommendation: `thinkthen help` lists commands grouped by job, one line each. `help JOB` and `help JOB ACTION` show examples. `help --json` prints the whole tree for a machine. `toolbox list` and `toolbox describe TOOL` print each tool's inputs, outputs, needs and cases result. The MCP tool returns the same text. Every refusal names the command to run next.

**Q28. What may the one MCP tool run, and does it take a string or a list?**
BioMCP's command tool takes one string and rejects commands that change anything. It now also offers six bounded typed tools beside that string tool, which the one-tool experiment should weigh. Recommendation: one string, parsed by the command line's parser with no shell. Allow everything except `proxy`, `mcp`, `login` and the `admin` actions that delete or restore, unless the server starts with a flag that allows them.

**Q29. Does each command name grow into permissions, metering and web routes?**
Ian's later enterprise notes propose the action as the billing unit, and his later interface notes want every screen to be a client of the proxy's API. Recommendation: every command is one named action, `JOB.ACTION`, such as `curate.settle`. The proxy's routes, the MCP tool, later permissions and later meters all use that name. Roles group job words: reviewer for `curate`, data scientist for `eval`, admin for `admin`, viewer for reading `answers`. Sign-in then adds no renaming.

**Q30. How does spending get previewed outside the functions?**
0.2 offers `--plan` and spend caps on every function. Recommendation: every command that can send takes them too. `run` adds up its function steps. `eval bench` prints requests per setup before it sends anything.

### Functions and tools

**Q31. What does `find` mean beside `rank` once the ranking dial exists?**
In 0.2, `rank` judges each record alone and "never runs a tournament" ([rank.md](../../../../specification/rank.md)), and `find` sends 2 to 255 units in one request and picks one ([find.md](../../../../specification/find.md)). Ian ruled the dial applies to `rank`, `find` and compaction. Recommendation: define each by what comes back. `filter` returns the records that pass a fixed bar, in input order. `rank` returns every record, best first, each with a score from 0 to 1. `find` returns one item or none. The dial is the method: alone, windows, tournament, all pairs. Defaults stay 0.2's: `rank` judges alone and `find` sends together. Past 255 items `find` refuses and names the dial setting. The `rank` help line about never comparing records changes to name the dial.

**Q32. Are segment, chunk and compact functions?**
The roadmap holds `segment` because a whole document must fit one request. 0.2 already reads documents by line, window or whole file, with positions. Recommendation: one structure serves three uses. `segment`, admitted only if its experiment shows prediction helps, returns the document's graph of nodes and edges. Chunking becomes a reader unit, so every function can read a document by segment with positions, as `--unit line` does today. Compacting becomes `rank` over segments with a size budget, a recipe or a `rank` setting. At most one new everyday verb results.

**Q33. Is a spreadsheet a new function, a command or a reader?**
Excel is a file format. 0.2 already reads CSV and TSV as framing. Recommendation: an Excel reader beside `--csv` and a writer that writes a new file by default and writes in place only when asked, after a backup. `annotate` maps each question to an output column, and a tool can wrap the whole step. No new function and no new job word.

**Q34. Is `run` of a one-step tool the same as calling its function?**
Shell gates depend on `decide`'s exit codes: 0 for yes, 1 for no, 3 for not sure ([channels.md](../../../../specification/channels.md)). Recommendation: yes. A one-step tool prints and exits exactly as its function does. A multi-step tool prints its last step's output and exits with its last step's code. A failed step uses the failure codes. A library's `run` returns the last step's typed result.

**Q35. Do `@FILE` and `@NAME` survive beside `run`?**
0.2 calls a saved question by `@NAME` from `questions/` under the configuration folder. Ian ruled that a tool replaces a question and the toolbox replaces the catalog. Two ways to call one named thing break one concept, one name. Recommendation: `run NAME` replaces `@NAME`. `@FILE` stays on functions to load settings from an explicit file.

**Q36. Where does a threshold live once a threshold belongs to a model?**
`data-model.md` says a threshold means something only for one model. 0.2 stores one threshold per question file and warns when the calibration profile differs. Recommendation: a tool's threshold may name a cut per model, with a default. `eval grade --write` writes the cut for the model it graded. The caller's code wins locally, and the proxy may override centrally, as ruled on 2026-10-06.

**Q37. Do outcomes belong to code or to curation?**
Code learns the outcome where it happens. People add outcomes too. Recommendation: `curate outcome` on the command line and an `outcome` function in the tier-two libraries and through the proxy. An outcome is evidence about the right answer and never changes the stored answer.

### People

**Q38. What does "who am I" look like in 0.3? (Ian)**
Ian asked for a command that creates a user record and makes it current, with no password check, much as a coding tool's sign-in records an identity. `login` is the word people expect and the word sign-in will need later. With no password it may suggest a protection that does not exist. Recommendation: `thinkthen login NAME`, which says plainly that 0.3 checks no password. `status` shows the current user. Later, the same command signs in to a proxy or the hosted service.

**Q39. How does an agent name itself and the person it acts for?**
Recommendation: `THINKTHEN_AGENT` names the agent for every write, and the current user is the person it acts for. Each write records both. A library engine accepts the same two values. A write with no user records the default user.
