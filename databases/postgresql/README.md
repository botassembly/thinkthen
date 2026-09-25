# The PostgreSQL surface

`CREATE EXTENSION thinkthen` adds the engine's verbs to SQL. The extension is the unpublished crate `thinkthen-postgresql`, built with pgrx 0.17 over the public `thinkthen` API (ticket 0111, ADR 0047). Every call reaches the real engine. `WHERE` and `ORDER BY` do the work of filter, rank, and find.

## Functions

| Function | Returns |
| --- | --- |
| `thinkthen_decide(question, evidence)` | `true`, `false`, or `NULL` for not sure |
| `thinkthen_decide(question, evidences text[])` | one row `(i, decided)` per element, `i` from 0 |
| `thinkthen_probability(question, evidence)` | the yes probability of a decide question |
| `thinkthen_choose(question, evidence, options text[])` | the picked option, or `NULL` |
| `thinkthen_score(question, evidence, levels text[])` | the position on the levels |
| `thinkthen_tag(question, evidence, labels text[])` | the labels that apply, as `text[]` |
| `thinkthen_annotate(set, evidence)` | each question's value, as `jsonb` |
| `thinkthen_details(question, evidence)` | the engine's detailed result, as `jsonb` |
| `thinkthen_warm(question, evidence)` | an aggregate that judges each distinct pair once and fills the answer cache |
| `thinkthen_recognize(body, kinds text[])` or `(body, spec)` | `(name, kind, start, end, strength)` rows |
| `thinkthen_relations(body, spec)` | `(relation, source_name, source_kind, target_name, target_kind, probability)` rows |
| `thinkthen_relate(query, rules text[])` or `(query, spec)` | `(relation, source, target, probability)` rows, with the query's ids |
| `thinkthen_usage()` | `(requests_sent, cache_answers, input_tokens, output_tokens)` for this backend |

A question, set, or spec is JSON text in the file grammar, or a file named with the `'@refund.json'` spelling. Bare text is never a path. For `choose`, `score`, and `tag`, the array joins the question as its options, levels, or labels. Pass `NULL` when the question already names them.

`start` and `end` count characters, so `substring(body from start + 1 for "end" - start)` is the name. The relate query returns `(id, name)` or `(id, name, kind)`. A two-column query takes bare relation names. Inline rules read `NAME` or `NAME=SOURCE:TARGET`, as the command's `--relation` does. The query may return at most 255 rows.

The slide sample runs as drawn:

```sql
SELECT id FROM tickets WHERE thinkthen_decide('@refund.json', body) IS NULL;
SELECT id, a->>'team', (a->>'urgency')::float AS urgency
FROM tickets, thinkthen_annotate('@form.json', body) AS a ORDER BY urgency DESC;
```

## Settings

| Setting | Who sets it | Meaning |
| --- | --- | --- |
| `thinkthen.deadline_ms` | any role | the per-call budget in milliseconds. -1 means none, and 0 means already spent |
| `thinkthen.throttle` | superuser | requests in flight at once, 0 to 32. -1 keeps the engine's value |
| `thinkthen.max_requests` | superuser | the most records one call answers. -1 means no limit |
| `thinkthen.max_requests_total` | superuser | the most requests one backend sends. -1 means no total |
| `thinkthen.cache` | superuser | the answer cache folder. Empty keeps `THINKTHEN_CACHE` or the platform folder |
| `thinkthen.cache_bytes` | superuser | the cache cap, such as `'500MB'`. -1 keeps the configured value. 0 refuses every call. The ceiling is 2,147,483,647 bytes |
| `thinkthen.file_directory` | superuser | the one folder an unprivileged role may read named files from |
| `thinkthen.api_key` | nobody | never read. A set value refuses the next call |

The throttle holds for the whole backend process. The first explicit throttle stays until the backend exits, and a later, different value is not applied. An administrator's `ALTER ROLE ... SET` applies an engine setting to one role.

`thinkthen.max_requests_total` caps spending on a large query, where each row is its own call. Before each call the backend adds the requests its engines have sent. Once the total is spent, the call refuses with 22023 and sends nothing. A batch with more records than remain sends only the ones that fit, then refuses. The total belongs to one backend process: each new connection forks a backend that starts from zero. A cancelled call's send already on the wire, and the engine's retries, can each pass the total by one call.

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

- **Named files.** `'@name'` reads one regular file of at most 1 MiB, opened once and never through a final symlink. The role needs the privileges of `pg_read_server_files`, or the file must sit inside `thinkthen.file_directory`. A confined path is judged before any open and opened beneath the folder with `openat2`. The opened file must have one link and a path inside the folder. Every unreadable cause gives one message, so a refusal tells nothing about the filesystem. `thinkthen_relate` reads no file. It runs its query through SPI.
- **The key.** The engine reads `THINKTHEN_API_KEY` from the server's environment and nothing else. `SET thinkthen.api_key` succeeds and the next call refuses with 22023, so the value never reaches the log.
- **The backend.** The engine builds in each backend on its first call, after the fork, from the server's environment. SQL cannot name an address. `_PG_init` registers settings and sends nothing.
- **Threads.** Each call runs on a worker thread that blocks every signal, so PostgreSQL's handlers run only on the backend thread. Every function is `PARALLEL RESTRICTED`.
- **One boundary.** `session_replication_role = replica` disables event triggers, so an update script that adds functions carries its own revoke.

## The check

`check.sh` runs on Linux only. It builds the extension against `/usr/bin/pg_config`, then runs PostgreSQL 16.15 from Ubuntu's server package as this user. The server listens on a socket in a private temporary folder and on no TCP port. Each test restarts it with its own loopback backend (ticket 0117) and cache folder. The server gets a fake key beside a 127.0.0.1 address alone. One-time setup fetches the package at `runtime.url` into `~/.cache/thinkthen-toolchains/postgresql/`, and `runtime.sha256` pins its hash. A missing package or tool reports "not run" with the fetch command. `NOTES.md` records what the port found.
