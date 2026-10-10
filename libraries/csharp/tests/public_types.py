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
run = Path(os.environ.get("THINKTHEN_TYPECASE_DLL", HERE / "bin/Release/net8.0/TypeCase.dll")).resolve(strict=True)
if os.environ.get("THINKTHEN_ARTIFACT") and not os.environ.get("THINKTHEN_TYPECASE_DLL"):
    raise ValueError("installed C# parity requires its package-referenced caller")
backend, port = shared.start_backend()
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
    # Shared recipes/assertions are read-only; execution uses each actual named consumer.
    import sqlite3
    sys.path.insert(0, str(ROOT / "conformance"))
    import parity, c_parity, c_images
    named = {c["id"]: c for c in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = []
    for row in parity.required_cases(parity.inventory(), consumer).values():
        failure = None
        try:
            value = c_parity.document(row, conformance, named)
            with tempfile.TemporaryDirectory(prefix=f"thinkthen-{consumer}-parity-") as folder:
                home = Path(folder)
                env = child_env(home=folder,
                                PATH=os.environ.get("PATH", "/usr/bin:/bin"),
                                LC_ALL="C.UTF-8",
                                DOTNET_CLI_TELEMETRY_OPTOUT="1")
                backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
                try:
                    env.update(THINKTHEN_API_KEY="sk-conformance-loopback", LIQUIDAI_API_KEY="sk-conformance-loopback", OPENROUTER_API_KEY="sk-conformance-loopback", PERPLEXITY_API_KEY="sk-conformance-loopback")
                    c_parity.prepare(home, value)
                    identities = []
                    for original in value.get("steps", [value]):
                        step = dict(original)
                        if step.get("copy_store"):
                            (home / "refreshed").mkdir()
                            with sqlite3.connect(home / "saved/thinkthen.sqlite") as source, sqlite3.connect(home / "refreshed/thinkthen.sqlite") as target:
                                source.backup(target)
                        if step.get("damage_store"):
                            with sqlite3.connect(home / "saved/thinkthen.sqlite") as db:
                                db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get("image_variants"):
                            backend.close()
                            backend = c_parity.Backend(ROOT / "target/debug/conformance-backend", env)
                            c_parity.prepare(home, step)
                        for key in ("paths", "image_paths"):
                            if key in step and not step.get("owned_jsonl"):
                                step[key] = [str(ROOT / path) for path in (step[key] or [])]
                        if (step.get("operation") or {}).get("injection") == "recording_read_failure":
                            step.update(paths=[str(home / "missing-input")], source_unit=1)
                        settings = {"cache": False, "model": "jev-latest" if "steps" in value else "jev-1.13.0", "batch": 1, "max_retries": 0}
                        settings.update(step.get("settings", {}))
                        settings["base_url"] = f"http://127.0.0.1:{backend.port}/{original['arm'] if value.get('image_variants') else value['arm']}"
                        replacements = {"$FOLDER": "saved", "$REFRESH": "refreshed", "$PROFILE": "profile.json"}
                        settings = {k: str(home / replacements[v]) if isinstance(v, str) and v in replacements else v for k, v in settings.items()}
                        if row["kind"] in ("images", "image-location"):
                            settings["record"] = str(home / "recorded")
                        def invoke(given):
                            framing = {**step, "engine_settings": c_parity.compact(given)}
                            if step.get("caption_files"):
                                framing["items"] = [""] * len(step["items"])
                            framed = c_parity.compact(framing) + "\n"
                            args = command + ["complete"]
                            if step.get("held_cancel"):
                                running = subprocess.Popen(args, env=env, cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                                try:
                                    running.stdin.write(framed)
                                    running.stdin.flush()
                                    assert backend.read("wait 1") == "wait 1"
                                    running.stdin.write("!")
                                    running.stdin.flush()
                                    assert running.stdout.readline() == "cancel-fired\n"
                                    backend.process.stdin.write("release\n")
                                    backend.process.stdin.flush()
                                    stdout, stderr = running.communicate(timeout=60)
                                    output = subprocess.CompletedProcess(args, running.returncode, stdout, stderr)
                                finally:
                                    backend.process.stdin.write("release\n")
                                    backend.process.stdin.flush()
                                    if running.poll() is None:
                                        running.kill()
                                        running.wait()
                            else:
                                output = subprocess.run(args, input=framed, env=env, cwd=home, capture_output=True, text=True, timeout=120 if value.get("image_variants") else 60)
                            assert output.returncode == 0 and not output.stderr, (consumer, row["id"], output.returncode, output.stderr[:1000])
                            return json.loads(output.stdout)
                        before = int(backend.read("count"))
                        got = invoke(settings)
                        if step["verb"] == "find" and got["code"] == 0:
                            assert all(r["answer_kind"] == "find" for r in got["rows"]), "find answer discriminator"
                        if value.get("identity_steps"):
                            identities.append(got)
                        if step.get("stored_answers") == 0:
                            path = home / "saved/thinkthen.sqlite"
                            if path.exists():
                                with sqlite3.connect(path) as db:
                                    assert db.execute("SELECT count(*) FROM answers").fetchone()[0] == 0
                        if value.get("image_variants"):
                            c_images.assert_images(original, got, json.loads(backend.read("capture"))["bodies"])
                        if step.get("owned_jsonl") and step.get("incremental"):
                            request = json.loads(json.loads(backend.read("capture"))["bodies"][0])
                            expected = {f"q{i+1}": {"type": "noul", "instructions": f"The text is {c_parity.compact(item)}. {step['question']['decide']}"} for i, item in enumerate(step["items"][:2])}
                            assert request["questions"] == expected
                        c_parity.assertions(row, original, got, int(backend.read("count")) - (before if step.get("count_delta") else 0))
                        if row["kind"] in ("images", "image-location"):
                            before = int(backend.read("count"))
                            saved = invoke({**{k:v for k,v in settings.items() if k != "record"}, "replay": str(home / "recorded")})
                            c_parity.assertions(row, original, saved, int(backend.read("count")))
                            assert saved["requests_sent"] == 0 and int(backend.read("count")) == before
                            assert saved["rows"][0]["answer_id"] == got["rows"][0]["answer_id"]
                    if value.get("identity_steps"):
                        ids = [entry["rows"][0]["observation_ids"] for entry in identities]
                        assert ids[0] == ids[1] == ids[2] and ids[3] != ids[0] and ids[4] == ids[3] and ids[5] == ids[0]
                        assert len({entry["call_id"] for entry in identities}) == 6
                        assert len({identities[i]["rows"][0]["answer_id"] for i in (0,1,2,5)}) == 1
                        assert identities[3]["rows"][0]["answer_id"] == identities[4]["rows"][0]["answer_id"] != identities[0]["rows"][0]["answer_id"]
                finally:
                    backend.close()
        except (AssertionError, ValueError, KeyError, TypeError, AttributeError, subprocess.SubprocessError, OSError) as error:
            failure = type(error).__name__ + ": " + str(error)
            failures.append((row["id"], failure))
            print(f"{consumer} fixture {row['id']} failed: {failure[:1500]}", file=sys.stderr)
        print("parity: " + json.dumps({"consumer": consumer, "case": row["id"], "checks": row.get("checks", ["named", "runtime"]), "status": "fail" if failure else "pass"}), flush=True)
    print(f"{consumer} native fixture failures: {len(failures)}")
    return bool(failures)


try:
    assert type_case(["carriers"], child_env(), "owned carrier fixtures") == {"carriers": "pass"}
    with tempfile.TemporaryDirectory(prefix="thinkthen-csharp-types-") as cache:
        # Ticket 0291, before any case sends: P1 and one invalid plan run with
        # no key through the public Engine.Plan; the zero budgets and the zero
        # cap refuse; the backend has read no request.
        env = child_env(home=cache, XDG_CACHE_HOME=cache, DOTNET_CLI_HOME=cache,
                        THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                        THINKTHEN_CACHE=str(Path(cache) / "plan"))
        p1 = next(case for case in corpus["cases"] if case["name"] == "plan-p1")
        actual = type_case(["plan", json.dumps(p1["plan_input"])], env, "plan-p1")
        assert actual == p1["response"] and checks["plan"].is_valid(actual), actual
        invalid = dict(p1["plan_input"], settings={"batch": 0})
        assert type_case(["plan", json.dumps(invalid)], env, "plan-batch-0")["failed"] == {"kind": "usage", "code": 1}
        env["THINKTHEN_API_KEY"] = "sk-type-contract-loopback"
        assert type_case(["limits"], env, "limits") == {"limits": "pass"}
        assert sent() == 0, "a plan or a refused call sent a request"
        for index, case in enumerate(corpus["cases"]):
            if "request" not in case or case.get("schema_only", False) or case.get("request_valid") is False or case["definition"] == "usage":
                continue
            route = case.get("case_id", "generic")
            if route != "generic":
                assert route in conformance, case["name"]
            env = child_env(home=cache, XDG_CACHE_HOME=cache, DOTNET_CLI_HOME=cache)
            env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1",
                       THINKTHEN_API_KEY="sk-type-contract-loopback", THINKTHEN_CACHE=str(Path(cache) / str(index)),
                       )
            request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":"))
            actual = type_case([request], env, case["name"])
            assert actual["code"] == 0, (case["name"], actual)
            rows = actual["rows"]
            if case["definition"] == "details":
                assert rows[0]["value"] is False and rows[0]["probability"] == 0.12 and rows[0]["answer_kind"] == "YesNo", actual
                count += 1
                continue
            if case["definition"] in ("decide", "choose", "tag", "score", "recognize"):
                actual = rows[0]["value"]
            elif case["definition"] == "filter":
                actual = [row["input"] for row in rows if row["value"] is True]
            elif case["definition"] == "rank":
                actual = [{"index": row["index"], "record": row["input"], "probability": row["probability"]} for row in rows]
            elif case["definition"] == "find":
                actual = None if rows[0]["value"] is None else {"index": rows[0]["index"], "unit": rows[0]["input"], "probability": rows[0]["probability"]}
            elif case["definition"] == "annotate":
                actual = [row["value"] for row in rows]
                if case["name"] in FIELDS:
                    states = [{key: "unresolved" if value is None else "failed " + value["failed"]["kind"] + " " + value["failed"]["cause"] if isinstance(value, dict) and "failed" in value else "answered" for key, value in row.items()} for row in actual]
                    assert states == FIELDS[case["name"]]
            elif case["definition"] == "relate":
                actual = {"edges": rows[0]["value"]}
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
        env=child_env(home=folder,XDG_CONFIG_HOME=folder,XDG_CACHE_HOME=folder,XDG_STATE_HOME=folder,
                      THINKTHEN_API_KEY="sk-native-complete-loopback",THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                      TT_NATIVE_SETTINGS=settings,TT_NATIVE_FILE=str(path))
        before=sent()
        assert type_case(["native"],env,"native complete")=={"native":"pass"}
        assert sent()-before==22,"complete native listener count"
        print("C# named native: ten functions, typed fields and physical file locations PASS",flush=True)

finally:
    shared.stop_backend(backend)
print(f"C# J1 public binding: {len(corpus['cases'])} schema cases, {count} typed runtime cases, plan P1, limits and annotate fields passed; 14 frozen C JSON syntax/control cases retain schema validation")

raise SystemExit(native_parity("csharp", [str(resolve_dotnet()), str(run)]))
