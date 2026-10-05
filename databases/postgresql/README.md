# The PostgreSQL surface

`CREATE EXTENSION thinkthen` adds the engine's verbs to SQL. The extension is the unpublished crate `thinkthen-postgresql`, built with pgrx 0.17 over the public `thinkthen` API (ticket 0111, ADR 0047). Every call reaches the real engine. `WHERE` and `ORDER BY` do the work of filter and rank. An ordered array gives find one complete set.

## Functions

The ordinary judgment signature is `question, input[, members][, settings]`. `settings` is PostgreSQL `json`; use `::json` for a positional literal. Its accepted keys and value rules are in [settings](../../specification/settings.md). Optional named parameters use `DEFAULT NULL`, so a call can use `threshold => '0.3:0.7'`, `context => 'reference text'`, `model => 'judge-b'`, `batch => 'max'`, or `deadline_ms => 3000` without filling earlier slots. A named field and the same settings key together refuse before sending. `choose`, `score`, and `tag` accept a `members text[]` parameter; omit it when the JSON question or settings names the list.

| Function | Returns |
| --- | --- |
| `thinkthen_decide(question, input[, settings])` | `true`, `false`, or SQL `NULL` for an unresolved decision |
| `thinkthen_choose(question, input[, members][, settings])` | selected text, or SQL `NULL` for no clear pick |
| `thinkthen_score(question, input[, members][, settings])` | numeric score |
| `thinkthen_tag(question, input[, members][, settings])` | matching labels as `text[]` |
| `thinkthen_decide_many`, `thinkthen_choose_many` | keyed `(key, value, probability)` rows |
| `thinkthen_score_many`, `thinkthen_tag_many` | keyed `(key, value)` rows; probability is not defined for these verbs |
| `thinkthen_rank(question, keyed_input[, settings])` | keyed `(key, rank, probability)` rows as `text, bigint, double precision`, best first |
| `thinkthen_plan(question, keyed_input[, settings])` | `jsonb` request-count and byte/token-band preview, with no send |
| `thinkthen_find(question, units text[][, settings])` | selected original index, value, probability and ordered candidates as `jsonb` |
| `thinkthen_annotate(set, input[, settings])` | each question's value as `jsonb`; settings apply batch, deadline or the engine's default model |
| `thinkthen_details`, `thinkthen_try_details` | detailed answer or recoverable typed result as `jsonb`; both take the judgment's named settings |
| `thinkthen_recognize`, `thinkthen_relations`, `thinkthen_relate` | their typed entity and relation rows; a relation row's last column, `either`, is true for a both-ways rule, whose ends are then in input order |
| `thinkthen_usage()` | this backend's `(requests_sent, cache_answers, input_tokens, output_tokens)` |

Each `_many` input is one `jsonb` object from caller keys to text. Join `d.key = CAST(s.id AS text)`; no array position is inferred. PostgreSQL `jsonb` normalizes repeated object keys before the extension sees them. A scalar judges one row per call; `_many` gives the engine the whole keyed set and packs it according to `batch`. The four keyed functions preserve SQL `NULL` on unresolved values and retain the key. Decide and choose report probability on the row; score and tag refuse a probability request. `thinkthen_plan` returns `records`, `requests`, `estimated_bytes`, an `estimated_input_tokens` lower/upper band, `upper_bound`, and `first_body_utf8`. The plan has no key requirement and no network call; `requests` is before cache answers, refusal splits and retries.

`thinkthen_rank` orders one keyed `jsonb` object by the probability of yes. `rank` runs 1, 2, 3 with no gaps, and `probability` is the yes probability that orders the rows. All the records go to one engine call, packed according to `batch`. Its question is nonblank literal text, including a literal `@` or `{`. Settings take `model`, `batch`, `context` and `deadline_ms`; `threshold`, `true`, `false`, `options`, `levels`, `labels` and `none` raise `usage` before a send. Equal probabilities come back in bytewise key order, not the order written, because the extension reads the object into a sorted map. An empty object or a SQL `NULL` object returns no rows without a send, and a `NULL` question raises `usage`. Build the object with `jsonb_object_agg`, and `ORDER BY rank` again after a join:

```sql
SELECT t.id, refund.rank, refund.probability
FROM tickets t
JOIN thinkthen_rank('Does the writer ask for a refund?',
                    (SELECT jsonb_object_agg(id, body) FROM tickets)) refund ON refund.key = CAST(t.id AS text)
ORDER BY refund.rank;
```

A judgment question is plain text, JSON text starting with `{` after leading white space, or a file named with the first-byte `'@refund.json'` spelling. Bare text, including `refund.json`, is a question and never a path. A question set or recognize/relation spec remains JSON or `@file`. To ask a literal question beginning with `@` or `{`, use a JSON question or named file.

Find's question is nonblank literal text, including a literal `@`. Its ordered `text[]` is one set, not independent records; use `array_agg(unit ORDER BY ordinal)` to preserve positions and duplicates. Settings `{"none":true}` offers a final none candidate. An empty array or SQL NULL argument returns SQL NULL without a send. A selected none returns a non-NULL object with null index and value. A NULL array member, blank text, one unit, more than 255 units (254 with none), or more than 16 MiB of combined unit text raises usage before a send. `deadline_ms` in settings overrides the existing `thinkthen.deadline_ms` GUC for that call. PostgreSQL statement cancellation still applies.

`start`, `end`, and `length` count characters, so `substring(body from start + 1 for length)` is the name. The relate query returns `(id, name)` or `(id, name, kind)`. A two-column query takes bare relation names. Inline rules read `NAME` or `NAME=SOURCE:TARGET`, as the command's `--relation` does. The query may return at most 255 rows.

`check.sh` runs this sample:

```sql
SELECT id FROM (
  SELECT id, thinkthen_decide('@refund.json', body) AS asks_refund FROM tickets
) AS judged WHERE asks_refund IS NULL;
SELECT id, triage->>'team', (triage->>'urgency')::float AS urgency
FROM tickets, thinkthen_annotate('@form.json', body) AS triage ORDER BY urgency DESC;
```

The old `thinkthen_warm`, `thinkthen_probability`, positional context, find's positional `none`, and decide array forms raise `usage` with replacement advice. `thinkthen_probability` points to `thinkthen_rank`. They do not silently send.

## Constrain a stored answer

`thinkthen_choose` returns plain `text`. A caller-owned PostgreSQL domain constrains a stored answer without changing that function:

```sql
CREATE DOMAIN team_label AS text CHECK (VALUE IN ('billing', 'shipping'));
CREATE TABLE judged (id bigint, team team_label);
INSERT INTO judged
SELECT id, team FROM (
  SELECT id, thinkthen_choose('Which team owns this?', body, ARRAY['billing', 'shipping']) AS team
  FROM tickets
) AS choices;
SELECT id, team FROM judged WHERE team = 'billing';
```

The domain rejects a different non-`NULL` label. A stored `NULL` can represent a choice below the cut or an exact tie. The domain permits `NULL`. With a valid question, `NULL` evidence also returns SQL `NULL` without a judgment; a `NULL` question instead raises `usage` during question parsing. A failed ThinkThen call raises its named error; do not replace the error with `NULL` merely to pass the domain. The constraint belongs to the stored column, and the function keeps its existing `text` result.

## Run facts

`thinkthen_details(question, evidence)` returns the command's `--details` line for one text as `jsonb`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. This SQL details view does not supply invocation cost or elapsed seconds.

The details digest includes a question's saved calibration `profile`. A different runtime `thinkthen.profile` name appears as `meta.profile_warning` with `tuned_for` and `running`. The selected runtime profile checks limits before sending.

`thinkthen_try_details(question, evidence)` returns `jsonb` with `status: "answered"` and the full details object, or `status: "failed"` and an error with public `kind`, fixed safe `message`, and typed `retryable`. It returns SQL NULL when either input is SQL NULL, before reading a question file or settings. An unresolved answer is answered with JSON `null` in its details. Usage, local, and backend row failures let later rows complete. Cancellation, deadline, and defect still raise. The failed value includes no question, evidence, key, file path, cache path, or backend address.

`thinkthen_usage()` returns this backend process's running totals of requests sent, cache answers and tokens.

## Price and elapsed time

SQL plans report an estimated input-token band before cache hits, retries or refusal splits. They do not predict output tokens, total dollars or future duration. SQL usage reports cumulative process totals, not the facts of one isolated call. Packed request metadata can appear on more than one result row; summing those rows counts the same request more than once.

`thinkthen_plan` previews one judgment question, not an annotate question set or a whole SQL pipeline. `thinkthen annotate --plan` previews CLI question-set packing; match the question set, backend, model and record framing, and treat that preview as a guide for SQL calls whose packing can differ. Reduce the planned cohort before calls when the configured allowance cannot admit the pipeline. Estimates neither authorize a larger budget nor predict the provider's bill.

Measure wall time around the SQL statement in the client. This includes database and client work and is not engine-only time. For an external price estimate, apply a known input/output tariff to complete provider-reported usage with decimal arithmetic. The provider's invoice determines actual charges. Missing rates or incomplete attempt usage mean unknown cost, not zero. Cumulative SQL counters do not prove usage completeness for failed attempts, and subtracting shared counters cannot isolate concurrent calls.

## Settings

| Setting | Who sets it | Meaning |
| --- | --- | --- |
| `thinkthen.deadline_ms` | any role | the per-call budget in milliseconds. -1 means none, and 0 means already spent |
| `thinkthen.throttle` | superuser | requests in flight at once, 1 to 32. -1 keeps the engine's value. PostgreSQL refuses any other value where it is set. The refusal reads `thinkthen usage: a throttle is a whole number from 1 through 32`. `SET` fails, and a configuration file's bad value draws the refusal as a warning and leaves -1 |
| `thinkthen.max_requests` | superuser | the most records one call answers. -1 means no limit |
| `thinkthen.max_request_bytes` | any role | positive request-byte ceiling for relation plans. -1 keeps the environment value |
| `thinkthen.batch` | any role | `max` or a decimal whole number of 1 or more records per request. Empty keeps `THINKTHEN_BATCH` or the default `max`; `1` retains historical singleton request bytes |
| `thinkthen.max_requests_total` | superuser | the most requests one backend sends. -1 means no total |
| `thinkthen.backend` | superuser | built-in or configured name; empty restores captured environment selection |
| `thinkthen.model` | any role | backend model. Empty keeps the environment value |
| `thinkthen.timeout` | any role | positive attempt timeout in seconds; -1 keeps the environment value |
| `thinkthen.max_retries` | any role | status retries, 0 or more; -1 keeps the environment value |
| `thinkthen.profile` | any role | version-one backend profile as JSON text; empty keeps the environment value |
| `thinkthen.record`, `thinkthen.replay` | superuser | absolute folders for live recording or strict offline replay. Empty keeps the environment value |
| `thinkthen.cache` | superuser | an absolute answer cache folder. `off` disables it; empty keeps `THINKTHEN_CACHE`, or no cache when that is unset |
| `thinkthen.file_directory` | superuser | the one folder an unprivileged role may read named files from. A relative `'@name'` resolves inside it |
| `thinkthen.api_key` | any role | never read by the engine; a nonempty interactive `SET` warns, and the next call refuses until reset |

The answer cache is off unless an administrator names a folder, by `THINKTHEN_CACHE` in the server's environment or by `thinkthen.cache`. The platform folder under the server's home is never used (ticket 0318). Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it. So a call refuses, before any request, a named cache, record or replay folder that another operating-system user owns or others can write; make it the server user's own with mode 0700. `cache prune` is the only thing that removes entries. Set `thinkthen.cache = 'off'` to disable a cache `THINKTHEN_CACHE` names.

The folder belongs to the server's operating-system user. Every role whose calls resolve to the same folder shares its answers, so row text leaves the database's own access control, row-level security included, and `meta.cached` tells one role that another already judged the same text. Name a shared folder only when every calling role may see every judged row. When roles must not share answers, give each its own folder with `ALTER ROLE ... SET thinkthen.cache`.

`SET thinkthen.backend = 'typesafe'` selects a named backend. `RESET` or an empty string restores captured environment selection. An ordinary role cannot change this setting, including a custom placeholder staged before module load. SQL accepts no address or key. The engine captures the server environment after fork. Changing the name changes the validated engine plan, while existing process totals and throttle ownership remain.

The throttle holds for the whole backend process. The first explicit throttle stays until the backend exits. A later equal value works; a different value raises usage with the active width. An administrator's `ALTER ROLE ... SET` applies an engine setting to one role.

`thinkthen.max_requests_total` caps attempted live sends in one backend, including retries and requests inside annotate or relate. An atomic reservation admits every transport attempt. A cache or replay answer can still complete after the send total is spent. A later request or split child that needs transport then raises 22023; previously completed attempts remain counted. A new connection forks a backend with a new total. A pool of N connections can spend up to N times its per-backend total. A cancelled call's send already on the wire remains counted. Each backend adds its sends to the count-only usage totals of the server's operating-system user when it exits, so `thinkthen status` run as that user shows the whole server's spend (ADR 0113). The total and the token cap still bind per backend.

## Errors

Each failure raises PostgreSQL's error with the message `thinkthen <kind>: <message> (retryable: yes|no)`.

| Kind | SQLSTATE |
| --- | --- |
| usage | 22023 |
| backend | 38000 |
| local | 58030 |
| cancelled, deadline | 57014 |
| defect | XX000 |

`pg_cancel_backend`, `pg_terminate_backend`, and `statement_timeout` stop any call within one 50 ms tick (ADR 0043, as amended). A send already on the wire still completes and is billed, and its answer lands in the cache.

## Authority: who may do what

- **Who may call.** `CREATE EXTENSION` revokes PUBLIC's grant on every function it installs. An event trigger revokes it again on any function a later update adds, and an administrator's grant on another function survives. Superusers keep access by their own right. A grant names the extension's own functions through `pg_depend`, never `ON ALL FUNCTIONS IN SCHEMA public`. The grant is the trust: a role that holds it can make paid calls.

The narrowed grant, byte for byte (`fixtures/grant.sql`):

```sql
DO $thinkthen_grant$
DECLARE signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d
          ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e
          ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('GRANT EXECUTE ON FUNCTION %s TO %I',
                       signature, 'the_app_role');
    END LOOP;
END
$thinkthen_grant$;
```

- **Named files.** `'@name'` reads one regular file of at most 1 MiB, opened once and never through a final symlink. The role needs the privileges of `pg_read_server_files`, or the file must sit inside `thinkthen.file_directory`. When `thinkthen.file_directory` is set, a relative name such as `'@refund.json'` names a file inside that folder, for every role. A superuser who keeps relative-named files in the data folder names them absolutely once the folder is set. When the folder is unset, a relative name starts from the backend's working folder, the server's data folder, and only a role with `pg_read_server_files` reads it. A confined path is judged before any open. Linux opens it beneath the folder with `openat2`. macOS opens it with `openat` and refuses a symlink at any step, even one that stays inside the folder, and it needs read permission on the folder. The opened file must have one link and a path inside the folder. Every unreadable cause gives one message, so a refusal tells nothing about the filesystem. `thinkthen_relate` reads no file. It runs its query through SPI.
- **The key.** The engine reads `THINKTHEN_API_KEY` from the server's environment and nothing else. `SET thinkthen.api_key` succeeds and a nonempty interactive value warns; the next call refuses with 22023 until reset. A server with `log_statement = all` or `pg_stat_statements` can record the `SET` statement, so do not put a real key there.
- **The backend.** The engine builds in each backend on its first call, after the fork, from the server's environment. SQL cannot name an address. `_PG_init` registers settings and sends nothing.
- **Threads.** Each call runs on a worker thread that blocks every signal, so PostgreSQL's handlers run only on the backend thread. Every function is `PARALLEL RESTRICTED`.
- **One boundary.** `session_replication_role = replica` disables event triggers, so an update script that adds functions carries its own revoke.

## The check

`check.sh` runs on Linux, and on macOS with the Homebrew bottle `runtime-darwin.env` pins. On Linux it builds the extension against `/usr/bin/pg_config`, then runs PostgreSQL 16.15 from Ubuntu's server package as this user. The server listens on a socket in a private temporary folder and on no TCP port. Each test restarts it with its own loopback backend (ticket 0117) and cache folder. The server gets a fake key beside a 127.0.0.1 address alone. One-time setup fetches the package at `runtime.url` into `~/.cache/thinkthen-toolchains/postgresql/`, and `runtime.sha256` pins its hash. A missing package or tool reports "not run" with the fetch command. `NOTES.md` records what the port found.
