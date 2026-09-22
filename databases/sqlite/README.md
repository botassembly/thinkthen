# The sqlite surface

Landed 2026-09-21. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in: `thinkthen_warm` fills the answers in one
pass at the process width, and the queries read them row by row. The width
is the number of requests in flight, and each in-flight request holds its
own connection: 1,000 records at width 32 measured 33 pooled connections.

## The ten functions

`thinkthen_decide`, `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`,
`thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`,
`thinkthen_warm` — the ruled names with the `thinkthen_` prefix. `NULL` is
"not sure". A question argument is plain text, a JSON question, or a
question file named with the command's spelling `'@refund.json'`, resolved
relative to the process working directory. `thinkthen_usage` is
cumulative and is never reset; take two snapshots and subtract them for a
delta, and the removed `'reset'` spelling refuses with a usage error
naming that substitution.

`thinkthen_recognize` and `thinkthen_relate` are table-valued functions
over the same engine. `SELECT t.id, n.text, n.kind FROM tickets t,
thinkthen_recognize(t.body, 'person,organization') n` answers one row per
name with the five columns `(text, kind, start, end, strength)`, the
offsets indexing the text the way SQLite's own `substr` counts, so
`substr(body, start + 1, end - start)` is the name. `SELECT * FROM
thinkthen_relate('alerts', 'id', 'body', 'caused_by')` reads the whole
named table at once — the one function a database cannot run row by row —
and answers one row per edge `(name, source, target, probability)`, with
the id column's values riding as `source` and `target`. More than 255
records is refused with a usage error naming the limit.

Each relate rule argument takes the ruled grammar, and every spelling is
parsed by the contract's own reader — this surface adds no parser:

- `'caused_by'` — a bare name, any kind to any kind (the deck's call).
- `'caused_by=person:organization'` — named ends, `*` for any kind at an
  end; `'caused_by=*:*'` equals the bare name.
- `'either:same_as'` or `'either:same_as=person:person'` — the both-ways
  rule, the `--either` option's spelling on this surface.
- `'{"relations":["caused_by"]}'` — the question file's `relate`
  section as inline JSON.
- `'@links.json'` — the file form; its `relate` section is read, or the
  whole file when it carries no section.

A named end with no colon is a usage error naming the ruled spelling
`NAME=SOURCE:TARGET`.

In SQL, `filter` is the `WHERE thinkthen_decide` pattern the slide draws,
and the slide's second and third queries are that pattern reading the
warm pass's saved answers. The aggregate forms of `rank` and `find` are
the database ADR's to rule; this surface claims no names for them.

## The engine

The extension binds `thinkthen-contract` and calls the stand-in through
the trait; when the real engine lands, one dependency changes. Load-time
init registers the functions and touches no wire. The engine builds
lazily on the first call, and a fork is repaired by the engine's process
check — proven through this surface: a child forked after a call answers
on its own wire call. A stopped query ends between records, one in-flight
round deep, through the wait's poll callback hearing
`sqlite3_is_interrupted` — no watchdog thread. Idle connections are
pruned and retried by the engine, and the pool is sized to the gate.

Two named divergences and one gap, stated rather than implied:

- **The session answer map is the stand-in's, not the engine's.** This
  surface's shim keeps a process-wide map of answered pairs and counts
  its hits as `cache_answers`, so a repeated pair inside one session
  costs nothing. ADR 0017 puts the cache and the counters in the engine;
  when the real engine lands its own cache replaces this map and the
  shim's map and hit counter are deleted together.
- **`SQLITE_DETERMINISTIC` stays off** on every function, per the ruled
  page, so no paid call is legal in an index expression or a CHECK
  constraint. Volatile is the ruled flag.
- **No per-call deadline option yet.** The host's own
  `sqlite3_interrupt` or statement discipline is the stop; conformance
  case 27 is skipped for that reason. A per-call budget beside the
  cancel token is the settled shape and a recorded gap here.

## Authority: who may do what

- **Question-file access.** The only file a call reads is the one its
  question argument names with the command's `'@name'` spelling, resolved
  against the process working directory. Nothing else is read: no
  configuration file, no directory listing, no table.
- **Backend selection.** The engine builds lazily on the first call from
  the process environment (`THINKTHEN_BASE_URL`, or the stand-in's
  `ENGINE_BASE_URL`; `ENGINE_NULL=1` for the in-process backend). SQL
  cannot name a backend, and load-time init registers the functions and
  touches no wire.
- **Credential source.** `THINKTHEN_API_KEY` in the process environment,
  read by the engine at send time and sent only to the named address. The
  stand-in reads no key and sends none (its own record); no credential is
  ever read from a fixture, a question file, or SQL, and none is logged.
- **Query execution.** SQLite's own step loop runs the SQL and calls the
  scalar functions once per row on the calling thread; `thinkthen_warm`
  judges its distinct texts in one bulk pass at the engine's width, and
  `thinkthen_relate` reads a whole named table, the one function a
  database cannot run row by row. The extension owns no thread between
  calls. No function carries `SQLITE_DETERMINISTIC`, so no paid call is
  legal in an index expression — `check.sh` proves both the flag and the
  refusal.
- **Connection lifetime.** The engine and its pool live as long as the
  process that loaded the extension; a fork is repaired by the engine's
  process-ID check on the next call (proven: a child forked after a call
  answers on its own wire call), idle connections are pruned by the
  engine, and the pool is sized to the width gate.
- **Cancellation channel.** The wait's poll reads `sqlite3_is_interrupted`
  on the loading connection, so the host's own `sqlite3_interrupt` (or a
  Ctrl-C in the CLI) cancels the token between records — no watchdog
  thread, and no query is left waiting on a stop that never lands.

## Building and checking

```
cargo build --release
cp target/release/libthinkthen0.so thinkthen.so
ENGINE_NULL=1 .runtimes/sqlite3 :memory: < tests/slide.sql   # the slide, as drawn
./check.sh                                                    # everything
```

The stock CLI is the official `sqlite-tools-linux-x64` bundle fetched
into `.runtimes/` (nothing outside the folder is touched, and no rc file
is modified). The entry point follows the basename rule: the artifact is
`libthinkthen0.so`, the entry is `sqlite3_thinkthen_init`, and
`.load ./thinkthen` resolves `thinkthen.so`.

The full record — the build traps, the wire numbers, the unchecked list —
is `NOTES.md`. The divergences the conformance slice reports are recorded
in `conformance/DIVERGENCES.md` as real-engine requirements.
