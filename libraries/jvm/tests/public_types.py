"""Run J1 schema controls and applicable runtime cases through each public JVM facade."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
import tempfile
from toolchains import JDK, KOTLIN, SCALA

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
PACKAGE = HERE.parent
TARGET = PACKAGE / "target"
SHARED = ROOT / "specification/fixtures/types"
spec = importlib.util.spec_from_file_location("shared_types", SHARED / "check.py")
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)

corpus = json.loads((SHARED / "corpus.json").read_text())
checks = shared.validators(json.loads((ROOT / "specification/result.schema.json").read_text()))
shared.check_schema(corpus["cases"], checks)
selected = sys.argv[1] if len(sys.argv) == 2 else None
runnable = {case["name"] for case in corpus["cases"]
            if "request" in case and not case.get("schema_only", False)}
if len(sys.argv) > 2 or (selected is not None and selected not in runnable):
    raise SystemExit("JVM J1 case selector must name an existing runtime case")
conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
installed = os.environ.get("THINKTHEN_RELEASE_JVM_DIR")
native = Path(os.environ["THINKTHEN_RELEASE_C_DIR"]) / "lib" if installed else TARGET / "native"
if os.environ.get("THINKTHEN_ARTIFACT") and not installed:
    raise ValueError("installed JVM parity requires its extracted JARs")
classes = Path(installed) / "test-classes" if installed else TARGET / "classes/typecase"
classes.mkdir(parents=True, exist_ok=True)
jars = Path(installed).resolve(strict=True) if installed else TARGET / "jars"
inventory_spec = importlib.util.spec_from_file_location("package_inventory", ROOT / "sdlc/scripts/package-inventory.py")
inventory = importlib.util.module_from_spec(inventory_spec)
inventory_spec.loader.exec_module(inventory)
definition = inventory.jvm_inventory(pom=(jars / "pom.xml").read_bytes() if installed else None)
package_jars = [(jars / filename).resolve(strict=True) for filename in definition['jars'].values()]
classpath = os.pathsep.join(map(str, package_jars))
compiler_env = child_env(JAVA_HOME=str(JDK), JAVACMD=str(JDK / "bin/java"),
                         PATH=str(JDK / "bin") + ":" + os.environ.get("PATH", "/usr/bin:/bin"))
java = [str(JDK / "bin/java"), "--enable-preview", "--enable-native-access=ALL-UNNAMED", "-Dthinkthen.library=" + str(native / "libthinkthen.so")]
subprocess.run([str(JDK / "bin/javac"), "--enable-preview", "--release", "21", "-cp", classpath, "-d", str(classes), str(HERE / "TypeCase.java"), str(HERE / "CarrierChecks.java"), str(HERE / "NativeChecks.java"), str(HERE / "NativeCases.java")], env=compiler_env, check=True)
subprocess.run([str(KOTLIN / "bin/kotlinc"), "-J-XX:ActiveProcessorCount=2", "-jvm-target", "21", "-classpath", str(classes) + ":" + classpath, str(HERE / "TypeCase.kt"), "-d", str(classes)], env=compiler_env, check=True)
subprocess.run([str(SCALA / "bin/scalac"), "-J-XX:ActiveProcessorCount=2", "-classpath", str(classes) + ":" + classpath, "-d", str(classes), str(HERE / "TypeCase.scala")], env=compiler_env, check=True)
backend, port = shared.start_backend()
FIELDS = {"17-annotate-partial": [{"refund": "unresolved", "team": "failed backend missing_probability",
                                   "severity": "answered", "topics": "answered"}]}


def type_case(args, env, name, lang="java"):
    main, runtime = {"java": ("TypeCase", ""), "kotlin": ("TypeCaseKt", str(KOTLIN / "lib/kotlin-stdlib.jar")),
                     "scala": ("scalaTypeCase", str(SCALA / "lib/scala.jar"))}[lang]
    cmd = java + ["-cp", f"{classes}:{classpath}" + (f":{runtime}" if runtime else ""), main, *args]
    result = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=40)
    assert result.returncode == 0, (lang, name, result.stderr)
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
    cases = {c["id"]: c for c in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    named = {c["id"]: c for c in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = []
    for row in parity.required_cases(parity.inventory(), consumer).values():
        failure = None
        try:
            value = c_parity.document(row, cases, named)
            with tempfile.TemporaryDirectory(prefix=f"thinkthen-{consumer}-parity-") as folder:
                home = Path(folder)
                env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": folder,
                       "XDG_CONFIG_HOME": str(home / "config"), "XDG_CACHE_HOME": str(home / "cache"),
                       "XDG_STATE_HOME": str(home / "state"), "LC_ALL": "C.UTF-8",
                       "DOTNET_CLI_TELEMETRY_OPTOUT": "1", "LD_LIBRARY_PATH": str(native)}
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
    # Pure carrier fixtures are independent checks and never counted as parity cases.
    subprocess.run(java + ["-cp", f"{classes}:{classpath}", "CarrierChecks"], env=compiler_env, check=True)
    assert type_case(["carriers"], compiler_env, "owned carrier fixtures", "kotlin") == {"carriers": "pass"}
    assert type_case(["carriers"], compiler_env, "owned carrier fixtures", "scala") == {"carriers": "pass"}
    if selected is None:
        # Ticket 0291, before any case sends: P1 and one invalid plan run with
        # no key through the public Door.plan; the zero budgets and the zero
        # cap refuse; the backend has read no request.
        with tempfile.TemporaryDirectory(prefix="thinkthen-jvm-plan-") as cache:
            env = child_env(HOME=cache, JAVA_HOME=str(JDK), JAVACMD=str(JDK / "bin/java"), LC_ALL="C.UTF-8",
                            THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1", THINKTHEN_CACHE=cache)
            p1 = next(case for case in corpus["cases"] if case["name"] == "plan-p1")
            actual = type_case(["plan", json.dumps(p1["plan_input"])], env, "plan-p1")
            assert actual == p1["response"] and checks["plan"].is_valid(actual), actual
            invalid = dict(p1["plan_input"], settings={"batch": 0})
            assert type_case(["plan", json.dumps(invalid)], env, "plan-batch-0") == {"failed": {"kind": "usage", "code": 1}}
            env["THINKTHEN_API_KEY"] = "sk-type-contract-loopback"
            assert type_case(["limits"], env, "limits") == {"limits": "pass"}
            assert sent() == 0, "a plan or a refused call sent a request"
            print("java plan P1, usage refusal, zero budgets and zero cap: zero sends", flush=True)
    for lang in ("java", "kotlin", "scala"):
        count = 0
        with tempfile.TemporaryDirectory(prefix=f"thinkthen-{lang}-types-") as cache:
            for index, case in enumerate(corpus["cases"]):
                if "request" not in case or case.get("schema_only", False) or (selected and case["name"] != selected):
                    continue
                route = case.get("case_id", "generic")
                if route != "generic":
                    assert route in conformance, case["name"]
                # Java decodes command-line arguments with the native locale charset.
                env = child_env(HOME=cache, JAVA_HOME=str(JDK), JAVACMD=str(JDK / "bin/java"), LC_ALL="C.UTF-8")
                env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1", THINKTHEN_API_KEY="sk-type-contract-loopback", THINKTHEN_CACHE=str(Path(cache) / str(index)))
                request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":"))
                actual = type_case([request], env, case["name"], lang)
                if lang == "java" and case["name"] in FIELDS:
                    # The shared null and failed annotate members, read through Door.field.
                    env["THINKTHEN_CACHE"] = str(Path(cache) / f"{index}-fields")
                    assert type_case(["fields", request], env, case["name"]) == FIELDS[case["name"]], case["name"]
                if "expected_error" in case:
                    assert actual == {"failed": {"kind": "usage", "code": 1}}, (lang, case["name"], actual)
                    count += 1
                    continue
                if case["definition"] != "usage":
                    assert checks["callSuccess"].is_valid(actual), (lang, case["name"], actual)
                    actual = actual["value"]
                if case["definition"] != "doorRequest":
                    assert checks[case["definition"]].is_valid(actual), (lang, case["name"], actual)
                if "response" in case:
                    assert actual == case["response"], (lang, case["name"], actual)
                if "expect_fields" in case:
                    assert shared.subset(actual, case["expect_fields"]), (lang, case["name"], actual)
                if "expect_keys" in case:
                    assert set(actual) == set(case["expect_keys"]), (lang, case["name"], actual)
                if "offsets" in case:
                    shared.check_offsets(case, actual, conformance)
                count += 1
        print(f"{lang} J1 public binding: {len(corpus['cases'])} schema cases, {count} runtime cases passed", flush=True)
    if selected is None:
        with tempfile.TemporaryDirectory(prefix="thinkthen-native-complete-") as folder:
            path=Path(folder)/"unicode.txt"
            path.write_bytes("Maria Chen\r\nAlex Lee\r\n".encode())
            settings=json.dumps({"base_url":f"http://127.0.0.1:{port}/arm/full/v1","model":"fixed","cache":False,"batch":"max","throttle":1,"max_retries":0})
            env=child_env(HOME=folder,XDG_CONFIG_HOME=folder,XDG_CACHE_HOME=folder,XDG_STATE_HOME=folder,
                          THINKTHEN_API_KEY="sk-native-complete-loopback",THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1",
                          TT_NATIVE_SETTINGS=settings,TT_NATIVE_FILE=str(path),LD_LIBRARY_PATH=str(native))
            for lang in ("java","kotlin","scala"):
                before=sent()
                assert type_case(["native"],env,"native complete",lang)=={"native":"pass"}
                assert sent()-before==22,(lang,"complete native listener count")
                print(lang+" named native: ten functions, typed fields and physical file locations PASS",flush=True)

finally:
    shared.stop_backend(backend)

if selected is not None:
    raise SystemExit(0)
failures = False
for lang, main, runtime in (("java", "TypeCase", ""), ("kotlin", "TypeCaseKt", str(KOTLIN / "lib/kotlin-stdlib.jar")), ("scala", "scalaTypeCase", str(SCALA / "lib/scala.jar"))):
    failures |= native_parity(lang, java + ["-cp", f"{classes}:{classpath}" + (f":{runtime}" if runtime else ""), main])
raise SystemExit(failures)
