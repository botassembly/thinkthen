"""Prove the installed PostgreSQL extension through ordinary SQL connections."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing, contextmanager
import json
from pathlib import Path
import signal
import subprocess
import sys
import tempfile

import psycopg2
from psycopg2 import sql
from source import admission, catalog, records

ROOT, LOOPBACK = map(Path, sys.argv[1:3])
GROUP = sys.argv[3:] or ["all"]
assert GROUP in (["all"], ["native"]), "expected all or native test group"
sys.path.insert(0, str(ROOT / "databases"))
from testing import child_environment


def completed(arguments, **extra):
    return subprocess.run(arguments, check=True, stdout=subprocess.DEVNULL,
                          env=child_environment(), **extra)


class Host:
    def __init__(self, url=None, encoding="UTF8"):
        self.scratch = tempfile.TemporaryDirectory(prefix="thinkthen-pg-")
        self.path = Path(self.scratch.name)
        self.data = self.path / "data"
        completed(["initdb", "-D", str(self.data), "-A", "trust", "-U", "thinkthen", "--no-locale", "-E", encoding])
        env = child_environment()
        if url:
            env.update(THINKTHEN_BASE_URL=url, THINKTHEN_API_KEY="loopback-test-key")
        subprocess.run(["pg_ctl", "-D", str(self.data), "-l", str(self.path / "server.log"),
                        "-o", f"-k {self.path} -c listen_addresses='' -c fsync=off", "-w", "start"],
                       env=env, check=True, stdout=subprocess.DEVNULL)
        self.connection = self.connect()
        self.query("CREATE EXTENSION thinkthen")
        self.query("SET thinkthen.allowed_backends = %s",
                   (json.dumps({"systemone": ["jev-1.13.0"], "stub": ["stub"]}),))

    def connect(self, user="thinkthen"):
        result = psycopg2.connect(host=str(self.path), dbname="postgres", user=user)
        result.autocommit = True
        return result

    def query(self, statement, parameters=()):
        with self.connection.cursor() as cursor:
            cursor.execute(statement, parameters)
            return cursor.fetchall() if cursor.description else []

    def configure(self, settings):
        self.query("SET thinkthen.settings = %s", (json.dumps(settings),))

    def call(self, function="decide", source="SELECT 'evidence'::text AS record", question="Ready?",
             options=None, parameters=None):
        return self.query(sql.SQL("SELECT * FROM {}(%s,%s,%s::json,%s::json)").format(
            sql.Identifier("thinkthen_" + function)),
            (source, question, json.dumps(options or {}), json.dumps(parameters or [])))

    def close(self):
        self.connection.close()
        try:
            completed(["pg_ctl", "-D", str(self.data), "-m", "fast", "-w", "stop"])
        finally:
            self.scratch.cleanup()

    def __enter__(self):
        return self

    def __exit__(self, *failure):
        if failure[0] is not None:
            print("\n".join((self.path / "server.log").read_text().splitlines()[-30:]), file=sys.stderr)
        self.close()


def command(server, value):
    server.stdin.write(value + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()


@contextmanager
def fixture(*arguments):
    server = subprocess.Popen([str(LOOPBACK), *map(str, arguments)], text=True,
                              stdin=subprocess.PIPE, stdout=subprocess.PIPE, env=child_environment())
    try:
        port = server.stdout.readline().strip()
        assert port
        yield server, f"http://127.0.0.1:{port}/v1"
        command(server, "shutdown")
        assert server.wait() == 0
    finally:
        if server.poll() is None:
            server.kill()
            server.wait()


def scrub(row):
    if "error" in row:
        row["error"].pop("message", None)
    if isinstance(row.get("answer"), dict):
        for entry in row["answer"].values():
            if isinstance(entry, dict) and isinstance(entry.get("error"), dict):
                entry["error"].pop("message", None)
    return row


def case_rows(host, case):
    host.configure(case.get("client", {}))
    request = case["request"]
    try:
        result = host.call(request["function"],
            "SELECT value AS record FROM json_array_elements($1::json) WITH ORDINALITY AS source(value,n) ORDER BY n",
            json.dumps(request["question"]), request.get("options", {}),
            [json.dumps(request["records"], ensure_ascii=False)])
    except psycopg2.Error as error:
        return {"error": error.diag.message_primary.split(":", 1)[0]}
    rows = []
    for position, status, answer, probability, error, detail, kept in result:
        record = request["records"][position - 1]
        identifier = record.get("id") if isinstance(record, dict) else None
        row = {"id": position if identifier is None else identifier, "status": status}
        if status != "failed":
            row["answer"] = answer
        if probability is not None:
            row["p"] = probability
        if error is not None:
            row["error"] = error
        if detail is not None:
            row["detail"] = detail
        if request["function"] == "filter":
            assert kept == (answer is True and status == "answered")
        rows.append(scrub(row))
    return rows


def shared_cases():
    with Host() as host:
        for case in json.loads((ROOT / "construction/stub.json").read_text()):
            assert case_rows(host, case) == case["expected"], case["request"]["function"]
    controls = 0
    count = 0
    for path in sorted((ROOT / "conformance").glob("*.json")):
        case = json.loads(path.read_text())
        if case.get("cancelled"):
            assert path.name == "cancel-before-call.json"
            controls += 1
            continue  # A pre-statement PG cancel is not retained; the native held-cancel case owns this control.
        with fixture(path) as (server, url), Host(url) as host:
            expected = case["expected"]
            if isinstance(expected, list) and case["request"]["function"] == "filter":
                failed = [r["error"]["code"] for r in expected if r["status"] == "failed"]
                if failed:
                    expected = {"error": failed[0]}  # FROM SRFs materialize before publishing any row.
            actual = case_rows(host, case)
            assert actual == expected, (path.name, actual, expected)
            assert int(command(server, "count")) == case["requests"], path.name
            count += 1
    assert controls == 1
    print(f"PostgreSQL: ten construction functions and {count} conformance cases pass; one pre-cancel control maps to native held cancellation")


def expect_error(host, operation, state, contains=None):
    try:
        operation()
    except psycopg2.Error as error:
        assert error.pgcode == state, (error.pgcode, error.diag.message_primary)
        if contains:
            assert contains in error.diag.message_primary, error.diag.message_primary
    else:
        raise AssertionError("statement unexpectedly succeeded")
    assert host.query("SELECT 1") == [(1,)]


def execution_grants():
    with Host() as host:
        host.configure({"adapter": "stub"})
        host.query("CREATE ROLE model_user")
        host.query("SET ROLE model_user")
        for case in json.loads((ROOT / "construction/stub.json").read_text()):
            function = case["request"]["function"]
            expect_error(host, lambda: host.call(function), "42501")
        host.query("RESET ROLE")
        host.query("GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,json) TO model_user")
        host.query("SET ROLE model_user")
        assert len(host.call()) == 1
        expect_error(host, lambda: host.call("filter"), "42501")
    print("PostgreSQL: only explicitly granted roles can call functions")


def backend_policy():
    with fixture(ROOT / "conformance/decide-yes.json") as (server, url), Host(url) as host:
        host.query("CREATE ROLE backend_user LOGIN")
        host.query("GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,json) TO backend_user")
        # A fresh backend must reject an unprivileged pre-load placeholder too.
        with closing(host.connect("backend_user")) as connection, connection.cursor() as cursor:
            cursor.execute("SET thinkthen.allowed_backends = %s", ('{"stub":["stub"]}',))
            cursor.execute("SET thinkthen.settings = %s", ('{"adapter":"stub"}',))
            try:
                cursor.execute("SELECT * FROM thinkthen_decide('SELECT ''evidence''::text AS record','Ready?')")
            except psycopg2.Error as error:
                assert error.pgcode == "22023", (error.pgcode, error.diag.message_primary)
            else:
                raise AssertionError("pre-load policy placeholder bypassed administrator control")
        host.query("RESET thinkthen.allowed_backends")
        host.query("SET ROLE backend_user")
        host.configure({"adapter": "stub"})
        expect_error(host, lambda: host.call(), "22023", "allowed")
        expect_error(host, lambda: host.query("SET thinkthen.allowed_backends = '{\"stub\":[\"stub\"]}'"), "42501")
        host.configure({})
        assert host.call()[0][5]["source"]["backend"] == "systemone"
        assert int(command(server, "count")) == 1
        host.configure({"model": "not-approved"})
        expect_error(host, lambda: host.call(), "22023", "allowed")
        assert int(command(server, "count")) == 1
        host.query("RESET ROLE")
        host.query("SET thinkthen.allowed_backends = %s", ('{"stub":["stub"]}',))
        host.query("SET ROLE backend_user")
        host.configure({"adapter": "stub"})
        assert host.call()[0][5]["source"]["backend"] == "stub"
        host.configure({})
        expect_error(host, lambda: host.call(), "22023", "allowed")
        host.query("RESET ROLE")
        # Tightening the policy must recheck even a previously cached client.
        host.configure({"adapter": "stub"})
        assert len(host.call()) == 1
        host.query("SET thinkthen.allowed_backends = '{}' ")
        expect_error(host, lambda: host.call(), "22023", "allowed")
        host.query("RESET thinkthen.allowed_backends")
        expect_error(host, lambda: host.call(), "22023", "allowed")
        assert int(command(server, "count")) == 1
    print("PostgreSQL: administrators admit backend/model pairs; callers cannot bypass the current policy")


def native_boundary():
    with Host() as host:
        host.configure({"adapter": "stub"})
        for case in json.loads((ROOT / "conformance/public-contract/diagnostics.json").read_text()):
            question = json.dumps({"type":"inline", "value":case["question"]})
            try:
                host.call("annotate", question=question)
            except psycopg2.Error as error:
                assert error.pgcode == "22023"
                assert error.diag.message_primary == case["code"] + ": " + case["message"]
            else:
                raise AssertionError("invalid annotation was accepted")
        for case in json.loads((ROOT / "conformance/public-contract/sql.json").read_text()):
            if "message" in case:
                host.query("SET thinkthen.settings = %s", (case["settings"],))
                expect_error(host, lambda: host.call(), "22023", case["code"] + ": " + case["message"])
        host.configure({"adapter": "stub"})
        for source, state in [
            ("", "42P11"),
            ("SELECT 'a'::text AS record; SELECT 'b'::text AS record", "42P11"),
            ("CREATE TABLE forbidden(x integer)", "42P11"),
            ("SELECT 1 AS record", "42804"),
            ("SELECT 'x'::text AS wrong", "22023"),
            ("SELECT NULL::text AS record", "22004"),
            ("SELECT (1/0)::text AS record", "22012"),
        ]:
            expect_error(host, lambda: host.call(source=source), state)
        assert host.query("SELECT to_regclass('forbidden')") == [(None,)]
        host.query("CREATE TABLE input(id bigint, body text)")
        host.query("INSERT INTO input VALUES(1,'alpha'),(2,'beta')")
        for source in ["DELETE FROM input RETURNING body AS record",
                       "WITH removed AS (DELETE FROM input RETURNING body) SELECT body AS record FROM removed",
                       "SELECT body AS record FROM input FOR UPDATE"]:
            expect_error(host, lambda: host.call(source=source), "0A000")
        assert host.query("SELECT count(*) FROM input") == [(2,)]
        admission(host, expect_error)
        expect_error(host, lambda: host.call(parameters=[["array"]]), "22023")
        expect_error(host, lambda: host.call(parameters={"not": "array"}), "22023")
        expect_error(host, lambda: host.query("SELECT * FROM thinkthen_decide(NULL,'Ready?')"), "22004")
        expect_error(host, lambda: host.call(options={"threshold": "wrong"}), "22023", "usage.threshold:")
        for forbidden in ("url", "key_source"):
            host.configure({"adapter": "stub", forbidden: None})
            expect_error(host, lambda: host.call(), "22023", "usage.")
        host.configure({"adapter": "stub"})
        records(host, json.loads((ROOT / "construction/stub.json").read_text())[0])
        assert len(host.call(source="SELECT n::text AS record FROM generate_series(1,300) AS source(n) LIMIT 3")) == 3
        assert host.query("SELECT count(*) FROM pg_cursors") == [(0,)]
        assert host.query("SELECT answer IS NULL, answer::text FROM thinkthen_find($source$SELECT ''::text AS record$source$,'Missing?')") == [(False, "null")]
        escaped = json.dumps({"a\0b": "data\0é"})
        assert len(host.call(source="SELECT $1::json AS record", parameters=[escaped])) == 1
        host.query("BEGIN")
        host.query("SET LOCAL thinkthen.settings = '{\"adapter\":\"invalid\"}'")
        host.query("ROLLBACK")
        assert len(host.call()) == 1
        host.query("BEGIN")
        host.configure({"adapter": "invalid"})
        host.query("ROLLBACK")
        assert len(host.call()) == 1
        nested = "SELECT answer::text AS record FROM thinkthen_decide('SELECT ''inside''::text AS record','Ready?')"
        assert len(host.call(source=nested)) == 1
        assert host.query("SELECT count(*) FROM pg_cursors") == [(0,)]
    with Host(encoding="LATIN1") as host:
        host.configure({"adapter": "stub", "model": "café"})
        row = host.call("choose", "SELECT $1::text AS record", "Café?", {"options": ["café", "thé"]}, ["olá"])[0]
        assert row[2] in ("café", "thé")
        expect_error(host, lambda: host.call("choose", options={"options": ["☃", "☄"]}), "22P05")
        assert len(host.call()) == 1
    print("PostgreSQL: source binding, chunks, settings, JSON preservation, encoding and cleanup pass")


def held_cancellation():
    for mode in ("headers", "body", "refusal-body"):
        with fixture("--held", mode) as (server, url), Host(url) as host:
            pid = host.query("SELECT pg_backend_pid()")[0][0]
            with ThreadPoolExecutor(max_workers=1) as workers:
                future = workers.submit(host.call)
                try:
                    assert command(server, "arrived") == "arrived"
                    masks = []
                    for task in Path(f"/proc/{pid}/task").iterdir():
                        if task.name != str(pid):
                            fields = dict(line.split(":", 1) for line in (task / "status").read_text().splitlines() if ":" in line)
                            masks.append(int(fields["SigBlk"].strip(), 16))
                    # Linux reserves 32/33 for NPTL and cannot mask SIGKILL/SIGSTOP.
                    expected = (1 << 64) - 1
                    for number in (signal.SIGKILL, signal.SIGSTOP, 32, 33):
                        expected &= ~(1 << (number - 1))
                    assert masks and all(mask == expected for mask in masks), masks
                    with closing(host.connect()) as control, control.cursor() as cursor:
                        cursor.execute("SELECT pg_cancel_backend(%s)", (pid,))
                        assert cursor.fetchone() == (True,)
                    try:
                        future.result()
                    except psycopg2.errors.QueryCanceled:
                        pass
                    else:
                        raise AssertionError("held query did not report native cancellation")
                    assert command(server, "closed") == "closed"
                finally:
                    if not future.done():
                        host.connection.cancel()
            assert host.query("SELECT 1") == [(1,)]
            host.configure({"adapter": "stub"})
            assert len(host.call()) == 1
            assert host.query("SELECT count(*) FROM pg_cursors") == [(0,)]
    with fixture("--unavailable") as (_, url), Host(url) as host:
        expect_error(host, lambda: host.call("filter"), "58000", "backend.transport:")
    print("PostgreSQL: held requests cancel natively; every observed engine descendant blocks all blockable signals")


def client_snapshots():
    with fixture(ROOT / "conformance/decide-yes.json") as (server, url), Host(url) as host:
        def backend():
            return host.call()[0][5]["source"]["backend"]
        host.configure({"batch": 1})
        host.query("SELECT * FROM thinkthen_decide('SELECT n::text AS record FROM generate_series(1,3) AS source(n)','Ready?') LIMIT 1")
        assert int(command(server, "count")) == 3
        host.query("SELECT * FROM thinkthen_decide('SELECT n::text AS record FROM generate_series(1,3) AS source(n) LIMIT 1','Ready?')")
        assert int(command(server, "count")) == 4
        host.configure({"adapter": "stub"})
        assert backend() == "stub"
        host.query("BEGIN")
        host.query("SET LOCAL thinkthen.settings = '{}'")
        assert backend() == "systemone"
        host.query("ROLLBACK")
        assert backend() == "stub"
        host.query("BEGIN")
        host.configure({})
        assert backend() == "systemone"
        host.query("SAVEPOINT before_setting")
        host.query("SET LOCAL thinkthen.settings = '{\"adapter\":\"stub\"}'")
        assert backend() == "stub"
        host.query("ROLLBACK TO SAVEPOINT before_setting")
        assert backend() == "systemone"
        host.query("COMMIT")
        assert backend() == "systemone"
        # IMMUTABLE source expressions run during planning, before an outer call opens.
        host.query("""CREATE FUNCTION planned_record() RETURNS text LANGUAGE plpgsql IMMUTABLE AS $body$
            BEGIN
                PERFORM set_config('thinkthen.settings','{"adapter":"stub"}',false);
                PERFORM * FROM thinkthen_decide('SELECT ''inner''::text AS record','Ready?');
                RETURN 'planned evidence';
            END $body$""")
        assert host.call(source="SELECT planned_record() AS record")[0][5]["source"]["backend"] == "stub"
        assert backend() == "stub"
        host.configure({})
        # The source replaces the cached client during an already opened outer
        # call, then starts a nested call. The outer call must retain its own one.
        source = """SELECT nested.answer::text AS record
            FROM (SELECT set_config('thinkthen.settings','{"adapter":"stub"}',false)) AS configured
            CROSS JOIN LATERAL thinkthen_decide(
                'SELECT ''inner''::text AS record', 'Ready?' || left(configured.set_config,0)) AS nested"""
        assert host.call(source=source)[0][5]["source"]["backend"] == "systemone"
        assert backend() == "stub"
        assert host.query("SELECT count(*) FROM pg_cursors") == [(0,)]
    print("PostgreSQL: successful configuration replacements roll back and active calls retain their client snapshots")


if GROUP == ["all"]:
    shared_cases()
execution_grants()
backend_policy()
native_boundary()
with Host() as host:
    host.configure({"adapter": "stub"})
    catalog(host)
client_snapshots()
held_cancellation()
completed(["/work/bundle/run-live", "--stub"])
print("PostgreSQL: the compiled manual runtime bundle runs all ten functions without a compiler")
