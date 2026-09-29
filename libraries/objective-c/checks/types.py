"""Run the shared J1 result corpus through the installed Dart public binding."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
SHARED = ROOT / "specification/fixtures/types"
spec = importlib.util.spec_from_file_location("shared_types", SHARED / "check.py")
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)

corpus = json.loads((SHARED / "corpus.json").read_text())
checks = shared.validators(json.loads((ROOT / "specification/result.schema.json").read_text()))
shared.check_schema(corpus["cases"], checks)
conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
backend, port = shared.start_backend()
run = HERE / "target/door"
count = 0
try:
    with tempfile.TemporaryDirectory(prefix="thinkthen-public-types-") as cache:
        for index, case in enumerate(corpus["cases"]):
            if "request" not in case or case.get("schema_only", False):
                continue
            route = case.get("case_id", "generic")
            if route != "generic":
                assert route in conformance, case["name"]
            env = os.environ.copy()
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1",
                       THINKTHEN_API_KEY="sk-type-contract-loopback", THINKTHEN_CACHE=str(Path(cache) / str(index)),
                       LD_LIBRARY_PATH=str(HERE / "target"))
            request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":"))
            result = subprocess.run([str(run), request], cwd=HERE, env=env, capture_output=True, text=True, timeout=40)
            assert result.returncode == 0, (case["name"], result.stderr)
            actual = json.loads(result.stdout)
            if "expected_error" in case:
                assert actual["error"] == "usage", (case["name"], actual)
                count += 1
                continue
            if case["definition"] != "usage":
                assert checks["callSuccess"].is_valid(actual), (case["name"], actual)
                actual = actual["value"]
            if case["definition"] != "doorRequest":
                assert checks[case["definition"]].is_valid(actual), case["name"]
            if "response" in case:
                assert actual == case["response"], (case["name"], actual)
            if "expect_fields" in case:
                assert shared.subset(actual, case["expect_fields"]), (case["name"], actual)
            if "expect_keys" in case:
                assert set(actual) == set(case["expect_keys"]), (case["name"], actual)
            if "expect_count" in case:
                edges = actual["edges"]
                assert len(edges) == case["expect_count"] and edges[0] == case["expect_first_edge"], case["name"]
            if "offsets" in case:
                shared.check_offsets(case, actual, conformance)
            count += 1
        settings_case = next(item for item in corpus["cases"] if item["name"] == "01-decide-yes-captured")
        env = os.environ.copy()
        env.update(THINKTHEN_BASE_URL="http://127.0.0.1:1/generic/v1",
                   THINKTHEN_API_KEY="sk-type-contract-loopback",
                   THINKTHEN_CACHE=str(Path(cache) / "settings"),
                   LD_LIBRARY_PATH=str(HERE / "target"))
        route = f"http://127.0.0.1:{port}/case/{settings_case['case_id']}/v1"
        request = json.dumps(settings_case["request"], ensure_ascii=False, separators=(",", ":"))
        env["TT_SETTINGS_JSON"] = json.dumps({"base_url": route})
        selected = subprocess.run([str(run), request], cwd=HERE, env=env, capture_output=True, text=True, timeout=40)
        assert selected.returncode == 0 and json.loads(selected.stdout)["value"] is True, (selected.stdout, selected.stderr)
        env["TT_SETTINGS_JSON"] = '{"not_a_setting":1}'
        refused = subprocess.run([str(run), request], cwd=HERE, env=env, capture_output=True, text=True, timeout=40)
        assert refused.returncode == 0 and json.loads(refused.stdout) == {"error": "usage"}, (refused.stdout, refused.stderr)
finally:
    shared.stop_backend(backend)
print(f"GNU Objective-C J1 public binding: {len(corpus['cases'])} schema cases, {count} runtime cases passed")
