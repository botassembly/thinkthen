"""Run the shared J1 corpus through the public Go binding and saved backend."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[3]
PORT = ROOT / "libraries/go"
CORPUS = ROOT / "specification/fixtures/types/corpus.json"
SPEC = importlib.util.spec_from_file_location("types_check", ROOT / "specification/fixtures/types/check.py")
checks_module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checks_module)


FIELDS = {"17-annotate-partial": [{"refund": "unresolved", "team": "failed backend missing_probability",
                                   "severity": "answered", "topics": "answered"}]}


def run_case(mode, stdin, env, name):
    process = subprocess.run([str(ROOT / "target/go/type-case"), *mode],
                             input=json.dumps(stdin, ensure_ascii=False, separators=(",", ":")),
                             text=True, capture_output=True, env=env, timeout=30)
    assert process.returncode == 0 and not process.stderr, (name, process.returncode, process.stderr)
    return json.loads(process.stdout)


def main():
    corpus = json.loads(CORPUS.read_text())
    cases = corpus["cases"]
    checks = checks_module.validators(json.loads((ROOT / "specification/result.schema.json").read_text()))
    checks_module.check_schema(cases, checks)
    conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    backend, port = checks_module.start_backend()
    run_count = 0
    try:
        with tempfile.TemporaryDirectory(prefix="thinkthen-php-types-") as folder:
            for index, case in enumerate(cases):
                if "plan_input" in case and not case.get("schema_only", False):
                    # A plan runs with no key, through the public Engine.Plan.
                    env = {"PATH": "/usr/bin:/bin", "HOME": folder,
                           "THINKTHEN_BASE_URL": f"http://127.0.0.1:{port}/generic/v1",
                           "THINKTHEN_CACHE": str(Path(folder) / str(index)),
                           "LD_LIBRARY_PATH": str(ROOT / "target/go/native/lib")}
                    actual = run_case(["plan"], case["plan_input"], env, case["name"])
                    assert actual == case["response"], (case["name"], actual)
                    assert checks[case["definition"]].is_valid(actual), case["name"]
                    run_count += 1
                    continue
                if "request" not in case or case.get("schema_only", False):
                    continue
                route = case.get("case_id", "generic")
                if route != "generic":
                    assert route in conformance, case["name"]
                env = {"PATH": "/usr/bin:/bin", "HOME": folder,
                       "THINKTHEN_BASE_URL": f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1",
                       "THINKTHEN_API_KEY": "sk-type-contract-loopback",
                       "THINKTHEN_CACHE": str(Path(folder) / str(index)),
                       "LD_LIBRARY_PATH": str(ROOT / "target/go/native/lib")}
                actual = run_case([], case["request"], env, case["name"])
                if case["name"] in FIELDS:
                    # The shared null and failed annotate members, read through ReadField.
                    states = run_case(["fields"], case["request"], env | {"THINKTHEN_CACHE": str(Path(folder) / f"{index}-fields")}, case["name"])
                    assert states == FIELDS[case["name"]], (case["name"], states)
                if "expected_error" in case:
                    assert actual == {"failed": {"kind": "usage", "code": 1}}, case["name"]
                    run_count += 1
                    continue
                if case["definition"] != "usage":
                    assert checks["callSuccess"].is_valid(actual), case["name"]
                    assert isinstance(actual["facts"], dict), case["name"]
                    actual = actual["value"]
                assert checks[case["definition"]].is_valid(actual), case["name"]
                if "response" in case:
                    assert actual == case["response"], case["name"]
                if "expect_fields" in case:
                    assert checks_module.subset(actual, case["expect_fields"]), case["name"]
                if "expect_keys" in case:
                    assert set(actual) == set(case["expect_keys"]), case["name"]
                if "offsets" in case:
                    checks_module.check_offsets(case, actual, conformance)
                run_count += 1
    finally:
        checks_module.stop_backend(backend)
    print(f"GO_TYPE_CORPUS_PASS {len(cases)} schema cases, {run_count} public binding cases")


if __name__ == "__main__":
    main()
