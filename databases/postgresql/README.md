# The postgresql surface

The extension is landed. Twelve SQL functions over the one engine —
`decide`, `probability`, `choose`, `score`, `tag`, `annotate`, `details`,
`usage`, `warm`, and `recognize`, `relate`, and the beta
`thinkthen_relations` — plus the array overload of `thinkthen_decide`, the
second bulk form `postgres.md` names. Every function is `PARALLEL
RESTRICTED`; `NULL` is "not sure"; a question file names itself with the
command's spelling, `'@refund.json'`, resolved against the backend's working
directory; the six error kinds map to six SQLSTATEs with the retry signal in
the message. The width (the engine's `width` setting or `ENGINE_WIDTH`) is
the number of requests in flight, and each in-flight request holds its own
connection: 1,000 records at width 32 measured 33 pooled connections.

Every single-request function — `decide`, `probability`, `choose`,
`score`, `tag`, `annotate`, `details`, `recognize`, `relations` — carries
the `thinkthen.deadline_ms` budget (milliseconds: `-1` none, `0` spent,
positive a budget; `Userset`, so any role can bound its own call, and a
superuser can set a database-wide default with `ALTER DATABASE ... SET
thinkthen.deadline_ms`). That setting is the enforced tool on the
single-row path: a backend thread waiting on the wire cannot hear
`pg_cancel_backend` or `statement_timeout` until the send returns, so a
spent or expired budget returns the deadline kind (SQLSTATE `57014`) with
nothing sent or the sent request abandoned. The batch paths carry
PostgreSQL's own interrupt instead.

`recognize` returns the five ruled columns `(text, kind, start, end,
strength)`, used with `LATERAL`; `start` and `end` count characters, so
`substring(text from start + 1 for end - start)` is the name. `relate`
takes a query (the record id first, its text second) and returns
`(name, source, target, probability)` with the id values, so the edges join
back to the query's table; more than 255 records is a usage error.
`thinkthen_relations(body, '@names.json')` is the beta companion returning
the relations as rows.

Every argument that names a file — a question, a question set, a
recognize spec — uses the command's `'@name'` spelling, resolved by the
backend against its working directory. A string that is not JSON and
carries no `@` is refused with the usage kind naming the required form;
it is never read as a path.

The acceptance sample from
the product deck's surfaces page runs
as drawn: the `IS NULL` query returns the `maybe` row a person should read,
and `thinkthen_annotate('@form.json', body)` orders by urgency 1.7 / 1.05 /
0.99. The deck's PostgreSQL tab draws that set as the bare name
`'form.json'`; every file name on this surface carries the ruled `@`
spelling (review 2, item 4: a bare string that is not JSON was read as a
server path, so a granted role could read files with no `@` required), so
the deck's line is stale for its owner and `check.sh` runs the ruled
spelling with the divergence recorded here. The `recognize` and `relate` calls in
`recognize-surfaces.md` run as drawn too: the LATERAL call, the names-become-
rows pattern with the no-request join, and `thinkthen_relate('SELECT id, body
FROM alerts', ARRAY['caused_by'])`.

`check.sh` packages the extension, proves it in a disposable `postgres:16`
container on the host network (picking a free port itself, because sibling
sessions race fixed ones), runs the slide sample, the offline conformance
slice through `runner.py`, the recognize/relate acceptance, the 255-record
refusal, and the wire suite when the loopback stub is up on 8219, and
removes the container afterwards. The running log is `NOTES.md`.

The known divergences: the stand-in never credits its in-process memory in
`cache_answers` (its own record says so; a real-engine requirement). Case
71's per-subject arm shares its input with the pairs arm and the stand-in
serves the ruled pairs form: a conformance-data finding for the build team.
The contract's settled `nearest` field rides `thinkthen_details` here like
every other member — PostgreSQL serializes the contract's own `Details`
struct, so the field needed no wiring in this shim — and the conformance
slice proves it: case 13's `nearest_level` (`mid`) is compared against
`(details->>'nearest')` in `runner.py`.

## Authority: who may do what

- **Who may call.** `CREATE EXTENSION` revokes the default PUBLIC grant
  on every function it installs — both `thinkthen_decide` overloads and
  the `thinkthen_warm` aggregate included — so an unprivileged role
  cannot make a paid call or read a file; superusers keep access by
  their own right. The grant for an application role names the
  extension's own functions through `pg_depend`, never `GRANT … ON ALL
  FUNCTIONS IN SCHEMA public`, which would also grant every other
  extension's functions (review 2, item 4):

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

  The grant is the trust: a role holding it can make paid calls, and can
  read files only by the file rule below.
- **Question-file access.** `'@name'` reads one file, resolved exactly
  as named, only through the file gate: the file must be a regular file
  of at most 1 MiB, and the calling role must hold the privileges of
  `pg_read_server_files` (PostgreSQL's own rule for server-file reads,
  which superusers satisfy) or read inside the one directory an
  administrator configured as `thinkthen.file_directory`. The path is
  opened once and never through a symlink as its last component; a
  symlink in a middle directory is followed. Confinement judges the
  opened file itself: its own path must sit inside the directory, and it
  must have exactly one link, so a hard link placed inside cannot name
  an outside file. Linux reads that path from `/proc/self/fd` and macOS
  from `fcntl(F_GETPATH)`; any other platform refuses every confined
  read. Confinement is settled before the size, so a path outside the
  directory gets the one refusal whatever its size, and only a file the
  role may read can answer "over the cap". Every unreadable cause
  carries one message, so a refused read tells nothing about the
  filesystem (review 3, items 2 and 8; review 4, item 7; review 5). `thinkthen_relate` reads no file: it
  runs the query text it is given through SPI. Nothing else is read.
- **An update's new functions.** The event trigger re-revokes PUBLIC on
  functions the extension itself creates, and only those: an
  administrator's grant on any other function survives it. One
  boundary, stated rather than fixed: `session_replication_role =
  replica` disables event triggers entirely, so a replica-session update
  bypasses the guard; an update script that adds functions should carry
  its own revoke block beside the trigger.
- **Backend selection.** The engine builds lazily in each backend, after
  the fork, from the server process's environment (`THINKTHEN_BASE_URL`,
  or the stand-in's `ENGINE_BASE_URL`; `ENGINE_NULL=1` for the in-process
  backend). SQL cannot name a backend, and `_PG_init` registers settings
  and touches nothing else.
- **Credential source.** The ruled channel is the `thinkthen.api_key`
  setting (`Suset`, no view shows it, `ALTER SYSTEM` cannot write it).
  This engine build takes no key from the host, so a set, non-blank value
  refuses every call with a usage error naming the setting and the
  engine's own channel, `THINKTHEN_API_KEY` in the server environment —
  a configured credential is never silently ignored, and the value never
  rides a message or the log. Which channel wins when both are present
  is the database ADR's open question 4; the refusal is the placeholder
  until it is answered.
- **Query execution.** The backend runs the SQL on its main thread, and
  every judging function is `PARALLEL RESTRICTED`, so a parallel plan
  cannot multiply the width. The batch paths (the array overload and
  `thinkthen_warm`) run the engine call on one worker thread while the
  backend polls its interrupt flag; `thinkthen_relate` executes its
  query through SPI and stops reading at the 256th row. No thread lives
  between calls.
- **Connection lifetime.** The engine and its pool live as long as the
  backend process; the engine stamps its process ID and rebuilds after a
  fork, and nothing is built in the postmaster.
- **Saved answers.** Every decision a warm pass or a single
  `thinkthen_decide` judges is saved in the backend, keyed by the
  question and the evidence, and `thinkthen_decide` reads it before
  sending. The table holds at most 16 MiB of question digests and
  evidence per backend. Past that the oldest answers leave first, and a
  later decide on an evicted pair sends again. `thinkthen_usage` counts
  an answer read back from the table in `cache_answers`.
- **Cancellation channel.** PostgreSQL's own interrupt flag: the batch
  poll reads `InterruptPending`, cancels the engine's token, and the
  proper error raises through `check_for_interrupts!()`, so
  `pg_cancel_backend` and `statement_timeout` both stop a waiting batch
  (the check measures 0.32 s and 1.31 s). A single-row call carries no
  token — the flag is only read when it returns — so its enforced tool is
  the `thinkthen.deadline_ms` budget above.
- **Batch errors.** A question resolves on the backend thread before any
  worker spawns, so a bad question file raises its own error (naming the
  file) instead of dying inside the worker; a worker that does panic is
  contained and reported as the defect kind carrying the panic's own
  message.
