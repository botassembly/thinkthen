"""Run the accepted Java, Kotlin, and Scala installed consumers from product-built JARs."""
import datetime
import json
import os
from pathlib import Path
import shutil
import subprocess
from toolchains import JDK, KOTLIN, SCALA

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "target"
RUN = TARGET / "installed" / datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
RUN.mkdir(parents=True)
tools = "bash env ls expr dirname uname readlink basename which sed grep cat tr cut head realpath find rm mkdir".split()
mounts = []
for name in tools:
    source = Path("/usr/bin") / name
    if source.exists():
        mounts += ["--ro-bind", str(source), "/usr/bin/" + name]
python = Path("/usr/bin/python3").resolve()
assert python.is_file(), "installed consumer needs /usr/bin/python3"
mounts += ["--ro-bind", str(python), "/usr/bin/python3"]
for index, lang in enumerate(("java", "kotlin", "scala")):
    trial = RUN / f"consumer-{lang}-{index}"
    for folder in ("project/thinkthen", "jars", "native/lib"):
        (trial / folder).mkdir(parents=True)
    for jar in (TARGET / "jars").glob("thinkthen-*.jar"):
        shutil.copyfile(jar, trial / "jars" / jar.name)
    shutil.copyfile(TARGET / "native/libthinkthen.so", trial / "native/lib/libthinkthen.so")
    for name in ("consumer-run.py", "backend.py", "process_group.py", "Matrix.java", "Direct.java",
                 "StrictScalar.java", "Concurrent.java", "BoundedString.java", "ResultEnvelopeTest.java"):
        shutil.copyfile(ROOT / "tests" / name, trial / "project" / name)
    shutil.copyfile(ROOT / "tests/thinkthen/ProbeDoor.java", trial / "project/thinkthen/ProbeDoor.java")
    for name in ("KotlinCaller.kt", "ScalaCaller.scala"):
        source = ROOT / ("kotlin" if name.endswith("kt") else "scala") / name
        shutil.copyfile(source, trial / "project" / name)
    command = ["/usr/bin/bwrap", "--clearenv", "--unshare-user", "--unshare-pid", "--unshare-net", "--die-with-parent",
               "--dir", "/usr", "--dir", "/usr/bin", "--dir", "/opt", "--ro-bind", "/etc", "/etc",
               "--ro-bind", "/usr/lib", "/usr/lib", "--ro-bind", "/usr/libexec", "/usr/libexec", "--ro-bind", "/usr/share", "/usr/share",
               "--ro-bind", "/lib64", "/lib64", "--symlink", "usr/lib", "/lib", "--symlink", "usr/bin", "/bin",
               "--ro-bind", str(JDK), "/opt/jdk", "--ro-bind", str(KOTLIN), "/opt/kotlin", "--ro-bind", str(SCALA), "/opt/scala",
               "--bind", str(trial), "/work", "--tmpfs", "/tmp", "--proc", "/proc", "--dev", "/dev", *mounts,
               "--setenv", "PATH", "/usr/bin:/opt/jdk/bin", "--chdir", "/work/project", "/usr/bin/python3", "consumer-run.py"]
    result = subprocess.run(command, env={"PATH": "/usr/bin:/bin"}, capture_output=True, text=True, timeout=420)
    (trial / "outer.log").write_text(result.stdout + result.stderr)
    assert result.returncode == 0, (lang, result.stdout[-1000:], result.stderr[-1000:])
    summary = json.loads((trial / "home/summary.json").read_text())
    assert len(summary["arrivals"]) == 60 and summary["attempts"] == summary["connections"] == 60, lang
    print(f"installed {lang}: 60 exact arrivals, strict cancellation and planted negatives PASS", flush=True)
