# The PostgreSQL surface: the running log

## Entry 1: environment and build, 2026-09-21

- `pg_config` → PostgreSQL 16.15 (Ubuntu); `~/.pgrx/config.toml` → `[configs] pg16 = "/usr/bin/pg_config"` (set up for experiment 207; no download, no server started). `cargo-pgrx 0.17.0`, Docker 29.1.3. The system postgres service is inactive; ports on this box move between sibling sessions (Entry 3).
- Crate: `databases/postgresql`, its own workspace so the root gates stay untouched, path dependencies on `contract/` and `standin/` (the one dependency that changes when the real engine lands). `panic = "unwind"` (205 finding R4/C1: an abort kills the backend).
- Package route from 207, confirmed: `(cd databases/postgresql && cargo pgrx package --pg-config /usr/bin/pg_config)`. The `--manifest-path` form still dies at pgrx's cargo metadata step. The hand-written `thinkthen.control` (`default_version = '0.0.1'`, the repo's `VERSION`) and `src/bin/pgrx_embed.rs` (`::pgrx::pgrx_embed!();`) are both required.
- Generated SQL: 12 functions plus the aggregate — the nine ruled functions, the `thinkthen_decide` array overload (`postgres.md`'s second bulk form), and the aggregate's three leaked `warm_warm_*` helpers, the known pgrx-0.17 pattern from 207. `PARALLEL RESTRICTED` on every judging function, spelled by pgrx itself.
- Code lines: **294 against the 299 ceiling** on `postgres.md`. The SQLSTATE mapping for the six kinds is the one part 207 did not carry (it raised everything as one class), and it fits inside the ceiling.

## Entry 2: the shape, and the two readings this lane decided

- **Six kinds to six SQLSTATEs**: usage → `22023` invalid parameter, backend → `38000` external routine, deadline and cancelled → `57014` query canceled (statement_timeout is a deadline; that is a PostgreSQL user's word for it), local → `58030` io, defect → `XX000`. The message carries the kind word and the retry signal: `thinkthen backend: HTTP 422 ... (retryable: no)`.
- **`'@file'` resolves against the backend's working directory.** The official image's backend runs with its data directory as the working directory, so `@refund.json` reads `/var/lib/postgresql/data/refund.json`. Where the file may be read from is the database ADR's open rule; this lane reads the path exactly as named, relative to the backend, and says so here.
- `choose`, `score`, and `tag` keep the ruled three-argument signature, `NULL` as the third argument when the JSON question carries its own members.

## Entry 3: the port trap, twice

Two failures of the same class, recorded because the check now guards against them: sibling sessions start and stop their own postgres containers on this box, and a fixed port races them. Port 5435 answered `pg_isready` before my container had a socket (a foreign postgres held it), and the first wire run picked the same port as the null container. The check now takes the first free port from a quiet range (`free_port`), skips the one already taken, and waits on the container's own socket (`/run/postgresql/.s.PGSQL.<port>` — not `/var/run`), never on TCP.

## Entry 4: the slide sample, as drawn

Container `postgres:16` on the host network, `ENGINE_NULL=1`, files at the backend's working directory, `CREATE EXTENSION thinkthen` (instant), fixtures from the deck's own `examples/` (byte-for-byte `refund.json` and `form.json`; `tickets.sql` seeds the refund row, the `maybe` row, and the plain row):

```sql
SELECT id, body FROM tickets WHERE thinkthen_decide('@refund.json', body) IS NULL;
```

returns exactly the `maybe` row — 0.55 under the 0.2:0.8 band is the NULL a person should read. The first fixture text said "put the money back" and "Maybe"; the null backend keys on the literal words `refund` and `maybe`, so the rows now use the deck's own ticket text.

```sql
SELECT id, a->>'team' AS team, (a->>'urgency')::float AS urgency
FROM tickets, thinkthen_annotate('form.json', body) AS a ORDER BY urgency DESC;
```

returns urgency 1.7 / 1.05 / 0.99 in order. `team` is NULL on every row under the null backend: `billing`, `shipping`, `account` tie at one third each and the honest winner under the cut is none. The slide comment promises "as jsonb", not a team value, so no finding; on the wire the choice answers.

## Entry 5: the conformance slice, offline

`python3 runner.py <container>`, each case one line:

```
19 of 20 cases ok, 1 diverged
diverge 17-usage-and-cache: expected 1/1 after the second call, got 2/0; the
  stand-in answers the repeat from in-process memory but never counts it in
  cache_answers, its own record says so — a real-engine requirement, not a
  surface gap
skip 05/06/09 (filter is WHERE, ORDER BY, and LIMIT here; no function ships)
skip 18-cancel-mid-batch (the token cannot be pre-fired through SQL; the
  statement_timeout proof below is this surface's cancel shape, and the
  engine-side gap is recorded in conformance/DIVERGENCES.md)
```

Every `runner.py` call is a new psql session, so the run itself is the per-backend proof: dozens of fresh backends, each with its own engine state, all answered (lazy init after the fork, the pid check never once saw a stale pool).

## Entry 6: the wire, on the lane's stub (8219, 300 ms)

- `thinkthen_decide('@refund.json', 'please refund the duplicate')` → `t`; `thinkthen_usage()` → `1 | 0 | 12` — requests count sends and the tokens column carries the stub's reported usage.
- The array overload: three evidences in, `(0, true), (1, NULL), (2, false)` out, three requests at the stub, one crossing.
- **Warm at the process width**: 64 rows → 701.791 ms wall, stub `max_in_flight 32`, 64 requests, 33 connections, frozen through a 3 s settle. The gate holds through the aggregate.
- **The cancel shape, 211-proven and not re-proven here, confirmed anyway**: 512 rows, `statement_timeout = '1s'` → control back at 1.322 s with the standard `canceling statement due to statement timeout`; the stub saw exactly 128 requests (four 32-wide rounds) and nothing after; 384 requests never started.

## Entry 7: cleanup and the guard

```
docker rm -f -v thinkthen-pg thinkthen-pg-wire
grep -c deno ~/.zshrc -> 0
```

`check.sh` removes both containers in its exit trap; `docker ps -a` shows none left. No tool was installed (pgrx, cargo, docker already present), so no installer could touch an rc file; the grep guard ran anyway and its result is above. No key, no paid call, nothing published, no sudo, nothing outside this folder and its two disposable containers.

## What was not run (unchecked)

- macOS packaging: this box is Linux; the packaging rehearsal owns that arm.
- `shared_preload_libraries` with a postmaster-built pool: `_PG_init` here registers the key setting and touches nothing else, per this brief. The preload question stays with the database ADR.
- The `thinkthen.api_key` GUC is registered (the ruled setting exists) but the engine reads the environment at send time; wiring the setting into the send is the database ADR's preload question.
- `details` on a score question carries no nearest level through the contract (`Details` has no such field). ADR 0017 pick 6 says the nearest level's name rides in details; the contract needs the field before the promise is reachable. Recorded for the contract's owner, not worked around.
