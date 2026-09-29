"""Check local C# and native package bytes and plant stale, tampered and secret variants."""
import hashlib
import io
import json
from pathlib import Path
import re
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
HEADER = ROOT.parents[1] / "libraries/c/include/thinkthen.h"
NATIVE = ROOT.parents[1] / "libraries/c/target/debug/libthinkthen_c.so"
NUPKG = ROOT / "target/scratch/managed/Botassembly.ThinkThen.0.0.1.nupkg"
ARCHIVE = ROOT / "target/artifacts/thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz"
BAD = (b"tt-canary-290", b"/home/ian", b"auth.json", b"-----BEGIN PRIVATE KEY-----")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def safe(label, data):
    assert not any(token in data for token in BAD), f"private pattern in {label}"


def inspect(header, package, native, expected=None):
    version = re.findall(r"(?m)^#define THINKTHEN_VERSION_(MAJOR|MINOR|PATCH)\s+(\d+)\s*$", header)
    assert version == [("MAJOR", "0"), ("MINOR", "0"), ("PATCH", "1")], "header version mismatch"
    actual = {"header": digest(header.encode()), "package": digest(package), "native": digest(native)}
    if expected is not None:
        assert actual == expected, "artifact SHA-256 mismatch"
    with zipfile.ZipFile(io.BytesIO(package)) as bundle:
        members = set(bundle.namelist())
        assert {"Botassembly.ThinkThen.nuspec", "lib/net8.0/ThinkThen.dll", "README.md", "LICENSE"} <= members
        assert not any(name.endswith((".so", ".dylib")) for name in members)
        assert b"<id>Botassembly.ThinkThen</id>" in bundle.read("Botassembly.ThinkThen.nuspec")
        assert bundle.read("README.md") == (ROOT / "README.md").read_bytes(), "stale package README"
        for name in members:
            safe(name, name.encode() + bundle.read(name))
    with tarfile.open(fileobj=io.BytesIO(native), mode="r:gz") as bundle:
        files = {member.name: member for member in bundle}
        assert "./lib/libthinkthen.so" in files and "./lib/libthinkthen.so.0" in files
        assert files["./lib/libthinkthen.so.0"].issym()
        assert bundle.extractfile(files["./lib/libthinkthen.so"]).read() == NATIVE.read_bytes(), "tampered native member"
    return actual


def fail(label, action):
    try:
        action()
    except AssertionError:
        print(f"planted {label}: rejected")
    else:
        raise AssertionError(f"planted {label}: accepted")


def rewrite_zip(package, name, suffix):
    result = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(package)) as source, zipfile.ZipFile(result, "w") as altered:
        for member in source.infolist():
            data = source.read(member)
            altered.writestr(member, data + suffix if member.filename == name else data)
    return result.getvalue()


def rewrite_tar(native):
    result = io.BytesIO()
    with tarfile.open(fileobj=io.BytesIO(native), mode="r:gz") as source, tarfile.open(fileobj=result, mode="w:gz") as altered:
        for member in source:
            if member.isfile():
                data = source.extractfile(member).read()
                if member.name == "./lib/libthinkthen.so":
                    data += b"tampered"
                    member.size = len(data)
                altered.addfile(member, io.BytesIO(data))
            else:
                altered.addfile(member)
    return result.getvalue()


header, package, native = HEADER.read_text(), NUPKG.read_bytes(), ARCHIVE.read_bytes()
receipt = inspect(header, package, native)
(ROOT / "target/artifacts/manifest.json").write_text(json.dumps(receipt, indent=2) + "\n")
fail("header-version", lambda: inspect(header.replace("#define THINKTHEN_VERSION_PATCH 1", "#define THINKTHEN_VERSION_PATCH 2"), package, native))
stale = rewrite_zip(package, "README.md", b"\nstale\n")
fail("stale-package-member", lambda: inspect(header, stale, native))
fail("stale-package-hash", lambda: inspect(header, stale, native, receipt))
tampered = rewrite_tar(native)
fail("tampered-native-member", lambda: inspect(header, package, tampered))
fail("tampered-native-hash", lambda: inspect(header, package, tampered, receipt))
fail("private-byte", lambda: safe("plant", b"/home/ian/private"))
secret_zip = rewrite_zip(package, "README.md", b"tt-canary-290")
fail("compressed-private-byte", lambda: inspect(header, secret_zip, native))
print("C# local packages: source members, hash receipt, header version, managed secrecy and planted negatives PASS")
