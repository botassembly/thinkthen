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


def native_parity(consumer, command):
    # Reuse the existing C projection/oracle; this adapter executes Go named calls.
    sys.path.insert(0, str(ROOT / "conformance"))
    import parity, c_parity
    cases = {c["id"]: c for c in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    named = {c["id"]: c for c in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = []
    for row in parity.required_cases(parity.inventory(), consumer).values():
        failure = None
        try:
            value = c_parity.document(row, cases, named)
            with tempfile.TemporaryDirectory(prefix=f"thinkthen-{consumer}-parity-") as folder:
                home = Path(folder)
                env = {"PATH": "/usr/bin:/bin", "HOME": folder,
                       "XDG_CONFIG_HOME": str(home / "config"), "XDG_CACHE_HOME": str(home / "cache"),
                       "XDG_STATE_HOME": str(home / "state"), "LC_ALL": "C.UTF-8", "DOTNET_CLI_TELEMETRY_OPTOUT": "1", "LD_LIBRARY_PATH": str(ROOT / "target/go/native/lib")}
                backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
                try:
                    env.update(THINKTHEN_API_KEY="sk-conformance-loopback", LIQUIDAI_API_KEY="sk-conformance-loopback", OPENROUTER_API_KEY="sk-conformance-loopback")
                    c_parity.prepare(home, value)
                    for step in value.get("steps", [value]):
                        step = dict(step)
                        for key in ("paths", "image_paths"):
                            if key in step and not step.get("owned_jsonl"):
                                step[key] = [str(ROOT / path) for path in (step[key] or [])]
                        if (step.get("operation") or {}).get("injection") == "recording_read_failure":
                            step.update(paths=[str(home / "missing-input")], source_unit=1)

                        settings = {"cache": False, "model": "jev-latest" if "steps" in value else "jev-1.13.0", "batch": 1, "max_retries": 0}
                        settings.update(step.get("settings", {}))
                        settings["base_url"] = f"http://127.0.0.1:{backend.port}/{value['arm']}"
                        settings = {k: str(home / "saved") if v == "$FOLDER" else str(home / "profile.json") if v == "$PROFILE" else v for k, v in settings.items()}
                        if row["kind"] in ("images", "image-location"):
                            settings["record"] = str(home / "recorded")
                        def invoke(given):
                            framed = c_parity.compact({**step, "engine_settings": c_parity.compact(given)})
                            args = ["complete:" + framed] if consumer == "scala" else ["complete", framed]
                            output = subprocess.run(command + args, env=env, cwd=home, capture_output=True, text=True, timeout=60)
                            assert output.returncode == 0 and not output.stderr, output.stderr
                            return json.loads(output.stdout)
                        before = int(backend.read("count"))
                        got = invoke(settings)
                        c_parity.assertions(row, step, got, int(backend.read("count")) - (before if step.get("count_delta") else 0))
                        if row["kind"] in ("images", "image-location"):
                            before = int(backend.read("count"))
                            saved = invoke({**{k:v for k,v in settings.items() if k != "record"}, "replay": str(home / "recorded")})
                            c_parity.assertions(row, step, saved, int(backend.read("count")))
                            assert saved["requests_sent"] == 0 and int(backend.read("count")) == before, saved
                            assert saved["rows"][0]["answer_id"] == got["rows"][0]["answer_id"], saved
                finally:
                    backend.close()
        except (AssertionError, ValueError, KeyError, TypeError, AttributeError, subprocess.SubprocessError, OSError) as error:
            failure = type(error).__name__ + ": " + str(error)
            failures.append((row["id"], failure))
            print(f"{consumer} fixture {row['id']} failed: {failure}", file=sys.stderr)
        print("parity: " + json.dumps({"consumer": consumer, "case": row["id"], "checks": row.get("checks", ["named", "runtime"]), "status": "fail" if failure else "pass"}), flush=True)
    print(f"{consumer} native fixture failures: {len(failures)}")
    return bool(failures)



try:
    assert type_case(["carriers"], child_env(), "owned carrier fixtures") == {"carriers": "pass"}
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
            if "offsets" in case:
                shared.check_offsets(case, actual, conformance)
            count += 1
    with tempfile.TemporaryDirectory(prefix="thinkthen-native-complete-") as folder:
        path=Path(folder)/"unicode.txt"
        path.write_bytes("Maria Chen\r\nAlex Lee\r\n".encode())
        settings=json.dumps({"base_url":f"http://127.0.0.1:{port}/arm/full/v1","model":"fixed","cache":False,"batch":"max","throttle":1,"max_retries":0})
        env=child_env(HOME=folder,XDG_CONFIG_HOME=folder,XDG_CACHE_HOME=folder,XDG_STATE_HOME=folder,
                      THINKTHEN_API_KEY="sk-native-complete-loopback",THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                      TT_NATIVE_SETTINGS=settings,TT_NATIVE_FILE=str(path),LD_LIBRARY_PATH=str(HERE.parent / "target/scratch/lib"))
        for lang in ("csharp",):
            before=sent()
            assert type_case(["native"],env,"native complete")=={"native":"pass"}
            assert sent()-before==22,(lang,"complete native listener count")
            print(lang+" named native: ten functions, typed fields and physical file locations PASS",flush=True)

finally:
    shared.stop_backend(backend)
print(f"C# J1 public binding: {len(corpus['cases'])} schema cases, {count} runtime cases, plan P1, limits and annotate fields passed")

raise SystemExit(native_parity("csharp", [str(resolve_dotnet()), str(run)]))
