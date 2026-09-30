"""Run the shared J1 result corpus through the installed C# public binding."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
import tempfile
from toolchains import dotnet as resolve_dotnet

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
run = HERE / "bin/Release/net8.0/TypeCase.dll"
count = 0
FIELDS = {"17-annotate-partial": [{"refund": "unresolved", "team": "failed backend missing_probability",
                                   "severity": "answered", "topics": "answered"}]}


def type_case(args, env, name):
    result = subprocess.run([str(resolve_dotnet()), str(run), *args], env=env, capture_output=True, text=True, timeout=40)
    assert result.returncode == 0 and not result.stderr, (name, result.returncode, result.stderr)
    return json.loads(result.stdout)


def sent():
    backend.stdin.write("count\n")
    backend.stdin.flush()
    return int(backend.stdout.readline())


try:
    with tempfile.TemporaryDirectory(prefix="thinkthen-csharp-types-") as cache:
        # Ticket 0291, before any case sends: P1 and one invalid plan run with
        # no key through the public Engine.Plan; the zero budgets and the zero
        # cap refuse; the backend has read no request.
        env = child_env(HOME=cache, XDG_CACHE_HOME=cache, DOTNET_CLI_HOME=cache,
                        THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                        THINKTHEN_CACHE=str(Path(cache) / "plan"), LD_LIBRARY_PATH=str(HERE.parent / "target/scratch/lib"))
        p1 = next(case for case in corpus["cases"] if case["name"] == "plan-p1")
        actual = type_case(["plan", json.dumps(p1["plan_input"])], env, "plan-p1")
        assert actual == p1["response"] and checks["plan"].is_valid(actual), actual
        invalid = dict(p1["plan_input"], settings={"batch": 0})
        assert type_case(["plan", json.dumps(invalid)], env, "plan-batch-0")["failed"] == {"kind": "usage", "code": 1}
        assert type_case(["helper"], env, "helper") == {"helper": "pass"}
        env["THINKTHEN_API_KEY"] = "sk-type-contract-loopback"
        assert type_case(["limits"], env, "limits") == {"limits": "pass"}
        assert sent() == 0, "a plan or a refused call sent a request"
        for index, case in enumerate(corpus["cases"]):
            if "request" not in case or case.get("schema_only", False):
                continue
            route = case.get("case_id", "generic")
            if route != "generic":
                assert route in conformance, case["name"]
            env = child_env(HOME=cache, XDG_CACHE_HOME=cache, DOTNET_CLI_HOME=cache)
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1",
                       THINKTHEN_API_KEY="sk-type-contract-loopback", THINKTHEN_CACHE=str(Path(cache) / str(index)),
                       LD_LIBRARY_PATH=str(HERE.parent / "target/scratch/lib"))
            request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":"))
            actual = type_case([request], env, case["name"])
            if case["name"] in FIELDS:
                # The shared null and failed annotate members, read through AnnotatedField.Read.
                env["THINKTHEN_CACHE"] = str(Path(cache) / f"{index}-fields")
                assert type_case(["fields", request], env, case["name"]) == FIELDS[case["name"]], case["name"]
            if "expected_error" in case:
                assert actual["failed"] == {"kind": "usage", "code": 1}, (case["name"], actual)
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
finally:
    shared.stop_backend(backend)
print(f"C# J1 public binding: {len(corpus['cases'])} schema cases, {count} runtime cases, plan P1, limits and annotate fields passed")
