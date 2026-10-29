"""Run installed C consumers against both canonical provider wire fixtures."""
import queue
import threading
import copy
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
source_schema = json.loads((manifest_file.parent / manifest["row_schema"]).read_text())["$defs"]["Source"]
names = ("PATH", "HOME", "SYSTEMROOT", "TMPDIR", "TEMP", "TMP", "LLVM_PROFILE_FILE")
env = {name: os.environ[name] for name in names if name in os.environ}
subprocess.run([consumer], check=True, env=env)


def line(stream):
    result = queue.Queue()
    threading.Thread(target=lambda: result.put(stream.readline()), daemon=True).start()
    return result.get(timeout=30)


@contextmanager
def fixture(path):
    with subprocess.Popen([loopback, str(path)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            port = int(line(server.stdout))
            yield server, f"http://127.0.0.1:{port}/v1"
            server.stdin.write("shutdown\n")
            server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait(timeout=30)


def normalize(rows, backend, name=None, model=None):
    for row in rows:
        if isinstance(row.get("function"), int):
            row["function"] = functions[row["function"]]
        source = row.pop("source")
        assert isinstance(source, dict)
        assert set(source_schema["required"]) <= source.keys() <= source_schema["properties"].keys(), source
        assert source["backend"] == backend
        assert source["model"] == (model or manifest["client"]["model"])
        assert isinstance(source["question_identity"], str) and source["question_identity"]
        assert source.get("name") == name, source
        if "error" in row:
            row["error"].pop("message", None)
        if row["function"] == "annotate" and isinstance(row.get("answer"), dict):
            for entry in row["answer"].values():
                if "error" in entry:
                    entry["error"].pop("message", None)


def same_json(actual, expected):
    if isinstance(actual, bool) or isinstance(expected, bool):
        return type(actual) is type(expected) and actual == expected
    if isinstance(actual, dict) and isinstance(expected, dict):
        return actual.keys() == expected.keys() and all(same_json(actual[key], expected[key]) for key in actual)
    if isinstance(actual, list) and isinstance(expected, list):
        return len(actual) == len(expected) and all(same_json(a, b) for a, b in zip(actual, expected))
    return actual == expected


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


def has_offsets(value):
    if isinstance(value, dict):
        return ("start" in value and "end" in value) or any(has_offsets(child) for child in value.values())
    return isinstance(value, list) and any(has_offsets(child) for child in value)


cases = [(name, json.loads((manifest_file.parent / name).read_text())) for name in manifest["cases"]]
# Settings and option admission use the same installed command boundary.
root = Path(__file__).resolve().parents[3]
for index, case in enumerate(json.loads((root / "construction/options.json").read_text())):
    case = dict(case)
    case["expected"] = {"error": case["expected_error"]} if "expected_error" in case else case["expected"]
    cases.append(("options:" + case["name"], case))
failures = []
not_applicable = 0
for name, case in cases:
    surface = "cpp" if offset_units else "c"
    if surface in case.get("not_applicable", {}):
        not_applicable += 1
        print(f"{surface}: {name}: not applicable: {case['not_applicable'][surface]}")
        continue
    path = manifest_file.parent / name
    if offset_units and isinstance(case["expected"], list):
        for row in case["expected"]:
            if not has_offsets(row.get("answer")) and not has_offsets(row.get("detail")):
                continue
            record = case["request"].get("records", [])[row["position"] - 1]
            pointer = case["request"]["question"].get("evidence")
            if pointer:
                for part in pointer.lstrip('/').split('/'):
                    key = part.replace('~1', '/').replace('~0', '~')
                    record = record[int(key)] if isinstance(record, list) else record[key]
            byte_offsets(row.get("answer"), record)
            byte_offsets(row.get("detail"), record)
    backends = ["stub"] if name.startswith("options:") else manifest["backends"]
    modes = [None, "shortcut"] if offset_units and name == "find-evidence-name.json" else [None]
    for backend, mode in ((backend, mode) for backend in backends for mode in modes):
        try:
            settings = dict(manifest["client"], **case.get("client", {}))
            settings.update(backend=backend, api_key="synthetic-loopback-key")
            with fixture(manifest_file.parent / manifest["cases"][0] if name.startswith("options:") else path) as (server, url):
                if backend != "stub": settings["url"] = url
                result = subprocess.run([driver, json.dumps(case["request"]), json.dumps(settings),
                                         *([mode] if mode else ["cancelled"] if case.get("cancelled") else [])],
                                        env=env, check=True, capture_output=True, text=True, timeout=60)
                actual = json.loads(result.stdout)
                expected = copy.deepcopy(case["expected"])
                expected_rows = expected if isinstance(expected, list) else case.get("expected_rows", [])
                expected_kept = [row["status"] == "answered" and row["answer"] is True
                                 for row in expected_rows if row["function"] == "filter"]
                assert same_json(actual["kept"], expected_kept), (name, backend, actual["kept"], expected_kept)
                for row, wanted in zip(actual["rows"], expected_rows):
                    for key, value in wanted.pop("source", {}).items():
                        assert row["source"][key] == value, (name, key, row["source"], value)
                normalize(actual["rows"], backend, case["request"].get("name"), "stub" if backend == "stub" else None)
                if "error" in actual:
                    assert actual["rows"] == case.get("expected_rows", []), (name, actual)
                    actual = {"error": actual["error"]}
                else:
                    actual = actual["rows"]
                assert same_json(actual, expected), (name, backend, actual, expected)
                server.stdin.write("count\n")
                server.stdin.flush()
                assert int(line(server.stdout)) == case["requests"], (name, backend)
                if mode:
                    print(f"cpp shared {mode}: {name}/{backend}")
        except Exception as error:
            failures.append((name, backend + ("/" + mode if mode else ""), str(error)))
if failures:
    names = [name + '/' + backend for name, backend, _ in failures]
    raise AssertionError(f"{len(failures)} canonical case failures: {names}; first: {failures[0][2][:2000]}")
print("installed consumer: canonical cases pass through both provider wires")

print(f"{surface}: {not_applicable} shared cases not applicable")
