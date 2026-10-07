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
classes = TARGET / "classes/typecase"
classes.mkdir(parents=True, exist_ok=True)
jars = TARGET / "jars"
classpath = ":".join(str(jars / f"thinkthen-{part}.jar") for part in ("door", "kotlin", "scala"))
compiler_env = child_env(JAVA_HOME=str(JDK), JAVACMD=str(JDK / "bin/java"),
                         PATH=str(JDK / "bin") + ":" + os.environ.get("PATH", "/usr/bin:/bin"))
java = [str(JDK / "bin/java"), "--enable-preview", "--enable-native-access=ALL-UNNAMED", "-Dthinkthen.library=" + str(TARGET / "native/libthinkthen.so")]
subprocess.run([str(JDK / "bin/javac"), "--enable-preview", "--release", "21", "-cp", classpath, "-d", str(classes), str(HERE / "TypeCase.java"), str(HERE / "CarrierChecks.java"), str(HERE / "NativeChecks.java")], env=compiler_env, check=True)
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
                          TT_NATIVE_SETTINGS=settings,TT_NATIVE_FILE=str(path),LD_LIBRARY_PATH=str(TARGET / "native"))
            for lang in ("java","kotlin","scala"):
                before=sent()
                assert type_case(["native"],env,"native complete",lang)=={"native":"pass"}
                assert sent()-before==22,(lang,"complete native listener count")
                print(lang+" named native: ten functions, typed fields and physical file locations PASS",flush=True)

finally:
    shared.stop_backend(backend)
