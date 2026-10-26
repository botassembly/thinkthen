"""Run the same canonical fixture protocol for the installed C++ consumer."""
from pathlib import Path
import runpy
import sys

sys.argv.append("utf8")
scope = runpy.run_path(str(Path(__file__).resolve().parents[2] / "c/tests/run.py"), run_name="__main__")

# A known vector keeps finite batching when a failed record sits between answers.
import json
import subprocess
case_path = scope["manifest_file"].parent / "tag-batching.json"
case = json.loads(case_path.read_text())
case["request"]["records"].insert(1, {"id": {"invalid": True}})
for backend in scope["manifest"]["backends"]:
    settings = {**scope["manifest"]["client"], **case["client"], "backend": backend, "api_key": "synthetic-loopback-key"}
    with scope["fixture"](case_path) as (server, url):
        settings["url"] = url
        result = subprocess.run(
            [scope["consumer"], json.dumps(case["request"]), json.dumps(settings)],
            env=scope["env"], check=True, capture_output=True, text=True)
        rows = json.loads(result.stdout)["rows"]
        scope["normalize"](rows, backend)
        assert len(rows) == 4 and rows[1]["position"] == 2, rows
        assert rows[1]["status"] == "failed" and rows[1]["error"]["code"] == "record.invalid", rows
        good = rows[:1] + rows[2:]
        for index, row in enumerate(good, 1):
            row["position"] = index
        assert good == case["expected"], good
        server.stdin.write("count\n")
        server.stdin.flush()
        assert int(server.stdout.readline()) == case["requests"]
