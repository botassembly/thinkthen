"""Shared cases through installed stable Java, Kotlin and Scala owned-session APIs.

Without --case this runs every required shared case and reports each failure.
The explicit selector is focused development evidence, never full parity.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import selectors
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from toolchains import JDK, KOTLIN, SCALA, stable
ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT / "conformance/children"))
from children import child_env
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument("--jars",type=Path,required=True)
parser.add_argument("--out",type=Path,required=True)
parser.add_argument("--case",action="append",default=[])
args=parser.parse_args()
selected=set(args.case)
stable()
jars=args.jars.resolve(strict=True)
classes=args.out.resolve() / "classes"
classes.mkdir(parents=True,exist_ok=True)
package_jars=sorted(jars.glob("thinkthen-*.jar"))
if not package_jars: raise ValueError("installed JVM parity needs built JARs")
classpath=os.pathsep.join(map(str,package_jars))
compiler_env=child_env(JAVA_HOME=str(JDK),JAVACMD=str(JDK / "bin/java"),PATH=str(JDK / "bin")+":"+os.environ.get("PATH","/usr/bin:/bin"),JAVA_OPTS="-Xmx1g -XX:ActiveProcessorCount=2")
if not os.environ.get('THINKTHEN_PARITY_PREPARED'):
    subprocess.run([str(JDK / "bin/javac"),"-J-Xmx1g","--release","22","-cp",classpath,"-d",str(classes),str(HERE / "SessionCases.java"),str(HERE / "CaseBatch.java"),str(HERE / "FixtureJson.java")],env=compiler_env,check=True)
    subprocess.run([str(KOTLIN / "bin/kotlinc"),"-J-Xmx1g","-J-XX:ActiveProcessorCount=2","-jvm-target","22","-classpath",str(classes)+":"+classpath+":"+str(KOTLIN / "lib/kotlinx-coroutines-core-jvm.jar"),str(HERE / "SessionCases.kt"),"-d",str(classes)],env=compiler_env,check=True)
    subprocess.run([str(SCALA / "bin/scalac"),"-J-Xmx1g","-J-XX:ActiveProcessorCount=2","-classpath",str(classes)+":"+classpath,"-d",str(classes),str(HERE / "SessionCases.scala")],env=compiler_env,check=True)
if os.environ.get('THINKTHEN_PARITY_PREPARE'):
    raise SystemExit(0)
java=[str(JDK / "bin/java"),"--enable-native-access=ALL-UNNAMED","-XX:ActiveProcessorCount=2","-Xmx1g"]
# Native code loads only from the installed classifier, never a separate C build.
native=jars


class CaseProcess:
    def __init__(self, command):
        self.command = command + ["batch"]
        self.stderr = tempfile.TemporaryFile(mode="w+")
        self.process = subprocess.Popen(self.command, env=child_env(LC_ALL="C.UTF-8"),
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=self.stderr)
        self.reader = selectors.DefaultSelector()
        self.reader.register(self.process.stdout, selectors.EVENT_READ)
        self.buffer = bytearray()

    def send(self, value):
        self.process.stdin.write(value.encode())
        self.process.stdin.flush()

    def line(self, timeout):
        deadline = time.monotonic() + timeout
        boundary = self.buffer.find(b"\n")
        while boundary < 0:
            remaining = deadline - time.monotonic()
            if remaining <= 0 or not self.reader.select(remaining):
                self.process.kill()
                self.process.wait()
                raise subprocess.TimeoutExpired(self.command, timeout)
            chunk = os.read(self.process.stdout.fileno(), 65536)
            if not chunk:
                self.stderr.seek(0)
                raise RuntimeError("JVM fixture process ended: " + self.stderr.read()[:1500])
            found = chunk.find(b"\n")
            if found >= 0:
                boundary = len(self.buffer) + found
            self.buffer.extend(chunk)
        line = self.buffer[:boundary].decode() + "\n"
        del self.buffer[:boundary + 1]
        return line

    def close(self):
        try:
            self.process.stdin.close()
        except BrokenPipeError:
            pass
        try:
            self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()
        self.reader.close()
        self.process.stdout.close()
        self.stderr.seek(0)
        errors = self.stderr.read()
        self.stderr.close()
        assert self.process.returncode == 0 and not errors, (self.process.returncode, errors[:1500])


def native_parity(consumer, running):
    # Shared recipes/assertions are read-only; execution uses each actual named consumer.
    import sqlite3
    sys.path.insert(0, str(ROOT / "conformance"))
    import parity, c_parity, c_images
    cases = {c["id"]: c for c in json.loads((ROOT / "conformance/cases.json").read_text())["cases"]}
    named = {c["id"]: c for c in json.loads((ROOT / "conformance/named-inputs.json").read_text())["cases"]}
    failures = []
    required = parity.required_cases(parity.inventory(), consumer)
    if selected and not selected.issubset(required):
        raise ValueError("unknown required case selector: " + repr(selected - required.keys()))
    for row in required.values():
        if selected and row["id"] not in selected:
            continue
        failure = None
        try:
            value = c_parity.document(row, cases, named)
            with tempfile.TemporaryDirectory(prefix=f"thinkthen-{consumer}-parity-") as folder:
                home = Path(folder) / "home"
                home.mkdir()
                env = child_env(home=home,
                                PATH=os.environ.get("PATH", "/usr/bin:/bin"),
                                LC_ALL="C.UTF-8",
                                DOTNET_CLI_TELEMETRY_OPTOUT="1",
                                LD_LIBRARY_PATH=str(native))
                backend = c_parity.Backend(Path(os.environ.get("CARGO_TARGET_DIR", str(ROOT / "target"))) / "debug/conformance-backend", env)
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
                            backend = c_parity.Backend(Path(os.environ.get("CARGO_TARGET_DIR", str(ROOT / "target"))) / "debug/conformance-backend", env)
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
                            framing = {**step, "engine_settings": c_parity.compact(given), "call_result_contract": "call_result" in step["expect"]}
                            if step.get("caption_files"):
                                framing["items"] = [""] * len(step["items"])
                            framing.update(batch_home=str(home), batch_environment=env)
                            running.send(c_parity.compact(framing) + "\n")
                            if step.get("held_cancel"):
                                try:
                                    assert backend.read("wait 1") == "wait 1"
                                    running.send("!")
                                    assert running.line(60) == "cancel-fired\n"
                                finally:
                                    backend.process.stdin.write("release\n")
                                    backend.process.stdin.flush()
                            result = json.loads(running.line(120 if value.get("image_variants") else 60))
                            assert running.line(60) == "case-complete\n", "fixture engine did not close"
                            return result
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
                            capture = json.loads(backend.read("capture"))["bodies"]
                            assert capture, got
                            request = json.loads(capture[0])
                            expected = {f"q{i+1}": {"type": "noul", "instructions": f"The text is {c_parity.compact(item)}. {step['question']['decide']}"} for i, item in enumerate(step["items"][:2])}
                            assert request["questions"] == expected
                        expected = original
                        if isinstance(step.get("raw"), str) and original["expect"].get("error") == "usage":
                            # SessionCases preserves authored bytes through RequestQuestionFile.
                            # question-file.md assigns malformed local files Local, unlike inline definitions.
                            expected = {**original, "expect": {**original["expect"], "error": "local"}}
                        c_parity.assertions(row, expected, got, int(backend.read("count")) - (before if step.get("count_delta") else 0))
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
        except (AssertionError, ValueError, KeyError, TypeError, AttributeError, IndexError, subprocess.SubprocessError, OSError, RuntimeError) as error:
            failure = type(error).__name__ + ": " + str(error)
            failures.append((row["id"], failure))
            print(f"{consumer} fixture {row['id']} failed: {failure[:1500]}", file=sys.stderr)
        print("parity: " + json.dumps({"consumer": consumer, "case": row["id"], "checks": row.get("checks", ["named", "runtime"]), "status": "fail" if failure else "pass"}), flush=True)
    print(f"{consumer} native fixture failures: {len(failures)}")
    return bool(failures)

def language_cases(language):
    started = time.monotonic()
    lang, main, runtime = language
    installed_cp = os.pathsep.join(str(jar) for jar in package_jars if jar.name == "thinkthen-" + ("session" if lang == "java" else lang) + ".jar" or jar.name.startswith("thinkthen-natives-"))
    command = java+["-cp",f"{classes}:{installed_cp}"+(f":{runtime}" if runtime else ""),main]
    running = CaseProcess(command)
    try:
        return native_parity(lang, running)
    finally:
        running.close()
        print("THINKTHEN_CONFORMANCE_DURATION " + json.dumps({"consumer": lang, "duration_seconds": time.monotonic() - started}), flush=True)


languages = (("java","thinkthen.SessionCases",""),
             ("kotlin","SessionCasesKt",str(KOTLIN / "lib/kotlin-stdlib.jar")+":"+str(KOTLIN / "lib/kotlinx-coroutines-core-jvm.jar")),
             ("scala","SessionCases",str(SCALA / "lib/scala.jar")))
if member := os.environ.get('THINKTHEN_PARITY_CONSUMER'):
    failures = [language_cases(next(language for language in languages if language[0] == member))]
else:
    with ThreadPoolExecutor(max_workers=len(languages)) as pool:
        failures = list(pool.map(language_cases, languages))
print("Focused shared cases only" if selected else "All required installed shared cases executed",flush=True)
raise SystemExit(any(failures))
