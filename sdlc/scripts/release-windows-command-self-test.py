#!/usr/bin/env python3
"""Prove Windows command packing and malformed archive refusals without a build."""

import hashlib
import os
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import zipfile


REPO = Path(__file__).resolve().parents[2]
TARGET = "x86_64-pc-windows-msvc"
SCRIPT = REPO / "sdlc/scripts/release-windows-command.py"


def run(*args, env=None):
    return subprocess.run(args, capture_output=True, text=True, env=env)


def expect(result, code=0, text=""):
    if result.returncode != code or text not in result.stderr:
        raise AssertionError((result.args, result.returncode, result.stdout, result.stderr))


def pe(machine=0x8664, flags=2, magic=0x20B):
    data = bytearray(128)
    data[:2] = b"MZ"
    struct.pack_into("<I", data, 60, 64)
    data[64:68] = b"PE\0\0"
    struct.pack_into("<H", data, 68, machine)
    struct.pack_into("<H", data, 86, flags)
    struct.pack_into("<H", data, 88, magic)
    return bytes(data)


def checksum(path):
    path.with_name(path.name + ".sha256").write_text(
        f"{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n")


def main():
    with tempfile.TemporaryDirectory(prefix="thinkthen-windows-pack-") as temporary:
        base = Path(temporary)
        binary = base / "thinkthen.exe"
        archive = base / f"thinkthen-0.2.0-{TARGET}.zip"
        binary.write_bytes(pe())
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)))
        first = archive.read_bytes()
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)))
        assert first == archive.read_bytes(), "ZIP is not deterministic"
        checksum(archive)
        expect(run("python3", str(SCRIPT), "check", str(archive)))
        with zipfile.ZipFile(archive) as source:
            assert source.namelist() == ["thinkthen.exe"]
            assert source.read("thinkthen.exe") == binary.read_bytes()
        for data, sentence in ((b"not PE", "not a Windows executable"),
                               (pe(machine=0xAA64), "PE32+ x86-64"),
                               (pe(flags=0x2002), "PE32+ x86-64"),
                               (pe(magic=0x10B), "PE32+ x86-64")):
            binary.write_bytes(data)
            expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)), 1, sentence)
            assert archive.read_bytes() == first, "invalid binary overwrote the archive"
        binary.write_bytes(pe())
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(base / "wrong.zip")),
               1, "version and Windows target")
        archive.write_bytes(b"bad zip")
        checksum(archive)
        expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "not a zip file")
        for members in (("thinkthen.exe", "extra"), ("../thinkthen.exe",),
                        ("thinkthen.exe", "thinkthen.exe"), ()):
            with zipfile.ZipFile(archive, "w") as output:
                for name in members:
                    output.writestr(name, pe())
            checksum(archive)
            expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "exactly thinkthen.exe")
        with zipfile.ZipFile(archive, "w") as output:
            member = zipfile.ZipInfo("thinkthen.exe")
            member.external_attr = 0o120777 << 16
            output.writestr(member, pe())
        checksum(archive)
        expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "linked executable")
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)))
        checksum(archive)
        sidecar = archive.with_name(archive.name + ".sha256")
        sidecar.write_text("0" * 64 + "  " + archive.name + "\n")
        expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "differs from its checksum")
        sidecar.unlink()
        expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "missing or linked")
        # Run the real shell packer against a synthetic Windows host and build output.
        source = base / "source"
        scripts = source / "sdlc/scripts"
        scripts.mkdir(parents=True)
        for name in ("release-pack", "scratch.sh", "release-windows-command.py"):
            shutil.copy2(REPO / "sdlc/scripts" / name, scripts / name)
        crate = source / "crates/thinkthen"
        crate.mkdir(parents=True)
        (crate / "Cargo.toml").write_text('[package]\nversion = "0.2.0"\n')
        tools = base / "tools"
        tools.mkdir()
        (tools / "rustc").write_text(f"#!/bin/sh\nprintf 'host: {TARGET}\\n'\n")
        (tools / "cargo").write_text(
            '#!/bin/sh\nmkdir -p "$CARGO_TARGET_DIR/' + TARGET + '/release"\n'
            'cp "$WINDOWS_TEST_BINARY" "$CARGO_TARGET_DIR/' + TARGET + '/release/thinkthen.exe"\n')
        for tool in tools.iterdir():
            tool.chmod(0o755)
        build = base / "build"
        (build / "debug").mkdir(parents=True)
        shutil.copy2(binary, build / "debug/thinkthen.exe")
        env = os.environ | {"PATH": str(tools) + os.pathsep + os.environ["PATH"],
                            "CARGO_TARGET_DIR": str(build), "WINDOWS_TEST_BINARY": str(binary)}
        for args in (("--reuse",), ()):
            out = base / ("reuse" if args else "release")
            expect(run("sh", str(scripts / "release-pack"), *args, TARGET, str(out), "command", env=env))
            packed = out / archive.name
            assert {path.name for path in out.iterdir()} == {archive.name, sidecar.name}
            assert packed.read_bytes() == first
            expect(run("python3", str(SCRIPT), "check", str(packed)))
        expect(run("sh", str(scripts / "release-pack"), "--reuse", TARGET,
                   str(base / "unsupported"), "python", env=env), 2, "not a Windows stage 1 command part")
    print("Windows command release self-test: passed")


if __name__ == "__main__":
    main()
