"""Each public JVM constructor selects the configured route and fake bearer."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from toolchains import JDK, KOTLIN, SCALA

ROOT = Path(__file__).resolve().parents[3]
PACKAGE = ROOT / "libraries/jvm"
sys.path.insert(0, str(ROOT / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths

with tempfile.TemporaryDirectory(prefix="jvm-named-") as scratch:
    folder = Path(scratch)
    cp = os.pathsep.join(str(p) for p in (PACKAGE / "target/jars").glob("thinkthen-*.jar"))
    assert cp, "build JVM package JARs first"
    cp += os.pathsep + scratch
    build_env = child_env(HOME=scratch)
    subprocess.run([str(JDK / "bin/javac"), "--enable-preview", "--release", "21", "-cp", cp,
                    "-d", scratch, str(PACKAGE / "tests/InstalledJava.java")], env=build_env, check=True, timeout=60)
    subprocess.run([str(KOTLIN / "bin/kotlinc"), "-J-XX:ActiveProcessorCount=2", "-jvm-target", "21",
                    "-classpath", cp, str(PACKAGE / "tests/InstalledKotlin.kt"), "-d", scratch], env=build_env, check=True, timeout=90)
    subprocess.run([str(SCALA / "bin/scalac"), "-J-XX:ActiveProcessorCount=2", "-classpath", cp,
                    "-d", scratch, str(PACKAGE / "tests/InstalledScala.scala")], env=build_env, check=True, timeout=90)
    for lang, main, runtime in (("java", "InstalledJava", ""), ("kotlin", "InstalledKotlinKt", str(KOTLIN / "lib/kotlin-stdlib.jar")),
                                ("scala", "installedScala", str(SCALA / "lib/scala.jar"))):
        server = subprocess.Popen([ROOT / "target/debug/conformance-backend"],
                                  env=child_env(THINKTHEN_TEST_MARKERS='{"local":"tt-named-loopback"}'),
                                  stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            row = next(row for row in ROWS if row["name"] == "typesafe")
            env = child_env(home=scratch,
                            THINKTHEN_API_KEY="tt-unnamed-loopback", TYPESAFE_API_KEY="tt-named-loopback",
                            THINKTHEN_BASE_URL=f"http://127.0.0.1:{port}/generic/v1", TT_NAMED_BACKEND="1")
            configuration(env, {"local":alias(row, f"http://127.0.0.1:{port}/arm/full/capture/v1")})
            run = subprocess.run([str(JDK / "bin/java"), "--enable-preview", "--enable-native-access=ALL-UNNAMED",
                                  "-XX:ActiveProcessorCount=2", f"-Dthinkthen.library={ROOT}/libraries/c/target/debug/libthinkthen_c.so",
                                  "-cp", cp + (os.pathsep + runtime if runtime else ""), main], env=env, capture_output=True, text=True, timeout=60)
            assert run.returncode == 0 and f"{lang.upper()}_NAMED_BACKEND_PASS" in run.stdout, (run.stdout, run.stderr)
            for command, expected in (("count", 1), ("paths", paths(row["path"])),
                                      ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                server.stdin.write(command + "\n"); server.stdin.flush()
                assert json.loads(server.stdout.readline()) == expected, command
            server.stdin.write("capture\n"); server.stdin.flush()
            bodies = json.loads(server.stdout.readline())["bodies"]
            expected = {"state":"Each question quotes the text it asks about.", "model":row["model"],
                        "questions":{"q1":{"type":"noul","instructions":f'The text is "named-{lang}". Is it?'}}}
            assert [json.loads(body) for body in bodies] == [expected]
            assert "tt-named-loopback" not in run.stdout + run.stderr + json.dumps(bodies)
            print(f"{lang}: named backend path, bearer, result and secrecy PASS")
        finally:
            server.stdin.close(); server.wait(timeout=10)
