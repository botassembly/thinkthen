"""Exercise the installed extension through an ordinary SQLite C host."""

import json
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

CONSUMER, EXTENSION, LOOPBACK, ROOT = sys.argv[1:]
ROOT = Path(ROOT)
sys.path.insert(0, str(ROOT / "databases"))
from testing import child_environment
DONE, ROW, ERROR, INTERRUPT = 101, 100, 1, 9


def environment(**extra):
    return {**child_environment(), **extra}


class Host:
    def __init__(self, url=None):
        env = environment()
        if url:
            env.update(THINKTHEN_BASE_URL=url, THINKTHEN_API_KEY="loopback-test-key")
        self.process = subprocess.Popen([CONSUMER, EXTENSION], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, env=env)

    def send(self, command):
        self.process.stdin.write(json.dumps(command) + "\n")
        self.process.stdin.flush()

    def receive(self):
        line = self.process.stdout.readline()
        assert line, "SQLite host exited before reporting its statement result"
        return json.loads(line)

    def sql(self, sql, parameters=(), **extra):
        self.send(dict(sql=sql, parameters=parameters, **extra))
        return self.receive()

    def ok(self, sql, parameters=(), **extra):
        result = self.sql(sql, parameters, **extra)
        assert result["code"] in (DONE, ROW), result
        return result["rows"]

    def configure(self, settings):
        return self.sql("SELECT thinkthen_configure(?)", [json.dumps(settings)])

    def close(self):
        self.process.stdin.close()
        assert self.process.wait(timeout=20) == 0

    def __enter__(self):
        return self

    def __exit__(self, *failure):
        if failure[0] is not None:
            self.process.kill()
            self.process.wait()
        else:
            self.close()


@contextmanager
def fixture(*arguments):
    with subprocess.Popen([LOOPBACK, *map(str, arguments)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=environment()) as server:
        try:
            port = int(server.stdout.readline())
            yield server, f"http://127.0.0.1:{port}/v1"
            assert command(server, "shutdown") == ""
            assert server.wait(timeout=20) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


def command(server, value):
    server.stdin.write(value + "\n")
    server.stdin.flush()
    return server.stdout.readline().strip()


def scrub(row):
    if "error" in row:
        row["error"].pop("message", None)
    if isinstance(row.get("answer"), dict):
        for entry in row["answer"].values():
            if isinstance(entry, dict) and isinstance(entry.get("error"), dict):
                entry["error"].pop("message", None)
    return row


def case_rows(host, case):
    configured = host.configure(case.get("client", {}))
    if configured["code"] != DONE:
        return {"error": configured["message"].split(":", 1)[0]}
    request = case["request"]
    host.ok("CREATE TABLE input(seq INTEGER PRIMARY KEY, record TEXT)")
    for n, record in enumerate(request["records"], 1):
        host.ok("INSERT INTO input VALUES(?,?)", [n, json.dumps(record, ensure_ascii=False)])
    result = host.sql(f"SELECT * FROM thinkthen_{request['function']}(?,?,?)", [
        "SELECT json(record) AS record FROM input ORDER BY seq", json.dumps(request["question"]),
        json.dumps(request.get("options", {}))])
    if result["code"] != DONE and not result["rows"]:
        return {"error": result["message"].split(":", 1)[0]}
    rows = []
    for position, status, answer, probability, error, detail, kept in result["rows"]:
        record = request["records"][position - 1]
        identifier = record.get("id") if isinstance(record, dict) else None
        if identifier is None:
            identifier = position
        row = {"id": identifier, "status": status}
        if answer is not None:
            row["answer"] = json.loads(answer)
        if probability is not None:
            row["p"] = probability
        if error is not None:
            row["error"] = json.loads(error)
        if detail is not None:
            row["detail"] = json.loads(detail)
        if request["function"] == "filter" and status != "failed":
            assert kept == int(row["answer"] is True)
        rows.append(scrub(row))
    if result["code"] != DONE:
        assert any(row.get("error", {}).get("code") == result["message"].split(":", 1)[0]
                   for row in rows), result
        if "expected_rows" in case:
            assert rows == case["expected_rows"], (rows, case["expected_rows"])
        return {"error": result["message"].split(":", 1)[0]}
    return rows


def shared_cases():
    for case in json.loads((ROOT / "construction/stub.json").read_text()):
        with Host() as host:
            actual = case_rows(host, case)
            assert actual == case["expected"], (case["request"]["function"], actual, case["expected"])
    controls = 0
    for path in sorted((ROOT / "conformance").glob("*.json")):
        case = json.loads(path.read_text())
        if case.get("cancelled"):
            assert path.name == "cancel-before-call.json"
            controls += 1
            continue  # SQLite's pre-statement interrupt is a no-op; native held-cancel below owns this control.
        with fixture(path) as (server, url), Host(url) as host:
            actual = case_rows(host, case)
            assert actual == case["expected"], (path.name, actual, case["expected"])
            assert int(command(server, "count")) == case["requests"], path.name
    assert controls == 1


def native_surface():
    with Host() as cold:
        result = cold.sql("SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')")
        assert result["code"] == ERROR and result["message"].startswith("usage.key-missing:"), result
    with Host() as host:
        assert host.configure({"adapter": "stub"})["code"] == DONE
        assert host.configure({"adapter": "stub"})["code"] == DONE
        for invalid in (None, 1, {}):
            assert host.sql("SELECT thinkthen_configure(?)", [invalid])["code"] == 20
        short = host.ok("SELECT * FROM thinkthen_decide('SELECT json_quote(''evidence'') AS record','Ready?')")
        assert host.ok("SELECT * FROM thinkthen_decide(?,?)", [
            "SELECT json_quote('evidence') AS ReCoRd", "Ready?"]) == short
        tagged = host.ok("SELECT * FROM thinkthen_decide(?,?)", [
            "SELECT json_quote('evidence') AS record", '{"type":"inline","value":"Ready?"}'])
        assert short == tagged
        text = '{"body":"this remains text","id":"inside the text"}'
        quoted = host.ok("SELECT * FROM thinkthen_decide(?,?,?,?)", [
            "SELECT json_quote(?1) AS record", "Ready?", "{}", json.dumps([text])])
        raw = host.ok("SELECT * FROM thinkthen_decide(?,?)", [
            "SELECT json_quote('{\"body\":\"this remains text\",\"id\":\"inside the text\"}') AS record", "Ready?"])
        assert quoted == raw
        assert host.ok("SELECT options,parameters FROM thinkthen_decide(?,?)", [
            "SELECT json_quote('evidence') AS record", "Ready?"]) == [["{}", "[]"]]
        arguments = ["SELECT json_quote('evidence') AS record", "Ready?", "{}", "[]"]
        assert host.ok("SELECT rowid,source_sql,question,options,parameters FROM thinkthen_decide(?,?,?,?)",
                       arguments) == [[1, *arguments]]
        for arguments in ([None, "Ready?"], [1, "Ready?"], [{}, "Ready?"],
                          ["SELECT '1' AS record", None], ["SELECT '1' AS record", "Ready?", None],
                          ["SELECT '1' AS record", "Ready?", "{}", None]):
            result = host.sql(f"SELECT * FROM thinkthen_decide({','.join('?' for _ in arguments)})", arguments)
            assert result["code"] == 20, result
        for sql in ("SELECT * FROM thinkthen_decide()", "SELECT * FROM thinkthen_decide('SELECT 1')"):
            assert host.sql(sql)["code"] == ERROR
        for source, code in (("", ERROR), ("-- comment", ERROR), ("SELECT '1' AS record\0", ERROR),
                             ("SELECT '1' AS record; SELECT 2", ERROR), ("SELECT '1' AS record, 2", 20),
                             ("CREATE TABLE forbidden(x)", 8), ("SELECT 1 AS wrong", 20),
                             ("SELECT 1 AS record", 20), ("SELECT NULL AS record", 20),
                             ("SELECT * FROM missing_source", ERROR)):
            result = host.sql("SELECT * FROM thinkthen_decide(?,?)", [source, "Ready?"])
            assert result["code"] == code, (source, result)
        for null_at in (1, 257):
            source = f"WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x<257) SELECT CASE WHEN x={null_at} THEN NULL ELSE 'alpha' END AS record FROM n"
            result = host.sql("SELECT * FROM thinkthen_decide(?,?)", [source, "Ready?"])
            assert result["code"] == 20 and "SQL NULL" in result["message"], result["message"]
            assert host.ok("SELECT 1") == [[1]]
        assert len(host.ok("SELECT * FROM thinkthen_decide('SELECT json(''null'') AS record','Ready?')")) == 1
        for settings in ({"url": "http://127.0.0.1:1"}, {"key_source": "PATH"}, {"batch": 0}):
            assert host.configure(settings)["code"] == ERROR
        assert host.ok("SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')")
        assert host.ok("SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')", interrupt_before=True)
        for question, options in (("{", "{}"), ("Ready?", ""), ("Ready?", "null"), ("Ready?", '{"unknown":true}')):
            result = host.sql("SELECT * FROM thinkthen_decide(?,?,?)", ["SELECT '1' AS record", question, options])
            assert result["code"] == ERROR and result["message"].startswith("usage."), result
        for parameters, code in (("[]", 25), ("[{}]", 20), ("[9223372036854775808]", 20), ("{}", 20)):
            result = host.sql("SELECT * FROM thinkthen_decide(?,?,?,?)", [
                "SELECT json_quote(?1) AS record", "Ready?", "{}", parameters])
            assert result["code"] == code, result
        for value in ("a'b", 9223372036854775807, -9223372036854775808, 0.5, True, None):
            rows = host.ok("SELECT * FROM thinkthen_decide(?,?,?,?)", [
                "SELECT json_quote(?1) AS record -- trailing comment", "Ready?", "{}", json.dumps([value])])
            assert len(rows) == 1
        for case in json.loads((ROOT / "conformance/public-contract/diagnostics.json").read_text()):
            question = json.dumps({"type":"inline", "value":case["question"]})
            result = host.sql("SELECT * FROM thinkthen_annotate(?,?)", ["SELECT 'alpha' AS record", question])
            assert result["code"] == ERROR and result["message"] == case["code"] + ": " + case["message"], result
        for case in json.loads((ROOT / "conformance/public-contract/sql.json").read_text()):
            if "message" in case:
                result = host.sql("SELECT thinkthen_configure(?)", [case["settings"]])
                assert result["code"] == ERROR and result["message"] == case["code"] + ": " + case["message"], result
        # SQLite TEXT and JSON strings preserve embedded zero bytes at both edges.
        nul = "before\0after"
        assert host.ok("SELECT position FROM thinkthen_decide(?,?,?,?)", [
            "SELECT json_quote(?1) AS record", "Ready?", "{}", json.dumps([nul])]) == [[1]]
        for function, question, options in (
            ("choose", "Which?", {"options": ["first\0label", "second\0label"]}),
            ("annotate", json.dumps({"type": "inline", "value": {
                "questions": {"named\0question": {"decide": "Ready?"}}}}), {}),
        ):
            result = host.sql(f"SELECT * FROM thinkthen_{function}(?,?,?)", [
                "SELECT '1' AS record", question, json.dumps(options)])
            assert result["code"] == ERROR and result["message"].startswith("usage.name"), result
        for identifier in (18446744073709551615, {"key\0suffix": "value"}):
            source = "SELECT json(?1) AS record"
            parameters = json.dumps([json.dumps({"id": identifier, "body": "evidence"})])
            assert host.ok("SELECT position FROM thinkthen_decide(?,?,?,?)", [
                source, "Ready?", "{}", parameters]) == [[1]]
        # Application-side exceptions survive source finalization.
        result = host.sql("SELECT * FROM thinkthen_decide(?,?)", [
            "SELECT json_extract('broken', '$') AS record", "Ready?"])
        assert result["code"] == ERROR and "malformed JSON" in result["message"], result
        host.ok("CREATE VIEW forbidden_view AS SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')")
        assert "unsafe" in host.sql("SELECT * FROM forbidden_view")["message"]
        host.ok("CREATE VIEW forbidden_config AS SELECT thinkthen_configure('{}')")
        assert "unsafe" in host.sql("SELECT * FROM forbidden_config")["message"]
        # Same-connection reentry, recursive source, and two live scan owners.
        source = "WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x<300) SELECT json_quote(x) AS record FROM n"
        sql = "SELECT position,answer,detail FROM thinkthen_decide(?,?)"
        drained = host.ok(sql, [source, "Ready?"])
        assert [row[0] for row in drained] == list(range(1, 301))
        first = host.ok(sql, [source, "Ready?"], operation="open", slot=0)
        assert first[0][0] == 1
        assert host.configure({"adapter": "stub", "batch": 1})["code"] == DONE
        assert host.ok(sql, [source, "Ready?"], operation="open", slot=1)[0][0] == 1
        assert host.ok("", operation="step", slot=0)[0][0] == 2
        assert host.ok("", operation="step", slot=1)[0][0] == 2
        for slot in (0, 1):
            assert host.sql("", operation="close", slot=slot)["code"] == 0
        nested = "SELECT answer AS record FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')"
        assert len(host.ok(sql, [nested, "Ready?"])) == 1
        # Correlated invocation restarts its own source statement for each outer row.
        assert len(host.ok("WITH q(s) AS (VALUES('SELECT ''1'' AS record'),('SELECT ''2'' AS record')) SELECT t.* FROM q CROSS JOIN thinkthen_decide(q.s,'Ready?') t")) == 2
        before = host.ok("SELECT tick_count()")[0][0]
        huge = "WITH RECURSIVE n(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM n WHERE x<1000000) SELECT tick(json_quote(x)) AS record FROM n"
        assert len(host.ok("SELECT * FROM thinkthen_decide(?,?) LIMIT 1", [huge, "Ready?"])) == 1
        consumed = host.ok("SELECT tick_count()")[0][0] - before
        assert 0 < consumed <= 256, consumed
        assert host.ok("SELECT 1") == [[1]]


def cancellation_and_backend():
    with fixture("--unavailable") as (server, url), Host(url) as host:
        result = host.sql("SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')")
        assert result["code"] == ERROR and result["message"].startswith("backend.transport:"), result
        assert len(result["rows"]) == 1, result
        failed = result["rows"][0]
        assert failed[1:4] == ["failed", None, None], failed
        assert json.loads(failed[4])["code"] == "backend.transport", failed
        result = host.sql("SELECT * FROM thinkthen_filter('SELECT ''1'' AS record','Ready?')")
        assert result["code"] == ERROR and result["message"].startswith("backend.transport:"), result
        assert command(server, "count") == "0"
    for reply in ("headers", "body", "refusal-body"):
        with fixture("--held", reply) as (server, url), Host(url) as host:
            host.send(dict(sql="SELECT * FROM thinkthen_decide('SELECT ''1'' AS record','Ready?')", interruptible=True))
            assert command(server, "arrived") == "arrived"
            host.process.stdin.write("i\n")
            host.process.stdin.flush()
            result = host.receive()
            assert result["code"] == INTERRUPT, result
            assert command(server, "closed") == "closed"
            assert command(server, "count") == "1"
            assert host.ok("SELECT 1") == [[1]]


shared_cases()
native_surface()
cancellation_and_backend()
print("SQLite installed boundary passes: ten construction functions, twenty conformance cases, native cancellation control, cursor and source behavior")
