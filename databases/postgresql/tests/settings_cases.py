#!/usr/bin/env python3
"""The nine shared settings cases through one PostgreSQL backend at a time."""

import json
import datetime
import fcntl
import os
import pathlib
import sqlite3
import subprocess
import sys
import time

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from children import child_env  # noqa: E402

CORPUS = pathlib.Path(__file__).resolve().parents[3] / "conformance" / "settings.json"
CALIBRATION = pathlib.Path(__file__).resolve().parents[3] / "conformance" / "calibration.json"
QUESTION = "'{\"decide\":\"Is this a refund?\"}'"


def quote(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def query(socket: str, statements: list[str]) -> subprocess.CompletedProcess[str]:
    command = ["psql", "-X", "-q", "-At", "-v", "ON_ERROR_STOP=1", "-h", socket,
               "-U", "postgres", "-d", "postgres"]
    for statement in ["SET statement_timeout='30s'", *statements]:
        command += ["-c", statement]
    return subprocess.run(command, capture_output=True, text=True, timeout=30, check=False, env=child_env())


def saved_calibration(socket: str) -> None:
    """A SQL details call preserves the shared saved digest and mismatch."""
    case = json.loads(CALIBRATION.read_text())
    question = json.dumps(case["question"], separators=(",", ":"))
    profile = json.dumps(case["runtime_profile"], separators=(",", ":"))
    done = query(socket, [f"SET thinkthen.profile = {quote(profile)}",
                          "SET thinkthen.cache = 'off'",
                          f"SELECT thinkthen_details({quote(question)}, {quote(case['evidence'])})::text"])
    assert done.returncode == 0, (done.stdout, done.stderr)
    detail = json.loads(done.stdout.strip().splitlines()[-1])
    assert detail["meta"]["question_sha256"] == case["question_sha256"], detail
    assert detail["meta"]["profile_warning"] == case["warning"], detail
    assert detail["meta"]["model"] == case["model"], detail


def usage_status(socket: str, folder: str, held: bool) -> None:
    """Observe two retained engines, keeping a good answer and safe failure."""
    usage = pathlib.Path(folder)
    usage.mkdir(mode=0o700, parents=True, exist_ok=True)
    month = usage / (datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m") + ".json")
    with (usage / ".lock").open("w") as lock:
        os.chmod(lock.name, 0o600)
        if held:
            fcntl.flock(lock, fcntl.LOCK_EX)
        process = subprocess.Popen(["psql", "-X", "-q", "-At", "-v", "ON_ERROR_STOP=1",
                                    "-h", socket, "-U", "postgres", "-d", "postgres"],
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, text=True, env=child_env())
        try:
            def ask(sql):
                process.stdin.write(sql + ";\n")
                process.stdin.flush()
                return process.stdout.readline().strip()
            assert json.loads(ask("SELECT thinkthen_usage_status()")) == {"state": "disabled"}
            process.stdin.write("SET thinkthen.cache = 'off';\n")
            assert ask("SELECT thinkthen_decide('Is it a refund?', 'refund now')") == "t"
            if held:
                assert json.loads(ask("SELECT thinkthen_usage_status()")) == {"state": "pending"}
            process.stdin.write("SET thinkthen.model = 'another';\n")
            json.loads(ask('SELECT thinkthen_plan(\'{"decide":"Is it a refund?"}\', \'{"a":"refund now"}\'::jsonb)'))
            if held:
                assert json.loads(ask("SELECT thinkthen_usage_status()")) == {"state": "pending"}
                month.write_text("not JSON")
                month.chmod(0o600)
                fcntl.flock(lock, fcntl.LOCK_UN)
            wanted = {"state": "failed", "advice": "check the usage folder permissions and free space"} if held else {"state": "written"}
            end = time.monotonic() + 3
            while time.monotonic() < end:
                state = json.loads(ask("SELECT thinkthen_usage_status()"))
                if state == wanted:
                    break
                time.sleep(.01)
            assert state == wanted, state
            assert json.loads(ask("SELECT thinkthen_usage_status()")) == wanted
            assert ask("SELECT requests_sent FROM thinkthen_usage()") == "1"
            process.stdin.close()
            assert process.wait(timeout=5) == 0, process.stderr.read()
        finally:
            if process.poll() is None:
                process.kill()
                process.wait(timeout=5)
            if held:
                month.unlink(missing_ok=True)
    print("pass PostgreSQL usage status, aggregation, unchanged counts")


def run_case(socket: str, case: dict, folder: pathlib.Path) -> None:
    for step in case["steps"]:
        settings = []
        for name, value in step["settings"].items():
            if value == "$PROFILE":
                value = json.dumps(case["profile"], separators=(",", ":"))
            elif value == "$FOLDER":
                value = str(folder)
            elif value is False:
                value = "off"
            if name == "timeout":
                value = f"{value}s"
            settings.append(f"SET thinkthen.{name} = {quote(str(value))}")
        if step.get("verb") == "relate":
            values = ", ".join(f"({at}, {quote(one['name'])}, {quote(one['kind'])})"
                               for at, one in enumerate(case["entities"]))
            source = f"SELECT * FROM (VALUES {values}) v(id, name, kind)"
            name, ends = case["relation"].split("=", 1)
            start, target = ends.split(":", 1)
            rule = json.dumps({"version": 1, "relate": {"relations": [
                {"name": name, "source": start, "target": target}]}})
            sql = f"SELECT count(*) FROM thinkthen_relate({quote(source)}, {quote(rule)})"
        elif step.get("verb") == "decide_many":
            records = json.dumps({str(index): value for index, value in enumerate(step["records"])})
            sql = f"SELECT * FROM thinkthen_decide_many({QUESTION}, {quote(records)}::jsonb)"
        elif "model" in step:
            sql = f"SELECT thinkthen_details({QUESTION}, {quote(step['text'])}) -> 'meta' ->> 'model'"
        else:
            sql = f"SELECT thinkthen_decide({QUESTION}, {quote(step['text'])})"
        done = query(socket, [*settings, sql])
        if "error" in step:
            marker = f"thinkthen {step['error']}:"
            assert done.returncode and marker in done.stderr, (case["id"], done.stdout, done.stderr)
        else:
            wanted = str(step.get("model", step.get("edges", "t")))
            assert done.returncode == 0 and done.stdout.strip() == wanted, (case["id"], done.stdout, done.stderr)
    if "entries" in case:
        # A recording keeps one store row per question (ADR 0111 section 3).
        store = folder / "thinkthen.sqlite"
        saved = 0
        if store.is_file():
            with sqlite3.connect(store) as connection:
                saved = connection.execute("SELECT count(*) FROM answers").fetchone()[0]
            connection.close()
        assert saved == case["entries"], (case["id"], saved)


def main() -> None:
    if sys.argv[1] == "usage-status":
        usage_status(sys.argv[2], sys.argv[3], sys.argv[4] == "held")
        return
    if sys.argv[1] == "calibration":
        saved_calibration(sys.argv[2])
        print("pass calibration")
        return
    corpus = json.loads(CORPUS.read_text())
    assert corpus["schema"] == "thinkthen.settings-cases/1"
    if sys.argv[1] == "plan":
        for case in corpus["cases"]:
            # PostgreSQL's keyed form prevalidates max_requests for its whole call.
            # The accepted host contract refuses three records before any send.
            sends = 0 if case["id"] == "max-requests-refuses-past-the-limit" else case["steps"][-1]["count"]
            print(case["id"], case["arm"].removesuffix("/v1"), sends, sep="\t")
    else:
        socket, name, path = sys.argv[1:]
        case = next(case for case in corpus["cases"] if case["id"] == name)
        folder = pathlib.Path(path)
        folder.mkdir(parents=True, exist_ok=True)
        run_case(socket, case, folder)
        print(f"pass {name}")


if __name__ == "__main__":
    main()
