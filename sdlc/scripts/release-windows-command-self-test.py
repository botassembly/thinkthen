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
import warnings
import sys
import importlib.util
import contextlib
import io
from unittest import mock


REPO = Path(__file__).resolve().parents[2]
C_FIXTURE = __import__('runpy').run_path(str(REPO / 'sdlc/scripts/release-windows-c-fixture.py'))
TARGET = "x86_64-pc-windows-msvc"
SCRIPT = REPO / "sdlc/scripts/release-windows-command.py"
VERSION = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
               if line.startswith('version = "'))


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


def host_setup(base):
    """The native Windows setup fetches the core locks and pinned wheel builder."""
    tools = base / "host-tools"
    tools.mkdir()
    bodies = {
        "rustc": f'''case $1 in
-vV) printf 'host: {TARGET}\\n' ;;
--version) echo 'rustc 1.95.0 (59807616e 2026-04-14)' ;;
--print) printf '%s/toolchains/1.95.0\\n' "$RUSTUP_HOME" ;;
esac''',
        "rustup": 'echo "rustup $*" >> "$WINDOWS_HOST_LOG"',
        "cargo": 'echo "cargo $*" >> "$WINDOWS_HOST_LOG"',
        "python3": f'''if [ "$1" = -m ]; then
    [ "$*" = "-m pip install --disable-pip-version-check maturin==1.15.0 jsonschema==4.25.1" ] || exit 92
    echo "python3 $*" >> "$WINDOWS_HOST_LOG"
    exit 0
fi
[ "$1" = - ] || exit 92
exec '{sys.executable}' "$@"''',
    }
    for tool, body in bodies.items():
        (tools / tool).write_text("#!/bin/sh\n" + body + "\n")
        (tools / tool).chmod(0o755)
    log = base / "host-calls"
    env = os.environ | {"PATH": str(tools) + os.pathsep + os.environ["PATH"],
                        "RUSTUP_HOME": str(base / "rustup"), "WINDOWS_HOST_LOG": str(log)}
    expect(run("sh", str(REPO / "sdlc/scripts/release-workflow"), "host-setup", TARGET, env=env))
    assert log.read_text().splitlines() == [
        "rustup toolchain install 1.95.0 --profile minimal --component clippy --component rustfmt",
        "cargo fetch --locked --manifest-path Cargo.toml",
        "cargo fetch --locked --manifest-path libraries/c/Cargo.toml",
        "cargo fetch --locked --manifest-path libraries/python/Cargo.toml",
        "python3 -m pip install --disable-pip-version-check maturin==1.15.0 jsonschema==4.25.1"]


def interruption_routing(smoke, base):
    """The hook passes the supplied path exactly and selects only its ignored fixture."""
    binary = base / "packed command" / "thinkthen.exe"
    binary.parent.mkdir()
    binary.write_bytes(pe())
    calls = []
    original = smoke.run
    try:
        smoke.run = lambda args, env, **options: calls.append((args, env, options))
        with contextlib.redirect_stdout(io.StringIO()):
            smoke.interrupt(binary, {"CARGO_NET_OFFLINE": "true"})
        assert calls == [(["cargo", "test", "--locked", "--offline", "-p", "thinkthen", "--test", "windows",
                          "interrupt::release_binary_console_interrupt", "--", "--exact", "--ignored"],
                          {"CARGO_NET_OFFLINE": "true", "THINKTHEN_WINDOWS_RELEASE_BINARY": str(binary.absolute())},
                          {"timeout": 600})]
        try:
            smoke.interrupt(base / "absent.exe", {})
        except RuntimeError as error:
            assert "regular exact executable" in str(error)
        else:
            raise AssertionError("absent release executable reached cargo")
        assert len(calls) == 1
    finally:
        smoke.run = original


def python_host_environment(smoke, base):
    """Copied Windows environment keys still reach the isolated wheel child."""
    for system, drive in (("SystemRoot", "SystemDrive"), ("SYSTEMROOT", "SYSTEMDRIVE")):
        expected = {"PATH": "owned-tools", system: "owned-system", drive: "C:",
                    "TEMP": str(base), "TMP": str(base), "HOME": str(base / "home"),
                    "APPDATA": str(base / "Roaming"), "LOCALAPPDATA": str(base / "Local")}
        supplied = expected | {"THINKTHEN_API_KEY": "ambient-fake-key", "HTTP_PROXY": "ambient-proxy"}
        stop = RuntimeError("stop before creating fixture virtual environment")
        with mock.patch.object(smoke, "run", side_effect=stop) as child:
            try:
                smoke.python_smoke(base, supplied, VERSION, base)
            except RuntimeError as error:
                assert error is stop
            else:
                raise AssertionError("virtual environment creation did not reach the child boundary")
        child.assert_called_once_with([sys.executable, "-I", "-m", "venv", str(base / "python")], expected)


def main():
    # The command smoke must send the recording's bytes through a Windows pipe unchanged.
    spec = importlib.util.spec_from_file_location("windows_smoke", REPO / "sdlc/scripts/release-windows-smoke.py")
    smoke = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(smoke)
    evidence = b"first\nsecond\r\nlast"
    smoke.run([sys.executable, "-c", "import sys; print(sys.stdin.buffer.read().hex())"],
              os.environ.copy(), text=evidence, output=evidence.hex())
    with tempfile.TemporaryDirectory(prefix="thinkthen-windows-pack-") as temporary:
        base = Path(temporary)
        python_host_environment(smoke, base)
        interruption_routing(smoke, base)
        host_setup(base)
        binary = base / "thinkthen.exe"
        archive = base / f"thinkthen-{VERSION}-{TARGET}.zip"
        binary.write_bytes(pe())
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)))
        first = archive.read_bytes()
        expect(run("python3", str(SCRIPT), "pack", str(binary), str(archive)))
        assert first == archive.read_bytes(), "ZIP is not deterministic"
        checksum(archive)
        archive.with_name(archive.name + ".sha256").write_bytes(
            f"{hashlib.sha256(archive.read_bytes()).hexdigest()}  {archive.name}\r\n".encode("ascii"))
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
            with warnings.catch_warnings(), zipfile.ZipFile(archive, "w") as output:
                warnings.simplefilter("ignore", UserWarning)
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
        for value in ("0" * 64 + "  " + archive.name,
                      hashlib.sha256(archive.read_bytes()).hexdigest() + "  other.zip"):
            sidecar.write_text(value + "\n")
            expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "differs from its checksum")
        sidecar.unlink()
        expect(run("python3", str(SCRIPT), "check", str(archive)), 1, "missing or linked")
        # Run the real shell packer against a synthetic Windows host and build output.
        source = base / "source"
        scripts = source / "sdlc/scripts"
        scripts.mkdir(parents=True)
        for name in ("release-pack", "scratch.sh", "release-windows-command.py", "release-windows-c.py",
                     "release-bounded.py", "release-owned-job.py"):
            shutil.copy2(REPO / "sdlc/scripts" / name, scripts / name)
        crate = source / "crates/thinkthen"
        crate.mkdir(parents=True)
        (crate / "Cargo.toml").write_text(f'[package]\nversion = "{VERSION}"\n')
        tools = base / "tools"
        tools.mkdir()
        (tools / "rustc").write_text(f"#!/bin/sh\nprintf 'host: {TARGET}\\n'\n")
        (tools / "cargo").write_text(
            '#!/bin/sh\nmkdir -p "$CARGO_TARGET_DIR/' + TARGET + '/release"\n'
            'cp "$WINDOWS_TEST_BINARY" "$CARGO_TARGET_DIR/' + TARGET + '/release/thinkthen.exe"\n')
        (tools / "cygpath").write_text('#!/bin/sh\nprintf "%s\\n" "$2"\n')
        # Reproduce the Windows default binary marker without changing bytes.
        (tools / "sha256sum").write_text(
            '#!/usr/bin/env python3\nimport hashlib, pathlib, sys\n'
            'for name in sys.argv[1:]:\n'
            ' print(f"{hashlib.sha256(pathlib.Path(name).read_bytes()).hexdigest()} *{name}")\n')
        # Native Python translates text stdout to CRLF, including redirected sidecars.
        (tools / "python3").write_text(
            f"#!{sys.executable}\nimport io, os, sys\n"
            'if sys.argv[1] == "-":\n'
            r' sys.stdout = io.TextIOWrapper(sys.stdout.buffer, newline="\r\n")' + '\n'
            ' sys.argv = sys.argv[1:]\n'
            ' exec(compile(sys.stdin.read(), "<stdin>", "exec"))\n'
            'else:\n'
            ' os.execv(sys.executable, [sys.executable, *sys.argv[1:]])\n')
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
            assert packed.with_name(packed.name + ".sha256").read_bytes() == (
                f"{hashlib.sha256(first).hexdigest()}  {packed.name}\n".encode("ascii"))
        wheels = source / "libraries/python/target/release-wheel"
        wheels.mkdir(parents=True)
        wheel = wheels / f"thinkthen-{VERSION}-cp310-abi3-win_amd64.whl"
        wheel.write_bytes(b"wheel fixture\r\n")
        packed_wheels = base / "packed-wheels" / ("platform-" + TARGET)
        expect(run("sh", str(scripts / "release-pack"), "--reuse", TARGET, str(packed_wheels), "python", env=env))
        assert (packed_wheels / (wheel.name + ".sha256")).read_bytes() == (
            f"{hashlib.sha256(wheel.read_bytes()).hexdigest()}  {wheel.name}\n".encode("ascii"))
        expect(run("sh", str(REPO / "sdlc/scripts/release-workflow"), "verify-family",
                   str(packed_wheels.parent), "thinkthen-*.whl", "1"))
        # The default runner path uses a scratch build instead of a caller's target directory.
        scratch = base / "scratch"
        scratch.mkdir()
        own_env = {key: value for key, value in env.items() if key != "CARGO_TARGET_DIR"}
        own_env["TMPDIR"] = str(scratch)
        own = base / "own-build"
        expect(run("sh", str(scripts / "release-pack"), TARGET, str(own), "command", env=own_env))
        expect(run("python3", str(SCRIPT), "check", str(own / archive.name)))
        assert not list(scratch.iterdir()), "scratch Windows build was retained"
        # Model two spellings of a Windows build folder. The native spelling alone has the binary.
        native = base / "native-build"
        (native / "debug").mkdir(parents=True)
        shutil.copy2(binary, native / "debug/thinkthen.exe")
        (build / "debug/thinkthen.exe").unlink()
        (tools / "cygpath").write_text(
            '#!/bin/sh\ncase "$2" in "$WINDOWS_POSIX_BUILD") printf "%s\\n" "$WINDOWS_NATIVE_BUILD" ;; '
            '*) printf "%s\\n" "$2" ;; esac\n')
        mapped_env = env | {"WINDOWS_POSIX_BUILD": str(build), "WINDOWS_NATIVE_BUILD": str(native)}
        mapped = base / "native-output"
        expect(run("sh", str(scripts / "release-pack"), "--reuse", TARGET, str(mapped), "command", env=mapped_env))
        expect(run("python3", str(SCRIPT), "check", str(mapped / archive.name)))
        expect(run("sh", str(scripts / "release-pack"), "--reuse", TARGET,
                   str(base / "unsupported"), "typescript", env=env), 2, "not a Windows stage 1 command part")
        # Five-target collection includes Windows and refuses bad Windows assets before copying.
        platforms = base / "platforms"
        for target in ("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu",
                       "x86_64-apple-darwin", "aarch64-apple-darwin"):
            folder = platforms / f"platform-{target}"
            folder.mkdir(parents=True)
            (folder / f"fixture-{target}").write_bytes(b"Unix retained")
        windows = platforms / f"platform-{TARGET}"
        shutil.copytree(base / "release", windows)
        c_zip = C_FIXTURE["create"](windows, VERSION)
        sample = windows / "thinkthen-first-run.tar.gz"
        sample.write_bytes(b"sample")
        checksum(sample)
        wheel = windows / f"thinkthen-{VERSION}-cp310-abi3-win_amd64.whl"
        wheel.write_bytes(b"wheel fixture")
        checksum(wheel)
        addon = windows / f"thinkthen-{VERSION}.tgz"
        addon.write_bytes(b"synthetic Windows addon contribution")
        checksum(addon)
        npm = base / "npm"
        npm.mkdir()
        version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                       if line.startswith('version = "'))
        for suffix in ("", ".sha256"):
            (npm / f"thinkthen-{version}.tgz{suffix}").write_text("npm fixture")
        gate = REPO / "sdlc/scripts/release-workflow"
        collected = base / "collected"
        expect(run("sh", str(gate), "collect", str(platforms), str(npm), str(collected)))
        assert {path.name for path in collected.iterdir()} == {
            c_zip.name, c_zip.name + ".sha256", archive.name, archive.name + ".sha256", sample.name, sample.name + ".sha256",
            wheel.name, wheel.name + ".sha256",
            f"thinkthen-{version}.tgz", f"thinkthen-{version}.tgz.sha256",
            "fixture-x86_64-unknown-linux-gnu", "fixture-aarch64-unknown-linux-gnu",
            "fixture-x86_64-apple-darwin", "fixture-aarch64-apple-darwin"}
        def malformed_c():
            c_zip.write_bytes(b"not ZIP")
            checksum(c_zip)
        def wrong_header():
            with zipfile.ZipFile(c_zip) as source:
                files = {name: source.read(name) for name in source.namelist()}
            files['include/thinkthen.h'] = b'wrong header'
            with zipfile.ZipFile(c_zip, 'w') as out:
                for name, data in files.items():
                    member = zipfile.ZipInfo(name)
                    member.external_attr = 0o100644 << 16
                    out.writestr(member, data)
            checksum(c_zip)
        mutations = (("missing addon checksum", lambda: addon.with_name(addon.name + ".sha256").unlink(), "npm addon contribution or checksum is missing"),
                     ("bad addon checksum", lambda: addon.with_name(addon.name + ".sha256").write_text("bad"), "npm addon contribution differs"),
                     ("missing C ZIP", lambda: c_zip.unlink(), "exactly the command ZIP"),
                     ("missing C checksum", lambda: c_zip.with_name(c_zip.name + ".sha256").unlink(), "exactly the command ZIP"),
                     ("bad C checksum", lambda: c_zip.with_name(c_zip.name + ".sha256").write_text("bad"), "C ZIP differs"),
                     ("malformed C", malformed_c, "not a zip file"),
                     ("wrong header", wrong_header, "header differs"),
                     ("missing ZIP", lambda: (windows / archive.name).unlink(),
                      "exactly the command ZIP"),
                     ("extra stage 2 binding", lambda: (windows / "thinkthen-c-extra.tar.gz").write_bytes(b"C"),
                      "exactly the command ZIP"),
                     ("bad checksum", lambda: (windows / (archive.name + ".sha256")).write_text("bad"),
                      "differs from its checksum"),
                     ("malformed ZIP", lambda: (windows / archive.name).write_bytes(b"not ZIP"),
                      "differs from its checksum"))
        for index, (name, mutate, sentence) in enumerate(mutations):
            originals = {path.name: path.read_bytes() for path in windows.iterdir()}
            mutate()
            refused = base / f"refused-{index}"
            expect(run("sh", str(gate), "collect", str(platforms), str(npm), str(refused)), 1, sentence)
            assert not refused.exists(), f"{name} created output before verification"
            for path in windows.iterdir():
                path.unlink()
            for name, data in originals.items():
                (windows / name).write_bytes(data)
        missing = platforms / "absent"
        windows.rename(missing)
        expect(run("sh", str(gate), "collect", str(platforms), str(npm), str(base / "absent-output")),
               1, "unexpected platform folder absent")
    print("Windows command release self-test: passed")


if __name__ == "__main__":
    main()
