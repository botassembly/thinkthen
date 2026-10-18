---
date: 2026-10-13
status: draft PRD
source: Ian's ruling of 2026-10-13 that the command line and the library functions follow use cases; the 0.3 design files in this folder; Ian's ThinkThen notes and the ThinkThen ideal state; ThinkThen 0.2's `specification/`, `thinkthen --help` and library exports. The full use-case list and 39 worked questions are in [00-use-cases-and-surfaces-questions.md](00-use-cases-and-surfaces-questions.md)
---

# PRD 00: Use cases and surfaces

Every later PRD builds on this one. It names who ThinkThen serves, the jobs they come to do, and one command and library shape for those jobs, now and later.

## Three releases

Ian agreed on 2026-10-14 to split this work into three releases, on one condition: the database is designed for curation and evals from the start.

| Release | Theme | What ships | PRDs |
| --- | --- | --- | --- |
| 0.3 | Own your answers | The answer database with its full schema, toolboxes and `run`, MCP as one tool, spreadsheets, and the publishing channels | 00, 01, 06, 07, 08, 09 |
| 0.4 | Grade and improve | Curation, evals and the ranking setting | 02, 03 |
| 0.5 | Serve and structure | The proxy and document structure | 04, 05 |

The six 0.3 experiments ran on 2026-10-14. Their results are folded into PRDs 01, 07, 08 and 09.

## Who and what's in it for them

- **The builder**, the 0.3 customer, asks typed questions from scripts and programs and owns every answer it paid for.
- **The agent** gains a third kind of search, learns the whole tool from help, and settles answers for its person.
- **The data person** gets answers as a SQL or data frame column and queries the store like any table.
- **The reviewer** settles right answers that grade every model and wording tried later.
- **The operator** runs one proxy: one store, one set of keys, one place to roll out changes.
- **Later,** the team admin, the manager and hosted customers reuse the same words through screens and sign-in.

## The jobs

Fifty-six use cases fall into seven jobs, each one a thing a person sits down to do.

1. **Ask.** Judge a text, record, file or spreadsheet, or run a named tool. The everyday surface.
2. **Build tools.** Keep questions and short pipelines as named tools in a local toolbox.
3. **Look back.** See, count and export the answers you own.
4. **Curate.** Find what needs a look, settle right answers, freeze answer keys, record outcomes.
5. **Evaluate.** Grade setups against right answers, pick thresholds, compare models and wordings.
6. **Set up.** Say who you are, see your setup, check backends, look after the database.
7. **Serve.** Run the proxy for a team, or MCP for an agent.

## The surface

The form is `thinkthen JOB ACTION`. Every library gets the ask job: the ten functions and `run`, in its idiom. Every other job lives in Rust and the command line only, by Ian's 2026-10-14 ruling. MCP takes the command line as one string and runs only the ask job and help.

| Job | Command line | Libraries | SQL and MCP | From 0.2 |
| --- | --- | --- | --- | --- |
| Ask | `decide choose tag score filter rank find annotate recognize relate run` | All 24 languages, same names as today, plus `run` | `thinkthen_decide` through `thinkthen_run`; MCP all | The ten functions stay. `@NAME` becomes `run NAME` |
| Build tools | `toolbox list`, `describe`, `check` | Rust only | MCP: configured tools listed in `help` | The named questions folder becomes the toolbox |
| Look back | `answers list`, `show`, `stats`, `export` | Rust only | SQL and data frames read published views; not MCP | `cache convert` becomes `answers export`. `transform` counts and cost become `answers stats` |
| Curate (0.4) | `curate queue`, `settle`, `key`, `outcome`, `export` | Rust only | Not MCP | New |
| Evaluate (0.4) | `eval grade`, `compare`, `bench`; later `optimize` | Rust only | Not MCP | `runs audit` becomes `eval grade`. `runs diff` becomes `eval compare`. Other transforms fold in |
| Set up | `status`, `login`, `admin backends`, `database`, `users` | 0.2's `usage` stays everywhere; the rest Rust only | Not MCP | `status` grows. `backends check` moves under `admin`. `cache prune` goes away: the database keeps everything, and clearing it is later work (Ian, 2026-10-14) |
| Serve | `mcp` in 0.3; `proxy` in 0.5 | None | MCP is this row | `mcp` becomes one tool |

Help groups the ten non-function words under the seven jobs.

## Principles every PRD follows

1. **Start from a use case.** Every command, option and table names its use case and the customer's gain.
2. **One concept, one name** across the command line, libraries, SQL, MCP, proxy routes and, later, permissions. Job words stay namespaces; JavaScript forbids a bare `eval`.
3. **Few top-level words.** The ten functions and `run` stay flat; the rest sit under job words. A new top-level word needs Ian.
4. **Store raw, read by rule.** Keep every answer with its probabilities and overwrite nothing. Thresholds and reuse rules apply on read, so changing them costs no call and no migration.
5. **0.2 behavior is the default.** Exact reuse, `rank` judging items alone, and results in a new file. New behavior is a setting someone turns on.
6. **One store per caller:** the local database, or the proxy's store for calls through a proxy. Words, JSON and exit codes match. The proxy never adds a behavior a local run cannot reproduce.
7. **Agents first; the command line is the superset.** Every library function has a command. Other commands print JSON lines, with `--table` for people. MCP returns the same help.
8. **Large models call the tool.** ThinkThen never calls one. People and agents curate through commands.
9. **Every write names who acted:** a person, or an agent and its person.
10. **Every send can be previewed and capped,** `run` and `eval bench` included.
11. **Judging never becomes acting.** Shell steps run only on the caller's machine. The proxy never serves tools; SQL refuses shell steps.
12. **Room to grow.** Each command is a named action, such as `curate.settle`, that screens, permissions and billing attach to later. Job words become roles: reviewer, data scientist, admin, viewer.

## Out of 0.3

Curation, evals and the ranking setting, which ship in 0.4. The proxy and document structure, which ship in 0.5. Removing or clearing stored answers. Screens. The hosted service and Google Sheets inside the sheet. Sign-in, keys, roles and budgets. Enterprise single sign-on, audit export and retention. Administration over the network. A built-in optimizer; GEPA ships as a recipe. Fine-tuning, small models per tool, automatic routing. Answering from settled right answers with no model call. PostgreSQL as the store.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 4 and 5. Answered: library reach and MCP reach (Ian, 2026-10-14), and the scope question, which the three releases settle.
