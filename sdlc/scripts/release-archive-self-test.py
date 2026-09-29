#!/usr/bin/env python3
"""Focused gitless release input proof. Cargo and Docker only stand in for builds."""

import os
from pathlib import Path
import shutil
import subprocess
import io
import tarfile
import tempfile


REPO = Path(__file__).resolve().parents[2]


def run(*args, cwd=REPO, env=None):
    return subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)


def expect(result, text, success=False):
    if (result.returncode == 0) != success or (text and text not in result.stderr + result.stdout):
        raise AssertionError((result.args, result.returncode, result.stdout, result.stderr, text))


def main():
    host = subprocess.check_output(["rustc", "-vV"], text=True).split("host: ", 1)[1].splitlines()[0]
    if host != "x86_64-unknown-linux-gnu":
        print(f"release archive self-test: skipped synthetic C fixture on {host}")
        return
    commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO, text=True).strip()
    version = next(line.split('"')[1] for line in (REPO / "crates/thinkthen/Cargo.toml").read_text().splitlines()
                   if line.startswith('version = "'))
    with tempfile.TemporaryDirectory(prefix="thinkthen-release-archive-") as temporary:
        base = Path(temporary)
        source = base / "source"
        source.mkdir()
        archive = base / "source.tar"
        with archive.open("wb") as output:
            subprocess.run(["git", "archive", "--format=tar", "HEAD"], cwd=REPO, stdout=output, check=True)
        subprocess.run(["tar", "-xf", str(archive), "-C", str(source)], check=True)
        if (source / ".git").exists():
            raise AssertionError("Git archive unexpectedly contains .git")
        # Exercise the working candidate before it is committed; archived product files still
        # come from HEAD and are compared by release-pack to the original tar below.
        shutil.copy2(REPO / "sdlc/scripts/release-pack", source / "sdlc/scripts/release-pack")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "legacy"), "first-run", cwd=source), "", success=True)
        if not (base / "legacy/thinkthen-first-run.tar.gz").is_file():
            raise AssertionError("gitless legacy archive missing")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "unselected"), "go", cwd=source), "archived source tar path must be absolute")

        fake_bin = base / "bin"
        fake_bin.mkdir()
        cargo = fake_bin / "cargo"
        cargo.write_text("#!/bin/sh\nprintf 'called\\n' >>\"$THINKTHEN_CARGO_CALLS\"\nexit 0\n")
        cargo.chmod(0o755)
        native = base / "native-target/release"
        native.mkdir(parents=True)
        (native / "libthinkthen_c.so").write_bytes(b"fixture shared")
        (native / "libthinkthen_c.a").write_bytes(b"fixture static")
        env = os.environ.copy()
        env.update(PATH=str(fake_bin) + os.pathsep + env["PATH"],
                   CARGO_TARGET_DIR=str(native.parent),
                   THINKTHEN_CARGO_CALLS=str(base / "cargo-calls"),
                   THINKTHEN_ARCHIVED_SOURCE_TAR=str(archive),
                   THINKTHEN_ARCHIVED_SOURCE_COMMIT=commit)
        parts = ("c", "go", "cpp")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "paired"), *parts, cwd=source, env=env), "", success=True)
        for kind in parts:
            if len(list((base / "paired").glob(f"thinkthen-{kind}-*.tar.gz"))) != 1:
                raise AssertionError(f"missing {kind} fixture archive")
        gate = str(REPO / "sdlc/scripts/release-workflow")
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), host, commit), "", success=True)
        missing_pair = base / "missing-pair"
        shutil.copytree(base / "paired", missing_pair)
        for file in missing_pair.glob("thinkthen-cpp-*"):
            file.unlink()
        expect(run("sh", gate, "go-cpp-gate", str(missing_pair), host, commit),
               "missing or linked thinkthen-cpp-")
        another_commit = "0" * 40
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), host, another_commit),
               "checkout differs from resolved SHA")
        altered_source = base / "altered-source"
        shutil.copytree(base / "paired", altered_source)
        for family in ("go", "cpp"):
            name = f"thinkthen-{family}-{version}-{host}.tar.gz"
            path = altered_source / name
            repacked = altered_source / f"{family}.tmp"
            with tarfile.open(path, "r:gz") as archive, tarfile.open(repacked, "w:gz") as output:
                for member in archive:
                    if member.name == "./THINKTHEN-PACKAGE-INPUTS":
                        data = archive.extractfile(member).read().replace(commit.encode(), another_commit.encode())
                        member.size = len(data)
                        output.addfile(member, io.BytesIO(data))
                    else:
                        output.addfile(member, archive.extractfile(member) if member.isfile() else None)
            repacked.replace(path)
            digest = subprocess.check_output(["sha256sum", str(path)], text=True).split()[0]
            (altered_source / f"{name}.sha256").write_text(f"{digest}  {name}\n")
        expect(run("sh", gate, "go-cpp-gate", str(altered_source), host, commit),
               "go source differs from resolved SHA")
        other_target = "aarch64-unknown-linux-gnu"
        expect(run("sh", gate, "go-cpp-gate", str(base / "paired"), other_target, commit),
               "unsupported target has thinkthen-go-")
        fake_curl = fake_bin / "curl"
        fake_curl.write_text("#!/bin/sh\nwhile [ $# -gt 0 ]; do\n"
                             "if [ \"$1\" = -o ]; then shift; printf wrong >\"$1\"; exit 0; fi\n"
                             "shift\ndone\nexit 2\n")
        fake_curl.chmod(0o755)
        github_path = base / "github-path"
        github_path.touch()
        tool_env = os.environ | {"PATH": str(fake_bin) + os.pathsep + os.environ["PATH"],
                                 "GITHUB_PATH": str(github_path)}
        expect(run("sh", gate, "go-cpp-tools", other_target, str(base / "other-tools"), env=tool_env),
               "Go/C++ installed tools require Linux x86-64")
        expect(run("sh", gate, "go-cpp-tools", host, str(base / "bad-go"), env=tool_env),
               "Go 1.27.1 archive differs from official checksum")
        if github_path.read_text():
            raise AssertionError("bad Go archive entered the runner path")
        wrong = env | {"THINKTHEN_ARCHIVED_SOURCE_COMMIT": "0" * 40}
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "wrong"), *parts, cwd=source, env=wrong),
               "archived source commit differs from selected checkout")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), "--reuse", host,
                   str(base / "reuse"), *parts, cwd=source, env=env),
               "source wrappers need a fresh C build")
        stale = base / "stale"
        stale.mkdir()
        (stale / f"thinkthen-c-{version}-{host}.tar.gz").write_bytes(b"prior C")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(stale), *parts, cwd=source, env=env),
               "output without an existing C archive or sidecar")
        c_header = source / "libraries/c/include/thinkthen.h"
        original_header = c_header.read_bytes()
        c_header.write_bytes(b"changed native header\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-changed"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.write_bytes(original_header)
        c_header.unlink()
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-missing"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.symlink_to("../../../LICENSE")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "native-type"), *parts, cwd=source, env=env),
               "source file differs: libraries/c/include/thinkthen.h")
        c_header.unlink()
        c_header.write_bytes(original_header)
        source_link = source / "CLAUDE.md"
        original_link = os.readlink(source_link)
        source_link.unlink()
        source_link.symlink_to("README.md")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "source-link"), *parts, cwd=source, env=env),
               "source link differs: CLAUDE.md")
        source_link.unlink()
        source_link.symlink_to(original_link)
        extra_config = source / "libraries/c/.cargo/config.toml"
        extra_config.parent.mkdir()
        extra_config.write_text("[build]\nrustflags = []\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "extra-config"), *parts, cwd=source, env=env),
               "unexpected source path: libraries/c/.cargo/config.toml")
        extra_config.unlink()
        extra_config.parent.rmdir()
        (source / "libraries/go/README.md").write_text("changed source\n")
        expect(run("sh", str(source / "sdlc/scripts/release-pack"), host,
                   str(base / "changed"), *parts, cwd=source, env=env),
               "source file differs: libraries/go/README.md")
        if (base / "cargo-calls").read_text().splitlines() != ["called"]:
            raise AssertionError("an archived-source refusal reached Cargo")
        expect(run("sh", str(REPO / "sdlc/scripts/release-pack"), host,
                   str(base / "override"), "go", env=env),
               "archive identity cannot override a repository checkout")

        # Capture only the launcher's transport inputs. Assert their identity below.
        docker = fake_bin / "docker"
        docker.write_text("#!/bin/sh\n"
                          "for arg do case $arg in *:/work) root=${arg%:/work} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_COMMIT=*) selected=${arg#*=} ;; "
                          "THINKTHEN_ARCHIVED_SOURCE_TAR=*) tar_path=${arg#*=} ;; esac; done\n"
                          "printf '%s\\n' \"$selected\" >\"$root/out/selected-commit\"\n"
                          "printf '%s\\n' \"$tar_path\" >\"$root/out/selected-tar\"\n"
                          "cp \"$root/source.tar\" \"$root/out/source.tar\"\n")
        docker.chmod(0o755)
        launcher_env = os.environ.copy()
        launcher_env["PATH"] = str(fake_bin) + os.pathsep + launcher_env["PATH"]
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = "0" * 40
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "checkout differs from resolved release SHA")
        launcher_env["THINKTHEN_RELEASE_EXPECTED_SHA"] = commit
        expect(run("sh", str(REPO / "sdlc/scripts/release-container"), str(base / "container-work"),
                   str(base / "container-out"), env=launcher_env), "", success=True)
        output = base / "container-out"
        if (output / "selected-commit").read_text().strip() != commit:
            raise AssertionError("launcher passed another selected commit")
        if (output / "selected-tar").read_text().strip() != "/work/source.tar":
            raise AssertionError("launcher passed another tar path")
        with (output / "source.tar").open("rb") as packed:
            actual = subprocess.check_output(["git", "get-tar-commit-id"], stdin=packed, text=True).strip()
        if actual != commit:
            raise AssertionError("launcher passed a tar from another source commit")
        fake_go = fake_bin / "fake-go"
        fake_go.write_text("#!/bin/sh\nprintf '%s\\n' \"$FAKE_GO_VERSION\"\n")
        fake_go.chmod(0o755)
        no_python = fake_bin / "no-python"
        no_python.write_text("#!/bin/sh\nexit 1\n")
        no_python.chmod(0o755)
        for version, diagnostic in (
            ("go version go1.21.9 linux/amd64", "Go 1.22 or newer is required"),
            ("go version go1.27rc1 linux/amd64", "a stable Go 1.x release is required"),
            ("go version go2.0.0 linux/amd64", "a stable Go 1.x release is required"),
            ("go version go1.22.12 linux/amd64", "Python jsonschema is unavailable"),
            ("go version go1.27.1 linux/amd64", "Python jsonschema is unavailable"),
        ):
            version_env = os.environ | {"THINKTHEN_GO_BIN": str(fake_go),
                                        "THINKTHEN_PYTHON_BIN": str(no_python),
                                        "FAKE_GO_VERSION": version}
            expect(run("sh", str(REPO / "libraries/go/check.sh"), "0", env=version_env), diagnostic)
        installed_env = os.environ | {"THINKTHEN_GO_BIN": str(fake_go),
                                      "THINKTHEN_PYTHON_BIN": str(no_python),
                                      "FAKE_GO_VERSION": "go version go1.27.1 linux/amd64",
                                      "THINKTHEN_ARTIFACT": str(base / "absent-wrapper"),
                                      "THINKTHEN_HEAVY_LOCK": str(base / "held-lock"),
                                      "THINKTHEN_HEAVY_LOCK_HELD": str(base / "held-lock")}
        for family in ("go", "cpp"):
            expect(run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=installed_env),
                   f"{family}: missing C archive")
            source_env = installed_env | {"THINKTHEN_ARTIFACT": ""}
            expect(run("sh", str(REPO / f"libraries/{family}/check.sh"), "0", env=source_env),
                   f"{family}: not run: Python jsonschema is unavailable")
    print("release archive self-test: gitless legacy and controlled Go/C++ inputs pass")


if __name__ == "__main__":
    main()
