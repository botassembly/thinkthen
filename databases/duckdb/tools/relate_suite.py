"""`thinkthen_relate` from rows, its guard, and its settings (ticket 0118
decisions 3 to 5, rows R2-2, R3-7, R2-6, R3-12, and R5-22).

Every case runs on its own loopback backend. The generic arm gives a
choice's first option 0.9, so each person works for the one organization.
"""

from __future__ import annotations

import sys
import tempfile
import time
from pathlib import Path

from harness import Backend, case, expect, main, rows, run, said

TABLE = (
    "CREATE TABLE t AS SELECT * FROM (VALUES (1, 'Ada', 'person'), (2, 'Acme', 'organization'),"
    " (3, 'Bob', 'person'), (4, 'Ada', 'person')) v(id, name, kind)"
)
RELATE = "SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM t', {}) ORDER BY ALL"
WORKS = "['works_for=person:organization']"
EDGES = [["works_for", "1", "2", 0.9], ["works_for", "3", "2", 0.9], ["works_for", "4", "2", 0.9]]
NOT_SELECT = "thinkthen usage: the relate query must be a SELECT; relate reads records, it does not write files, attach databases, change settings, or load extensions"


@case
def relate_from_rows_maps_each_entity_back_to_its_ids():
    """Decision 3: rows dedupe by name and kind, and each edge returns one
    row per matching id pair. Ids 1 and 4 share a name and a kind."""
    with Backend() as backend:
        got = run([TABLE, RELATE.format(WORKS)], backend.base())
        expect(rows(got[1]), EDGES, "edges")
        expect(backend.count(), 1, "counted sends")


@case
def rules_read_as_inline_json_or_a_file():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        spec = '{"version": 1, "relate": {"relations": [{"name": "works_for", "source": "person", "target": "organization"}]}}'
        path = Path(folder) / "r.json"
        path.write_text(spec)
        got = run([TABLE, RELATE.format(f"'{spec}'"), RELATE.format(f"'@{path}'")], backend.base())
        expect(rows(got[1]), EDGES, "JSON rules")
        expect(rows(got[2]), EDGES, "file rules")


@case
def two_columns_read_kind_star_and_take_bare_rules_only():
    with Backend() as backend:
        pairs = "SELECT * FROM thinkthen_relate('SELECT id, name FROM t WHERE id < 3', {})"
        got = run([TABLE, pairs.format("['same_as']") + " ORDER BY ALL", pairs.format(WORKS)], backend.base())
        expect(rows(got[1]), [["same_as", "1", "2", 0.9], ["same_as", "2", "1", 0.9]], "a directed rule over one kind")
        expect(said(got[2]), "thinkthen usage: a relate query of id and name reads every kind as *, so every rule is bare or *:*", "a typed rule")


@case
def r2_2_a_delete_is_refused_and_the_rows_stay():
    with Backend() as backend:
        got = run(
            [TABLE, "BEGIN", RELATE.format(WORKS).replace("SELECT id, name, kind FROM t", "DELETE FROM t"), "ROLLBACK", "SELECT count(*) FROM t"],
            backend.base(),
        )
        expect(said(got[2]), NOT_SELECT, "a DELETE")
        expect(rows(got[4]), [[4]], "rows after the rollback")
        expect(backend.count(), 0, "counted sends")


@case
def r3_7_statements_that_write_or_attach_refuse():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        statements = [
            f"COPY t TO ''{folder}/copy.csv''",
            f"EXPORT DATABASE ''{folder}/export''",
            "ATTACH '':memory:'' AS smuggled",
            "SET GLOBAL enable_external_access = false",
            "LOAD json",
            "SET VARIABLE smuggled = 1",
        ]
        got = run([TABLE, *[f"SELECT * FROM thinkthen_relate('{sql}', {WORKS})" for sql in statements]], backend.base())
        for sql, result in zip(statements, got[1:], strict=True):
            expect(said(result), NOT_SELECT, sql)
        expect(sorted(path.name for path in Path(folder).iterdir()), [], "files written")
        expect(backend.count(), 0, "counted sends")


@case
def r2_6_a_nested_relate_refuses_at_once():
    with Backend() as backend:
        inner = "SELECT * FROM thinkthen_relate(''SELECT id, name, kind FROM t'', [''works_for=person:organization''])"
        started = time.monotonic()
        got = run([TABLE, f"SELECT * FROM thinkthen_relate('SELECT source, relation, ''person'' FROM ({inner})', {WORKS})"], backend.base(), timeout=30)
        expect(said(got[1]).split(";")[0], "thinkthen usage: the relate query calls thinkthen_relate while its own query is running", "a nested relate")
        expect(time.monotonic() - started < 10, True, "refused promptly")


@case
def r3_12_more_than_255_rows_refuses_under_the_cap():
    with Backend() as backend:
        started = time.monotonic()
        got = run(
            ["CREATE TABLE big AS SELECT i AS id, 'n' || i AS name, 'thing' AS kind FROM range(8000000) t(i)",
             "SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM big', ['near'])"],
            backend.base(),
            timeout=120,
        )
        elapsed = time.monotonic() - started
        expect(said(got[1]), "thinkthen usage: the relate query returned more than 255 rows, and relate reads at most 255; add a WHERE or a LIMIT", "eight million rows")
        expect(backend.count(), 0, "counted sends")
        expect(elapsed < 60, True, f"the refusal came in {elapsed:.1f}s")


@case
def r5_22_the_time_limit_stops_a_slow_query():
    with Backend() as backend:
        # A streaming scan whose filter never fills the LIMIT: no step holds
        # its input, so the plan guard passes it and only the timer stops it.
        slow = "SELECT i AS id, ''n'' AS name, ''k'' AS kind FROM range(100000000000) t(i) WHERE i < 0"
        started = time.monotonic()
        got = run(["SET thinkthen_relate_seconds = 2", f"SELECT * FROM thinkthen_relate('{slow}', ['near'])"], backend.base(), timeout=30)
        elapsed = time.monotonic() - started
        expect(said(got[1]), "thinkthen usage: the relate query ran past its 2-second limit and was stopped; filter the rows first or raise SET thinkthen_relate_seconds (0 turns the limit off)", "a slow query")
        expect(elapsed < 10, True, f"stopped in {elapsed:.1f}s")


@case
def relate_setting_ranges():
    with Backend() as backend:
        got = run([TABLE, "SET thinkthen_relate_seconds = -1", RELATE.format(WORKS)], backend.base())
        expect(said(got[2]), "thinkthen usage: a relate time limit is a whole number of seconds, 0 for none", "a negative limit")
        expect(backend.count(), 0, "counted sends")


@case
def a_temporary_table_names_the_adr_0038_boundary():
    with Backend() as backend:
        got = run([TABLE.replace("CREATE TABLE t", "CREATE TEMP TABLE tt"), RELATE.format(WORKS).replace("FROM t'", "FROM tt'")], backend.base())
        expect(said(got[1]).split(";")[0], "thinkthen local: the relate query names the temporary table tt, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables", "a temp table")


@case
def the_cache_probe_runs_on_relates_bind():
    """Decision 5: a cache folder the caller's settings refuse reads the
    probe's refusal with zero sends and nothing created."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        cache = Path(folder) / "cache"
        got = run([TABLE, "SET enable_external_access = false", f"SET thinkthen_cache = '{cache}'", RELATE.format(WORKS)], backend.base())
        expect(said(got[3]), "thinkthen usage: the cache folder is outside what this database's file settings allow", "a refused folder")
        expect(cache.exists(), False, "the cache folder exists")
        expect(backend.count(), 0, "counted sends")


@case
def throttle_reaches_relate_and_a_second_run_reads_the_cache():
    """Shared rule 7: throttle 8 holds 8 requests in flight on the held arm,
    and a second relate over the same rows answers from the cache."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        rows16 = "CREATE TABLE p AS SELECT i AS id, 'Person ' || i AS name, 'person' AS kind FROM range(16) t(i)"
        pairs = "SELECT count(*) FROM thinkthen_relate('SELECT id, name, kind FROM p', ['knows'])"
        extra = {"THINKTHEN_CACHE": str(Path(folder) / "cache")}
        got = run([rows16, "SET thinkthen_throttle = 8", pairs], backend.base(), extra=extra, timeout=120)
        first = backend.count()
        again = run([rows16, "SET thinkthen_throttle = 8", pairs], backend.base(), extra=extra, timeout=120)
        expect(rows(again[2]), rows(got[2]), "the second run's edges")
        expect(backend.count(), first, "counted sends on the second run")


if __name__ == "__main__":
    sys.exit(main())
