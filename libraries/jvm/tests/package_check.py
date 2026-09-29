"""Validate local JAR contents against this build and plant stale and private members."""
import hashlib
import io
import json
from pathlib import Path
import zipfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "target"
BAD = (b"tt-canary-289", b"/home/ian", b"auth.json", b"-----BEGIN PRIVATE KEY-----")


def inspect(name, source):
    with zipfile.ZipFile(io.BytesIO(source)) as bundle:
        members = {item.filename: bundle.read(item) for item in bundle.infolist() if not item.is_dir()}
    assert members, f"empty {name} JAR"
    expected = {path.relative_to(TARGET / "classes" / name).as_posix(): path.read_bytes()
                for path in (TARGET / "classes" / name).rglob("*") if path.is_file()}
    assert members.pop("META-INF/MANIFEST.MF").startswith(b"Manifest-Version: 1.0"), "bad JAR manifest"
    assert members == expected, f"stale or extra {name} JAR member"
    assert not any("ProbeDoor" in member or "TypeCase" in member for member in members), "diagnostic class escaped product JAR"
    assert not any(token in data for member, content in members.items() for data in (member.encode(), content) for token in BAD), "private byte in JAR"
    return hashlib.sha256(source).hexdigest()


def reject(label, action):
    try:
        action()
    except AssertionError:
        print(f"planted {label}: rejected")
    else:
        raise AssertionError(f"planted {label}: accepted")


pom = ET.fromstring((ROOT / "pom.xml").read_text())
metadata = {child.tag.rsplit("}", 1)[-1]: (child.text or "").strip() for child in pom}
assert metadata["groupId"] == "io.github.botassembly" and metadata["artifactId"] == "thinkthen-jvm" and metadata["version"] == "0.0.1"
receipt = {}
for name in ("door", "kotlin", "scala"):
    jar = TARGET / "jars" / f"thinkthen-{name}.jar"
    source = jar.read_bytes()
    receipt[name] = inspect(name, source)
    altered = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(source)) as original, zipfile.ZipFile(altered, "w") as copy:
        for item in original.infolist():
            data = original.read(item)
            copy.writestr(item, data + b"stale" if item.filename.endswith(".class") and item.filename == next(k for k in original.namelist() if k.endswith(".class")) else data)
    reject(f"stale-{name}-class", lambda: inspect(name, altered.getvalue()))
    secret = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(source)) as original, zipfile.ZipFile(secret, "w") as copy:
        for item in original.infolist():
            copy.writestr(item, original.read(item))
        copy.writestr("private.txt", b"tt-canary-289")
    reject(f"compressed-private-{name}", lambda: inspect(name, secret.getvalue()))
(TARGET / "jars/manifest.json").write_text(json.dumps(receipt, indent=2) + "\n")
print("JVM JARs: exact compiled members, metadata, no diagnostic exports and planted negatives PASS")
