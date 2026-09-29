"""Run J1 schema controls and applicable runtime cases through each public JVM facade."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
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
conformance = {case["id"]: case for case in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
classes = TARGET / "classes/typecase"
classes.mkdir(parents=True, exist_ok=True)
jars = TARGET / "jars"
classpath = ":".join(str(jars / f"thinkthen-{part}.jar") for part in ("door", "kotlin", "scala"))
java = [str(JDK / "bin/java"), "--enable-preview", "--enable-native-access=ALL-UNNAMED", "-Dthinkthen.library=" + str(TARGET / "native/libthinkthen.so")]
subprocess.run([str(JDK / "bin/javac"), "--enable-preview", "--release", "21", "-cp", classpath, "-d", str(classes), str(HERE / "TypeCase.java")], check=True)
subprocess.run([str(KOTLIN / "bin/kotlinc"), "-J-XX:ActiveProcessorCount=2", "-jvm-target", "21", "-classpath", classpath, str(HERE / "TypeCase.kt"), "-d", str(classes)], check=True)
subprocess.run([str(SCALA / "bin/scalac"), "-J-XX:ActiveProcessorCount=2", "-classpath", classpath, "-d", str(classes), str(HERE / "TypeCase.scala")], check=True)
backend, port = shared.start_backend()
try:
    for lang, main, runtime in (("java", "TypeCase", ""), ("kotlin", "TypeCaseKt", str(KOTLIN / "lib/kotlin-stdlib.jar")), ("scala", "scalaTypeCase", str(SCALA / "lib/scala.jar"))):
        count = 0
        with tempfile.TemporaryDirectory(prefix=f"thinkthen-{lang}-types-") as cache:
            for index, case in enumerate(corpus["cases"]):
                if "request" not in case or case.get("schema_only", False):
                    continue
                route = case.get("case_id", "generic")
                if route != "generic":
                    assert route in conformance, case["name"]
                env = os.environ.copy()
                env.update(THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/{'generic' if route == 'generic' else 'case/' + route}/v1", THINKTHEN_API_KEY="sk-type-contract-loopback", THINKTHEN_CACHE=str(Path(cache) / str(index)))
                request = json.dumps(case["request"], ensure_ascii=False, separators=(",", ":"))
                cmd = java + ["-cp", f"{classes}:{classpath}" + (f":{runtime}" if runtime else ""), main, request]
                result = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=40)
                assert result.returncode == 0, (lang, case["name"], result.stderr)
                actual = json.loads(result.stdout)
                if "expected_error" in case:
                    assert actual["error"] == "usage", (lang, case["name"], actual)
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
                if "expect_count" in case:
                    edges = actual["edges"]
                    assert len(edges) == case["expect_count"] and edges[0] == case["expect_first_edge"], (lang, case["name"], actual)
                if "offsets" in case:
                    shared.check_offsets(case, actual, conformance)
                count += 1
        print(f"{lang} J1 public binding: {len(corpus['cases'])} schema cases, {count} runtime cases passed", flush=True)
finally:
    shared.stop_backend(backend)
