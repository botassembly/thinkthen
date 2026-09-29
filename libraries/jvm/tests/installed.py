"""Run the accepted Java, Kotlin, and Scala installed consumers from product-built JARs."""
import datetime
import json
import os
from pathlib import Path
import shutil
import subprocess
import zipfile
import xml.etree.ElementTree as ET
from toolchains import JDK, KOTLIN, SCALA

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "target"
RELEASE = os.environ.get("THINKTHEN_RELEASE_JVM_DIR")
RELEASE_C = os.environ.get("THINKTHEN_RELEASE_C_DIR")
assert bool(RELEASE) == bool(RELEASE_C), "installed release needs both package paths"
if RELEASE:
    package = Path(RELEASE)
    assert {p.name for p in package.iterdir()} == {
        "thinkthen-door.jar", "thinkthen-kotlin.jar", "thinkthen-scala.jar",
        "pom.xml", "LICENSE", "README.md", "THINKTHEN-PACKAGE-INPUTS"}
    pom = ET.fromstring((package / "pom.xml").read_text())
    metadata = {item.tag.rsplit("}", 1)[-1]: (item.text or "").strip() for item in pom}
    assert (metadata["groupId"], metadata["artifactId"], metadata["version"]) == (
        "io.github.botassembly", "thinkthen-jvm", "0.0.1")
    for name, required in (("door", "thinkthen/Door.class"), ("kotlin", "KotlinFacade.class"),
                           ("scala", "ScalaFacade.class")):
        with zipfile.ZipFile(package / f"thinkthen-{name}.jar") as bundle:
            members = set(bundle.namelist())
            assert required in members and not any("Stale.class" in item for item in members), (name, members)
            assert not any(token in bundle.read(item) for item in members
                           for token in (b"/home/ian", b"thinkthen_panic_probe", b"tt-canary-275")), name
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
    for jar in ((Path(RELEASE) if RELEASE else TARGET / "jars").glob("thinkthen-*.jar")):
        shutil.copyfile(jar, trial / "jars" / jar.name)
    native_source = Path(RELEASE_C) / "lib/libthinkthen.so" if RELEASE else TARGET / "native/libthinkthen.so"
    native_copy = trial / "native/lib/libthinkthen.so"
    shutil.copyfile(native_source, native_copy)
    if RELEASE:
        assert native_source != native_copy and native_source.read_bytes() == native_copy.read_bytes()
    names = ("consumer-run.py", "backend.py", "process_group.py")
    if RELEASE: names += (f"Installed{lang.capitalize()}.{dict(java='java',kotlin='kt',scala='scala')[lang]}",)
    else: names += ("Matrix.java", "Direct.java", "StrictScalar.java", "Concurrent.java",
                   "BoundedString.java", "ResultEnvelopeTest.java")
    for name in names:
        shutil.copyfile(ROOT / "tests" / name, trial / "project" / name)
    if not RELEASE:
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
    if RELEASE:
        command += [lang]
    result = subprocess.run(command, env={"PATH": "/usr/bin:/bin"}, capture_output=True, text=True, timeout=420)
    (trial / "outer.log").write_text(result.stdout + result.stderr)
    assert result.returncode == 0, (lang, result.stdout[-1000:], result.stderr[-1000:])
    summary = json.loads((trial / "home/summary.json").read_text())
    if RELEASE:
        assert summary["arrivals"] == [f"release-{lang}"] and summary["attempts"] == summary["connections"] == 1, summary
        print(f"installed {lang}: one exact body and native load PASS", flush=True)
    else:
        assert len(summary["arrivals"]) == 60 and summary["attempts"] == summary["connections"] == 60, lang
        print(f"installed {lang}: 60 exact arrivals, strict cancellation and planted negatives PASS", flush=True)
