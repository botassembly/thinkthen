#!/usr/bin/env python3
"""Synthetic outside-in receipt and package refusals for release-managed-pair."""
import hashlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import zipfile

HELPER = Path(__file__).with_name("release-managed-pair.py")
COMMIT = "a" * 40
VERSION = "0.0.1"
TARGET = "x86_64-unknown-linux-gnu"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_tar(path, files, commit=None):
    with tarfile.open(path, "w:gz" if path.suffix == ".gz" else "w", pax_headers={"comment": commit} if commit else {}) as archive:
        for name, data in files.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            archive.addfile(info, io.BytesIO(data))


def write_zip(path, files):
    with zipfile.ZipFile(path, "w") as archive:
        for name, data in files.items():
            archive.writestr(name, data)


def read_zip(data):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        return {name: archive.read(name) for name in archive.namelist()}


def save_json(path, data):
    path.write_text(json.dumps(data) + "\n")


def run(base, mode, expected):
    cmd = [sys.executable, str(HELPER), mode,
           "--source-tar", str(base / "source.tar"), "--source-receipt", str(base / "source.json"),
           "--c-archive", str(base / f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"),
           "--c-receipt", str(base / "c.json"), "--managed-receipt", str(base / "managed.json"),
           "--output", str(base / "out"), "--target", TARGET, "--version", VERSION]
    if mode == "assemble":
        cmd += ["--nupkg", str(base / f"Botassembly.ThinkThen.{VERSION}.nupkg"),
                "--jars", str(base / "jars")]
    result = subprocess.run(cmd, capture_output=True, text=True)
    assert result.returncode == expected, (result.returncode, result.stdout, result.stderr)


def fixture(base):
    source = {
        "libraries/csharp/README.md": b"C# readme\n", "libraries/csharp/LICENSE": b"MIT\n",
        "libraries/jvm/README.md": b"JVM readme\n", "libraries/jvm/LICENSE": b"MIT\n",
        "libraries/jvm/pom.xml": b"<project><groupId>io.github.botassembly</groupId><artifactId>thinkthen-jvm</artifactId><version>0.0.1</version></project>",
    }
    write_tar(base / "source.tar", source, COMMIT)
    save_json(base / "source.json", {"commit": COMMIT, "sha256": sha((base / "source.tar").read_bytes())})
    c_name = f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"
    write_tar(base / c_name, {"include/thinkthen.h": b"header"})
    c_hash = sha((base / c_name).read_bytes())
    save_json(base / "c.json", {"name": c_name, "sha256": c_hash})
    (base / (c_name + ".sha256")).write_text(f"{c_hash}  {c_name}\n")
    nupkg = base / f"Botassembly.ThinkThen.{VERSION}.nupkg"
    nuspec = b"<package><metadata><id>Botassembly.ThinkThen</id><version>0.0.1</version></metadata></package>"
    write_zip(nupkg, {"Botassembly.ThinkThen.nuspec": nuspec,
                      "lib/net8.0/ThinkThen.dll": b"DLL", "README.md": source["libraries/csharp/README.md"],
                      "LICENSE": source["libraries/csharp/LICENSE"], "_rels/.rels": b"rels",
                      "[Content_Types].xml": b"types",
                      "package/services/metadata/core-properties/abc.psmdcp": b"props"})
    jars = base / "jars"
    jars.mkdir()
    managed = {"nupkg": sha(nupkg.read_bytes())}
    classes = {
        "door": "Door$Answer Door$Failure Door$FailureKind Door$NativeFailure Door$Outcome Door$Token Door ResultEnvelope$Reader ResultEnvelope$Value ResultEnvelope".split(),
        "kotlin": "KotlinCallerKt KotlinFacade$RunningDecision KotlinFacade".split(),
        "scala": "ScalaCaller$package$ ScalaCaller$package ScalaFacade$RunningDecision ScalaFacade scalaCaller".split(),
    }
    for kind in ("door", "kotlin", "scala"):
        path = jars / f"thinkthen-{kind}.jar"
        files = {"META-INF/MANIFEST.MF": b"Manifest-Version: 1.0\n"}
        files.update({("thinkthen/" if kind == "door" else "") + name + ".class": b"class"
                      for name in classes[kind]})
        if kind == "kotlin":
            files["META-INF/main.kotlin_module"] = b"module"
        if kind == "scala":
            files.update({name + ".tasty": b"tasty" for name in
                          ("ScalaCaller$package", "ScalaFacade", "scalaCaller")})
        write_zip(path, files)
        managed[kind] = sha(path.read_bytes())
    save_json(base / "managed.json", managed)
    (base / "out").mkdir()
    return source


def mutate_outer(base, family, change):
    name = f"thinkthen-{family}-{VERSION}-{TARGET}.tar.gz"
    path = base / "out" / name
    with tarfile.open(path) as archive:
        files = {member.name.removeprefix("./"): archive.extractfile(member).read()
                 for member in archive if member.isfile()}
    change(files)
    write_tar(path, files)
    (base / "out" / (name + ".sha256")).write_text(f"{sha(path.read_bytes())}  {name}\n")


def case(label, change, mode="verify"):
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        source = fixture(base)
        run(base, "assemble", 0)
        change(base, source)
        run(base, mode, 1)
        print(f"{label}: refused")


def main():
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        fixture(base)
        run(base, "assemble", 0)
        run(base, "verify", 0)
    def source_attack(base, source):
        source["libraries/csharp/README.md"] = b"altered"
        write_tar(base / "source.tar", source, COMMIT)
    case("same Git header, altered source", source_attack)
    def c_attack(base, source):
        path = base / f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"
        write_tar(path, {"include/thinkthen.h": b"other valid C"})
        (base / (path.name + ".sha256")).write_text(f"{sha(path.read_bytes())}  {path.name}\n")
    case("substituted C with matching sidecar", c_attack)
    def inner_attack(base, source):
        def change(files):
            name = f"Botassembly.ThinkThen.{VERSION}.nupkg"
            members = read_zip(files[name])
            members["lib/net8.0/ThinkThen.dll"] = b"changed"
            buf = io.BytesIO()
            write_zip(buf, members)
            files[name] = buf.getvalue()
        mutate_outer(base, "csharp", change)
    case("changed inner DLL and outer hash", inner_attack)
    case("changed POM and outer hash", lambda base, source: mutate_outer(
        base, "jvm", lambda files: files.__setitem__("pom.xml", b"<project/>")))
    case("missing wrapper member", lambda base, source: mutate_outer(
        base, "jvm", lambda files: files.pop("thinkthen-scala.jar")))
    case("extra wrapper member", lambda base, source: mutate_outer(
        base, "csharp", lambda files: files.__setitem__("private.so", b"native")))
    case("wrong inner package member", lambda base, source: mutate_outer(
        base, "csharp", lambda files: files.__setitem__(
            f"Botassembly.ThinkThen.{VERSION}.nupkg", extra_nupkg(files))))
    print("release-managed-pair self-test: pass")


def extra_nupkg(files):
    name = f"Botassembly.ThinkThen.{VERSION}.nupkg"
    members = read_zip(files[name])
    members["runtimes/linux/native/libthinkthen.so"] = b"native"
    buf = io.BytesIO()
    write_zip(buf, members)
    return buf.getvalue()


if __name__ == "__main__":
    main()
