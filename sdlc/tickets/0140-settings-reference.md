---
flow: build
priority: 140
opens: specification/settings.md specification/README.md sdlc/scripts/settings sdlc/scripts/spec sdlc/scripts/README.md sdlc/tickets/README.md sdlc/issues sdlc/records sdlc/tickets
---

# 0140: every setting is explained in one place

Status: ready. Design review pending. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user who wants to change how ThinkThen asks, answers, or runs opens one page, `specification/settings.md`, and finds every setting there. Each row says what the setting does, its default, the values it takes, and how each surface spells it. A check fails the gate when a command flag, a product environment variable, or a question-file key has no row, or when a row names one that no longer exists.

This is ticket C1 of `sdlc/issues/2026-09-26-batching-design.md` (main at `9b667c09` or later), section "C1: the settings reference". Ian ruled on 2026-09-26 that every setting is explained in one place, ruling 11 of that design. `sdlc/issues/2026-09-26-recognize-design.md` points to this page from its "Defaults and what a caller can change" section and its ticket preamble. Ian can overturn the ruling.

C1 needs no batching code. It lands after ticket 0139, the batching ADR B0, as Ian's ticket order says: B0, then C1. B0 writes no code, so the wait costs nothing. The table rows are the settings on main when C1 lands. The settings B0 records but no code has yet arrive as "on the way" lines, per decision 1.

## What happens today

No page lists every setting. Each lives on the page of the thing it changes, and several pages disagree about which surfaces carry it.

- `specification/question-file.md:80-91` tables the question's settings and their two homes, the command line and the file.
- `specification/backends.md:55` and `:61` state `--model`, `--timeout` and `--max-retries` with their defaults.
- `specification/records.md:127-135` states `--jobs`, its default of 4, and why the default stays at 4.
- ADR 0033 states the answer cache, `THINKTHEN_CACHE`, the read-only configuration file and its four fields, and the address and model precedence.
- ADR 0017 section 5 and its ticket 0077 note state the library throttle. ADR 0041 states the deadline on every library and SQL surface.
- Each library README and each database README states its own spellings.

The command's settings come from clap in `crates/thinkthen/src/cli/args.rs`, `args/command.rs`, `args/find.rs` and `args/relate.rs`. `thinkthen --help` lists 16 commands. The list below comes from the long help of a build of 2026-09-25, checked against the clap source at `d410ef4a`. The builder reads the help again at build time. These are the long flags besides `--help` and `--version`:

- Every judging verb: `--details`, `--input`, `--lines`, `--jsonl`, `--csv`, `--tsv`, `--field`, `--dry-run`, `--url`, `--profile`, `--model`, `--record`, `--replay`, `--cache`, `--no-cache`, `--timeout`, `--jobs`, `--max-retries`. `find` takes neither `--csv`, `--tsv` nor `--jobs`.
- Per verb: `--true`, `--false`, `--threshold`, `--quiet`, `--raw`, `--option`, `--options`, `--label`, `--top`, `--kind`, `--relation`, `--relation-threshold`, `--either`, `--kind-field`, `--none`.
- `audit` (`cli/audit.rs:28-70`): `--by`, `--threshold`, `--id`, `--seed`, `--target`, `--optimize`, `--write`, `--curve`, `--pooled`, `--table`. `diff`: `--key`, `--threshold`, `--compare-threshold`, `--id`, `--table`. `check`: `--url`, `--model`, `--timeout`, `--dry-run`. `status`: `--json`. `cache prune`: `--max-size`, `--older-than`, `--answered-by-other-than`.

Hidden flags such as `rank --threshold` and `filter --top` exist only to be refused by name. Help does not show them, and they are not settings.

Product code reads three `THINKTHEN_` variables: `THINKTHEN_API_KEY` (`core/backend.rs:12`), `THINKTHEN_BASE_URL` and `THINKTHEN_CACHE` (`cli/edge.rs:51-56`, `public/settings.rs:90-98`). It also reads `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` and `HOME` for its paths (`config.rs:127-148`). Product files also hold six `THINKTHEN_TEST_` names that only tests set: `RETRY_WAIT_MS`, `SIGINT_ACK`, `IDENTITY_PAUSE`, `IDENTITY_READY`, `IDENTITY_RESUME`, and `FORM_CHILD`.

The read-only configuration file, `thinkthen.config/1` under ADR 0033, holds `url`, `model`, `cache` and `cache_bytes` (`config.rs:28-37`). It is a tier of its own, between the environment and the built-in default.

`specification/question-file.schema.json` gives each verb entry its top-level keys: the verb key that holds the question text, `true`, `false`, `options`, `labels`, `levels`, `threshold`, `on`, `model`, `profile`, and `relate`'s `version`.

The library and SQL surfaces, as main has them at `d410ef4a`:

- Every engine starts from `EngineBuilder::from_env` (`public/settings.rs:82`), so every surface reads the same three variables and the configuration file. The timeout of 30 seconds, two retries and no backend profile are fixed in `build` (`settings.rs:233-238`). No library or SQL surface can change them.
- Rust: `EngineBuilder::base_url`, `api_key`, `model`, `throttle` (1 to 32), `max_requests`, `default_cache`, `cache_at`, `no_cache`, `cache_bytes`. Question builders take `model`, `cut`, `cut_at` and `band`. `CallOptions` takes a deadline and a cancel (`public/options.rs:88-160`). The Polars door has no settings of its own.
- Python: `Engine(base_url=, model=, throttle=, max_requests=, cache=, cache_bytes=)`, a per-call `deadline=` and `token=`, and `threshold=` and `relation_threshold=` on questions. No key argument.
- TypeScript: `new Engine({baseUrl, model, throttle, maxRequests, cache, cacheBytes})`, per-call `deadlineMs` and `signal`, and `threshold` in the question. No key argument.
- Ruby: `Engine.new(base_url:, model:, throttle:, max_requests:, cache:, cache_bytes:)`, per-call `deadline:` and `cancel:`, and `threshold:` and `relation_threshold:`. No key argument.
- R: `tt_engine(base_url, model, throttle, max_requests, cache, cache_bytes)`, per-call `deadline=`, and `threshold=` and `relation_threshold`. The key comes from `THINKTHEN_API_KEY` alone.
- C: `thinkthen_engine_new(void)` reads the environment and the configuration file only. Every `_opts` call takes `deadline_ms` and a cancel token. The model and threshold go inside the question JSON.
- DuckDB: `SET thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_max_requests_total`, `thinkthen_cache`, `thinkthen_cache_bytes`, `thinkthen_relate_seconds` (default 60) and `thinkthen_relate_holding_rows` (default 1,000,000). A final deadline argument in milliseconds on each scalar but `recognize`. No address, key, model or cache-off setting.
- PostgreSQL: the settings `thinkthen.deadline_ms`, `thinkthen.throttle`, `thinkthen.max_requests`, `thinkthen.max_requests_total`, `thinkthen.cache`, `thinkthen.cache_bytes` and `thinkthen.file_directory` (`databases/postgresql/src/call.rs:391-449`). `thinkthen.api_key` exists only so a set value is refused. No address, model or cache-off setting.
- SQLite: the functions `thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_max_requests_total`, `thinkthen_cache` (NULL turns the cache off) and `thinkthen_cache_bytes`, called before the first call (`databases/sqlite/src/settings.rs:127-188`). An optional final deadline argument in milliseconds. No address, key or model setting.
- `cache_bytes` checks its value and has no effect on every library and SQL surface, and the configuration file's `cache_bytes` is read only by `cache prune`'s default. The DuckDB README line 39 and the SQLite README lines 50 and 56 still say it caps the cache. Ian ruled on 2026-09-25 that no setting may do nothing. Item 4 of `sdlc/issues/2026-09-25-public-library-api-gaps.md` records that ruling and the removal, which has no ticket yet. The row says the setting has no effect on those surfaces until the removal lands. The README lines are not in that issue, so the C1 build files them (see "The surface-gap issue").
- `max_requests`, the process request total, the deadline and the two DuckDB relate settings have no command flag.

## Design

### The page

`specification/settings.md` opens with three short prose parts, then the table, then two prose lists.

1. **What counts as a setting.** A value a caller chooses that changes how the tool reads, asks, answers, stores, or reports. Every flag in the command's help is one. The question text, the records, and help itself are not.
2. **Precedence.** One rule, in prose above the table: the typed value, then the environment, then the question file, then the configuration file, then the built-in default. A setting skips the tiers it has no home in. Only a per-call value counts as typed: the command flag, or an argument on one library or SQL call, such as a question's `threshold` or a call's deadline. An engine-level library setting, such as `Engine(throttle=)` or `EngineBuilder::throttle`, and a SQL session setting, such as DuckDB's `SET thinkthen_throttle` or SQLite's `thinkthen_throttle(n)`, sit in the environment tier beside the `THINKTHEN_` variables. They are set once for a process or session, as a variable is. This follows the batching design's section 6, which puts the engine setting and the SQL `SET` in the environment tier. The page states the two orders on record that the rule covers: ADR 0033's address order, flag then `THINKTHEN_BASE_URL` then the configuration `url` then the built-in address, and its model order, flag then the question file then the configuration `model` then `jev-latest`. It states the cache order: `--cache` or `--no-cache`, then `THINKTHEN_CACHE`, then the configuration `cache`, then the platform folder.
3. **How to read a cell.** A surface cell holds that surface's spelling in backticks. A cell reads exactly `not on this surface` where the setting does not reach that surface.

The table has these columns, in this order:

| # | Column |
| --- | --- |
| 1 | Setting |
| 2 | What it does |
| 3 | Default |
| 4 | Allowed values |
| 5 | Command flag |
| 6 | Environment variable |
| 7 | Configuration file |
| 8 | Question-file key |
| 9 | Rust |
| 10 | Python |
| 11 | TypeScript |
| 12 | Ruby |
| 13 | R |
| 14 | C |
| 15 | DuckDB |
| 16 | PostgreSQL |
| 17 | SQLite |

Polars and pandas columns take the library's spelling, as the batching design's section 6 says, so they get no column. The page says so in one sentence under the table.

The Default cell names the record that set the default, as a link. Where no record gives a reason, the cell names the page that states the value, and the page's "Defaults with no recorded reason" list names the gap. The builder does not invent a reason.

Under the table:

- **Defaults with no recorded reason.** One line for each gap in the row list below.
- **Settings on the way.** One line for each setting a design adds and no code has yet. Each line says in one sentence what the setting does and its designed default, and cites the design section and the ticket that brings it. The ticket that lands the setting moves its line into the table in the same commit. The lines:
  - `batch` (`--batch N`, `THINKTHEN_BATCH`, the question file's `batch`): the most records one request carries. Default `max`, as many as fit under the backend's limits; `--batch 1` sends one record a request. Batching design ruling 1, sections 2 and 6. Ticket B4.
  - `context` (`--context FILE`): a reference text sent with every batch as shared evidence. Default none. Batching design section 1 and section 6. Ticket B7.
  - `--facts`: prints one `thinkthen.run/1` summary line to standard error at the end of a run. Default off; library results carry `facts` on every call with no setting. Batching design ruling 7 and "Run facts". Tickets B5 and B12a to B12f.
  - `relate --jobs`: how many of one relate's requests run at once. Designed default 4, the engine throttle. Batching design ticket J1.
  - `recognize --jobs`: how many of one long text's pieces run at once. Designed default 4. Recognize design ticket R4b.
  - `keep` (`--word-keep`): whole words that never split, so `McDonald's` keeps its `'s`. Default empty, so a possessive `'s` is peeled off a name. Recognize design section 2 and the friction list. Tickets R2 and R6.
  - `infixes` (`--word-infix`): marks split out of a word between letters, so `--word-infix -` turns `London-based` into `London`. Default the two dashes `–` and `—`, so hyphens stay joined. Recognize design section 2. Tickets R2 and R6.
  - The other word lists, `prefixes`, `suffixes` and `trim`: marks peeled from a word's edges and trimmed from a name's edges. Defaults in the recognize design section 2's table; question file only. Ticket R2.
  - `boundary` (`--boundary confirm|run`): whether a run of name words whose kinds change is asked about as one name or several. Default `confirm`. Recognize design section 3. Tickets R3 and R6.
  - `window` (`--window N`): how many neighbouring words on each side a piece of a long text carries. Default 200, from 0 to 5,000. Recognize design section 6. Ticket R4.
  - The hard cap: the largest text `recognize` takes. Fixed at 600,000 bytes with no option; a longer text exits 2. Recognize design section 6. Ticket R4.

  The worked examples of `'s` possessives with `keep` and of hyphens with `infixes` come with R2, which brings those settings. The builder checks each line against the designs on main at build time, and against ticket 0139's ADR once it lands.

### The rows

One row per setting. A row may name several flags when they are one setting, as the framing flags are. Two rows may name one flag when it means two things on two commands, as `--threshold` does on the judging verbs and on `audit`. The builder fills each cell from the code on main at build time, not from this list. The Default column's source is named for each row.

| Setting | Flags and names | Default and its record |
| --- | --- | --- |
| Threshold | `--threshold`, `threshold`, library and SQL `threshold` | 0.5 on `decide`, `tag`, `filter`. ADR 0007 point 5 and `threshold.md`. No reason on record for 0.5 beyond "the one threshold": gap |
| Relation threshold | `--relation-threshold`, `relation_threshold` | 0.5. `recognize.md:31`. No reason on record: gap |
| What true means, what false means | `--true`, `--false`, `true`, `false` | No text. `question-file.md:84-85` |
| Options | `--option`, `--options`, `options` | None; `choose` requires 2 to 255. `question-file.md:86` |
| Labels | `--label`, `labels` | None; `tag` requires 1 to 20. `question-file.md:87` |
| Levels | `levels` | None; `score` requires 2 to 10. `question-file.md:88` |
| Kinds | `--kind` | `recognize.md` |
| Relations, either, kind field | `--relation`, `--either`, `--kind-field` | `recognize.md`, `relate.md` |
| None option | `--none` | Off. ADR 0030 |
| Top | `--top` | All records. `rank.md` |
| Input file | `--input` | Standard input. `records.md` |
| Framing | `--lines`, `--jsonl`, `--csv`, `--tsv` | One document; lines on `filter`, `rank`, `find`. `records.md`, ticket 0137, ADR 0030 |
| Evidence pointer | `--field`, `on` | The whole record. `question-file.md:90` |
| Details | `--details` | Off. `result.md` |
| Quiet, raw | `--quiet`, `--raw` | Off. `channels.md` |
| Dry run | `--dry-run` | Off. `channels.md` |
| Address | `--url`, `THINKTHEN_BASE_URL`, configuration `url` | The built-in address. ADR 0010. Not on the SQL surfaces by tickets 0109 decision 2 and 0110 decision 5 |
| Key | `THINKTHEN_API_KEY` | None; required for a live call. ADR 0010, `backends.md`. Environment only on every surface but Rust; the same SQL records |
| Model | `--model`, `model`, configuration `model` | `jev-latest`. `backends.md:55`. No reason on record for the alias over a pinned version: gap. The open default-model issue in `issue-backlog-2026-09-25.md` section C holds it |
| Backend profile | `--profile` | None. ADR 0032 |
| Calibration identity | `profile` in the question file | Absent. ADR 0032, `question-file.md:91` |
| Timeout | `--timeout` | 30 seconds. `backends.md:61`. No reason on record: gap |
| Retries | `--max-retries` | 2. `backends.md:61`. No reason on record: gap |
| Throttle | `--jobs`, library throttle, DuckDB `thinkthen_throttle` | 4, 1 to 32. `records.md:129-131` and the closed issue `2026-09-24-the-default-jobs-width-runs-past-the-documented-limit.md` |
| Request limit | library `max_requests`, SQL `max_requests` | None. ADR 0017, each surface README |
| Process request total | DuckDB `thinkthen_max_requests_total`, PostgreSQL `thinkthen.max_requests_total`, SQLite `thinkthen_max_requests_total` | None. Each surface README |
| Deadline and cancel | library and SQL deadline, cancel token or signal | No deadline, spelled -1. ADR 0041 |
| Relate time limit, relate row guard | DuckDB `thinkthen_relate_seconds`, `thinkthen_relate_holding_rows` | 60 seconds and 1,000,000 rows. ADR 0038 if it states them, else the DuckDB README. Gap if neither gives a reason |
| File directory | PostgreSQL `thinkthen.file_directory` | Empty. The PostgreSQL README |
| Key refusal | PostgreSQL `thinkthen.api_key` | Empty; a set value refuses the next call. The PostgreSQL README |
| Answer cache | `--cache`, `--no-cache`, `THINKTHEN_CACHE`, configuration `cache` | On, in the platform folder. ADR 0033 |
| Recording | `--record`, `--replay` | Off. `recording.md` |
| Prune target | `cache prune --max-size`, configuration `cache_bytes`, library and SQL `cache_bytes` (no effect) | 100,000,000 allocated bytes. Ian ruled 100 MB on 2026-09-21, ADR 0017 section 5; about 25,000 answers at one 4 KB block each. ADR 0033 keeps it as a maintenance target |
| Prune selectors | `--older-than`, `--answered-by-other-than` | None. ADR 0033 |
| Audit settings | `audit --by`, `--threshold`, `--id`, `--seed`, `--target`, `--optimize`, `--write`, `--curve`, `--pooled`, `--table` | `audit.md:22-27`: `--by question`, `--id /id`, `--seed 0`, `--target 0.9`, `--optimize accuracy`. No reason on record for 0.9: gap |
| Diff settings | `diff --key`, `--threshold`, `--compare-threshold`, `--id`, `--table` | `diff.md` |
| Status format | `status --json` | Text. ADR 0034 |
| Check settings | `check --url`, `--model`, `--timeout`, `--dry-run` | `check.md`; the address and model rows' orders apply |

The builder may split or merge audit, diff and check rows when a flag means something different there. It cites the page each default comes from. A row whose default record turns out to hold a reason moves off the gap list.

### The check

`sdlc/scripts/settings` is one Python script. `settings [PAGE]` checks `PAGE`, by default `specification/settings.md`. `settings --self-test` runs the planted cases first. The `spec` rung runs both, right after it builds the binary and puts it on `PATH`. The `spec` rung is the one rung that builds `thinkthen`, and the check reads the real help.

It reads the table from the page: the first pipe table after the heading `## Settings`, with fenced code stripped first. It then checks five things and prints one sentence per failure.

1. **Headers.** The header row equals the 17 columns above, in order. A moved, renamed, missing or extra column fails: `settings.md: column 3 is "Allowed values", expected "Default"`.
2. **Cells.** Every row has 17 cells, and no cell is empty. Each Setting cell is unique.
3. **Flags.** The check runs `thinkthen --help`, reads the commands under `Commands:`, and runs `thinkthen COMMAND --help` for each, recursing into a command that lists its own commands, such as `cache` and `transform`. It skips `help`. It collects each long flag under `Options:`. `--help` and `--version` sit on the check's one named list of flags that are not settings. Every other flag in any help must appear in backticks in some Command flag cell: `settings.md: --jobs is in the help and has no row`. Every `--flag` in a Command flag cell must appear in some help: `settings.md: --planted is in a row and in no help`.
4. **Environment names.** The check reads tracked product source under `crates/*/src/`, `libraries/` and `databases/`, skipping `tests/` and `tools/` folders and files named `tests.rs` or `*_tests.rs` or under a `*_tests/` folder. It collects each `THINKTHEN_` name written as a quoted string or read as `env.THINKTHEN_…`. Names starting `THINKTHEN_TEST_` sit on the check's one named list of test-only names. Every other name must appear in backticks in some Environment variable cell, and every name in such a cell must be one the scan found. Both directions print one sentence naming the variable. Constants such as the C header's `THINKTHEN_OK` are identifiers, not quoted, and do not count.
5. **Question-file keys.** The check reads `specification/question-file.schema.json` and collects the top-level property names of each verb entry. The verb keys and `version` sit on the check's named list of keys that are not settings: they hold the question itself and the file's format. Every other key must appear in backticks in some Question-file key cell, and every key in such a cell must be in the schema.

It prints `settings: N rows, F flags, E environment names, K question-file keys, 0 failures` and exits 0, or prints each failure sentence on standard error and exits 1.

### The surface-gap issue

Filling the rows shows settings some surfaces cannot reach. The C1 build files one new issue, `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md`, and edits no existing issue. `sdlc/issues/2026-09-25-public-library-api-gaps.md` belongs to ticket 0134 while it is in flight. The new issue covers:

- The timeout, retries and backend profile cannot be set on any library or SQL surface. `build` fixes them at 30 seconds, 2 retries and no profile (`public/settings.rs:233-238`). ADR 0017 section 5 lists the engine settings a host can reach and names none of the three. No record says the omission is deliberate, so the issue files it.
- The model cannot be set as an engine setting on any SQL surface or in C. ADR 0017 section 5 gives the model as "the question, then the engine value". A SQL or C caller can still name it in the question JSON. The issue files the missing engine value.
- The DuckDB README line 39 and the SQLite README lines 50 and 56 say `cache_bytes` caps the cache. It has no effect. The issue files the wrong claim and cites item 4 of `sdlc/issues/2026-09-25-public-library-api-gaps.md` for the removal itself.

One gap is deliberate and the issue cites it and files nothing: SQL cannot name an address or a key. Ticket 0109 decision 2, ticket 0110 decision 5 and `databases/postgresql/README.md:94` rule that the address and key come from the environment on every SQL surface. The page's address and key rows cite the same records.

### How the page keeps up with the flags

A ticket that adds or changes a setting updates its row in the same commit. `sdlc/tickets/README.md` gains that sentence. The check enforces it for the three things a machine can read: a new flag, a new product variable, or a new question-file key fails the `spec` rung until its row exists. A removed one fails until its row goes. The check cannot see a changed default, a changed allowed range, or a library or SQL spelling. Review holds those, and the "Deferred gaps" section says so.

Each batching and recognize ticket that adds a setting therefore meets the check the moment its flag enters help. It cannot land without the row. That is how the "Settings on the way" list drains into the table.

## Decisions

Each is the owner's decision under Ian's ruling. Ian can overturn any of them.

1. **Only settings on main get rows.** The design's C1 section asks for rows for every designed setting, each naming its ticket. A row for a flag that help does not show is a stale row, and the check must fail stale rows. So designed settings sit in a "Settings on the way" list under the table, each naming its ticket, and move into the table when their ticket lands. The table stays strictly true, and the check needs no exemption for planned rows.
2. **A configuration-file column, seventh.** The design lists 12 surface columns and leaves out the read-only configuration file. ADR 0033 makes it a real tier for the address, the model, the cache and the prune target. Leaving it out would hide where `url` and `model` can come from. It sits between the environment and the question-file columns, and the precedence prose places it after the question file.
3. **The precedence prose adds the configuration tier and drops ADR 0007's profile order.** The design says the profile's place comes from ADR 0007. ADR 0007's selection order of `--profile`, `THINKTHEN_PROFILE`, the file's `profile` and `jev` named a profile map that ADR 0010 and ticket 0007 removed. Today `--profile FILE` loads a backend profile and has no environment or file tier, per ADR 0032. The question file's `profile` is calibration identity, which is a different setting with its own row. The page states the orders the code follows. Ticket 0139, B0, is being revised to stop citing ADR 0007's profile order and to say that the batch, context and facts rows arrive as "on the way" lines. The build cites 0139's text once it says so, and stops if it does not.
4. **Every flag in help is a setting, including output and question-part flags.** The design rules that every flag in help needs a row. So `--details`, `--true`, `--option` and the `audit` and `diff` flags get rows. The page's opening definition makes the rule plain. Help and version are the one exempt list.
5. **The check runs in the `spec` rung, against the real help.** `lint` does not build the binary. Parsing clap's derive attributes in Python would drift from what the user sees. `spec` already builds `thinkthen` and puts it on `PATH`, so the check reads exactly the help a user reads.
6. **The Default cell names its record, and gaps are listed.** Where no record gives a reason, the page says so in one list. This follows the workspace rule that a default needs a supported reason on record, and it makes the gaps easy to find.
7. **No site change.** The website agent owns `site/`. The design says the site generates its Settings page from the table. C1 files `sdlc/issues/2026-09-26-site-builds-its-settings-page-from-the-settings-table.md` for the website owner. The fixed columns and the check make the table safe to parse.
8. **The page is a reference, not a new contract.** `specification/README.md` lists it as "Reference: every setting, with a link to the page that fixes it". A row restates a rule that its owning page fixes. When the two disagree, the owning page is the contract and the row is the bug.

## Edge cases

| Input the check meets | Expected result |
| --- | --- |
| The real page on main at build time | Exit 0, the count line |
| A row removed for a flag help still shows | Exit 1, `--FLAG is in the help and has no row` |
| A row naming a flag no help shows | Exit 1, `--FLAG is in a row and in no help` |
| Two header cells swapped | Exit 1, one sentence per column out of place |
| A header renamed, added or dropped | Exit 1, the column sentence |
| A row with 16 cells, or one empty cell | Exit 1, naming the row's Setting cell |
| Two rows with one Setting name | Exit 1, naming it |
| One flag named in two rows, such as `--threshold` | Passes |
| A flag on one command, named in a row for another | Passes. The check matches names, not command and name pairs. Deferred gap |
| A hidden flag, such as `filter --top` | Not in help, so it needs no row. A row naming only `--top` still passes, because `rank --help` shows it |
| A new product variable read with no row | Exit 1, naming it |
| A row naming `THINKTHEN_PROFILE`, which nothing reads | Exit 1, naming it |
| A `THINKTHEN_TEST_` name in product code | Skipped by the named list |
| The C constant `THINKTHEN_OK` | Not quoted, so not collected |
| A question-file key in the schema with no row | Exit 1, naming it |
| `thinkthen` not on `PATH` | Exit 1, `settings: thinkthen is not on PATH`, before any other check |
| A flag or table inside a fenced block on the page | Ignored |
| The table missing, or no `## Settings` heading | Exit 1, `settings.md: no table under ## Settings` |

## Proof

The proof is the check itself, run by the `spec` rung against the real help, the real source and the real schema. Its self-test plants each fault into a temporary copy of the real page and runs the whole check against the real binary. Each case pins the whole sentence set, as `sdlc/scripts/tickets --self-test` does.

| Case | What the copy changes | Expected, pinned |
| --- | --- | --- |
| (a) Missing row | Deletes the row holding `--jobs` | `settings.md: --jobs is in the help and has no row` |
| (b) Stale row | Adds a row naming `--planted-setting` and `THINKTHEN_PLANTED` | `settings.md: --planted-setting is in a row and in no help` and `settings.md: THINKTHEN_PLANTED is in a row and read by no product code` |
| (c) Moved column | Swaps the Default and Allowed values headers | The two column sentences for columns 3 and 4 |
| (d) Missing variable | Removes `THINKTHEN_CACHE` from its cell | `settings.md: THINKTHEN_CACHE is read by product code and has no row` |
| (e) Missing key | Removes `on` from its cell | `settings.md: on is a question-file key and has no row` |
| (f) The real page | No change | No failure |

The builder also runs each plant against the rung, not just the self-test. It edits the real page, runs `sdlc/scripts/spec`, records it red, and restores the file. For (a) the plant is the real risk: a ticket adds a flag and no row.

The four questions:

- **(a) Missing row.** It protects the ruling that every flag has a row. A new flag landed without its row fails it, which is the drift this ticket exists to stop. No check today reads help against a page. It needs no hook: the script takes a page path, as `tickets` takes a root, and runs the real binary.
- **(b) Stale row.** It protects the table from rows for settings that left, or for designed settings entered early. A removed flag with its row kept fails it. Nothing else catches a stale row. No hook.
- **(c) Moved column.** It protects the fixed column order the site parses. A reordered or renamed header fails it. Nothing else reads the header. No hook.
- **(d) Missing variable.** It protects the environment direction, which reads source, not help. A new `THINKTHEN_` read with no row fails it. The `children` check pins the rung's environment, not the product's reads. No hook.
- **(e) Missing key.** It protects the question-file direction, which reads the schema. A new schema key with no row fails it. No hook.
- **(f) The real page.** It proves the check passes on the true page, so the plants are not red for another reason.

No Rust test is added. The check is a gate script, and the rung that runs it is its test.

## Pages and help

- `specification/settings.md`: new.
- `specification/README.md`: one row for the new page, marked as a reference.
- `sdlc/scripts/README.md`: one row for `settings`, and the `spec` row names it.
- `sdlc/tickets/README.md`: the sentence "A ticket that adds or changes a setting updates its row in `specification/settings.md` in the same commit."
- `sdlc/issues/2026-09-26-site-builds-its-settings-page-from-the-settings-table.md`: new, for the website owner.
- `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md`: new, the surface gaps.
- No help text changes. No page other than these changes its rules.

## Budgets

Nonblank lines, measured with `grep -c .` on the diff.

- `specification/settings.md`: at most 110 nonblank lines. The table holds about 40 rows.
- `sdlc/scripts/settings`: at most 200 nonblank lines, self-test included.
- `sdlc/scripts/spec`: at most 4 added.
- `specification/README.md`, `sdlc/scripts/README.md`, `sdlc/tickets/README.md`: at most 6 changed together.
- The site issue: at most 15. The surface-gap issue: at most 25.
- No Rust source changes, so `sdlc/ratchet.json` does not move. No dependency. No public surface changes, so the `surfaces` rung is not required.

## Stop rules

1. Stop before crossing a budget or adding a dependency.
2. Stop if any plant stays green.
3. Stop if filling a row needs a code change, such as a setting that does nothing. Ian ruled on 2026-09-25 that no setting may do nothing. File the finding in `sdlc/issues/` and write the row as main has it. The library and SQL `cache_bytes` is the known case. Its removal is item 4 of `sdlc/issues/2026-09-25-public-library-api-gaps.md`, so the row says it has no effect on those surfaces and cites that issue. Any other setting found with no effect gets an issue.
4. Stop if the check cannot read a surface's help without a key, a network call or a backend.
5. Stop if a page on main contradicts another about a setting's default in a way the builder cannot settle from the record. File it, cite both in the row, and hand back.
6. Stop if the change needs a file another in-flight ticket owns. Tickets 0135 and 0138 change `cli/args/command.rs`. C1 merges `origin/main` before its final run and writes rows for the help main has then.

## Scope and exclusions

Excluded: any code change, any setting change, `site/`, and the batching and recognize settings, which their tickets add. The Polars and pandas spellings, which follow the library. `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` and `HOME` get no rows of their own. The cache and configuration rows name them in prose.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh read-only Claude session for design and for code.

## Complexity

Contract 1; state and timing 0; reach 1; proof 1; cost of error 1; total 4. Final level: 1. The risk is a check that passes a stale page, which plants (a) and (b) guard.

## Deferred gaps

- The check does not read the library or SQL cells against code. A renamed Python keyword or DuckDB `SET` passes. Review holds it until each surface's conformance cases can name their settings. The batching tickets B12a to B13e add shared conformance cases per setting, which could feed the check later.
- The check does not read Default or Allowed values against code. A changed default passes. Review holds it.
- The check matches flag names, not command and flag pairs. A flag dropped from one command but kept on another passes.
- The configuration file column is not checked against `config.rs`. The file has four fields and changes rarely.
- Defaults with no reason on record stay gaps: the timeout of 30 seconds, two retries, the 0.5 cut and relation cut, the `jev-latest` model alias, and `audit`'s 0.9 target. The page lists them. Each needs a measurement or a ruling.
- The site's Settings page waits for the website owner, per the issue C1 files.
- The surface gaps in `sdlc/issues/2026-09-26-settings-some-surfaces-cannot-reach.md` stay open for their owners. C1 changes no surface.

## What Ian can overturn

- Ian's ruling 11 itself.
- Decision 1: designed settings sit in a list under the table until their ticket lands. The other choice puts them in the table and teaches the check to exempt rows marked as planned.
- Decision 2: a seventh column for the configuration file.
- Decision 3: the precedence prose places the configuration file after the question file and drops ADR 0007's removed profile order.
- Decision 4: output and question-part flags are settings and get rows.
- Decision 5: the check runs in the `spec` rung.
- The precedence split: only per-call values are typed, and engine-level library settings and SQL session settings sit in the environment tier, following the batching design's section 6.
- Decision 8: the page is a reference, and the owning page stays the contract.

## Closes

No issue closes. The batching and recognize designs stay open until their last tickets land. C1 settles their "Every setting is explained in one place" row. The lander marks C1 landed in no design file, because other tickets own those files.

## Evidence

- Starts from: Ian's ruling 11 and section "C1: the settings reference" of `sdlc/issues/2026-09-26-batching-design.md` at `9b667c09`. The settings sections of `sdlc/issues/2026-09-26-recognize-design.md`. The command's long help on main at `d410ef4a`, read command by command. The environment reads in `cli/edge.rs`, `public/settings.rs`, `core/backend.rs` and `config.rs`. `question-file.schema.json`. ADRs 0007, 0010, 0017, 0030, 0032, 0033, 0034 and 0041. `question-file.md`, `backends.md` and `records.md`. The library and database sources listed under "What happens today".
- Keeps: Every setting, flag, default and help line. Every owning page stays the contract for its setting. Every rung's other checks.
- Changes: A new issue naming the settings some surfaces cannot reach. A new reference page, `specification/settings.md`, with one fixed-column table of every setting on main. A new check, `sdlc/scripts/settings`, in the `spec` rung. The ticket rule that a setting change updates its row in the same commit. An issue for the website owner.
- Proof: The check's self-test plants (a) to (e) against the real binary, source and schema, each pinned to its whole sentence set, and case (f) passes on the real page. Each plant also runs once against the real `spec` rung and is recorded red.
- Defers: Library and SQL cells, defaults and allowed values checked only by review. Flag matching by name alone. The configuration column unchecked. Defaults with no recorded reason. The site page. The batching and recognize rows, which their tickets add.
