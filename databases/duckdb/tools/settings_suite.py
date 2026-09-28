"""The engine settings, the process request total, the cache-folder check, `@file` access, and
volatile scalars (ticket 0110 decisions 5, 10, and 16).

The access table is experiment 253's 35 cases: five file settings over
seven folders. DuckDB's own `COPY ... TO` judges each cache folder, and
DuckDB's own `read_text` judges each `@file`.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

from harness import CASES, EXTENSION, Backend, case, child_env, expect, main, rows, run, said
from settings_cases import shared_settings_corpus

ASK = "SELECT thinkthen_decide('Is it a refund?', 'refund now')"
PROBE_REFUSAL = "thinkthen usage: the cache folder is outside what this database's file settings allow"
SHAPE = "thinkthen usage: a cache folder set from SQL is an absolute local path with no scheme"
THROTTLE = "thinkthen usage: a throttle is a whole number from 1 through 32"
SPENT = "thinkthen usage: this process has spent its request total of 3; raise SET thinkthen_max_requests_total or RESET it"

case(shared_settings_corpus)

@case
def caller_settings_and_warm_share_one_engine():
    """Warm fills only the calling session's selected cache and model plan."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        query = "SELECT thinkthen_decide('Is it a refund?', 'refund now')"
        got = run([
            f"SET thinkthen_cache = '{folder}'",
            "SET thinkthen_model = 'other-model'",
            "SELECT thinkthen_warm('Is it a refund?', 'refund now')",
            query,
            ["B", query],
            "SELECT metric, value FROM thinkthen_usage()",
        ], backend.base())
        expect(rows(got[2]), [[1]], "warm result")
        expect(rows(got[3]), [[True]], "same-session scalar result")
        expect(rows(got[4]), [[True]], "other-session scalar result")
        usage = dict(rows(got[5]))
        expect(usage["cache_answers"], 1, "same-session warm cache hit")
        expect(backend.count(), 2, "one warm send and one isolated-session send")


@case
def retry_spends_the_process_total_before_transport():
    """A second attempt cannot pass a total of one after a 503."""
    with Backend() as backend:
        got = run(["SET thinkthen_max_retries = 1", "SET thinkthen_max_requests_total = 1", ASK], backend.base("arm/503"))
        expect(said(got[2]), SPENT.replace("of 3", "of 1"), "typed retry denial")
        expect(backend.count(), 1, "the refused retry opens no transport")


@case
def inline_profile_and_record_replay_respect_caller_settings():
    """An inline profile refuses before send; record and strict replay share a folder."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        profile = json.dumps({"schema": "thinkthen.backend-profile/1", "name": "small", "max_evidence_bytes": 4})
        got = run([f"SET thinkthen_profile = '{profile}'", ASK], backend.base())
        expect(said(got[1]).startswith("thinkthen usage: profile small allows at most 4 evidence bytes"), True, "inline profile")
        expect(backend.count(), 0, "profile sends nothing")
        got = run([f"SET thinkthen_record = '{folder}'", ASK, "RESET thinkthen_record",
                   f"SET thinkthen_replay = '{folder}'", ASK,
                   "SELECT thinkthen_decide('Is it a refund?', 'another text')"], backend.base())
        expect(rows(got[1]), [[True]], "record result")
        expect(rows(got[4]), [[True]], "replay result")
        expect(said(got[5]).startswith("thinkthen local:"), True, "strict replay miss")
        expect(backend.count(), 1, "replay and miss send nothing")


@case
def recording_folder_obeys_current_caller_permission():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        got = run([f"SET thinkthen_record = '{folder}'", "SET enable_external_access = false", ASK], backend.base())
        expect(said(got[2]), "thinkthen usage: the recording folder is outside what this database's file settings allow", "recording permission")
        expect(backend.count(), 0, "permission refusal sends nothing")


@case
def throttle_conflict():
    with Backend() as backend:
        got = run(["SET thinkthen_throttle = 8", ASK, "SET thinkthen_throttle = 4", ASK], backend.base())
        expect(rows(got[1]), [[True]], "the first throttle answers")
        expect(said(got[3]), "thinkthen usage: throttle 8 is already active for this process; use throttle 8 or drop the throttle argument", "a second throttle")
        expect(backend.count(), 1, "counted sends")


@case
def throttle_range():
    with Backend() as backend:
        for value in (33, 0, 264, -1):
            got = run([f"SET thinkthen_throttle = {value}", ASK], backend.base())
            expect("error" in got[0], False, f"SET thinkthen_throttle = {value}")
            expect(said(got[1]), THROTTLE, f"throttle {value}")
        expect(backend.count(), 0, "counted sends")


@case
def request_limit_range():
    with Backend() as backend:
        for value in (0, -1):
            got = run([f"SET thinkthen_max_requests = {value}", ASK], backend.base())
            expect(said(got[1]), "thinkthen usage: a request limit is a whole number of 1 or more", f"max_requests {value}")
        expect(backend.count(), 0, "counted sends")


@case
def the_process_request_total_holds_across_calls():
    """Ian's ruling of 2026-09-25: a total per process, checked before each call."""
    with Backend() as backend:
        ten = "SELECT thinkthen_decide('Is it a refund?', 'refund ' || i) FROM range(10) t(i)"
        got = run(["SET thinkthen_max_requests_total = 3", ten, ASK], backend.base())
        expect(said(got[1]), SPENT, "ten rows under a total of 3")
        expect(backend.count(), 3, "counted sends")
        expect(said(got[2]), SPENT, "the next call")
        expect(backend.count(), 3, "counted sends after the refusal")


@case
def seventeen_idle_plans_keep_all_prior_usage():
    """An idle plan can leave the resident map without resetting spending."""
    with Backend() as backend:
        statements = []
        for value in range(1, 18):
            statements += [f"SET thinkthen_max_requests = {value}",
                           f"SELECT thinkthen_decide('Is it a refund?', 'unique row {value}')"]
        statements += ["SELECT metric, value FROM thinkthen_usage()",
                       "SET thinkthen_max_requests_total = 17",
                       "SELECT thinkthen_decide('Is it a refund?', 'after total')"]
        got = run(statements, backend.base("arm/full"))
        for index in range(1, 34, 2):
            expect(rows(got[index]), [[True]], f"plan {(index + 1) // 2} answers")
        totals = dict(rows(got[34]))
        expect({name: totals[name] for name in ("requests_sent", "input_tokens", "output_tokens")},
               {"requests_sent": 17, "input_tokens": 17, "output_tokens": 17},
               "historical sends and tokens survive retirement")
        expect(said(got[36]), "thinkthen usage: this process has spent its request total of 17; raise SET thinkthen_max_requests_total or RESET it", "spent total survives retirement")
        expect(backend.count(), 17, "all sends remain counted")


HELD_PLANS = r"""
import concurrent.futures, json, sys
import duckdb
def opened(limit):
    db = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    db.execute(f"LOAD '{sys.argv[1]}'")
    db.execute("SET thinkthen_throttle = 16")
    db.execute(f"SET thinkthen_max_requests = {limit}")
    return db
def held(limit):
    db = opened(limit)
    return db.execute(f"SELECT thinkthen_decide('Is it a refund?', 'held {limit}')").fetchall()
with concurrent.futures.ThreadPoolExecutor(max_workers=16) as pool:
    waiting = [pool.submit(held, limit) for limit in range(1, 17)]
    print("ready", flush=True)
    sys.stdin.readline()
    extra = opened(17)
    try:
        extra.execute("SELECT thinkthen_decide('Is it a refund?', 'seventeenth')").fetchall()
        first = "answered"
    except Exception as error:
        first = str(error)
    print(json.dumps({"first": first}), flush=True)
    sys.stdin.readline()
    previous = [future.result(timeout=30) for future in waiting]
    again = extra.execute("SELECT thinkthen_decide('Is it a refund?', 'seventeenth')").fetchall()
    print(json.dumps({"previous": previous, "again": again}), flush=True)
"""


@case
def sixteen_held_plans_refuse_without_eviction():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        process = subprocess.Popen(
            [sys.executable, "-c", HELD_PLANS, str(EXTENSION)],
            env=child_env(backend.base("arm/held"), Path(folder)),
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        try:
            expect(process.stdout.readline().strip(), "ready", "the child started")
            expect(backend.wait(16), 16, "all 16 sends are held")
            process.stdin.write("go\n")
            process.stdin.flush()
            first = json.loads(process.stdout.readline())["first"]
            expect("16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process" in first,
                   True, "pathless cap refusal")
            expect(backend.count(), 16, "the 17th sent nothing")
            backend.release()
            process.stdin.write("go\n")
            process.stdin.flush()
            last = json.loads(process.stdout.readline())
            expect(last["previous"], [[[True]]] * 16, "held plans finish")
            expect(last["again"], [[True]], "released plan makes room")
            expect(backend.count(), 17, "the 17th now sends")
        finally:
            backend.release()
            process.kill()
            process.communicate(timeout=10)


@case
def a_negative_request_total_refuses_before_the_map():
    with Backend() as backend:
        # A NULL text reaches no engine call, so only the init's check sees it.
        got = run([ASK, "SET thinkthen_max_requests_total = -1", "SELECT thinkthen_decide('Is it a refund?', x) FROM (VALUES (NULL::VARCHAR)) t(x)"], backend.base())
        expect(said(got[2]), "thinkthen usage: a request total is a whole number of 0 or more", "a total of -1 on a map hit")
        expect(backend.count(), 1, "counted sends")


@case
def an_annotate_set_spends_one_request_per_text():
    """The engine asks every member of a library set in one request per
    text, since a library set reads each record whole and so holds one
    group. A total of 2 over three texts sends 2 and refuses."""
    with Backend() as backend:
        both = '{"version": 1, "questions": {"refund": {"decide": "Is it a refund?"}, "area": {"choose": "Which area?", "options": ["billing", "login"]}}}'
        got = run(["SET thinkthen_max_requests_total = 2", f"SELECT thinkthen_annotate('{both}', x) FROM (VALUES ('one'), ('two'), ('three')) t(x)"], backend.base())
        expect(said(got[1]), SPENT.replace("of 3", "of 2"), "three texts under a total of 2")
        expect(backend.count(), 2, "counted sends")


@case
def cache_folder_shape():
    with Backend() as backend:
        for folder in ("~/c", "c", "s3://b/c", "file:///tmp/c"):
            got = run([f"SET thinkthen_cache = '{folder}'", ASK], backend.base())
            expect(said(got[1]), SHAPE, f"cache folder {folder}")
        expect(backend.count(), 0, "counted sends")


@case
def settings_keep_the_environments_seeding():
    with Backend() as backend, tempfile.TemporaryDirectory() as cache:
        details = "SELECT CAST(thinkthen_details('Is it a refund?', 'refund now') ->> '$.meta.cached' AS BOOLEAN)"
        first = run([details], backend.base(), extra={"THINKTHEN_CACHE": cache})
        expect(rows(first[0]), [[False]], "the first run asks")
        sent = backend.count()
        second = run(["SET thinkthen_throttle = 8", details], backend.base(), extra={"THINKTHEN_CACHE": cache})
        expect(rows(second[1]), [[True]], "the second run answers from the environment's cache")
        expect(backend.count() - sent, 0, "counted sends in the second run")


@case
def volatile_scalars_are_not_folded_at_plan_time():
    with Backend() as backend:
        got = run(["EXPLAIN SELECT thinkthen_decide('q', 'a constant text')"], backend.base(), timeout=30)
        rows(got[0])
        expect(backend.count(), 0, "counted sends while planning")


def folders(root: Path) -> dict[str, str]:
    ok, out = root / "ok", root / "out"
    ok.mkdir()
    out.mkdir()
    (ok / "file").write_text("x")
    os.symlink(out, ok / "link")
    return {
        "ok": str(ok),
        "ok/new": f"{ok}/new",
        "out": str(out),
        "ok/../out": f"{ok}/../out",
        "ok/../out/new": f"{ok}/../out/new",
        "ok/link": f"{ok}/link",
        "ok/link/new": f"{ok}/link/new",
    }


def settings(root: Path) -> dict[str, list[str]]:
    ok = root / "ok"
    return {
        "default": [],
        "external_off": ["SET enable_external_access = false"],
        "dirs_ok": [f"SET allowed_directories = ['{ok}']", "SET enable_external_access = false"],
        "paths_ok": [f"SET allowed_paths = ['{ok}']", "SET enable_external_access = false"],
        "local_disabled": ["SET disabled_filesystems = 'LocalFileSystem'"],
    }


def judged(result: dict) -> str:
    """DuckDB's own answer. `read_text` on a missing file counts 0 rows."""
    if "error" not in result:
        return "missing" if result["rows"] == [[0]] else "allowed"
    return "refused" if "Permission Error" in result["error"] else "missing"


@case
def access_cases_match_duckdb():
    """The 35 cache cases against COPY, then caller-file cases against
    read_text for decide, warm, and relate. Refused paths send nothing."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        root = Path(folder)
        paths = folders(root)
        for place in ("ok", "out"):
            (root / place / "q.json").write_text('{"decide": "Is it a refund?"}')
            (root / place / "r.json").write_text('{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]}}')
        wrong = []
        for setting, setup in settings(root).items():
            for name, path in paths.items():
                before = sorted(os.listdir(root / "out")), sorted(os.listdir(root / "ok"))
                sent = backend.count()
                got = run(
                    [f"SET thinkthen_cache = '{path}'", *setup, f"COPY (SELECT 1) TO '{path}/oracle.csv'", ASK],
                    backend.base(),
                )
                oracle = judged(got[-2]) != "refused"
                ours = got[-1].get("error") is None or PROBE_REFUSAL not in said(got[-1])
                for leftover in (Path(path) / "oracle.csv",):
                    leftover.unlink(missing_ok=True)
                if oracle != ours:
                    wrong.append(f"cache {setting} {name}: COPY {'allows' if oracle else 'refuses'}, we {'allow' if ours else 'refuse'}")
                if not ours and (backend.count() != sent or before != (sorted(os.listdir(root / "out")), sorted(os.listdir(root / "ok")))):
                    wrong.append(f"cache {setting} {name}: a refused folder sent or created something")
                sent = backend.count()
                relate = f"SELECT count(*) FROM thinkthen_relate('SELECT 1 AS id, ''Ada'' AS name, ''person'' AS kind WHERE FALSE', '@{path}/r.json')"
                files = run(
                    [*setup, f"SELECT count(*) FROM read_text('{path}/q.json')", *(f"SELECT {verb}('@{path}/q.json', 'refund now')" for verb in ("thinkthen_decide", "thinkthen_warm")), relate],
                    backend.base(),
                )
                oracle = judged(files[-4])
                for verb, got in (("decide", files[-3]), ("warm", files[-2]), ("relate", files[-1])):
                    ours = "allowed" if "error" not in got else ("refused" if "file settings refuse it" in said(got) else "missing")
                    if oracle != ours:
                        wrong.append(f"@file {verb} {setting} {name}: read_text {oracle}, we {ours}")
                if oracle != "allowed" and backend.count() != sent:
                    wrong.append(f"@file {setting} {name}: a file DuckDB did not read sent something")
        # Ticket 0129 decision 7 retains warm's @~ refusal.
        home = root / "home"
        home.mkdir()
        (home / "q.json").write_text('{"decide": "Is it a refund?"}')
        tilde = run([f"SET home_directory = '{home}'", *(f"SELECT {verb}('@~/q.json', 'refund now')" for verb in ("thinkthen_decide", "thinkthen_warm"))], backend.base())
        expect(rows(tilde[1]), [[True]], "decide reads ~ from the session's home_directory")
        expect(said(tilde[2]), "thinkthen usage: thinkthen_warm cannot read an '@~' path; write the full path", "warm and ~")
        if wrong:
            raise AssertionError("; ".join(wrong))


@case
def two_databases_each_judge_their_own_access():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        question = Path(folder) / "q.json"
        question.write_text('{"decide": "Is it a refund?"}')
        cache = Path(folder) / "cache"
        got = run(
            [
                ["B", "SET enable_external_access = false"],
                f"SET thinkthen_cache = '{cache}'",
                ["B", f"SET thinkthen_cache = '{cache}'"],
                ASK,
                ["B", ASK],
            ],
            backend.base(),
        )
        expect(rows(got[3]), [[True]], "A's decide")
        expect(said(got[4]), PROBE_REFUSAL, "B's decide on A's folder")
        expect(backend.count(), 1, "counted sends")
        files = run(
            [
                ["B", "SET enable_external_access = false"],
                f"SELECT thinkthen_decide('@{question}', 'refund now')",
                ["B", f"SELECT thinkthen_decide('@{question}', 'refund now')"],
            ],
            backend.base(),
        )
        expect(rows(files[1]), [[True]], "A reads the file")
        expect(said(files[2]), f"thinkthen local: the question file {question} was not read: this database's file settings refuse it", "B's read")
        warm = run(
            [
                ["B", "SET enable_external_access = false"],
                f"SELECT thinkthen_warm('@{question}', 'refund now')",
                ["B", f"SELECT thinkthen_warm('@{question}', 'refund now')"],
            ],
            backend.base(),
        )
        expect(rows(warm[1]), [[1]], "A's warm reads the file")
        expect(said(warm[2]), f"thinkthen local: the question file {question} was not read: this database's file settings refuse it", "B's warm")


def opens(trace: Path, name: str) -> int:
    """How many `openat` calls in an `strace -f` log name the file `name`."""
    return sum(1 for line in trace.read_text().splitlines() if "openat(" in line and f'/{name}"' in line)


@case
def prepared_file_arguments_recheck_current_permissions():
    """A prepared plan cannot spend a file read authorized only at bind."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        root = Path(folder)
        question, names, rules = (root / name for name in ("q.json", "names.json", "rules.json"))
        question.write_text('{"decide":"Is it a refund?"}')
        names.write_text('{"version":1,"recognize":{"kinds":{"person":"A person."}}}')
        rules.write_text('{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]}}')
        statements = [
            "CREATE TABLE t AS SELECT * FROM (VALUES (1, 'Ada', 'person'), (2, 'Acme', 'organization')) v(id, name, kind)",
            f"PREPARE scalar AS SELECT thinkthen_decide('@{question}', 'refund now')",
            f"PREPARE relations AS SELECT thinkthen_relations('Ada', '@{names}')",
            f"PREPARE relate AS SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM t', '@{rules}')",
            "SET enable_external_access = false",
            "EXECUTE scalar", "EXECUTE relations", "EXECUTE relate",
        ]
        got = run(statements, backend.base())
        for index in (1, 2, 3, 4):
            expect(rows(got[index]), [], f"prepared setup {index}")
        for index, role, path in ((5, "question", question), (6, "question", names), (7, "rules", rules)):
            expect(said(got[index]),
                   f"thinkthen local: the {role} file {path} was not read: this database's file settings refuse it",
                   f"prepared {role} access after revoke")
        expect(backend.count(), 0, "prepared file refusals send nothing")


@case
def prepared_relate_uses_executing_session_settings():
    with Backend() as backend:
        got = run([
            "CREATE TABLE t AS SELECT * FROM (VALUES (1, 'Ada', 'person'), (2, 'Acme', 'organization')) v(id, name, kind)",
            "PREPARE relate AS SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM t', ['works_for=person:organization'])",
            "SET thinkthen_max_requests_total = 0",
            "EXECUTE relate",
        ], backend.base())
        expect(rows(got[1]), [], "relate prepares with its prior settings")
        expect(said(got[3]), "thinkthen usage: this process has spent its request total of 0; raise SET thinkthen_max_requests_total or RESET it", "relate uses the executing session's total")
        expect(backend.count(), 0, "the current total refuses before a send")


def file_opens_under_strace(count: int):
    """A refused file stays shut and repeated rows share one file open."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        question = Path(folder) / "q.json"
        question.write_text('{"decide": "Is it a refund?"}')
        many, refused = Path(folder) / "many.trace", Path(folder) / "refused.trace"
        got = run(
            ["SET threads = 1", f"SELECT sum(thinkthen_decide('@{question}', 'refund now')::INTEGER) FROM range({count})"],
            backend.base(),
            wrap=["strace", "-f", "-e", "trace=openat", "-o", str(many)],
            timeout=300,
        )
        expect(rows(got[1]), [[count]], f"yes answers over {count} rows")
        expect(opens(many, "q.json"), 1, f"opens of q.json over {count} rows")
        got = run(
            ["SET enable_external_access = false", f"SELECT thinkthen_decide('@{question}', 'refund now')"],
            backend.base(),
            wrap=["strace", "-f", "-e", "trace=openat", "-o", str(refused)],
            timeout=120,
        )
        expect(said(got[1]), f"thinkthen local: the question file {question} was not read: this database's file settings refuse it", "the refused read")
        expect(opens(refused, "q.json"), 0, "opens of q.json with access off")


@case
def two_file_rows_share_one_open_and_a_refusal_opens_none():
    file_opens_under_strace(2)


@case
def r1_15_and_r2_18_file_opens_under_strace():
    """The original 20,000-row file-open campaign stays opt in."""
    file_opens_under_strace(20_000)


FORKED = r"""
import json, os, sys, time
import duckdb
def opened():
    database = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    database.execute(f"LOAD '{sys.argv[1]}'")
    return database
def usage(database):
    return dict(database.execute("SELECT * FROM thinkthen_usage()").fetchall())
parent = opened()
for limit in range(1, 18):
    parent.execute(f"SET thinkthen_max_requests = {limit}")
    parent.execute(f"SELECT thinkthen_decide('Is it a refund?', 'parent {limit}')").fetchall()
before = usage(parent)
reader, writer = os.pipe()
pid = os.fork()
if pid == 0:
    os.close(reader)
    try:
        child = opened()
        zero = usage(child)
        child.execute("SET thinkthen_max_requests_total = 1")
        answer = child.execute("SELECT thinkthen_decide('Is it a refund?', 'child text')").fetchall()
        report = {"answer": answer, "zero": zero, "after": usage(child)}
    except BaseException as error:
        report = {"error": str(error)}
    os.write(writer, json.dumps(report).encode())
    os._exit(0)
os.close(writer)
deadline = time.monotonic() + 30
while os.waitpid(pid, os.WNOHANG) == (0, 0):
    if time.monotonic() > deadline:
        os.kill(pid, 9)
        os.waitpid(pid, 0)
        break
    time.sleep(0.05)
child = os.read(reader, 65536).decode() or '{"error": "the child timed out"}'
print(json.dumps({"before": before, "after": usage(parent), "child": json.loads(child)}), flush=True)
"""


@case
def a_forked_child_answers_from_a_zero_total():
    """A child drops the parent's retired and resident counters after fork."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        done = subprocess.run(
            [sys.executable, "-c", FORKED, str(EXTENSION)],
            env=child_env(backend.base("arm/full"), Path(folder)),
            capture_output=True,
            text=True,
            timeout=90,
            check=False,
        )
        lines = [line for line in done.stdout.splitlines() if line.startswith("{")]
        if not lines:
            raise AssertionError(f"the parent printed nothing: {done.stderr[-800:]}")
        got = json.loads(lines[-1])
        expect(got["child"].get("answer"), [[True]], "the child answers under a total of one")
        for metric in ("requests_sent", "input_tokens", "output_tokens"):
            expect(got["before"][metric], 17, f"parent {metric} after retirement")
            expect(got["after"][metric], 17, f"parent {metric} after fork")
            expect(got["child"]["zero"][metric], 0, f"child resets {metric}")
            expect(got["child"]["after"][metric], 1, f"child counts new {metric}")
        expect(backend.count(), 18, "counted sends")


if __name__ == "__main__":
    stress = {"r1_15_and_r2_18_file_opens_under_strace"}
    only_stress = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"
    CASES[:] = [function for function in CASES if (function.__name__ in stress) == only_stress]
    sys.exit(main())
