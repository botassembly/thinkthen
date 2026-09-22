# The duckdb surface

The SQL functions over the one engine, per ADR 0017 and the database
pages: `thinkthen_decide`, `thinkthen_probability`, `thinkthen_choose`,
`thinkthen_score`, `thinkthen_tag`, `thinkthen_annotate`,
`thinkthen_details`, `thinkthen_usage`, `thinkthen_warm`, and the
recognize pair `thinkthen_recognize` and `thinkthen_relations` (beta),
beside `thinkthen_relate`. `filter`, `rank`, and `find` get no functions;
`WHERE`, `ORDER BY`, and `LIMIT` are those verbs. `NULL` is "not sure",
and a failure is an error that never reads as `NULL`.

The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`,
runs as drawn against the stand-in: `tools/run_slide.sh` extracts the
slide's code block verbatim and runs it in the stock CLI. One finding for
the slide's owner sits in NOTES: the sample's `choose` column reads NULL
against the offline stand-in (its options carry no keyword the offline
rules can lift) and the loopback stub cannot answer a choice question at
all, because it answers one probability per request. The shape is proven;
the values await a backend that distinguishes options.

## The ruled shape this surface takes

- `thinkthen_recognize(body, kinds)` returns a `LIST` of structs
  `(text, kind, start, end, strength)`; `unnest()` makes rows. `start`
  and `end` count code points — the recordings' unit and DuckDB's own
  string indexing — so `body[start + 1 : end]` slices the name back out.
- `thinkthen_relations(body, '@names.json')` (beta) returns a `LIST` of
  structs `(name, source, source_kind, target, target_kind, probability)`
  from the question file's `recognize` section; `source` and `target`
  hold the entity's text, not its id, under the ruled end names, and
  `source_kind` and `target_kind` hold the kind words. The spec is
  a column argument, so the call sits in a `FROM` like
  `thinkthen_recognize` and `unnest()` makes rows.
- `thinkthen_relate(query, rules)` takes a query whose first column is
  each record's id and whose second is its text, and returns rows
  `(name, source, target, probability)`. The deck drew a raw subquery in
  the first argument; the stable C API registers no table function that
  takes one — the binder refuses subqueries for every function but a
  table-in-out function, and the C API cannot register those — so the
  query crosses as a string, the shape PostgreSQL's row takes. Ids come
  back as their text, so an integer id joins with one cast. More than 255
  records is a usage error before anything is asked.
- A question argument is plain text under the grammar's default cut, a
  file named `'@refund.json'`, or the file grammar's own JSON. One door,
  one grammar; the surface adds no parser.
- A verb whose members ride in a `LIST` argument — `choose`, `score`,
  `tag` — takes its question as plain text and its members from the list.
  A file or JSON question beside a list is a usage error, because the
  members would come twice.
- `thinkthen_score` returns the specification's position from 0 to K−1;
  the nearest level's name rides in `thinkthen_details.nearest`.
- `thinkthen_details` is one struct for every verb. For a decide it
  carries the probability, the answer's word, the model, the digest, and
  the sends that produced the judgment. For the other verbs the
  yes-probability does not exist, so probability, answer, and sends read
  NULL and the question's own model and digest carry the audit; a score
  question also names its nearest level.
- `thinkthen_annotate` returns a JSON object, one field per question in
  the set's name order, because a scalar's return type is declared at
  registration and a set's questions are not known there. DuckDB's own
  JSON functions read the object.
- `thinkthen_warm(question, text)` is the aggregate: it collects the
  distinct texts a scan sees and judges them once through the engine's
  batch door. The width (the engine's `width` setting or `ENGINE_WIDTH`)
  is the number of requests in flight, and each in-flight request holds
  its own connection: 1,000 records at width 32 measured 33 pooled
  connections.
- Every call carries the process-wide cancel token, and the extension
  takes SIGINT at LOAD and chains to the CLI's own handler, so a Ctrl-C
  stops between requests. The punch-list proof is `tools/host_signal.py`,
  run by `check.sh`: a host handler installed after LOAD fires and the
  extension keeps answering, and a handler installed before LOAD is
  chained to while a running query stops. The token is one shot per
  process — the proof's `note` line and the punch-list report record the
  long-lived-host finding.
- There is no per-call deadline option on this surface yet: the host's
  own statement timeout is the stop, and conformance case 27 is skipped
  for that reason. A per-call budget beside the cancel token is the
  settled shape and a recorded gap here, named rather than implied.

## Authority: who may do what

The boundary the architect drew, per item: this extension converts
arguments, coordinates the host, calls the engine, and presents results.
Everything else — parsing, scheduling, retries, cache, counters,
recognition, relations — is the engine's.

- **Question-file access.** The only file a call reads is the one its
  question argument names with the command's `'@name'` spelling, resolved
  against the process working directory. Nothing else is read: no
  configuration file, no directory listing, no table.
- **Backend selection.** The engine builds lazily on the first call from
  the process environment (`THINKTHEN_BASE_URL`, or the stand-in's
  `ENGINE_BASE_URL`; `ENGINE_NULL=1` for the in-process backend). SQL
  cannot name a backend, and LOAD registers the functions and the SIGINT
  handler and touches no wire.
- **Credential source.** `THINKTHEN_API_KEY` in the process environment,
  read by the engine at send time and sent only to the named address. The
  stand-in reads no key and sends none (its own record); no credential is
  ever read from a fixture, a question file, or SQL, and none is logged.
- **Query execution.** DuckDB's executor runs the SQL and calls the
  scalar functions inside its own execution, on the threads it uses; the
  stable C API offers no parallel-safety marking for a scalar function.
  The aggregate and the relate query run through the engine's bulk doors
  at the width, which is the number of requests in flight. The extension
  owns no thread between calls.
- **Connection lifetime.** The engine and its pool live as long as the
  host process; the engine's process-ID check repairs a fork on the next
  call, idle connections are pruned by the engine, and the pool is sized
  to the width gate.
- **Cancellation channel.** SIGINT, taken at LOAD and chained to the
  handler that was there. The handler cancels the process-wide token that
  every call carries, so a stop lands between requests. A host that
  installs its own handler after LOAD displaces the extension's and keeps
  its own (proven by `tools/host_signal.py`); the token is one shot per
  process, which the CLI never notices and a long-lived host should treat
  as a restart signal.

## Build and run

The version pin is the trap 207 found: the Rust path builds against the
unstable C API, so the extension and the CLI must be the same version.
`TARGET_DUCKDB_VERSION=v1.5.5` in the Makefile, and the stock v1.5.5 CLI
in `duckdb-bin/` (fetched user-level; the machine CLI is older and cannot
load what this path builds).

```
make configure && make release
./duckdb-bin/duckdb -unsigned -c "LOAD '$PWD/build/release/thinkthen.duckdb_extension'; \
  SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today');"
./check.sh
```

Nothing published. No container needed for the checks; the wire suite
wants the loopback stub on port 8217.
