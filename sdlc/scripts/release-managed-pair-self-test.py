#!/usr/bin/env python3
"""Synthetic outside-in receipt and package refusals for release-managed-pair."""
import hashlib
import os
import re
import shutil
import io
import importlib.machinery
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import zipfile

HELPER = Path(__file__).with_name("release-managed-pair.py")
COMMIT = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip()
# The managed gate reads the version from the crate, so the fixture does too (ticket 0376).
VERSION = re.search(r'(?m)^version = "([^"]+)"$', (Path(__file__).resolve().parents[2] / "crates/thinkthen/Cargo.toml").read_text())[1]
TARGET = "x86_64-unknown-linux-gnu"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_tar(path, files, commit=None, links=None, directories=()):
    with tarfile.open(path, "w:gz" if path.suffix == ".gz" else "w", pax_headers={"comment": commit} if commit else {}) as archive:
        for name in directories:
            info = tarfile.TarInfo(name)
            info.type = tarfile.DIRTYPE
            archive.addfile(info)
        for name, data in files.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            archive.addfile(info, io.BytesIO(data))
        for name, target in (links or {}).items():
            info = tarfile.TarInfo(name)
            info.type = tarfile.SYMTYPE
            info.linkname = target
            archive.addfile(info)


SOURCE_LINKS = {"CLAUDE.md": "AGENTS.md", "crates/thinkthen/LICENSE": "../../LICENSE"}
C_HEADER = "".join(f"#define THINKTHEN_VERSION_{name} {part}\n"
                   for name, part in zip(("MAJOR", "MINOR", "PATCH"), VERSION.split("."))).encode()


def write_c_tar(path, shared=b"fixture shared library"):
    write_tar(path, {"./include/thinkthen.h": C_HEADER,
                     "./lib/libthinkthen.a": b"fixture static library",
                     "./lib/libthinkthen.so": shared,
                     "./lib/pkgconfig/thinkthen.pc": f"Name: thinkthen\nVersion: {VERSION}\n".encode()},
              links={"./lib/libthinkthen.so.0": "libthinkthen.so"},
              directories=(".", "./include", "./lib", "./lib/pkgconfig"))


def write_zip(path, files):
    with zipfile.ZipFile(path, "w") as archive:
        for name, data in files.items():
            archive.writestr(name, data)


def read_zip(data):
    with zipfile.ZipFile(io.BytesIO(data)) as archive:
        return {name: archive.read(name) for name in archive.namelist()}


def save_json(path, data):
    path.write_text(json.dumps(data) + "\n")


def run(base, mode, expected, message=None):
    cmd = [sys.executable, str(HELPER), mode,
           "--source-tar", str(base / "source.tar"), "--source-receipt", str(base / "source.json"),
           "--c-archive", str(base / "out" / f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"),
           "--c-receipt", str(base / "c.json"), "--managed-receipt", str(base / "managed.json"),
           "--output", str(base / "out"), "--target", TARGET, "--version", VERSION]
    if mode == "assemble":
        cmd += ["--nupkg", str(base / f"Botassembly.ThinkThen.{VERSION}.nupkg"),
                "--jars", str(base / "jars")]
    result = subprocess.run(cmd, capture_output=True, text=True)
    assert result.returncode == expected, (result.returncode, result.stdout, result.stderr)
    if message is not None:
        assert message in result.stderr, (message, result.stderr)


def gate(base, expected):
    provenance = base / "provenance"
    provenance.mkdir(exist_ok=True)
    for name in ("source.tar", "source.json", "c.json", "managed.json"):
        shutil.copyfile(base / name, provenance / name)
    result = subprocess.run(["sh", str(HELPER.with_name("release-workflow")), "managed-gate",
                             str(base / "out"), TARGET, COMMIT, str(provenance)],
                            capture_output=True, text=True)
    assert result.returncode == expected, (result.returncode, result.stdout, result.stderr)


def fixture(base):
    source = {
        "libraries/c/include/thinkthen.h": C_HEADER,
        "libraries/csharp/README.md": b"C# readme\n", "libraries/csharp/LICENSE": b"MIT\n",
        "libraries/jvm/README.md": b"JVM readme\n", "libraries/jvm/LICENSE": b"MIT\n",
        "libraries/jvm/pom.xml": (HELPER.parents[2] / "libraries/jvm/pom.xml").read_bytes(),
    }
    write_tar(base / "source.tar", source, COMMIT, SOURCE_LINKS)
    save_json(base / "source.json", {"commit": COMMIT, "sha256": sha((base / "source.tar").read_bytes())})
    c_name = f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"
    (base / "out").mkdir()
    write_c_tar(base / "out" / c_name)
    c_hash = sha((base / "out" / c_name).read_bytes())
    save_json(base / "c.json", {"name": c_name, "sha256": c_hash})
    (base / "out" / (c_name + ".sha256")).write_text(f"{c_hash}  {c_name}\n")
    nupkg = base / f"Botassembly.ThinkThen.{VERSION}.nupkg"
    nuspec = f"<package><metadata><id>Botassembly.ThinkThen</id><version>{VERSION}</version></metadata></package>".encode()
    write_zip(nupkg, {"Botassembly.ThinkThen.nuspec": nuspec,
                      "lib/net8.0/ThinkThen.dll": b"DLL", "runtimes/linux-x64/native/libthinkthen.so": b"fixture shared library", "README.md": source["libraries/csharp/README.md"],
                      "LICENSE": source["libraries/csharp/LICENSE"], "_rels/.rels": b"rels",
                      "[Content_Types].xml": b"types",
                      "package/services/metadata/core-properties/abc.psmdcp": b"props"})
    jars = base / "jars"
    jars.mkdir()
    managed = {"nupkg": sha(nupkg.read_bytes())}
    inventory_members = {}
    # Synthetic members exercise the archive boundary, not the public API inventory.
    classes = {'door': ['Door'], 'kotlin': ['KotlinFacade'], 'scala': ['ScalaFacade']}
    for kind in ("door", "kotlin", "scala"):
        path = jars / f"thinkthen-{kind}.jar"
        files = {"META-INF/": b"", "META-INF/MANIFEST.MF": b"Manifest-Version: 1.0\n"}
        if kind == "door":
            files["thinkthen/"] = b""
        files.update({("thinkthen/" if kind == "door" else "") + name + ".class": b"class"
                      for name in classes[kind]})
        if kind == "kotlin":
            files["META-INF/main.kotlin_module"] = b"module"
        if kind == "scala":
            files.update({name + ".tasty": b"tasty" for name in
                          classes[kind]})
        inventory_members[kind] = sorted(name for name in files if not name.endswith('/') and name != 'META-INF/MANIFEST.MF')
        write_zip(path, files)
        managed[kind] = sha(path.read_bytes())
    spec = importlib.machinery.SourceFileLoader('package_inventory', str(HELPER.with_name('package-inventory.py'))).load_module()
    definition = spec.jvm_inventory()
    door = jars / 'thinkthen-door.jar'
    files = read_zip(door.read_bytes())
    files.pop('META-INF/MANIFEST.MF')
    files['META-INF/thinkthen/product-inventory.json'] = json.dumps(definition).encode()
    write_zip(door, files)
    managed['door'] = sha(door.read_bytes())
    classifier, asset = next((name, asset) for name, asset in definition['native'].items() if asset['target'] == TARGET)
    path = jars / asset['jar']
    write_zip(path, {name: b'fixture shared library' for name in asset['files']})
    managed[classifier] = sha(path.read_bytes())
    save_json(jars / 'product-inventory.json', dict(definition, members=inventory_members))
    save_json(base / "managed.json", managed)
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


def before_extraction(base):
    observer = base / "observer"
    observer.mkdir()
    marker = base / "tool-calls"
    for name in ("tar", "docker", "dotnet"):
        tool = observer / name
        tool.write_text("#!/bin/sh\nprintf '%s\n' '" + name + "' >>\"$TT_OBSERVER\"\nexit 92\n")
        tool.chmod(0o755)
    env = os.environ.copy()
    env.pop("THINKTHEN_API_KEY", None)
    env.update(PATH=str(observer) + os.pathsep + env["PATH"], TT_OBSERVER=str(marker),
               THINKTHEN_RELEASE_EXPECTED_SHA=COMMIT,
               THINKTHEN_RELEASE_SOURCE_TAR=str(base / "source.tar"),
               THINKTHEN_RELEASE_SOURCE_RECEIPT=str(base / "source.json"))
    workflow = str(HELPER.with_name("release-workflow"))
    container = subprocess.run(["sh", str(HELPER.with_name("release-container")),
                                str(base / "container-work"), str(base / "container-out")],
                               env=env, capture_output=True, text=True)
    assert container.returncode == 1 and "source tar differs" in container.stderr, container.stderr
    build = subprocess.run(["sh", workflow, "managed-build", str(base), str(base / "out"),
                            TARGET, COMMIT], env=env, capture_output=True, text=True)
    assert build.returncode == 1 and "source tar differs" in build.stderr, build.stderr
    assert not marker.exists(), "untrusted source reached extraction or a build tool"


def case(label, change, mode="verify"):
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        source = fixture(base)
        run(base, "assemble", 0)
        if label == "substituted C with matching sidecar":
            captured = base / "captured"
            captured.mkdir()
            shutil.copyfile(base / "source.json", captured / "source.json")
            c_path = base / "out" / f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"
            result = subprocess.run(["sh", str(HELPER.with_name("release-workflow")),
                                     "c-capture", str(c_path), str(captured), TARGET],
                                    capture_output=True, text=True)
            assert result.returncode == 0, result.stderr
            shutil.copyfile(captured / "c.json", base / "c.json")
        change(base, source)
        run(base, mode, 1)
        gate(base, 1)
        if label == "same Git header, altered source":
            before_extraction(base)
        if label == "substituted C with matching sidecar":
            build = subprocess.run(["sh", str(HELPER.with_name("release-workflow")),
                                    "managed-build", str(base), str(base / "out"), TARGET, COMMIT],
                                   capture_output=True, text=True)
            assert build.returncode == 1 and "C archive differs" in build.stderr, build.stderr
        print(f"{label}: refused")


def gate_case(label, change):
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        fixture(base)
        run(base, "assemble", 0)
        change(base / "out")
        gate(base, 1)
        print(f"{label}: refused")


def changed_zip(data, remove=None, extra_dir=None, symlink=None):
    output = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(data)) as original, zipfile.ZipFile(output, "w") as changed:
        for item in original.infolist():
            if item.filename == remove:
                continue
            payload = original.read(item)
            if item.filename == symlink:
                item.create_system = 3
                item.external_attr = 0o120777 << 16
            changed.writestr(item, payload)
        if extra_dir:
            changed.writestr(extra_dir, b"")
    return output.getvalue()


def inner_rejection(label, kind, mutate, message):
    member = f"Botassembly.ThinkThen.{VERSION}.nupkg" if kind == "nupkg" else f"thinkthen-{kind}.jar"
    family = "csharp" if kind == "nupkg" else "jvm"
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        fixture(base)
        path = (base / member) if kind == "nupkg" else (base / "jars" / member)
        altered = mutate(path.read_bytes())
        path.write_bytes(altered)
        values = json.loads((base / "managed.json").read_text())
        values[kind] = sha(altered)
        save_json(base / "managed.json", values)
        run(base, "assemble", 1, message)
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        fixture(base)
        run(base, "assemble", 0)
        altered = None
        def change(files):
            nonlocal altered
            altered = mutate(files[member])
            files[member] = altered
        mutate_outer(base, family, change)
        values = json.loads((base / "managed.json").read_text())
        values[kind] = sha(altered)
        save_json(base / "managed.json", values)
        run(base, "verify", 1, message)
    print(f"{label}: assemble and verify refused at {message}")


def workflow_mutations():
    checker = importlib.machinery.SourceFileLoader(
        "managed_workflows", str(HELPER.with_name("workflows"))).load_module()
    original = (HELPER.parents[2] / ".github/workflows/release.yml").read_text()
    cases = (
        ("source capture", "release-workflow source-capture", "release-workflow absent-source"),
        ("C capture", "release-workflow c-capture", "release-workflow absent-c"),
        ("internal upload", "name: provenance-x86", "name: public-provenance"),
        ("smoke gate", 'release-workflow managed-gate platform "$TARGET" "$SHA" provenance',
         'release-workflow absent-gate platform "$TARGET" "$SHA" provenance'),
        ("draft gate", 'release-workflow managed-gate "platform/platform-$target" "$target" "$SHA" provenance',
         'release-workflow absent-gate "platform/platform-$target" "$target" "$SHA" provenance'),
    )
    for label, before, after in cases:
        assert before in original, label
        altered = original.replace(before, after, 1)
        findings = checker.release("release.yml", checker.yaml.safe_load(altered))
        assert any("provenance" in item or "receipt capture" in item or "managed" in item
                   for item in findings), (label, findings)
    reordered = original.replace("release-workflow source-capture", "ORDER_PLACEHOLDER", 1).replace(
        "release-workflow c-capture", "release-workflow source-capture", 1).replace(
        "ORDER_PLACEHOLDER", "release-workflow c-capture", 1)
    findings = checker.release("release.yml", checker.yaml.safe_load(reordered))
    assert any("receipt capture order" in item for item in findings), findings


def tracked_links():
    """Every symlink Git tracks passes the source capture check."""
    helper = importlib.machinery.SourceFileLoader("managed_pair", str(HELPER)).load_module()
    data = subprocess.check_output(["git", "archive", "--format=tar", "HEAD"], cwd=HELPER.parents[2])
    helper.tar_files(data, "source")


def main():
    workflow_mutations()
    tracked_links()
    with tempfile.TemporaryDirectory() as temporary:
        base = Path(temporary)
        fixture(base)
        run(base, "assemble", 0)
        run(base, "verify", 0)
        gate(base, 0)
    identity = b"<groupId>io.github.botassembly</groupId>"
    for label, replacement, message in (
        ("duplicate project identity", identity * 2, "ambiguous XML groupId"),
        ("missing project identity", b"", "ambiguous XML groupId"),
        ("wrong project identity with matching dependency", b"<groupId>wrong</groupId>", "source POM identity differs"),
    ):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            source = fixture(base)
            source["libraries/jvm/pom.xml"] = source["libraries/jvm/pom.xml"].replace(
                identity, replacement, 1).replace(b"<groupId>org.jetbrains.kotlinx</groupId>", identity)
            write_tar(base / "source.tar", source, COMMIT, SOURCE_LINKS)
            save_json(base / "source.json", {"commit": COMMIT, "sha256": sha((base / "source.tar").read_bytes())})
            run(base, "assemble", 1, message)
            print(f"{label}: refused")
    def source_attack(base, source):
        source["libraries/csharp/README.md"] = b"altered"
        write_tar(base / "source.tar", source, COMMIT, SOURCE_LINKS)
    case("same Git header, altered source", source_attack)
    def c_attack(base, source):
        path = base / "out" / f"thinkthen-c-{VERSION}-{TARGET}.tar.gz"
        write_c_tar(path, b"other valid shared library")
        (base / "out" / (path.name + ".sha256")).write_text(f"{sha(path.read_bytes())}  {path.name}\n")
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
    def jar_attack(base, source):
        def change(files):
            members = read_zip(files["thinkthen-door.jar"])
            members["thinkthen/Door.class"] = b"changed class"
            buf = io.BytesIO()
            write_zip(buf, members)
            files["thinkthen-door.jar"] = buf.getvalue()
        mutate_outer(base, "jvm", change)
    case("changed inner JAR and outer hash", jar_attack)
    case("changed POM and outer hash", lambda base, source: mutate_outer(
        base, "jvm", lambda files: files.__setitem__("pom.xml", b"<project/>")))
    case("missing wrapper member", lambda base, source: mutate_outer(
        base, "jvm", lambda files: files.pop("thinkthen-scala.jar")))
    case("extra wrapper member", lambda base, source: mutate_outer(
        base, "csharp", lambda files: files.__setitem__("private.so", b"native")))
    case("wrong inner package member", lambda base, source: mutate_outer(
        base, "csharp", lambda files: files.__setitem__(
            f"Botassembly.ThinkThen.{VERSION}.nupkg", extra_nupkg(files))))
    inner_rejection("missing NuGet native asset", "nupkg",
                    lambda data: changed_zip(data, remove="runtimes/linux-x64/native/libthinkthen.so"),
                    "nupkg inventory differs")
    inner_rejection("missing JVM native resource", "natives-linux-x64",
                    lambda data: changed_zip(data, remove=f"META-INF/thinkthen/native/{TARGET}/libthinkthen.so"),
                    "JVM native asset differs from captured C")
    for required in ("_rels/.rels", "[Content_Types].xml"):
        inner_rejection(f"missing required {required}", "nupkg",
                        lambda data, required=required: changed_zip(data, remove=required),
                        "nupkg inventory differs")
    for kind in ("nupkg", "door"):
        inner_rejection(f"unexpected {kind} directory", kind,
                        lambda data: changed_zip(data, extra_dir="surprise/"),
                        "directory")
    for kind, name in (("nupkg", "README.md"), ("door", "thinkthen/Door.class")):
        inner_rejection(f"symlink typed {kind} {name}", kind,
                        lambda data, name=name: changed_zip(data, symlink=name),
                        "unsafe")
    gate_case("missing selected family", lambda out: (out / f"thinkthen-jvm-{VERSION}-{TARGET}.tar.gz").unlink())
    gate_case("extra selected suffix", lambda out: (out / "thinkthen-csharp-extra").write_bytes(b"extra"))
    gate_case("wrong selected target", lambda out: (out / f"thinkthen-jvm-{VERSION}-aarch64-unknown-linux-gnu.tar.gz").write_bytes(b"other"))
    gate_case("linked selected entry", lambda out: (out / "thinkthen-csharp-linked").symlink_to("missing"))
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
