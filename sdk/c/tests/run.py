"""Run installed C consumers against both canonical provider wire fixtures."""
import json
import os
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

consumer, loopback, driver, manifest_path, *offset_units = sys.argv[1:]
assert offset_units in ([], ["utf8"])
manifest_file = Path(manifest_path)
manifest = json.loads(manifest_file.read_text())
registry = json.loads((manifest_file.parent / manifest["registry"]).read_text())
functions = {entry["value"]: entry["name"] for entry in registry["functions"]}
names = ("PATH", "HOME", "SYSTEMROOT", "TMPDIR", "TEMP", "TMP", "LLVM_PROFILE_FILE")
env = {name: os.environ[name] for name in names if name in os.environ}
subprocess.run([consumer], check=True, env=env)


@contextmanager
def fixture(path):
    with subprocess.Popen([loopback, str(path)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            port = int(server.stdout.readline())
            yield server, f"http://127.0.0.1:{port}/v1"
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait() == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


def normalize(rows, backend):
    for row in rows:
        if isinstance(row.get("function"), int):
            row["function"] = functions[row["function"]]
        source = row.pop("source")
        assert source["backend"] == backend
        assert source["model"] == manifest["client"]["model"]
        assert source["question_identity"]
        if "error" in row:
            row["error"].pop("message", None)
        if row["function"] == "annotate" and isinstance(row.get("answer"), dict):
            for entry in row["answer"].values():
                if "error" in entry:
                    entry["error"].pop("message", None)


def byte_offsets(value, text):
    if isinstance(value, dict):
        if "start" in value and "end" in value:
            assert isinstance(text, str), "span fixture needs explicit evidence text"
            for key in ("start", "end"):
                value[key] = len(text[:value[key]].encode("utf-8"))
        for child in value.values():
            byte_offsets(child, text)
    elif isinstance(value, list):
        for child in value:
            byte_offsets(child, text)


for name in manifest["cases"]:
    path = manifest_file.parent / name
    case = json.loads(path.read_text())
    if offset_units and isinstance(case["expected"], list):
        for row in case["expected"]:
            record = case["request"]["records"][row["position"] - 1]
            byte_offsets(row.get("answer"), record)
            byte_offsets(row.get("detail"), record)
    for backend in manifest["backends"]:
        settings = dict(manifest["client"], **case.get("client", {}))
        settings.update(backend=backend, api_key="synthetic-loopback-key")
        with fixture(path) as (server, url):
            settings["url"] = url
            result = subprocess.run([driver, json.dumps(case["request"]), json.dumps(settings)],
                                    env=env, check=True, capture_output=True, text=True)
            actual = json.loads(result.stdout)
            normalize(actual["rows"], backend)
            if "error" in actual:
                assert actual["rows"] == [], (name, actual)
                actual = {"error": actual["error"]}
            else:
                actual = actual["rows"]
            assert actual == case["expected"], (name, backend, actual, case["expected"])
            server.stdin.write("count\n")
            server.stdin.flush()
            assert int(server.stdout.readline()) == case["requests"], (name, backend)
print("installed consumer: canonical cases pass through both provider wires")
